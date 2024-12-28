//!
//! String case transformation extension traits
//!

/// Word boundary separators
const SEPARATORS: &str = "-_";

pub trait PascalCaseExt {
    fn to_pascal_case(&self) -> String;
}

impl PascalCaseExt for String {
    fn to_pascal_case(&self) -> String {
        let mut s = String::new();
        let mut capitalise_next = true; // always on first char

        let mut char_stream = self.chars().peekable();
        while let Some(current_char) = char_stream.next() {
            let next_char = char_stream.peek();
            if next_char.is_none() {
                // `current_char` is last in the stream
                s.push(current_char.to_lowercase().next().unwrap());
                break;
            }

            if SEPARATORS.contains(current_char) {
                capitalise_next = true;
                continue;
            }
            if current_char.is_numeric() {
                capitalise_next = true;
                s.push(current_char);
                continue;
            }

            if capitalise_next {
                s.push(current_char.to_uppercase().next().unwrap());
                capitalise_next = false;
                continue;
            }

            // lowercase this char if followed by another uppercase or punctuation
            // e.g. AA => aA, A- => a-
            // has the affect of transforming: 'ABCDe' into 'AbcDe'
            if current_char.is_ascii_uppercase()
                && next_char
                    .is_some_and(|x| x.is_numeric() || x.is_uppercase() || x.is_ascii_punctuation())
            {
                s.push(current_char.to_lowercase().next().unwrap());
            } else {
                s.push(current_char);
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
        let mut was_sep = false;
        if let Some(c) = self.chars().next() {
            was_sep = SEPARATORS.contains(c);
            s.push(c.to_lowercase().next().unwrap());
        }
        for c in self.chars().skip(1) {
            if c.is_lowercase() {
                was_sep = false;
                s.push(c);
            } else {
                if !was_sep {
                    s.push('_');
                }
                was_sep = SEPARATORS.contains(c);
                if was_sep {
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
        if let Some(c) = self.chars().next() {
            s.push(c.to_uppercase().next().unwrap());
        }
        for c in self.chars().skip(1) {
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
