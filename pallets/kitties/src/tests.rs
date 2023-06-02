use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_create() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		System::set_block_number(1);

		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_eq!(KittiesModule::next_kitty_id(), 1);

		assert_noop!(KittiesModule::transfer(Origin::signed(2), 0, 1), Error::<Test>::NotOwner);
	});
}
