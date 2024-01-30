pub struct Scanner {
    chars: Vec<char>,
    cursor: usize,
}

impl Scanner {
    pub fn new(string: String) -> Self {
        Self {
            chars: string.chars().collect(),
            cursor: 0,
        }
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn peek(&self) -> Option<&char> {
        self.chars.get(self.cursor)
    }

    pub fn is_at_end(&self) -> bool {
        self.cursor == self.chars.len()
    }

    pub fn pop(&mut self) -> Option<&char> {
        match self.chars.get(self.cursor) {
            Some(c) => {
                self.cursor += 1;
                Some(c)
            },
            None => None,
        }
    }

    pub fn match_char(&mut self, target: &char) -> bool {
        match self.chars.get(self.cursor) {
            Some(c) => {
                match target == c {
                    true => { self.cursor += 1; return true},
                    false => return false,
                }
            },
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner_routine() {
        let mut scanner = Scanner::new("test".to_string());

        assert_eq!(scanner.cursor(), 0);
        assert_eq!(scanner.peek(), Some(&'t'));
        assert_eq!(scanner.pop(), Some(&'t'));
        assert_eq!(scanner.is_at_end(), false);
    }
}
