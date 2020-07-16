//!
//! String case transformation extension traits
//!

/// Word boundary separators
const SEPARATORS: &'static str = "-_";

pub trait PascalCaseExt {
    fn to_pascal_case(&self) -> String;
}

impl PascalCaseExt for String {
    fn to_pascal_case(&self) -> String {
        let mut s = String::new();
        if let Some(c) = self.chars().nth(0) {
            s.push(c.to_uppercase().next().unwrap());
        }
        let mut follows_separator = false;
        for c in self.chars().skip(1).into_iter() {
            if SEPARATORS.contains(c) {
                follows_separator = true;
                continue;
            }
            if follows_separator {
                s.push(c.to_uppercase().next().unwrap());
                follows_separator = false;
            } else {
                s.push(c);
            }
        }
        s
    }
}

pub trait SnakeCaseExt {
    fn to_snake_case(&self) -> String;
}

impl SnakeCaseExt for String {
    fn to_snake_case(&self) -> String {
        let mut s = String::new();
        if let Some(c) = self.chars().nth(0) {
            s.push(c.to_lowercase().next().unwrap());
        }
        for c in self.chars().skip(1).into_iter() {
            if c.is_lowercase() {
                s.push(c)
            } else {
                s.push('_');
                if SEPARATORS.contains(c) {
                    continue;
                } else {
                    s.push(c.to_lowercase().next().unwrap())
                }
            }
        }
        s
    }
}

pub trait ShoutySnakeCaseExt {
    fn to_shouty_snake_case(&self) -> String;
}

impl ShoutySnakeCaseExt for String {
    fn to_shouty_snake_case(&self) -> String {
        let mut s = String::new();
        if let Some(c) = self.chars().nth(0) {
            s.push(c.to_uppercase().next().unwrap());
        }
        for c in self.chars().skip(1).into_iter() {
            if c.is_uppercase() {
                s.push('_');
                s.push(c)
            } else {
                if SEPARATORS.contains(c) {
                    s.push('_');
                    continue;
                }
                if let Some(n) = c.to_uppercase().next() {
                    s.push(n)
                }
            }
        }
        s
    }
}
