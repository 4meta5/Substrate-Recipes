# Transaction Fees

`runtimes/weight-fee-runtime`
<a target="_blank" href="https://playground.substrate.dev/?deploy=recipes&files=%2Fhome%2Fsubstrate%2Fworkspace%2Fruntimes%2Fweight-fee-runtime%2Fsrc%2Flib.rs">
	<img src="https://img.shields.io/badge/Playground-Try%20it!-brightgreen?logo=Parity%20Substrate" alt ="Try on playground"/>
</a>
<a target="_blank" href="https://github.com/substrate-developer-hub/recipes/tree/master/runtimes/weight-fee-runtime/src/lib.rs">
	<img src="https://img.shields.io/badge/Github-View%20Code-brightgreen?logo=github" alt ="View on GitHub"/>
</a>

Substrate provides the
[`transaction_payment` pallet](https://substrate.dev/rustdocs/v3.0.0/pallet_transaction_payment/index.html) for
calculating and collecting fees for executing transactions. Fees are broken down into two
components:

-   Byte fee - A fee proportional to the transaction's length in bytes. The proportionality constant
	is a parameter in the `transaction_payment` pallet.
-   Weight fee - A fee calculated from the transaction's weight. Weights quantify the time spent
	executing the transaction. Learn more in the [recipe on weights](./weights.md). The conversion
	doesn't need to be linear, although it often is. The same conversion function is applied across
	all transactions from all pallets in the runtime.
-   Fee Multiplier - A multiplier for the computed fee, that can change as the chain progresses.
	This topic is not (yet) covered further in the recipes.

```
total_fee = transaction_length * length_fee + weight_to_fee(total_weight)
```

## Setting the Parameters

Each of the parameters described above is set in the
[transaction payment pallet](https://substrate.dev/rustdocs/v3.0.0/pallet_transaction_payment/index.html)'s
configuration trait. For example, the `super-runtime` sets these parameters as follows.

src:
[`runtimes/super-runtime/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/runtimes/super-runtime/src/lib.rs)

```rust, ignore
parameter_types! {
	pub const TransactionByteFee: u128 = 1;
}

impl transaction_payment::Config for Runtime {
	type Currency = balances::Module<Runtime>;
	type OnTransactionPayment = ();
	type TransactionByteFee = TransactionByteFee;
	type WeightToFee = IdentityFee<Balance>;
	type FeeMultiplierUpdate = ();
}
```

## 1 to 1 Conversion

In many cases converting weight to fees one-to-one, as shown above, will suffice and can be
accomplished with
[`IdentityFee`](https://substrate.dev/rustdocs/v3.0.0/frame_support/weights/struct.IdentityFee.html). This
approach is also taken in the
[node template](https://github.com/paritytech/substrate/blob/2d39ec2c4aaec1cc0f91fcb91734de8f408dc1b2/bin/node-template/runtime/src/lib.rs#L246).
It is also possible to provide a type that makes a more complex calculation. Any type that
implements
[`WeightToFeePolynomial`](https://substrate.dev/rustdocs/v3.0.0/frame_support/weights/trait.WeightToFeePolynomial.html)
will suffice.

## Linear Conversion

Another common way to convert weight to fees is linearly. When converting linearly, the weight is
multiplied by a constant coefficient to determine the fee to charge. This is demonstrated in the
`weight-fee-runtime` with the `LinearWeightToFee` struct.

We declare the struct with an associated type `C`, which will provide the coefficient.

```rust, ignore
pub struct LinearWeightToFee<C>(sp_std::marker::PhantomData<C>);
```

Then we implement `WeightToFeePolynomial` for it. When implementing this trait, your main job is to
return a set of
[`WeightToFeeCoefficient`](https://substrate.dev/rustdocs/v3.0.0/frame_support/weights/struct.WeightToFeeCoefficient.html)s.
These coefficients can have integer and fractional parts and be positive or negative. In our
`LinearWeightToFee` there is a single integer coefficient supplied by the associated type.

```rust, ignore
impl<C> WeightToFeePolynomial for LinearWeightToFee<C>
where
	C: Get<Balance>,
{
	type Balance = Balance;

	fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
		let coefficient = WeightToFeeCoefficient {
			coeff_integer: C::get(),
			coeff_frac: Perbill::zero(),
			negative: false,
			degree: 1,
		};

		// Return a smallvec of coefficients. Order does not need to match degrees
		// because each coefficient has an explicit degree annotation.
		smallvec!(coefficient)
	}
}
```

This struct is reusable, and works with different coefficients. Using it looks like this.

```rust, ignore
parameter_types! {
	// Used with LinearWeightToFee conversion. Leaving this constant intact when using other
	// conversion techniques is harmless.
	pub const FeeWeightRatio: u128 = 1_000;

	// --snip--
}

impl transaction_payment::Config for Runtime {

	// Convert dispatch weight to a chargeable fee.
	type WeightToFee = LinearWeightToFee<FeeWeightRatio>;

	// --snip--
}
```

## Quadratic Conversion

More complex polynomials can also be used. When using complex polynomials, it is unlikely that your
logic will be reused among multiple chains, so it is generally not worth the overhead of making the
coefficients configurable. The `QuadraticWeightToFee` demonstrates a 2nd-degree polynomial with
hard-coded non-integer signed coefficients.

```rust, ignore
pub struct QuadraticWeightToFee;

impl WeightToFeePolynomial for QuadraticWeightToFee {
	type Balance = Balance;

	fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
		let linear = WeightToFeeCoefficient {
			coeff_integer: 2,
			coeff_frac: Perbill::from_percent(40),
			negative: true,
			degree: 1,
		};
		let quadratic = WeightToFeeCoefficient {
			coeff_integer: 3,
			coeff_frac: Perbill::zero(),
			negative: false,
			degree: 2,
		};

		// Return a smallvec of coefficients. Order does not need to match degrees
		// because each coefficient has an explicit degree annotation. In fact, any
		// negative coefficients should be saved for last regardless of their degree
		// because large negative coefficients will likely cause saturation (to zero)
		// if they happen early on.
		smallvec![quadratic, linear]
	}
}
```

## Collecting Fees

Having calculated the amount of fees due, runtime authors must decide which asset the fees should be
paid in. A common choice is the use the
[`Balances` pallet](https://substrate.dev/rustdocs/v3.0.0/pallet_balances/index.html), but any type that
implements the [`Currency` trait](https://substrate.dev/rustdocs/v3.0.0/frame_support/traits/trait.Currency.html)
can be used.

src:
[`runtimes/weight-fee-runtime/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/runtimes/weight-fee-runtime/src/lib.rs)

```rust, ignore
impl transaction_payment::Config for Runtime {

	// A generic asset whose ID is stored in the generic_asset pallet's runtime storage
	type Currency = SpendingAssetCurrency<Self>;

	// --snip--
}
```
