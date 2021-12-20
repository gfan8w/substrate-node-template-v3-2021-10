use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use super::*;


#[test]
fn create_claim_ok() {
	new_test_ext().execute_with(|| {
		let claim ="hello".as_bytes().to_vec();
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
		assert_eq!(Proofs::<Test>::get(&claim), Some((1,frame_system::Pallet::<Test>::block_number())));
	});
}


#[test]
fn create_duplicated_claim_failed() {
	new_test_ext().execute_with(|| {
		let claim ="hello".as_bytes().to_vec();
		PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_noop!(PoeModule::create_claim(Origin::signed(1), claim.clone()),Error::<Test>::ProofAlreadyExist);
	});
}

#[test]
fn create_too_large_claim_failed() {
	new_test_ext().execute_with(|| {
		let max = MaxClaimLength::get();
		let claim =vec![0u8; (max + 1) as usize];
		assert_noop!(PoeModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ClaimTooLarge);
	});
}

#[test]
fn revoke_claim_ok() {
	new_test_ext().execute_with(|| {
		let claim ="hello".as_bytes().to_vec();
		PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
		assert_eq!(Proofs::<Test>::get(claim.clone()),None);
	});
}

#[test]
fn revoke_claim_not_exist() {
	new_test_ext().execute_with(|| {
		let claim ="hello".as_bytes().to_vec();
		assert_noop!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()), Error::<Test>::ProofNotExist);
	});
}

#[test]
fn revoke_claim_not_owner_failed() {
	new_test_ext().execute_with(|| {
		let claim ="hello".as_bytes().to_vec();
		let user_b =2;
		PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_noop!(PoeModule::revoke_claim(Origin::signed(user_b), claim.clone()),Error::<Test>::NotClaimOwner);

	});
}


#[test]
fn transfer_claim_not_exist() {
	new_test_ext().execute_with(|| {
		let claim ="hello".as_bytes().to_vec();
		let user_a: <Test as frame_system::Config>::AccountId = 1;
		let user_b =2;
		assert_noop!(PoeModule::transfer_claim(Origin::signed(user_a),user_b, claim.clone()),Error::<Test>::ProofNotExist);
	});
}

#[test]
fn transfer_claim_not_claim_owner() {
	new_test_ext().execute_with(|| {
		let claim ="hello".as_bytes().to_vec();
		let user_a: <Test as frame_system::Config>::AccountId = 1;
		let user_b: <Test as frame_system::Config>::AccountId = 2;
		let user_ghost: <Test as frame_system::Config>::AccountId = 100;

		PoeModule::create_claim(Origin::signed(user_a), claim.clone());
		assert_noop!(PoeModule::transfer_claim(Origin::signed(user_ghost), user_b, claim.clone()),Error::<Test>::NotClaimOwner);
	});
}

#[test]
fn transfer_claim_claim_ok() {
	new_test_ext().execute_with(|| {
		let claim ="hello".as_bytes().to_vec();
		let user_a: <Test as frame_system::Config>::AccountId = 1;
		let user_b: <Test as frame_system::Config>::AccountId = 2;

		PoeModule::create_claim(Origin::signed(user_a), claim.clone());
		assert_ok!(PoeModule::transfer_claim(Origin::signed(user_a), user_b, claim.clone()));
		assert_eq!(Proofs::<Test>::get(&claim),Some((user_b,frame_system::Pallet::<Test>::block_number())))
	});
}

/*
运行结果：
running 10 tests
test mock::__construct_runtime_integrity_test::runtime_integrity_tests ... ok
test tests::transfer_claim_not_exist ... ok
test tests::revoke_claim_not_exist ... ok
test tests::create_too_large_claim_failed ... ok
test tests::create_claim_ok ... ok
test tests::revoke_claim_ok ... ok
test tests::transfer_claim_claim_ok ... ok
test tests::transfer_claim_not_claim_owner ... ok
test tests::create_duplicated_claim_failed ... ok
test tests::revoke_claim_not_owner_failed ... ok
*/














