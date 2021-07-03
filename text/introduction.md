# Substrate Recipes 🍴😋🍴

_A Hands-On Cookbook for Aspiring Blockchain Chefs_

Substrate Recipes is a cookbook of working examples that demonstrate best practices when building
blockchains with **[Substrate](https://substrate.dev)**. Each recipe contains
complete working code as well as a detailed writeup explaining the code.

## How to Use This Book

You can read this book in any particular order. If you have a certain topic you want to dive into, or
 know the subject/keyword to look for, please use the search button (the small magnifier on top
left) to search for the subject. The list is organized roughly in order of increasing complexity.

You can't learn to build blockchains by reading alone. As you work through the recipes, practice compiling, testing, and hacking on each Recipes. Play with
the code, extract patterns, and apply them to a problem that you want to solve!

If you haven't already, you should probably clone this repository right now.
```bash
git clone https://github.com/substrate-developer-hub/recipes.git
```

## Getting Help

When learning any new skill, you will inevitably get stuck at some point. When you do get stuck you
can seek help in several ways:

-   Ask a question on [Stack Overflow](https://stackoverflow.com/questions/tagged/substrate)
-   Ask a question in the
    [Substrate Technical Element channel](https://app.element.io/#/room/!HzySYSaIhtyWrwiwEV:matrix.org)
-   Open a [new issue](https://github.com/substrate-developer-hub/recipes/issues/new) against this
    repository

## Prerequisites

Each recipe targets a specific aspect of Substrate development and explains the details of that aspect. In all recipes some basic familiarity with Substrate development and a working Rust environment are assumed. Generally speaking you should meet the following prerequisites:

- Have a working Substrate development environment. There are excellent docs on [setting up a Substrate development environment](https://substrate.dev/docs/en/knowledgebase/getting-started/).
- Understand the first ten chapters of [The Rust Book](https://doc.rust-lang.org/book/index.html). Rather than learning Rust _before_ you learn Substrate, consider learning Rust _as_
you learn Substrate.
- Complete the first few [Official Substrate Tutorials](https://substrate.dev/en/tutorials).

## Structure of a Substrate Node

It is useful to recognize that
[coding is all about abstraction](https://youtu.be/05H4YsyPA-U?t=1789).

To understand how the code in this repository is organized, let's first take a look at how a
Substrate node is constructed. Each node has many components that manage things like the transaction
queue, communicating over a P2P network, reaching consensus on the state of the blockchain, and the
chain's actual runtime logic. Each aspect of the node is interesting in its own right, and the
runtime is particularly interesting because it contains the business logic (aka "state transition
function") that codifies the chain's functionality.

Much, but not all, of the Recipes focuses on writing runtimes with FRAME, Parity's Framework for
composing runtimes from individual building blocks called Pallets. Runtimes built with FRAME
typically contain several such pallets. The kitchen node you built previously follows this paradigm.

![Substrate Architecture Diagram](img/substrate-architecture.png)

## The Directories in our Kitchen

If you haven't already, you should clone it now. There are five primary directories in this repository.

-   **Consensus**: Consensus engines for use in Substrate nodes.
-   **Nodes**: Complete Substrate nodes ready to run.
-   **Pallets**: Pallets for use in FRAME-based runtimes.
-   **Runtimes**: Runtimes for use in Substrate nodes.
-   **Text**: Source of [the book](https://substrate.dev/recipes) written in markdown. This is what
    you're reading right now.

Exploring those directories reveals a tree that looks like this

```
recipes
|
+-- consensus
  |
  +-- manual-seal
  |
  +-- sha3pow
|
+-- nodes
	|
	+-- basic-pow
	|
	+-- ...
	|
	+-- rpc-node
|
+-- pallets
	|
	+-- basic-token
	|
	+ ...
	|
	+-- weights
|
+-- runtimes
	|
	+-- api-runtime
	|
	+ ...
	|
	+-- weight-fee-runtime
|
+-- text
```

## Inside the Kitchen Node

Let us take a deeper look at the
[Kitchen Node](https://github.com/substrate-developer-hub/recipes/tree/master/nodes/kitchen-node).

Looking inside the Kitchen Node's `Cargo.toml` file we see that it has many dependencies. Most of
them come from Substrate itself. Indeed most parts of this Kitchen Node are not unique or
specialized, and Substrate offers robust implementations that we can use. The runtime does not come
from Substrate. Rather, we use our super-runtime which is in the `runtimes` folder.

**`nodes/kitchen-node/Cargo.toml`**

```TOML
# This node is compatible with any of the runtimes below
# ---
# Common runtime configured with most Recipes pallets.
runtime = { package = "super-runtime", path = "../../runtimes/super-runtime" }

# Runtime with custom weight and fee calculation.
# runtime = { package = "weight-fee-runtime", path = "../../runtimes/weight-fee-runtime"}

# Runtime with off-chain worker enabled.
# To use this runtime, compile the node with `ocw` feature enabled,
#   `cargo build --release --features ocw`.
# runtime = { package = "ocw-runtime", path = "../../runtimes/ocw-runtime" }

# Runtime with custom runtime-api (custom API only used in rpc-node)
# runtime = { package = "api-runtime", path = "../../runtimes/api-runtime" }
# ---
```

The commented lines, quoted above, show that the Super Runtime is not the only runtime we could have
chosen. We could also use the Weight-Fee runtime, and I encourage you to try that experiment
(remember, instructions to re-compile the node are in the previous section).

Every node must have a runtime. You may confirm that by looking at the `Cargo.toml` files of the
other nodes included in our kitchen.

## Inside the Super Runtime

Having seen that the Kitchen Node depends on a runtime, let us now look deeper at the
[Super Runtime](https://github.com/substrate-developer-hub/recipes/tree/master/runtimes/super-runtime).

**`runtimes/super-runtime/Cargo.toml`**

```TOML
# -- snip --

# Substrate Pallets
pallet-balances = { version = '3.0', default-features = false }
pallet-transaction-payment = { version = '3.0', default-features = false }
# Recipe Pallets
adding-machine = { path = "../../pallets/adding-machine", default-features = false }
basic-token = { path = "../../pallets/basic-token", default-features = false }
```

Here we see that the runtime depends on many pallets. Some of these pallets come from Substrate
itself. Indeed, Substrate offers a rich collection of commonly used pallets which you may use in
your own runtimes. This runtime also contains several custom pallets that are written right here in
our Kitchen.

## Common Patterns

We've just observed the general pattern used throughout the recipes. From the inside out, we see a
piece of pallet code stored in `pallets/<pallet-name>/src/lib.rs`. The pallet is then included into
a runtime by adding its name and relative path in `runtimes/<runtime-name>/Cargo.toml`. That runtime
is then installed in a node by adding its name and relative path in `nodes/<node-name>/Cargo.toml`.

Some recipes explore aspects of Blockchain development that are outside of the runtime. Looking back
to our node architecture at the beginning of this section, you can imagine that changing a node's
RPC or Consensus would be conceptually similar to changing its runtime.
