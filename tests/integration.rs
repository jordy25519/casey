#![cfg(test)]
#![feature(proc_macro_hygiene)]

use casey::{lower, snake, upper};

#[test]
fn it_works_to_uppercase() {
    #[allow(non_snake_case)]
    fn ABC() -> bool {
        true
    }
    assert!(upper!(abc)());
}

#[test]
fn it_works_to_lowercase() {
    fn abc() -> bool {
        true
    }
    assert!(lower!(ABC)());
}

#[test]
fn it_works_to_snakecase() {
    fn a_b_c() -> bool {
        true
    }
    fn a_b_c_1() -> bool {
        true
    }
    assert!(snake!(ABC)());
    assert!(snake!(aBC)());
    assert!(snake!(a_b_c)());
    assert!(snake!(a_bC1)());
}
