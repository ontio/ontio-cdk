use crate::{MyToken, MyTokenInstance};
use ontio_std::mock::build_runtime;
use ontio_std::types::Address;

#[test]
fn get_total_supply() {
    let mut token = MyTokenInstance;
    let owner = &Address::random();
    assert!(token.initialize(owner));
    assert_eq!(token.initialize(owner), false);
    let total = token.total_supply();
    assert_eq!(token.balance_of(owner), total);
}

#[test]
fn transfer_no_witness() {
    let mut token = MyTokenInstance;
    let owner = &Address::random();
    let b = &Address::random();
    assert!(token.initialize(owner));
    assert_eq!(token.transfer(owner, b, 123), false);
}

#[test]
fn transfer() {
    let owner = &Address::random();
    let b = &Address::random();
    build_runtime().witness(&[owner]);
    let mut token = MyTokenInstance;
    assert!(token.initialize(owner));

    assert_eq!(token.transfer(owner, b, 123), true);
    assert_eq!(token.balance_of(b), 123);

    let total = token.total_supply();
    assert_eq!(token.balance_of(owner), total - 123);
}

#[test]
fn approve() {
    let owner = &Address::random();
    let alice = &Address::random();
    let mut token = MyTokenInstance;
    let handle = build_runtime();
    handle.witness(&[owner]);
    assert!(token.initialize(owner));
    assert!(token.approve(owner, alice, 100));
    assert_eq!(token.allowance(owner, alice), 100);
    assert!(!token.transfer_from(alice, owner, 100));
    handle.witness(&[alice]);
    assert!(token.transfer_from(alice, owner, 100));
    assert_eq!(token.allowance(owner, alice), 0);
}

#[test]
fn transfer_multi() {
    let owner = &Address::random();
    let alice = &Address::random();
    let bob = &Address::random();
    build_runtime().witness(&[owner, alice]);
    let mut token = MyTokenInstance;
    assert!(token.initialize(owner));
    let states = [(owner.clone(), alice.clone(), 1), (owner.clone(), bob.clone(), 2)];
    assert_eq!(token.transfer_multi(&states), true);
    assert_eq!(token.balance_of(&alice), 1);
    assert_eq!(token.balance_of(&bob), 2);
}
