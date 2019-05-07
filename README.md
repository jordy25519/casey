# Casey
[![Build Status](https://travis-ci.org/holygits/casey.svg?branch=master)](https://travis-ci.org/holygits/casey)
Case transforming macros

Casey can transform the case of given `ident`s.  
Niche but maybe useful in other macros.
```rust
use casey::{camel, lower, shouty, snake, upper};

lower!(ABC);    // renders: `abc`
upper!(abc);    // renders: `ABC`
snake!(ABC);    // renders: `a_b_c`
camel!(ab_c);   // renders: `AbC`
shouty!(a_b_c); // renders: `A_B_C`
```
