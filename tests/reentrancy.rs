use stylus_sdk::testing::{assert_err, assert_ok};
use reentrancy_guard_stylus::MyContract;

#[test]
fn test_safe_withdraw() {
    let mut contract = MyContract::default();
    contract.deposit(100);
    assert_ok!(contract.safe_withdraw(50));
    assert_eq!(contract.balance, 50);
}

#[test]
fn test_unsafe_withdraw() {
    let mut contract = MyContract::default();
    contract.deposit(100);
    contract.unsafe_withdraw(50);
    assert_eq!(contract.balance, 50); // Still reduces balance, but not reentrancy-protected
}