# Basic Proof of Work
*[`nodes/basic-pow`](https://github.com/substrate-developer-hub/recipes/tree/master/nodes/basic-pow)*

The `basic-pow` node uses a minimal [Proof of Work](https://en.wikipedia.org/wiki/Proof_of_work) consensus engine to reach agreement over the blockchain. This node is kept intentionally simple. It omits some features that make Proof of Work practical for real-world use such as difficulty adjustment and block rewards. Nonetheless, it is a real usable consensus engine that will teach us many useful aspects of dealing with consensus and prepare us to understand more advanced consensus engines in the future. In particular we will learn about:
* Substrate's [`BlockImport` trait](https://substrate.dev/rustdocs/master/sp_consensus/block_import/trait.BlockImport.html)
* Substrate's [import pipeline](https://substrate.dev/rustdocs/master/sp_consensus/import_queue/index.html)
* Structure of a typical [Substrate Service](https://substrate.dev/rustdocs/master/sc_service/index.html)
* Configuring [`InherentDataProvider`](https://substrate.dev/rustdocs/master/sp_authorship/struct.InherentDataProvider.html)s

## The Structure of a Node

You may remember from the [hello-substrate recipe](../2-appetizers/1-hello-substrate.md) that a Substrate node has two parts. An outer part that is responsible for gossiping transactions and blocks, handling [rpc requests](./custom-rpc.md), and reaching consensus. And a runtime that is responsible for the business logic of the chain. This architecture diagram illustrates the distinction.

![Substrate Architecture Diagram](../img/substrate-architecture.png)

In principle the consensus engine, part of the outer node, is agnostic over the runtime that is used with it. But in practice, most consensus engines will require the runtime to provide certain [runtime APIs](./runtime-api.md) that effect the engine. For example Aura, and Babe, query the runtime for the set of validators. A more real-world PoW consensus would query the runtime for the bock difficulty. Additionally, some runtimes, rely on the consensus engine to provide [PreRuntime Digests](https://substrate.dev/rustdocs/v2.0.0-alpha.3/sp_runtime/generic/enum.DigestItem.html#variant.PreRuntime). For example Runtimes that include the Babe pallet, expect a preruntime digest containing information about the current babe slot. Because of these requirements, this node will use a dedicated `pow-runtime`. The contents of that runtime should be familiar, and will not be discussed here.


## Proof of Work Algorithms

Proof of work is not a single consensus algorithm. Rather it is a class of Algorithms represented by the [`PowAlgorithm` trait](https://substrate.dev/rustdocs/master/sc_consensus_pow/trait.PowAlgorithm.html). Before we can build a PoW node we must specify a concrete PoW algorithm by implementing this trait. We specify our algorithm in the `pow.rs` file.

```rust, ignore
/// A concrete PoW Algorithm that uses Sha3 hashing.
#[derive(Clone)]
pub struct Sha3Algorithm;
```

We will use the [sha3 hashing algorithm](https://en.wikipedia.org/wiki/SHA-3) which we have indicated in the name of our struct. Because this is a _minimal_ PoW algorithm, our struct can also be quite simple. In fact, it is a [unit struct](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html). A more complex PoW algorithm that interfaces with the runtime, would need to hold a reference to the client. An example of this (on an older Substrate codebase) can be seen in [Kulupu](https://github.com/kulupu/kulupu/)'s [RandomXAlgorithm](https://github.com/kulupu/kulupu/blob/3500b7f62fdf90be7608b2d813735a063ad1c458/pow/src/lib.rs#L137-L145).

### Difficulty

The first fucntion we must provide returns the difficults of the next block to be mined. In our basic PoW, this function is quite simple. The difficulty is fixed. This means that as more mining power joins the network, te block time will become faster.

```rust, ignore
impl<B: BlockT<Hash=H256>> PowAlgorithm<B> for Sha3Algorithm {
	type Difficulty = U256;

	fn difficulty(&self, _parent: &BlockId<B>) -> Result<Self::Difficulty, Error<B>> {
		// This basic PoW uses a fixed difficulty.
		// Raising this difficulty will make the block time slower.
		Ok(U256::from(1000_000))
	}

	// --snip--
}
```

### Verification

Our PoW algorithm must also be able to verify blocks provided by other authors. We are first given the pre-hash, which is a hash of the block before the proof of work seal is attached. We are also given the seal, which testifies that the work has been done, and the difficulty that the block author needed to meet. This function first confirms that the provided seal actually meets the target difficulty, then it confirms that the seal is actually valid for the given pre-hash.

```rust, ignore
fn verify(
	&self,
	_parent: &BlockId<B>,
	pre_hash: &H256,
	seal: &RawSeal,
	difficulty: Self::Difficulty
) -> Result<bool, Error<B>> {
	// Try to construct a seal object by decoding the raw seal given
	let seal = match Seal::decode(&mut &seal[..]) {
		Ok(seal) => seal,
		Err(_) => return Ok(false),
	};

	// See whether the hash meets the difficulty requirement. If not, fail fast.
	if !hash_meets_difficulty(&seal.work, difficulty) {
		return Ok(false)
	}

	// Make sure the provided work actually comes from the correct pre_hash
	let compute = Compute {
		difficulty,
		pre_hash: *pre_hash,
		nonce: seal.nonce,
	};

	if compute.compute() != seal {
		return Ok(false)
	}

	Ok(true)
}
```

### Mining

Finally our proof of work algorithm needs to be able to mine blocks of our own.

```rust, ignore
fn mine(
	&self,
	_parent: &BlockId<B>,
	pre_hash: &H256,
	difficulty: Self::Difficulty,
	round: u32 // The number of nonces to try during this call
) -> Result<Option<RawSeal>, Error<B>> {
	// Get a randomness source from the environment; fail if one isn't available
	let mut rng = SmallRng::from_rng(&mut thread_rng())
		.map_err(|e| Error::Environment(format!("Initialize RNG failed for mining: {:?}", e)))?;

	// Loop the specified number of times
	for _ in 0..round {

		// Choose a new nonce
		let nonce = H256::random_using(&mut rng);

		// Calculate the seal
		let compute = Compute {
			difficulty,
			pre_hash: *pre_hash,
			nonce,
		};
		let seal = compute.compute();

		// If we solved the PoW then return, otherwise loop again
		if hash_meets_difficulty(&seal.work, difficulty) {
			return Ok(Some(seal.encode()))
		}
	}

	// Tried the specified number of rounds and never found a solution
	Ok(None)
}
```

Notice that this function takes a parameter for the number of rounds of mining it should attempt. If no block has been successfully mined in this time, the method will return. This gives the service a chance to check whether any new blocks have been received from other authors since the mining started. If a valid block has been received, then we will start mining on it. If no such block has been received, we will go in for another try at mining on the same block as before.

## The Service Builder

talk about builder pattern
link to https://substrate.dev/rustdocs/master/sc_service/struct.ServiceBuilder.html

structure of service.rs
	macro
	new full
	new light

## Chain Spec

All of the node's in the recipes have a `chain_spec.rs` file, and they mostly look the same. `basic-pow`'s chain spec will also be familiar, but it is shorter and simpler. There are a few specific differences worth observing.

We don't need the help function
```rust, ignore
/// Taken from the super-runtime chain_spec.rs
/// Helper function to generate session key from seed
pub fn get_authority_keys_from_seed(seed: &str) -> (BabeId, GrandpaId)
```

We don't provide any initial authorities