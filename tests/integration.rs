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
    assert!(pascal!(HELLO_WORLD)());
}

#[test]
fn it_works_to_shoutycase() {
    const HELLO_WORLD: bool = true;

    assert!(shouty!(helloWorld));
    assert!(shouty!(hello_world));
    assert!(shouty!(HelloWorld));
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
    let a = "hello";
    let b = "world";
    lower!(println!("{} {}", A, B));
}

#[test]
fn declare_struct() {
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
            fn Test() -> bool { true }
        }
    );
    assert!(mockstruct::test());

    // type names will be affected by the case change
    type BOOL = bool;
    upper!(
        struct MockStruct {}
        impl MockStruct {
            fn Test() -> bool { true }
        }
    );
    assert!(MOCKSTRUCT::TEST());

    // type names will be affected by the case change
    type Bool = bool;
    pascal!(
    struct MOCK_STRUCT {}
    impl MOCK_STRUCT {
        fn TEST() -> bool { true }
    });
    assert!(MockStruct::Test());
}
