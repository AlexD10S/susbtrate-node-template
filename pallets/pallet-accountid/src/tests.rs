use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(PalletAccountId::do_something(Origin::signed(1)));
		// Read pallet storage and assert an expected result.
		//assert_eq!(PalletAccountId::something(), Some(42));
	});
}
