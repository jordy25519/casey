#![cfg(test)]

use casey::{lower, pascal, shouty, snake, upper};

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

#[test]
fn it_works_to_pascalcase() {
    #[allow(non_snake_case)]
    fn HelloWorld() -> bool {
        true
    }

    assert!(pascal!(helloWorld)());
    assert!(pascal!(hello_world)());
}

#[test]
fn it_works_to_shoutycase() {
    const HELLO_WORLD: bool = true;

    assert!(shouty!(helloWorld));
    assert!(shouty!(hello_world));
}

mod abc {
    pub fn abc() -> bool {
        true
    }
}

#[test]
fn it_works_with_mod() {
    assert!(lower!(ABC::abc()));
}

#[test]
fn macro_args() {
    #[allow(non_snake_case)]
    let A = "hello";
    #[allow(non_snake_case)]
    let B = "world";
    lower!(println!("{} {}", A, B));
}

#[test]
fn declare_struct() {
    // Gotcha here is that syntactic tokens are idents as well e.g. `fn`, `impl`, `struct`
    // This means only transformation which produce valid idents will produce valid rust
    // e.g. `upper!(struct) => STRUCT` is invalid...
    snake!(
        struct MockStruct {}
        impl MockStruct {
            fn test() -> bool { true }
        }
    );
    assert!(mock_struct::test());

    lower!(
        struct MockStruct {}
        impl MockStruct {
            fn test() -> bool { true }
        }
    );
    assert!(mockstruct::test());
}
