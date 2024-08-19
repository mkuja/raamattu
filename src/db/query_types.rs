use std::fmt::Display;

#[derive(sqlx::FromRow, Debug, Clone)]
#[allow(dead_code)]
pub struct Word {
    pub index: u64,
    pub word: String,
    pub ends_with_rparen: u8,
    pub ends_with_colon: u8,
    pub ends_with_period: u8,
    pub ends_with_comma: u8,
    pub ends_with_quote: u8,
    pub ends_with_dquote: u8,
    pub ends_with_semicolon: u8,
    pub starts_with_quote: u8,
    pub starts_with_dquote: u8,
    pub starts_with_lparen: u8,
    pub book_number: i32,
    pub chapter_number: i32,
    pub verse_number: i32,
//    verse: Option<Arc<Verse>>,
}

// {{{ impl Display for Word 
impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        // Handle possible single character prefix to the word.
        if self.starts_with_lparen > 0 {
            result.push('(');
        } else if self.starts_with_quote > 0 {
            result.push('\'');
        } else if self.starts_with_dquote > 0 {
            result.push('"');
        }

        // Add the word.
        result.push_str(&self.word);

        // Handle possible one or two character postfix to the word.
        result.push_str("  ");

        // RPAREN
        if self.ends_with_rparen > 0 {
            if self.ends_with_rparen == 2 {
                let _ = result.pop();
                result.push(')')
            } else {
                // Else it is 1
                let tmp = result.pop();
                let _ = result.pop();
                result.push(')');
                result.push(tmp.unwrap());
            }
        }

        // COLON
        if self.ends_with_colon > 0 {
            if self.ends_with_colon == 2 {
                let _ = result.pop();
                result.push(':')
            } else {
                // Else it is 1
                let tmp = result.pop();
                let _ = result.pop();
                result.push(':');
                result.push(tmp.unwrap());
            }
        }

        // PERIOD
        if self.ends_with_period > 0 {
            if self.ends_with_period == 2 {
                let _ = result.pop();
                result.push('.')
            } else {
                // Else it is 1
                let tmp = result.pop();
                let _ = result.pop();
                result.push('.');
                result.push(tmp.unwrap());
            }
        }

        // COMMA
        if self.ends_with_comma > 0 {
            if self.ends_with_comma == 2 {
                let _ = result.pop();
                result.push(',')
            } else {
                // Else it is 1
                let tmp = result.pop();
                let _ = result.pop();
                result.push(',');
                result.push(tmp.unwrap());
            }
        }

        // QUOTE
        if self.ends_with_quote > 0 {
            if self.ends_with_quote == 2 {
                let _ = result.pop();
                result.push('\'')
            } else {
                // Else it is 1
                let tmp = result.pop();
                let _ = result.pop();
                result.push('\'');
                result.push(tmp.unwrap());
            }
        }

        // DOUBLE QUOTE
        if self.ends_with_dquote > 0 {
            if self.ends_with_dquote == 2 {
                let _ = result.pop();
                result.push('"')
            } else {
                // Else it is 1
                let tmp = result.pop();
                let _ = result.pop();
                result.push('"');
                result.push(tmp.unwrap());
            }
        }

        // SEMICOLON
        if self.ends_with_semicolon > 0 {
            if self.ends_with_semicolon == 2 {
                let _ = result.pop();
                result.push(';')
            } else {
                // Else it is 1
                let tmp = result.pop();
                let _ = result.pop();
                result.push(';');
                result.push(tmp.unwrap());
            }
        }
        write!(f, "{}", result.trim())
    }
}

