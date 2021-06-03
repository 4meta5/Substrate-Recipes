# Off-chain Indexing

There are times when on-chain extrinsics need to pass data to the off-chain worker context with
predictable write behavior. We can surely pass this piece of data via on-chain storage, but this is
costly and it will make the data propagate among the blockchain network. If this is not a piece of
information that need to be saved on-chain, another way is to save this data in off-chain local
storage via off-chain indexing.

As off-chain indexing is called in on-chain context, **if it is agreed upon by the blockchain
consensus mechanism, then it is expected to run predictably by all nodes in the network**. One use case
is to store only the hash of certain information in on-chain storage for verification purpose but
keeping the full data set off-chain for lookup later. In this case the original data can be saved
via off-chain indexing.

Notice as off-chain indexing is called and data is saved on every block import (this also includes
forks), the consequence is that in case non-unique keys are used the data might be overwritten by different forked blocks and the content of off-chain database will be different between nodes.
Care should be taken in choosing the right indexing `key` to prevent potential overwrites if not
desired.

We will demonstrate this in [`ocw-demo` pallet](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/ocw-demo/src/lib.rs).
Knowledge discussed in this chapter built upon [using local storage in off-chain worker context](./storage.md).

> **Notes**
>
> In order to see the off-chain indexing feature in effect, please run the kitchen node with
> off-chain indexing flag on, as `./target/release/kitchen-node --dev --tmp --enable-offchain-indexing true`

## Writing to Off-chain Storage From On-chain Context

src: [`pallets/ocw-demo/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/ocw-demo/src/lib.rs)

```rust
#[derive(Debug, Deserialize, Encode, Decode, Default)]
struct IndexingData(Vec<u8>, u64);

const ONCHAIN_TX_KEY: &[u8] = b"ocw-demo::storage::tx";

// -- snip --

pub fn submit_number_signed(origin, number: u64) -> DispatchResult {
	// -- snip --
	let key = Self::derived_key(frame_system::Module::<T>::block_number());
	let data = IndexingData(b"submit_number_unsigned".to_vec(), number);
	offchain_index::set(&key, &data.encode());
}

impl<T: Config> Module<T> {
	fn derived_key(block_number: T::BlockNumber) -> Vec<u8> {
		block_number.using_encoded(|encoded_bn| {
			ONCHAIN_TX_KEY.clone().into_iter()
				.chain(b"/".into_iter())
				.chain(encoded_bn)
				.copied()
				.collect::<Vec<u8>>()
		})
	}
}
```

We first define a key used in the local off-chain storage. It is formed in the `derive_key` function
that append an encoded block number to a pre-defined prefix. Then we write to the storage with
`offchain_index::set(key, value)` function. Here `offchain_index::set()` accepts values in byte
format (`&[u8]`) so we encode the data structure `IndexingData` first. If you refer back to
[`offchain_index` API rustdoc](https://substrate.dev/rustdocs/v3.0.0/sp_io/offchain_index/index.html),
you will see there are only `set()` and `clear()` functions. This means from the on-chain context,
we only expect to write to this local off-chain storage location but not reading from it, and we
cannot pass data within on-chain context using this method.

## Reading the Data in Off-chain Context

src: [`pallets/ocw-demo/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/ocw-demo/src/lib.rs)

```rust
fn offchain_worker(block_number: T::BlockNumber) {
	// -- snip --

	// Reading back the off-chain indexing value. It is exactly the same as reading from
	// ocw local storage.
	let key = Self::derived_key(block_number);
	let oci_mem = StorageValueRef::persistent(&key);

	if let Some(Some(data)) = oci_mem.get::<IndexingData>() {
		debug::info!("off-chain indexing data: {:?}, {:?}",
			str::from_utf8(&data.0).unwrap_or("error"), data.1);
	} else {
		debug::info!("no off-chain indexing data retrieved.");
	}

	// -- snip --
}
```

We read the data back in the `offchain_worker()` function as we would normally read from the
local off-chain storage. We first specify the memory space with `StorageValueRef::persistent()` with
its key, and then read back the data with `get` and decode it to `IndexingData`.

## Reference

- [`offchain_index` API rustdoc](https://substrate.dev/rustdocs/v3.0.0/sp_io/offchain_index/index.html)
