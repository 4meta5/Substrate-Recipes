# Basic Proof of Work

`nodes/basic-pow`
<a target="_blank" href="https://playground.substrate.dev/?deploy=recipes&files=%2Fhome%2Fsubstrate%2Fworkspace%2Fnodes%2Fbasic-pow%2Fsrc%2Fservice.rs">
	<img src="https://img.shields.io/badge/Playground-Try%20it!-brightgreen?logo=Parity%20Substrate" alt ="Try on playground"/>
</a>
<a target="_blank" href="https://github.com/substrate-developer-hub/recipes/tree/master/nodes/basic-pow/src/service.rs">
	<img src="https://img.shields.io/badge/Github-View%20Code-brightgreen?logo=github" alt ="View on GitHub"/>
</a>

The `basic-pow` node demonstrates how to wire up a custom consensus engine into the Substrate
Service. It uses a minimal proof of work consensus engine to reach agreement over the blockchain. It
will teach us many useful aspects of dealing with consensus and prepare us to understand more
advanced consensus engines in the future. In particular we will learn about:

-   Substrate's
	[`BlockImport` trait](https://substrate.dev/rustdocs/v3.0.0/sp_consensus/block_import/trait.BlockImport.html)
-   Substrate's [import pipeline](https://substrate.dev/rustdocs/v3.0.0/sp_consensus/import_queue/index.html)
-   Structure of a typical [Substrate Service](https://substrate.dev/rustdocs/v3.0.0/sc_service/index.html)
-   Configuration of
	[`InherentDataProvider`](https://substrate.dev/rustdocs/v3.0.0/sp_authorship/struct.InherentDataProvider.html)s

## The Structure of a Node

A Substrate node has two parts. An outer part that is responsible for gossiping transactions and
blocks, handling [rpc requests](./custom-rpc.md), and reaching consensus. And a runtime that is
responsible for the business logic of the chain. This architecture diagram illustrates the
distinction.

![Substrate Architecture Diagram](img/substrate-architecture.png)

In principle, the consensus engine (part of the outer node) is agnostic to the runtime that is used
with it. But in practice, most consensus engines will require the runtime to provide certain
[runtime APIs](./runtime-api.md) that affect the engine. For example, Aura and Babe query the
runtime for the set of validators. A more real-world PoW consensus would query the runtime for the
block difficulty. Additionally, some runtimes rely on the consensus engine to provide
[pre-runtime digests](https://substrate.dev/rustdocs/v3.0.0/sp_runtime/generic/enum.DigestItem.html#variant.PreRuntime).
For example, runtimes that include the Babe pallet expect a pre-runtime digest containing
information about the current babe slot.

In this recipe we will avoid those practical complexities by using the
[Minimal Sha3 Proof of Work](./sha3-pow-consensus.md) consensus engine, which is truly isolated from the runtime. This node works with most of the recipes' runtimes, and has the super runtime installed by default.

## The Substrate Service

The [Substrate Service](https://substrate.dev/rustdocs/v3.0.0/sc_service/index.html) is the main
coordinator of the various parts of a Substrate node, including consensus. The service is large and
takes many parameters, so in each node, it is put together in a dedicated `src/service.rs` file.

The particular part of the service that is relevant here is
[`ImportQueue`](https://substrate.dev/rustdocs/v3.0.0/sc_service/trait.ImportQueue.html).
Here we construct an instance of the
[`PowBlockImport` struct](https://substrate.dev/rustdocs/v3.0.0/sc_consensus_pow/struct.PowBlockImport.html),
providing it with references to our client, our `MinimalSha3Algorithm`, and some other necessary
data.

```rust, ignore
let pow_block_import = sc_consensus_pow::PowBlockImport::new(
	client.clone(),
	client.clone(),
	sha3pow::MinimalSha3Algorithm,
	0, // check inherents starting at block 0
	select_chain.clone(),
	inherent_data_providers.clone(),
	can_author_with,
);

let import_queue = sc_consensus_pow::import_queue(
	Box::new(pow_block_import.clone()),
	None,
	sha3pow::MinimalSha3Algorithm,
	inherent_data_providers.clone(),
	&task_manager.spawn_handle(),
	config.prometheus_registry(),
)?;
```

Once the `PowBlockImport` is constructed, we can use it to create an actual import queue that the
service will use for importing blocks into the client.

### The Block Import Pipeline

You may have noticed that when we created the `PowBlockImport` we gave it two separate references to
the client. The second reference will always be to a client. But the first is interesting. The
[rustdocs tell us](https://substrate.dev/rustdocs/v3.0.0/sc_consensus_pow/struct.PowBlockImport.html#method.new)
that the first parameter is `inner: BlockImport<B, Transaction = TransactionFor<C, B>>`. Why would a
block import have a reference to another block import? Because the "block import pipeline" is
constructed in an onion-like fashion, where one layer of block import wraps the next. Learn more
about this pattern in the knowledgebase article on the
[block import pipeline](https://substrate.dev/docs/en/knowledgebase/advanced/block-import).

### Inherent Data Providers

Both the BlockImport and the `import_queue` are given an instance called `inherent_data_providers`.
This object is created in a helper function defined at the beginning of `service.rs`

```rust, ignore
pub fn build_inherent_data_providers() -> Result<InherentDataProviders, ServiceError> {
	let providers = InherentDataProviders::new();

	providers
		.register_provider(sp_timestamp::InherentDataProvider)
		.map_err(Into::into)
		.map_err(sp_consensus::error::Error::InherentData)?;

	Ok(providers)
}
```

Anything that implements the
[`ProvideInherentData` trait](https://substrate.dev/rustdocs/v3.0.0/sp_inherents/trait.ProvideInherentData.html)
may be used here. The block authoring logic must supply all inherents that the runtime expects. In
the case of this basic-pow chain, that is just the
[`TimestampInherentData`](https://substrate.dev/rustdocs/v3.0.0/sp_timestamp/trait.TimestampInherentData.html)
expected by the [timestamp pallet](https://substrate.dev/rustdocs/v3.0.0/pallet_timestamp/index.html). In order
to register other inherents, you would call `register_provider` multiple times, and map errors
accordingly.

## Mining

We've already implemented a mining algorithm as part of our
[`MinimalSha3Algorithm`](./sha3-pow-consensus.md), but we haven't yet told our service to actually
mine with that algorithm. This is our last task in the `new_full` function.

```rust, ignore
let proposer = sc_basic_authorship::ProposerFactory::new(
	task_manager.spawn_handle(),
	client.clone(),
	transaction_pool,
	prometheus_registry.as_ref(),
);

let (_worker, worker_task) = sc_consensus_pow::start_mining_worker(
	Box::new(pow_block_import),
	client,
	select_chain,
	MinimalSha3Algorithm,
	proposer,
	network.clone(),
	None,
	inherent_data_providers,
	// time to wait for a new block before starting to mine a new one
	Duration::from_secs(10),
	// how long to take to actually build the block (i.e. executing extrinsics)
	Duration::from_secs(10),
	can_author_with,
);
```

We begin by testing whether this node participates in consensus, which is to say we check whether
the user wants the node to act as a miner. If this node is to be a miner, we gather references to
various parts of the node that the
[`start_mining_worker` function](https://substrate.dev/rustdocs/v3.0.0/sc_consensus_pow/fn.start_mining_worker.html) requires.

With the worker built, we let the task manager spawn it.

```rust, ignore
task_manager
	.spawn_essential_handle()
	.spawn_blocking("pow", worker_task);
```

## The Light Client

The last thing in the `service.rs` file is constructing the
[light client](https://www.parity.io/what-is-a-light-client/)'s service. This code is quite similar
to the construction of the full service.

## Note of Finality

If we run the `basic-pow` node now, we see in console logs, that the finalized block always remains
at 0.

```
...
2020-03-22 12:50:09 Starting consensus session on top of parent 0x85811577d1033e918b425380222fd8c5aef980f81fa843d064d80fe027c79f5a
2020-03-22 12:50:09 Imported #189 (0x8581…9f5a)
2020-03-22 12:50:09 Prepared block for proposing at 190 [hash: 0xdd83ba96582acbed59aacd5304a9258962d1d4c2180acb8b77f725bd81461c4f; parent_hash: 0x8581…9f5a; extrinsics (1): [0x77a5…f7ad]]
2020-03-22 12:50:10 Idle (1 peers), best: #189 (0x8581…9f5a), finalized #0 (0xff0d…5cb9), ⬇ 0.2kiB/s ⬆ 0.4kiB/s
2020-03-22 12:50:15 Idle (1 peers), best: #189 (0x8581…9f5a), finalized #0 (0xff0d…5cb9), ⬇ 0 ⬆ 0
```

This is expected because Proof of Work is a consensus mechanism with probabilistic finality. This
means a block is never truly finalized and can always be reverted. The further behind the blockchain
head a block is, the less likely it is going to be reverted.
