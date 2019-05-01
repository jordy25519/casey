# Casey
Case transforming macros

Casey can transform the case of given `ident`s.
Niche but maybe useful in other macros.
```rust
use casey::{lower, snake, upper};

lower!(ABC); // renders as: `abc`
upper!(abc); // renders as: `ABC`
snake!(ABC); // renders as: `a_b_c`
```
