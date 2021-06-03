# Hello Substrate

`pallets/hello-substrate`
<a target="_blank" href="https://playground.substrate.dev/?deploy=recipes&files=%2Fhome%2Fsubstrate%2Fworkspace%2Fpallets%2Fhello-substrate%2Fsrc%2Flib.rs">
	<img src="https://img.shields.io/badge/Playground-Try%20it!-brightgreen?logo=Parity%20Substrate" alt ="Try on playground"/>
</a>
<a target="_blank" href="https://github.com/substrate-developer-hub/recipes/tree/master/pallets/hello-substrate/src/lib.rs">
	<img src="https://img.shields.io/badge/Github-View%20Code-brightgreen?logo=github" alt ="View on GitHub"/>
</a>

This pallet has one
dispatchable call that prints a message to the node's output. Printing to the node log is not common for runtimes, but can be quite useful when debugging and as a "hello world" example. Because this is the first pallet in the recipes, we'll also take a look at the general structure of a pallet.

## No Std

The very first line of code tells the rust compiler that this crate should not use rust's standard
library except when explicitly told to. This is useful because Substrate runtimes compile to Web
Assembly where the standard library is not available.

```rust, ignore
#![cfg_attr(not(feature = "std"), no_std)]
```

## Imports

Next, you'll find imports that come from various parts of the Substrate framework. All pallets will
import from a few common crates including
[`frame-support`](https://substrate.dev/rustdocs/v3.0.0/frame_support/index.html), and
[`frame-system`](https://substrate.dev/rustdocs/v3.0.0/frame_system/index.html). Complex pallets will have many
imports. The `hello-substrate` pallet uses these imports.

```rust, ignore
use frame_support::{debug, decl_module, dispatch::DispatchResult};
use frame_system::ensure_signed;
use sp_runtime::print;
```

## Tests

Next we see a reference to the tests module. This pallet, as with most recipes pallets, has tests written in a separate file called
`tests.rs`.

## Configuration Trait

Next, each pallet has a configuration trait which is called [`Config`](https://substrate.dev/rustdocs/v3.0.0/frame_system/pallet/trait.Config.html). The configuration trait can be
used to access features from other pallets, or [constants](./constants.md) that affect
the pallet's behavior. This pallet is simple enough that our configuration trait can remain empty,
although it must still exist.

```rust, ignore
pub trait Config: frame_system::Config {}
```

## Dispatchable Calls

A Dispatchable call is a function that a blockchain user can call as part of an Extrinsic.
"Extrinsic" is Substrate jargon meaning a call from outside of the chain. Most of the time they are
transactions, and for now it is fine to think of them as transactions. Dispatchable calls are
defined in the
[`decl_module!` macro](https://substrate.dev/rustdocs/v3.0.0/frame_support/macro.decl_module.html).

```rust, ignore
decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {

		/// A function that says hello to the user by printing messages to the node log
		#[weight = 10_000]
		pub fn say_hello(origin) -> DispatchResult {
			// --snip--
		}

		// More dispatchable calls could go here
	}
}
```

As you can see, our `hello-substrate` pallet has a dispatchable call that takes a single argument,
called `origin`. The call returns a
[`DispatchResult`](https://substrate.dev/rustdocs/v3.0.0/frame_support/dispatch/type.DispatchResult.html) which
can be either `Ok(())` indicating that the call succeeded, or an `Err` which is demonstrated in most other recipes pallets.

### Weight Annotations

Right before the `hello-substrate` function, we see the line `#[weight = 10_000]`. This line
attaches a default weight to the call. Ultimately weights affect the fees a user will have to pay to
call the function. Weights are a very interesting aspect of developing with Substrate, but they too
shall be covered later in the section on [Weights](./weights.md). For now, and for many of
the recipes pallets, we will simply use the default weight as we have done here.

## Inside a Dispatchable Call

Let's take a closer look at our dispatchable call.

```rust, ignore
pub fn say_hello(origin) -> DispatchResult {
	// Ensure that the caller is a regular keypair account
	let caller = ensure_signed(origin)?;

	// Print a message
	print("Hello World");
	// Inspecting variables
	debug::info!("Request sent by: {:?}", caller);

	// Indicate that this call succeeded
	Ok(())
}
```

This function essentially does three things. First, it uses the
[`ensure_signed` function](https://substrate.dev/rustdocs/v3.0.0/frame_system/fn.ensure_signed.html) to ensure
that the caller of the function was a regular user who owns a private key. This function also
returns who that caller was. We store the caller's identity in the `caller` variable.

Second, it prints a message and logs the caller. Notice that we aren't using Rust's normal
`println!` macro, but rather a special
[`print` function](https://substrate.dev/rustdocs/v3.0.0/sp_runtime/fn.print.html) and
[`debug::info!` macro](https://substrate.dev/rustdocs/v3.0.0/frame_support/debug/macro.info.html). The reason for
this is explained in the next section.

Finally, the call returns `Ok(())` to indicate that the call has succeeded. At a glance it seems
that there is no way for this call to fail, but this is not quite true. The `ensure_signed`
function, used at the beginning, can return an error if the call was not from a signed origin. This
is the first time we're seeing the important paradigm "**Verify first, write last**". In Substrate
development, it is important that you always ensure preconditions are met and return errors at the
beginning. After these checks have completed, then you may begin the function's computation.

## Printing from the Runtime

Printing to the terminal from a Rust program is typically very simple using the `println!` macro.
However, Substrate runtimes are compiled to both Web Assembly and a regular native binary, and do
not have access to rust's standard library. That means we cannot use the regular `println!`. I
encourage you to modify the code to try using `println!` and confirm that it will not compile.
Nonetheless, printing a message from the runtime is useful both for logging information, and also
for debugging.

![Substrate Architecture Diagram](./img/substrate-architecture.png)

At the top of our pallet, we imported `sp_runtime`'s
[`print` function](https://substrate.dev/rustdocs/v3.0.0/sp_runtime/fn.print.html). This special function allows
the runtime to pass a message for printing to the outer part of the node which is not compiled to
Wasm and does have access to the standard library and can perform regular IO. This function is only
able to print items that implement the
[`Printable` trait](https://substrate.dev/rustdocs/v3.0.0/sp_runtime/traits/trait.Printable.html). Luckily all
the primitive types already implement this trait, and you can implement the trait for your own
datatypes too.

**Print function note:** To actually see the printed messages, we need to use the flag
`-lruntime=debug` when running the kitchen node. So, for the kitchen node, the command would become
`./target/release/kitchen-node --dev -lruntime=debug`.

The next line demonstrates using `debug::info!` macro to log to the screen and also inspecting the
variable's content. The syntax inside the macro is very similar to what regular rust macro
`println!` takes.

You can specify the logger target with
```rust, ignore
debug::debug!(target: "mytarget", "called by {:?}", sender);
```
Now you can filter logs with
```
kitchen-node --dev -lmytarget=debug
```
If you do not specify the logger target, it will be set to the crate's name (not to `runtime`!).

**Runtime logger note:** When we execute the runtime in native, `debug::info!` messages are printed.
However, if we execute the runtime in Wasm, then an additional step to initialise
[RuntimeLogger](https://substrate.dev/rustdocs/v3.0.0/frame_support/debug/struct.RuntimeLogger.html) is required:
```
debug::RuntimeLogger::init();
```
You'll need to call this inside every pallet dispatchable call before logging.
