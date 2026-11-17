pub trait Matcher {
    type Output;
    fn match_string(self, data: &String, cursor: &mut TracePos) -> Result<Self::Output, String>
    where
        Self: Sized;
}

#[derive(Clone, Copy)]
pub struct Keyword {
    str: &'static str,
}

impl Keyword {
    pub fn new(str: &'static str) -> Self {
        Self { str }
    }
}

impl Matcher for Keyword {
    type Output = ();
    fn match_string(self, data: &String, cursor: &mut TracePos) -> Result<Self::Output, String>
    where
        Self: Sized,
    {
        if data.get(cursor.char..(self.str.len() + cursor.char)) == Some(self.str) {
            cursor.shift_by_str(self.str);
            Ok(())
        } else {
            Err(cursor.err(format!("Expected {}", self.str)))
        }
    }
}

#[derive(Clone)]
pub struct TracePos {
    pub line: usize,
    pub col: usize,
    pub char: usize,
    pub in_indent: usize,
}

impl TracePos {
    pub fn shift_by_str(&mut self, str: &'static str) {
        let newlines: Vec<(usize, char)> = str
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '\n')
            .collect();
        if newlines.len() > 0 {
            self.line += newlines.len();
            self.col = str.len() - newlines[newlines.len() - 1].0 - 1;
        }
        self.char += str.len()
    }
    pub fn err(&self, msg: String) -> String {
        format!("{} at {}", msg, self)
    }
    pub fn newline(&mut self) {
        self.line += 1;
        self.col = 0;
        self.char += 1;
    }
    pub fn shift(&mut self, count: usize) {
        self.col += count;
        self.char += count;
    }
    pub fn shift_by_char(&mut self, char: char) {
        self.char += char.len_utf8();
        self.col += char.len_utf8();
        if char == '\n' {
            self.line += 1;
            self.col = 0;
        }
    }
    pub fn new() -> Self {
        Self {
            line: 0,
            col: 0,
            char: 0,
            in_indent: 0,
        }
    }
    pub fn variant_err(&self, msgs: Vec<String>) -> String {
        let mut err = format!("All variants failed at {}", self);
        for i_msg in msgs {
            err.push_str(
                format!("\n    {}", i_msg.replace(|char| char == '\n', "\n    ")).as_str(),
            );
        }
        err
    }
}

impl std::fmt::Display for TracePos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {}, column {}", self.line, self.col)
    }
}

#[derive(Clone, Copy)]
pub struct StringMatcher();

impl Matcher for StringMatcher {
    type Output = String;
    fn match_string(self, data: &String, cursor: &mut TracePos) -> Result<Self::Output, String>
    where
        Self: Sized,
    {
        let chars = match data.get(cursor.char..) {
            Some(str) => str.chars(),
            None => return Err(cursor.err("Attempt to read past end of data".to_string())),
        };
        let mut chars = chars;
        match chars.next() {
            Some('"') => cursor.shift(1),
            Some(_) => {
                return Err(cursor.err("Expected double quote at start of string".to_string()));
            }
            None => return Err(cursor.err("Attempt to read past end of data".to_string())),
        }

        let mut out = String::new();
        let mut escaped = false;
        for char in chars {
            if escaped {
                if char == 'n' {
                    out.push('\n');
                    cursor.shift(1);
                } else {
                    out.push(char);
                    cursor.shift_by_char(char);
                }
                escaped = false;
                continue;
            }

            match char {
                '\\' => {
                    cursor.shift(1);
                    escaped = true;
                }
                '"' => {
                    cursor.shift(1);
                    return Ok(out);
                }
                other => {
                    out.push(other);
                    cursor.shift_by_char(other);
                }
            }
        }

        Err(cursor.err("Attempt to read past end of data".to_string()))
    }
}

#[derive(Clone)]
pub struct FilterMatcher<F: FnMut(char) -> bool>(pub F);

impl<F: FnMut(char) -> bool> Matcher for FilterMatcher<F> {
    type Output = String;
    fn match_string(mut self, data: &String, cursor: &mut TracePos) -> Result<Self::Output, String>
    where
        Self: Sized,
    {
        let mut out = String::new();
        let chars = match data.get(cursor.char..) {
            Some(str) => str.chars(),
            None => return Err(cursor.err("Attempt to read past end of data".to_string())),
        };
        for char in chars {
            if self.0(char) {
                out.push(char);
                cursor.shift_by_char(char);
            } else {
                break;
            }
        }
        Ok(out)
    }
}

#[derive(Clone)]
pub struct MultipleMatcher<T: Matcher + Clone>(T);

impl<T: Matcher + Clone> Matcher for MultipleMatcher<T> {
    type Output = Vec<T::Output>;
    fn match_string(self, data: &String, cursor: &mut TracePos) -> Result<Self::Output, String>
    where
        Self: Sized,
    {
        let mut out = Vec::new();
        loop {
            let prev_cursor = cursor.clone();
            match self.0.clone().match_string(data, cursor) {
                Ok(o) => out.push(o),
                Err(_) => {
                    *cursor = prev_cursor;
                    return Ok(out);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Keyword, Matcher, StringMatcher, TracePos};

    crate::variant_matcher! {
        TokenMatcher for Token {
            Text(text: StringMatcher),
            QuotedStart(quoted_start: Keyword),
        }
    }

    #[test]
    fn variant_matcher_returns_matcher_output_as_variant_data() {
        let mut cursor = TracePos::new();
        let result = TokenMatcher::new(StringMatcher(), Keyword::new("\""))
            .match_string(&"\"hello\"".to_string(), &mut cursor)
            .unwrap();

        assert!(matches!(result, Token::Text(text) if text == "hello"));
        assert_eq!(cursor.char, 7);
    }

    #[test]
    fn variant_matcher_rolls_back_cursor_between_alternatives() {
        let mut cursor = TracePos::new();
        let result = TokenMatcher::new(StringMatcher(), Keyword::new("\""))
            .match_string(&"\"unterminated".to_string(), &mut cursor)
            .unwrap();

        assert!(matches!(result, Token::QuotedStart(())));
        assert_eq!(cursor.char, 1);
    }

    #[test]
    fn variant_matcher_combines_errors_without_advancing_cursor() {
        let mut cursor = TracePos::new();
        let error = TokenMatcher::new(StringMatcher(), Keyword::new("x"))
            .match_string(&"y".to_string(), &mut cursor)
            .err()
            .unwrap();

        assert!(error.contains("All variants failed"));
        assert_eq!(cursor.char, 0);
    }
}
