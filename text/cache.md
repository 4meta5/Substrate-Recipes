# Cache Multiple Calls

`pallets/storage-cache`
<a target="_blank" href="https://playground.substrate.dev/?deploy=recipes&files=%2Fhome%2Fsubstrate%2Fworkspace%2Fpallets%2Fstorage-cache%2Fsrc%2Flib.rs">
	<img src="https://img.shields.io/badge/Playground-Try%20it!-brightgreen?logo=Parity%20Substrate" alt ="Try on playground"/>
</a>
<a target="_blank" href="https://github.com/substrate-developer-hub/recipes/tree/master/pallets/storage-cache/src/lib.rs">
	<img src="https://img.shields.io/badge/Github-View%20Code-brightgreen?logo=github" alt ="View on GitHub"/>
</a>

Calls to runtime storage have an associated cost and developers should strive to minimize the number
of calls.

```rust, ignore
decl_storage! {
	trait Store for Module<T: Config> as StorageCache {
		// copy type
		SomeCopyValue get(fn some_copy_value): u32;

		// clone type
		KingMember get(fn king_member): T::AccountId;
		GroupMembers get(fn group_members): Vec<T::AccountId>;
	}
}
```

## Copy Types

For [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html) types, it is easy to reuse
previous storage calls by simply reusing the value, which is automatically cloned upon reuse. In the
code below, the second call is unnecessary:

```rust, ignore
fn increase_value_no_cache(origin, some_val: u32) -> DispatchResult {
	let _ = ensure_signed(origin)?;
	let original_call = <SomeCopyValue>::get();
	let some_calculation = original_call.checked_add(some_val).ok_or("addition overflowed1")?;
	// this next storage call is unnecessary and is wasteful
	let unnecessary_call = <SomeCopyValue>::get();
	// should've just used `original_call` here because u32 is copy
	let another_calculation = some_calculation.checked_add(unnecessary_call).ok_or("addition overflowed2")?;
	<SomeCopyValue>::put(another_calculation);
	let now = <frame_system::Module<T>>::block_number();
	Self::deposit_event(RawEvent::InefficientValueChange(another_calculation, now));
	Ok(())
}
```

Instead, the initial call value should be reused. In this example, the `SomeCopyValue` value is
[`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html) so we should prefer the following
code without the unnecessary second call to storage:

```rust, ignore
fn increase_value_w_copy(origin, some_val: u32) -> DispatchResult {
	let _ = ensure_signed(origin)?;
	let original_call = <SomeCopyValue>::get();
	let some_calculation = original_call.checked_add(some_val).ok_or("addition overflowed1")?;
	// uses the original_call because u32 is copy
	let another_calculation = some_calculation.checked_add(original_call).ok_or("addition overflowed2")?;
	<SomeCopyValue>::put(another_calculation);
	let now = <frame_system::Module<T>>::block_number();
	Self::deposit_event(RawEvent::BetterValueChange(another_calculation, now));
	Ok(())
}
```

## Clone Types

If the type was not `Copy`, but was [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html),
then it is still better to clone the value in the method than to make another call to runtime
storage.

The runtime methods enable the calling account to swap the `T::AccountId` value in storage if

1. the existing storage value is not in `GroupMembers` AND
2. the calling account is in `GroupMembers`

The first implementation makes a second unnecessary call to runtime storage instead of cloning the
call for `existing_key`:

```rust, ignore
fn swap_king_no_cache(origin) -> DispatchResult {
	let new_king = ensure_signed(origin)?;
	let existing_king = <KingMember<T>>::get();

	// only places a new account if
	// (1) the existing account is not a member &&
	// (2) the new account is a member
	ensure!(!Self::is_member(&existing_king), "current king is a member so maintains priority");
	ensure!(Self::is_member(&new_king), "new king is not a member so doesn't get priority");

	// BAD (unnecessary) storage call
	let old_king = <KingMember<T>>::get();
	// place new king
	<KingMember<T>>::put(new_king.clone());

	Self::deposit_event(RawEvent::InefficientKingSwap(old_king, new_king));
	Ok(())
}
```

If the `existing_key` is used without a `clone` in the event emission instead of `old_king`, then
the compiler returns the following error:

```bash
error[E0382]: use of moved value: `existing_king`
	--> src/lib.rs:93:63
	|
80 |             let existing_king = <KingMember<T>>::get();
	|                 ------------- move occurs because `existing_king` has type `<T as frame_system::Config>::AccountId`, which does not implement the `Copy` trait
...
85 |             ensure!(!Self::is_member(existing_king), "is a member so maintains priority");
	|                                      ------------- value moved here
...
93 |             Self::deposit_event(RawEvent::InefficientKingSwap(existing_king, new_king));
	|                                                               ^^^^^^^^^^^^^ value used here after move

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
error: Could not compile `storage-cache`.

To learn more, run the command again with --verbose.
```

Fixing this only requires cloning the original value before it is moved:

```rust, ignore
fn swap_king_with_cache(origin) -> DispatchResult {
	let new_king = ensure_signed(origin)?;
	let existing_king = <KingMember<T>>::get();
	// prefer to clone previous call rather than repeat call unnecessarily
	let old_king = existing_king.clone();

	// only places a new account if
	// (1) the existing account is not a member &&
	// (2) the new account is a member
	ensure!(!Self::is_member(&existing_king), "current king is a member so maintains priority");
	ensure!(Self::is_member(&new_king), "new king is not a member so doesn't get priority");

	// <no (unnecessary) storage call here>
	// place new king
	<KingMember<T>>::put(new_king.clone());

	Self::deposit_event(RawEvent::BetterKingSwap(old_king, new_king));
	Ok(())
}
```

Not all types implement [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html) or
[`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html), so it is important to discern other
patterns that minimize and alleviate the cost of calls to storage.
