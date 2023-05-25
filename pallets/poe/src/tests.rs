use super::*;
use crate::Error;
use crate::mock::*;
use frame_support::{assert_noop, assert_ok, BoundedVec};
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;


#[test]
fn create_claim_works() {
	new_test_ext().execute_with(
		|| {
			let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
			assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

			assert_eq!(
				Proofs::<Test>::get(&claim),
				Some((1, frame_system::Pallet::<Test>::block_number()))
			)
		}
	)
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
	new_test_ext().execute_with(
		|| {
			let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
			let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

			assert_noop!(
				PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()),
				Error::<Test>::ProofAlreadyExist
			);
		}
	);
}


#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

		assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()));
	}
	);
}


#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();


		assert_noop!(
				PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()),
				Error::<Test>::ClaimNotExist
			);
	}
	)
}


#[test]
fn revoke_claim_failed_with_wrong_owner() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

		assert_noop!(
				PoeModule::revoke_claim(RuntimeOrigin::signed(2), claim.clone()),
				Error::<Test>::NotClaimOwner
			);
	}
	)
}


#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let sender = 1;
		let dest = 2;

		// 创建一个声明并将其所有权转移给目标账户
		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(sender), claim.clone()));
		assert_ok!(PoeModule::transfer_claim(RuntimeOrigin::signed(sender), claim.clone(), dest));

		// 确保声明的所有权已经转移给目标账户
		let (owner, _block_number) = Proofs::<Test>::get(&claim).unwrap();
		assert_eq!(owner, dest);

		// 确保非声明所有者无法转移声明的所有权
		assert_noop!(
				PoeModule::transfer_claim(RuntimeOrigin::signed(dest), claim.clone(), 3),
				Error::<Test>::NotClaimOwner
			);

		// 确保不存在的声明无法转移所有权
		let invalid_claim = BoundedVec::try_from(vec![2, 3]).unwrap();
		assert_noop!(
				PoeModule::transfer_claim(RuntimeOrigin::signed(sender), invalid_claim, dest),
				Error::<Test>::ClaimNotExist
			);
	});
}




#[test]
fn create_claim_works_2() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let sender = 1;

		// 创建一个新的声明
		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(sender), claim.clone()));

		// 确保声明已经被添加到存证列表中
		assert_eq!(
			Proofs::<Test>::get(&claim),
			Some((sender.clone(), frame_system::Pallet::<Test>::block_number()))
		);

		// 确保重复创建同一个声明会失败
		assert_noop!(
			PoeModule::create_claim(RuntimeOrigin::signed(sender), claim.clone()),
			Error::<Test>::ProofAlreadyExist
		);
	});
}


#[test]
fn revoke_claim_works_2() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let sender = 1;

		// 创建一个新的声明
		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(sender), claim.clone()));

		// 撤销声明
		assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(sender), claim.clone()));

		// 确保声明已经被移除
		assert_eq!(Proofs::<Test>::get(&claim), None);

		// 确保非声明所有者无法撤销声明
		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrigin::signed(2), claim.clone()),
			Error::<Test>::NotClaimOwner
		);

		// 确保不存在的声明无法被撤销
		let invalid_claim = BoundedVec::try_from(vec![2, 3]).unwrap();
		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrigin::signed(sender), invalid_claim),
			Error::<Test>::ClaimNotExist
		);
	});
}


