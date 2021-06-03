# Transactions in Off-chain Workers

`pallets/ocw-demo`
<a href="https://playground.substrate.dev/?deploy=recipes&files=%2Fhome%2Fsubstrate%2Fworkspace%2Fpallets%ocw-demo%2Fsrc%2Flib.rs" target="_blank">![Try on playground](https://img.shields.io/badge/Playground-Try%20it!-brightgreen?logo=Parity%20Substrate)</a>
<a href="https://github.com/substrate-developer-hub/recipes/tree/master/pallets/ocw-demo/src/lib.rs" target="_blank">![View on GitHub](https://img.shields.io/badge/Github-View%20Code-brightgreen?logo=github)</a>

## Compiling this Pallet

This `ocw-demo` pallet is included in the
[ocw-runtime](https://github.com/substrate-developer-hub/recipes/tree/master/runtimes/ocw-runtime).
In order to use this runtime in the kitchen node, we open the `nodes/kitchen-node/Cargo.toml` file,
enable the `ocw-runtime` package and comment out the `super-runtime` package.

Then we build the kitchen node with `ocw` feature flag:

```bash
# Switch to kitchen-node directory
cd nodes/kitchen-node

# Compile with OCW feature
cargo build --release --features ocw
```

With this feature flag, an account key is injected into the Substrate node keystore.

src:
[`nodes/kitchen-node/src/service.rs`](https://github.com/substrate-developer-hub/recipes/blob/master/nodes/kitchen-node/src/service.rs)

```rust
// Initialize seed for signing transaction using off-chain workers
#[cfg(feature = "ocw")]
{
	sp_keystore::SyncCryptoStore::sr25519_generate_new(
		&*keystore,
		runtime::ocw_demo::KEY_TYPE,
		Some("//Alice"),
	)
	.expect("Creating key with account Alice should succeed.");
}
```

## Life-cycle of Off-chain Worker

Running the `kitchen-node` you will see log messages similar to the following:

```
2021-04-09 16:30:21 Running in --dev mode, RPC CORS has been disabled.
2021-04-09 16:30:21 Kitchen Node
2021-04-09 16:30:21 ✌️  version 3.0.0-6a528b4-x86_64-linux-gnu
2021-04-09 16:30:21 ❤️  by Substrate DevHub <https://github.com/substrate-developer-hub>, 2019-2021
2021-04-09 16:30:21 📋 Chain specification: Development
2021-04-09 16:30:21 🏷 Node name: needless-body-2155
2021-04-09 16:30:21 👤 Role: AUTHORITY
2021-04-09 16:30:21 💾 Database: RocksDb at /tmp/substratek7h0lC/chains/dev/db
2021-04-09 16:30:21 ⛓  Native runtime: ocw-runtime-1 (ocw-runtime-1.tx1.au1)
2021-04-09 16:30:21 🔨 Initializing Genesis block/state (state: 0xe76c…ae9b, header-hash: 0x3e88…db95)
2021-04-09 16:30:21 Using default protocol ID "sup" because none is configured in the chain specs
2021-04-09 16:30:21 🏷 Local node identity is: 12D3KooWPwkfdk29ZeqfSF8acAgRR6ToTofjQq11PYhi9WDpQijq
2021-04-09 16:30:22 📦 Highest known block at #0
2021-04-09 16:30:22 〽️ Prometheus server started at 127.0.0.1:9615
2021-04-09 16:30:22 Listening for new connections on 127.0.0.1:9944.
2021-04-09 16:30:27 💤 Idle (0 peers), best: #0 (0x3e88…db95), finalized #0 (0x3e88…db95), ⬇ 0 ⬆ 0
...
```

First, pay attention the line `⛓  Native runtime: ocw-runtime-1 (ocw-runtime-1.tx1.au1)`
to ensure we are running the kitchen-node with the `ocw-runtime`.

Other than that, you will realized the chain is just sitting idled. This is because currently off-chain
worker is only run after a block is imported. Our kitchen node is configured to use
[instant-seal consensus](../kitchen-node.md), meaning that we need to send a transaction to trigger a
block to be imported.

Once a transaction is sent, such as using [Polkadot-JS App](https://polkadot.js.org/apps?rpc=ws://localhost:9944)
to perform a balance transfer, the following more interesting logs are shown.

```
2021-04-09 16:32:13 🙌 Starting consensus session on top of parent 0x3e88096c5794c8a8ba5b81994a5f7b5dcd48c013413afae94c92cd9eb851db95
2021-04-09 16:32:13 🎁 Prepared block for proposing at 1 [hash: 0x2ad95670b92fd9bc46be6e948eae6cbd8e420e61055bc67245c2698669d44508; parent_hash: 0x3e88…db95; extrinsics (2): [0x6e19…1309, 0x8927…b1a3]]
2021-04-09 16:32:13 Instant Seal success: CreatedBlock { hash: 0x2ad95670b92fd9bc46be6e948eae6cbd8e420e61055bc67245c2698669d44508, aux: ImportedAux { header_only: false, clear_justification_requests: false, needs_justification: false, bad_justification: false, is_new_best: true } }
2021-04-09 16:32:13 ✨ Imported #1 (0x2ad9…4508)
2021-04-09 16:32:13 Entering off-chain worker
2021-04-09 16:32:13 🙌 Starting consensus session on top of parent 0x2ad95670b92fd9bc46be6e948eae6cbd8e420e61055bc67245c2698669d44508
2021-04-09 16:32:13 submit_number_unsigned: 1
2021-04-09 16:32:13 Number vector: [1]
...
```

Let's take a deeper look at what's happening here. Referring to the code at
[`pallets/ocw-demo/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/ocw-demo/src/lib.rs),
there is an `fn offchain_worker()` function inside `decl_module!`. This is the entry point of the
off-chain worker logic which is executed once per block import.

As off-chain workers, by definition, run computation off-chain, they cannot alter the block state
directly. In order to do so, they need to send transactions back on-chain. Three kinds of transaction
can be sent here:

- [Signed transactions](#signed-transactions) are used if the transaction requires the sender to be
specified.
- [Unsigned transactions](#unsigned-transactions) are used when the sender does not need to be known.
- [Unsigned transactions with signed payloads](#unsigned-transactions-with-signed-payloads) are used
if the transaction requires the sender to be specified but the sender account not be charged for the transaction fee.

We will walk through each of them in the following.

## Signed Transactions

> **Notes**: This example will have account `Alice` submitting signed transactions to the node in
> the off-chain worker, and these transactions have associated fees. If you run the node in development
> mode (with `--dev` flag) using the default sr25519 crypto signature, `Alice` will have sufficient funds
> initialized in the chain and this example will run fine. Otherwise, please be aware `Alice` account
> must be funded to run this example.

### Setup: Pallet `ocw-demo`

For signed transactions, we have to define a crypto signature sub-module:

src:
[`pallets/ocw-demo/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/ocw-demo/src/lib.rs)

```rust
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"demo");

pub mod crypto {
	use crate::KEY_TYPE;
	use sp_runtime::app_crypto::{app_crypto, sr25519};
	// -- snip --
	app_crypto!(sr25519, KEY_TYPE);
}
```

`KEY_TYPE` is the application key prefix for the pallet in the underlying storage. This is to be used
for signing transactions.

Second, we have our pallet configration trait be additionally bounded by `CreateSignedTransaction`
and add an additional associated type `AuthorityId`. This tell the runtime that this pallet can
create signed transactions.

src:
[`pallets/ocw-demo/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/ocw-demo/src/lib.rs)

```rust
pub trait Config: frame_system::Config + CreateSignedTransaction<Call<Self>> {
	/// The identifier type for an offchain worker.
	type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
	// -- snip --
}
```

### Setup: Runtime `ocw-runtime`

Going back to our runtime `ocw-runtime`, in addition of implementing the pallet
 configuration trait `ocw_demo::Config`, we also implement
 `frame_system::offchain::CreateSignedTransaction`,
`frame_system::offchain::SigningTypes`, and `frame_system::offchain::SendTransactionTypes`.

src:
[`runtimes/ocw-runtime/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/runtimes/ocw-runtime/src/lib.rs)

```rust
pub type SignedPayload = generic::SignedPayload<Call, SignedExtra>;

impl ocw_demo::Config for Runtime {
	type AuthorityId = ocw_demo::crypto::TestAuthId;
	type Call = Call;
	type Event = Event;
}

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Runtime
where
	Call: From<LocalCall>,
{
	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: Call,
		public: <Signature as sp_runtime::traits::Verify>::Signer,
		account: AccountId,
		index: Index,
	) -> Option<(
		Call,
		<UncheckedExtrinsic as sp_runtime::traits::Extrinsic>::SignaturePayload,
	)> {
		let period = BlockHashCount::get() as u64;
		let current_block = System::block_number()
			.saturated_into::<u64>()
			.saturating_sub(1);
		let tip = 0;
		let extra: SignedExtra = (
			frame_system::CheckTxVersion::<Runtime>::new(),
			frame_system::CheckGenesis::<Runtime>::new(),
			frame_system::CheckEra::<Runtime>::from(generic::Era::mortal(period, current_block)),
			frame_system::CheckNonce::<Runtime>::from(index),
			frame_system::CheckWeight::<Runtime>::new(),
			pallet_transaction_payment::ChargeTransactionPayment::<Runtime>::from(tip),
		);

		#[cfg_attr(not(feature = "std"), allow(unused_variables))]
		let raw_payload = SignedPayload::new(call, extra)
			.map_err(|e| {
				debug::native::warn!("SignedPayload error: {:?}", e);
			})
			.ok()?;

		let signature = raw_payload.using_encoded(|payload| C::sign(payload, public))?;

		let address = account;
		let (call, extra, _) = raw_payload.deconstruct();
		Some((call, (address, signature, extra)))
	}
}

// -- snip --
```

Let's focus on the `CreateSignedTransaction` implementation first.
The overall objective here is to perform the following:

- Signing the on-chain `call` and `extra` payload of the call. This together is called the signature.
- Finally returning the on-chain `call`, the account/address making the signature, the signature
itself, and the `extra` payload.

Next, the remaining two traits are also implemented.

src:
[`runtimes/ocw-runtime/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/runtimes/ocw-runtime/src/lib.rs)

```rust
impl frame_system::offchain::SigningTypes for Runtime {
	type Public = <Signature as sp_runtime::traits::Verify>::Signer;
	type Signature = Signature;
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for Runtime
where
	Call: From<C>,
{
	type OverarchingCall = Call;
	type Extrinsic = UncheckedExtrinsic;
}
```

By now, we have completed the setup of implementing the necessary trait for our runtime to
create signed transactions.

### Sending Signed Transactions

A signed transaction is sent with `frame_system::offchain::SendSignedTransaction::send_signed_transaction`,
as shown below:

src:
[`pallets/ocw-demo/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/ocw-demo/src/lib.rs)

```rust
fn offchain_signed_tx(block_number: T::BlockNumber) -> Result<(), Error<T>> {
	// We retrieve a signer and check if it is valid.
	//   Since this pallet only has one key in the keystore. We use `any_account()1 to
	//   retrieve it. If there are multiple keys and we want to pinpoint it, `with_filter()` can be chained,
	//   ref: https://substrate.dev/rustdocs/v3.0.0/frame_system/offchain/struct.Signer.html
	let signer = Signer::<T, T::AuthorityId>::any_account();

	// Translating the current block number to number and submit it on-chain
	let number: u64 = block_number.try_into().unwrap_or(0) as u64;

	// `result` is in the type of `Option<(Account<T>, Result<(), ()>)>`. It is:
	//   - `None`: no account is available for sending transaction
	//   - `Some((account, Err(())))`: error occured when sending the transaction
	//   - `Some((account, Ok(())))`: transaction is successfully sent
	let result = signer.send_signed_transaction(|_acct|
		// This is the on-chain function
		Call::submit_number_signed(number)
	);

	// Display error if the signed tx fails.
	if let Some((acc, res)) = result {
		if res.is_err() {
			debug::error!("failure: offchain_signed_tx: tx sent: {:?}", acc.id);
			return Err(<Error<T>>::OffchainSignedTxError);
		}
		// Transaction is sent successfully
		return Ok(());
	}

	// The case of `None`: no account is available for sending
	debug::error!("No local account available");
	Err(<Error<T>>::NoLocalAcctForSignedTx)
}
```

On the above code, we first retrieve a signer. Then we send a signed transaction on-chain by calling
`send_signed_transaction` with a closure returning the on-chain call,
`Call::submit_number_signed(number)`.

Then we use the signer to send signed transaction, and the result is in the type of
`Option<(Account<T>, Result<(), ()>)>`. So we handle each of the following cases:

- `None`: when no account is available for sending transaction
- `Some((account, Err(())))`: when an error occured when sending the transaction
- `Some((account, Ok(())))`: when transaction is successfully sent

Eventually, the `call` transaction is made on-chain via the
`frame_system::offchain::CreateSignedTransaction::create_transaction()` function we defined in our
runtime.

## Unsigned Transactions

### Setup: Pallet `ocw-demo`

By default unsigned transactions are rejected by the runtime unless they are explicitly
allowed. So we write the logic to validate unsigned transactions:

src:
[`pallets/ocw-demo/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/ocw-demo/src/lib.rs)

```rust
impl<T: Config> frame_support::unsigned::ValidateUnsigned for Module<T> {
	type Call = Call<T>;

	fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
		let valid_tx = |provide| ValidTransaction::with_tag_prefix("ocw-demo")
			.priority(T::UnsignedPriority::get())
			.and_provides([&provide])
			.longevity(3)
			.propagate(true)
			.build();

		match call {
			Call::submit_number_unsigned(_number) => valid_tx(b"submit_number_unsigned".to_vec()),
			// -- snip --
			_ => InvalidTransaction::Call.into(),
		}
	}
}
```

We implement the `ValidateUnsigned` trait for `Module`, and add the allowance logic inside
`validate_unsigned` function. We verify that if the call is `Call::submit_number_unsigned` we return
a [`ValidTransaction`](https://substrate.dev/rustdocs/v3.0.0/sp_runtime/transaction_validity/struct.ValidTransaction.html) object using the [builder pattern](https://rust-unofficial.github.io/patterns/patterns/creational/builder.html).

The `ValidTransaction` object contains certain fields:

- `priority`: determine the ordering of two transactions, given their dependencies are satisfied.
- `provides`: contain a list of tags provided by this transaction. Successfully importing the
	transaction will enable other transactions that depend on these tags be included. Using both `provides`
	and `requires` tags allow Substrate to build a dependency graph of transactions and import them in
	the right order.
- `longevity`: this transaction longevity describes the minimum number of blocks the transaction
	has to be valid for. After this period the transaction should be removed from the pool or revalidated.
- `propagate`: indicate if the transaction should be propagated to other peers. By setting to
	`false` the transaction will still be considered for inclusion in blocks on
	the current node but will never be sent to other peers.

### Setup: Runtime `ocw-runtime`

Finally, to tell the runtime that we have our own `ValidateUnsigned` logic, we need to pass
this as a parameter when constructing the runtime:

src:
[`runtimes/ocw-runtime/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/runtimes/ocw-runtime/src/lib.rs)

```rust
construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		//...snip
		OcwDemo: ocw_demo::{Module, Call, Storage, Event<T>, ValidateUnsigned},
	}
);
```

### Sending Unsigned Transactions

We can now send an unsigned transaction from offchain worker with the
`T::SubmitUnsignedTransaction::submit_unsigned` function, as shown in the code.

src:
[`pallets/ocw-demo/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/ocw-demo/src/lib.rs)

```rust
fn offchain_unsigned_tx(block_number: T::BlockNumber) -> Result<(), Error<T>> {
	let number: u64 = block_number.try_into().unwrap_or(0) as u64;
	let call = Call::submit_number_unsigned(number);

	// `submit_unsigned_transaction` returns a type of `Result<(), ()>`
	//   ref: https://substrate.dev/rustdocs/v3.0.0/frame_system/offchain/struct.SubmitTransaction.html#method.submit_unsigned_transaction
	SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into())
		.map_err(|_| {
			debug::error!("Failed in offchain_unsigned_tx");
			<Error<T>>::OffchainUnsignedTxError
		})
}
```

As in signed transactions, we prepare a function reference with its parameters and call
[`frame_system::offchain::SubmitTransaction::submit_unsigned_transaction`](https://substrate.dev/rustdocs/v3.0.0/frame_system/offchain/struct.SubmitTransaction.html#method.submit_unsigned_transaction).

## Unsigned Transactions with Signed Payloads

With this type of transaction, we first specify a signer, sign the transaction, and then send
it back on-chain as unsigned transaction. The main difference with signed transactions is that the signer
account will not be charged for the transaction fee. This is not the case for signed transaction normally.

But this could potentially be an attack vector, so extra precaution should be added as to what counted
as a valid unsigned transaction.

Since we are still sending unsigned transactions, we need to add extra code in `ValidateUnsigned`
implementation to validate them.

### Sending Unsigned Transactions with Signed Payloads

We send unsigned transactions with signed payloads as followed.

src:
[`pallets/ocw-demo/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/ocw-demo/src/lib.rs)

```rust
fn offchain_unsigned_tx_signed_payload(block_number: T::BlockNumber) -> Result<(), Error<T>> {
	// Retrieve the signer to sign the payload
	let signer = Signer::<T, T::AuthorityId>::any_account();

	let number: u64 = block_number.try_into().unwrap_or(0) as u64;

	// `send_unsigned_transaction` is returning a type of `Option<(Account<T>, Result<(), ()>)>`.
	//   Similar to `send_signed_transaction`, they account for:
	//   - `None`: no account is available for sending transaction
	//   - `Some((account, Ok(())))`: transaction is successfully sent
	//   - `Some((account, Err(())))`: error occured when sending the transaction
	if let Some((_, res)) = signer.send_unsigned_transaction(
		|acct| Payload { number, public: acct.public.clone() },
		Call::submit_number_unsigned_with_signed_payload
	) {
		return res.map_err(|_| {
			debug::error!("Failed in offchain_unsigned_tx_signed_payload");
			<Error<T>>::OffchainUnsignedTxSignedPayloadError
		});
	} else {
		// The case of `None`: no account is available for sending
		debug::error!("No local account available");
		Err(<Error<T>>::NoLocalAcctForSigning)
	}
}
```

What is unique here is that
[`send_unsigned_transaction` function](https://substrate.dev/rustdocs/v3.0.0/frame_system/offchain/trait.SendUnsignedTransaction.html#tymethod.send_unsigned_transaction) takes two functions. The first, expressed as a closure,
returns a `SignedPayload` object, and the second returns an on-chain call to be made.

We have defined our `SignedPayload` object earlier in the pallet.

src:
[`pallets/ocw-demo/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/ocw-demo/src/lib.rs)

```rust
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct Payload<Public> {
	number: u64,
	public: Public
}

impl <T: SigningTypes> SignedPayload<T> for Payload<T::Public> {
	fn public(&self) -> T::Public {
		self.public.clone()
	}
}
```

## Conclusion

By now, you should be able to code your own off-chain workers that send signed transactions, unsigned
transactions, and unsigned transactions with signed payloads back on chain.
