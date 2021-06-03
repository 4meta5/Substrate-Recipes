# Using and Storing Structs

`pallets/struct-storage`
<a target="_blank" href="https://playground.substrate.dev/?deploy=recipes&files=%2Fhome%2Fsubstrate%2Fworkspace%2Fpallets%2Fstruct-storage%2Fsrc%2Flib.rs">
	<img src="https://img.shields.io/badge/Playground-Try%20it!-brightgreen?logo=Parity%20Substrate" alt ="Try on playground"/>
</a>
<a target="_blank" href="https://github.com/substrate-developer-hub/recipes/tree/master/pallets/struct-storage/src/lib.rs">
	<img src="https://img.shields.io/badge/Github-View%20Code-brightgreen?logo=github" alt ="View on GitHub"/>
</a>

In Rust, a `struct`, or structure, is a custom data type that lets you name and package together
multiple related values that make up a meaningful group. If you’re familiar with an object-oriented
language, a `struct` is like an object’s data attributes (read more in
[The Rust Book](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)).

## Defining a Struct

To define a _simple_ custom struct for the runtime, the following syntax may be used:

```rust, ignore
#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct MyStruct {
	some_number: u32,
	optional_number: Option<u32>,
}
```

In the code snippet above, the
[derive macro](https://doc.rust-lang.org/rust-by-example/trait/derive.html) is declared to ensure
`MyStruct` conforms to shared behavior according to the specified
[traits](https://doc.rust-lang.org/book/ch10-02-traits.html):
`Encode, Decode, Default, Clone, PartialEq`. If you wish the store this struct in blockchain
storage, you will need to derive (or manually ipmlement) each of these traits.

To use the `Encode` and `Decode` traits, it is necessary to import them.

```rust, ignore
use frame_support::codec::{Encode, Decode};
```

## Structs with Generic Fields

The simple struct shown earlier only uses Rust primitive types for its fields. In the common case
where you want to store types that come from your pallet's configuration trait (or the configuration
trait of another pallet in your runtime), you must use generic type parameters in your struct's
definition.

```rust, ignore
#[derive(Encode, Decode, Clone, Default, RuntimeDebug)]
pub struct InnerThing<Hash, Balance> {
	number: u32,
	hash: Hash,
	balance: Balance,
}
```

Here you can see that we want to store items of type `Hash` and `Balance` in the struct. Because
these types come from the system and balances pallets' configuration traits, we must specify them as
generics when declaring the struct.

It is often convenient to make a type alias that takes `T`, your pallet's configuration trait, as a
single type parameter. Doing so simply saves you typing in the future.

```rust, ignore
type InnerThingOf<T> = InnerThing<<T as frame_system::Config>::Hash, <T as pallet_balances::Config>::Balance>;
```

## Structs in Storage

Using one of our structs as a storage item is not significantly different than using a primitive
type. When using a generic struct, we must supply all of the generic type parameters. This snippet
shows how to supply thos parameters when you have a type alias (like we do for `InnerThing`) as well
as when you don't. Whether to include the type alias is a matter of style and taste, but it is
generally preferred when the entire type exceeds the preferred line length.

```rust, ignore
decl_storage! {
	trait Store for Module<T: Config> as NestedStructs {
		InnerThingsByNumbers get(fn inner_things_by_numbers):
			map hasher(blake2_128_concat) u32 => InnerThingOf<T>;
		SuperThingsBySuperNumbers get(fn super_things_by_super_numbers):
			map hasher(blake2_256) u32 => SuperThing<T::Hash, T::Balance>;
	}
}
```

Interacting with the storage maps is now exactly as it was when we didn't use any custom structs

```rust, ignore
fn insert_inner_thing(origin, number: u32, hash: T::Hash, balance: T::Balance) -> DispatchResult {
	let _ = ensure_signed(origin)?;
	let thing = InnerThing {
					number,
					hash,
					balance,
				};
	<InnerThingsByNumbers<T>>::insert(number, thing);
	Self::deposit_event(RawEvent::NewInnerThing(number, hash, balance));
	Ok(())
}
```

## Nested Structs

Structs can also contain other structs as their fields. We have demonstrated this with the type
`SuperThing`. As you see, any generic types needed by the inner struct must also be supplied to the
outer.

```rust, ignore
#[derive(Encode, Decode, Default, RuntimeDebug)]
pub struct SuperThing<Hash, Balance> {
	super_number: u32,
	inner_thing: InnerThing<Hash, Balance>,
}
```
