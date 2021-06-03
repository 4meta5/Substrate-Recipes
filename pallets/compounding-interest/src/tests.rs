use crate::{self as compounding_interest, Config, Event as PalletEvent};
use frame_support::{assert_ok, construct_runtime, parameter_types, traits::OnFinalize};
use sp_core::H256;
use sp_io::TestExternalities;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
type Block = frame_system::mocking::MockBlock<TestRuntime>;

construct_runtime!(
	pub enum TestRuntime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Module, Call, Config, Storage, Event<T>},
		CompoundingInterest: compounding_interest::{Module, Call, Event},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::simple_max(1024);
}
impl frame_system::Config for TestRuntime {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type Origin = Origin;
	type Index = u64;
	type Call = Call;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
}

impl Config for TestRuntime {
	type Event = Event;
}

struct ExternalityBuilder;

impl ExternalityBuilder {
	pub fn build() -> TestExternalities {
		let storage = frame_system::GenesisConfig::default()
			.build_storage::<TestRuntime>()
			.unwrap();
		let mut ext = TestExternalities::from(storage);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}

#[test]
fn deposit_withdraw_discrete_works() {
	ExternalityBuilder::build().execute_with(|| {
		// Deposit 10 tokens
		assert_ok!(CompoundingInterest::deposit_discrete(Origin::signed(1), 10));

		// Withdraw 5 tokens
		assert_ok!(CompoundingInterest::withdraw_discrete(Origin::signed(1), 5));

		// Test that the expected events were emitted
		let our_events = System::events()
			.into_iter()
			.map(|r| r.event)
			.filter_map(|e| {
				if let Event::compounding_interest(inner) = e {
					Some(inner)
				} else {
					None
				}
			})
			.collect::<Vec<_>>();

		let expected_events = vec![
			PalletEvent::DepositedDiscrete(10),
			PalletEvent::WithdrewDiscrete(5),
		];

		assert_eq!(our_events, expected_events);

		// Check that five tokens are still there
		assert_eq!(CompoundingInterest::discrete_account(), 5);
	})
}

#[test]
fn discrete_interest_works() {
	ExternalityBuilder::build().execute_with(|| {
		// Deposit 100 tokens
		assert_ok!(CompoundingInterest::deposit_discrete(
			Origin::signed(1),
			100
		));

		// balance should not change after the 3rd block
		CompoundingInterest::on_finalize(3);
		assert_eq!(CompoundingInterest::discrete_account(), 100);

		// on_finalize should compute interest on 10th block
		CompoundingInterest::on_finalize(10);

		// Test that the expected events were emitted
		let our_events = System::events()
			.into_iter()
			.map(|r| r.event)
			.filter_map(|e| {
				if let Event::compounding_interest(inner) = e {
					Some(inner)
				} else {
					None
				}
			})
			.collect::<Vec<_>>();

		let expected_events = vec![
			PalletEvent::DepositedDiscrete(100),
			PalletEvent::DiscreteInterestApplied(50),
		];

		assert_eq!(our_events, expected_events);

		// Check that the balance has updated
		assert_eq!(CompoundingInterest::discrete_account(), 150);
	})
}
