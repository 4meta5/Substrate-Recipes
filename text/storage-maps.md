# Storage Maps

`pallets/simple-map`
<a target="_blank" href="https://playground.substrate.dev/?deploy=recipes&files=%2Fhome%2Fsubstrate%2Fworkspace%2Fpallets%2Fsimple-map%2Fsrc%2Flib.rs">
	<img src="https://img.shields.io/badge/Playground-Try%20it!-brightgreen?logo=Parity%20Substrate" alt ="Try on playground"/>
</a>
<a target="_blank" href="https://github.com/substrate-developer-hub/recipes/tree/master/pallets/simple-map/src/lib.rs">
	<img src="https://img.shields.io/badge/Github-View%20Code-brightgreen?logo=github" alt ="View on GitHub"/>
</a>

In this recipe, we will see how
to store a mapping from keys to values, similar to Rust's own
[`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html).

## Declaring a `StorageMap`

We declare a single storage map with the following syntax:

```rust, ignore
decl_storage! {
	trait Store for Module<T: Config> as SimpleMap {
		SimpleMap get(fn simple_map): map hasher(blake2_128_concat) T::AccountId => u32;
	}
}
```

Much of this should look familiar to you from storage values. Reading the line from left to right we
have:

-   `SimpleMap` - the name of the storage map
-   `get(fn simple_map)` - the name of a getter function that will return values from the map.
-   `: map hasher(blake2_128_concat)` - beginning of the type declaration. This is a map and it will
	use the
	[`blake2_128_concat`](https://substrate.dev/rustdocs/v3.0.0/frame_support/trait.Hashable.html#tymethod.blake2_128_concat)
	hasher. More on this below.
-   `T::AccountId => u32` - The specific key and value type of the map. This is a map from
	`AccountId`s to `u32`s.

## Choosing a Hasher

Although the syntax above is complex, most of it should be straightforward if you've understood the
recipe on storage values. The last unfamiliar piece of writing a storage map is choosing which
hasher to use. In general you should choose one of the three following hashers. The choice of hasher
will affect the performance and security of your chain. If you don't want to think much about this,
just choose `blake2_128_concat` and skip to the next section.

### `blake2_128_concat`

This is a cryptographically secure hash function, and is always safe to use. It is reasonably
efficient, and will keep your storage tree balanced. You _must_ choose this hasher if users of your
chain have the ability to affect the storage keys. In this pallet, the keys are `AccountId`s. At
first it may _seem_ that the user doesn't affect the `AccountId`, but in reality a malicious user
can generate thousands of accounts and use the one that will affect the chain's storage tree in the
way the attacker likes. For this reason, we have chosen to use the `blake2_128_concat` hasher.

### `twox_64_concat`

This hasher is _not_ cryptographically secure, but is more efficient than blake2. Thus it represents
trading security for performance. You should _not_ use this hasher if chain users can affect the
storage keys. However, it is perfectly safe to use this hasher to gain performance in scenarios
where the users do not control the keys. For example, if the keys in your map are sequentially
increasing indices and users cannot cause the indices to rapidly increase, then this is a perfectly
reasonable choice.

### `identity`

The `identity` "hasher" is really not a hasher at all, but merely an
[identity function](https://en.wikipedia.org/wiki/Identity_function) that returns the same value it
receives. This hasher is only an option when the key type in your storage map is _already_ a hash,
and is not controllable by the user. If you're in doubt whether the user can influence the key just
use blake2.

## The Storage Map API

This pallet demonstrated some of the most common methods available in a storage map including
`insert`, `get`, `take`, and `contains_key`.

```rust, ignore
// Insert
<SimpleMap<T>>::insert(&user, entry);

// Get
let entry = <SimpleMap<T>>::get(account);

// Take
let entry = <SimpleMap<T>>::take(&user);

// Contains Key
<SimpleMap<T>>::contains_key(&user)
```

The rest of the API is documented in the rustdocs on the
[`StorageMap` trait](https://substrate.dev/rustdocs/v3.0.0/frame_support/storage/trait.StorageMap.html). You do
not need to explicitly `use` this trait because the `decl_storage!` macro will do it for you if you
use a storage map.
