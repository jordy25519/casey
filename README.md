# Casey
Case transforming macros

Casey can transform the case of given `ident`s.
Niche but maybe useful in other macros.
```rust
use casey::{camel, lower, shouty, snake, upper};

lower!(ABC); // renders as: `abc`
upper!(abc); // renders as: `ABC`
snake!(ABC); // renders as: `a_b_c`
camel!(ab_c); // renders as: `AbC`
shouty!(a_b_c); // renders as: `A_B_C`
```
