# Casey
[![Build Status](https://travis-ci.org/holygits/casey.svg?branch=master)](https://travis-ci.org/holygits/casey)  

Case transforming macros

Casey transforms the case of given input `ident`s.  
Niche but maybe useful in other macros.  
```rust
use casey::{pascal, lower, shouty, snake, upper};

lower!(ABC);    // renders: `abc`
upper!(abc);    // `ABC`
snake!(ABC);    // `a_b_c`
pascal!(ab_c);  // `AbC`
shouty!(a_b_c); // `A_B_C`
```

## Token Stream
Casey macros can operate on `TokenStream`s e.g.  
```rust
    snake!(
        struct MockStruct {}
        impl MockStruct {
            fn test() -> bool { true }
        }
    );
    assert!(mock_struct::test());
```
All `ident` tokens in the stream will have the case transformation applied (keywords will be ignored).  

## Gotchas
Type names, including built-in types are not valid identifiers e.g. `bool`, `usize`, `i32` etc. and **will** be transformed by casey.  
```rust
pascal!(let test: bool = true); // renders: `let Test: Bool = true;`
```
