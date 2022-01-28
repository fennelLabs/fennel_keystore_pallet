use crate::{mock::*};
use frame_support::{assert_ok};

#[test]
fn issue_key() {
    new_test_ext().execute_with(|| {
        let mut fingerprint: Vec<u8> = Vec::new();
        let mut location: Vec<u8> = Vec::new();
        fingerprint.push(1);
        location.push(1);
        assert_ok!(KeystoreModule::issue_key(Origin::signed(1), fingerprint, location));
    });
}

