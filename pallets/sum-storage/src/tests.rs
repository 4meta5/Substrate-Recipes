use crate::{self as sum_storage, Config};

use frame_support::{assert_ok, construct_runtime, parameter_types};
use sp_core::H256;
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
		SumStorage: sum_storage::{Module, Call, Storage, Event<T>},
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

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::default()
		.build_storage::<TestRuntime>()
		.unwrap()
		.into()
}

#[test]
fn default_sum_zero() {
	new_test_ext().execute_with(|| {
		assert_eq!(SumStorage::get_sum(), 0);
	});
}

#[test]
fn sums_thing_one() {
	new_test_ext().execute_with(|| {
		assert_ok!(SumStorage::set_thing_1(Origin::signed(1), 42));
		assert_eq!(SumStorage::get_sum(), 42);
	});
}

#[test]
fn sums_thing_two() {
	new_test_ext().execute_with(|| {
		assert_ok!(SumStorage::set_thing_2(Origin::signed(1), 42));
		assert_eq!(SumStorage::get_sum(), 42);
	});
}

#[test]
fn sums_both_values() {
	new_test_ext().execute_with(|| {
		assert_ok!(SumStorage::set_thing_1(Origin::signed(1), 42));
		assert_ok!(SumStorage::set_thing_2(Origin::signed(1), 43));
		assert_eq!(SumStorage::get_sum(), 85);
	});
}
