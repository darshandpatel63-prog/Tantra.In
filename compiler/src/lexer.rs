use std::collections::HashMap;

use unicode_ident::{is_xid_continue, is_xid_start};
use unicode_normalization::UnicodeNormalization;
use unicode_security::confusable_detection::char_confusable_prototype;

use crate::diagnostic::{Diagnostic, Span};
use crate::token::{Token, TokenKind};

pub struct Lexer<'a> {
    source: &'a str,
    chars: Vec<char>,
    offsets: Vec<usize>,
    index: usize,
    diagnostics: Vec<Diagnostic>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut chars = Vec::new();
        let mut offsets = Vec::new();
        for (offset, ch) in source.char_indices() {
            offsets.push(offset);
            chars.push(ch);
        }
        offsets.push(source.len());

        Self {
            source,
            chars,
            offsets,
            index: 0,
            diagnostics: Vec::new(),
        }
    }

    pub fn lex(mut self) -> (Vec<Token>, Vec<Diagnostic>) {
        let mut tokens = Vec::new();

        while !self.at_end() {
            if self.skip_whitespace_and_comments() {
                continue;
            }

            let start = self.position();
            let ch = self.advance().unwrap();

            let token = match ch {
                c if is_identifier_start(c) => self.identifier(start),
                c if c.is_ascii_digit() => self.number(start, c),
                '"' => self.string(start),
                '\'' => self.character(start),
                '+' => TokenKind::Plus,
                '-' if self.match_char('>') => TokenKind::Arrow,
                '-' => TokenKind::Minus,
                '*' if self.match_char('*') => TokenKind::Power,
                '*' => TokenKind::Star,
                '/' => TokenKind::Slash,
                '%' => TokenKind::Percent,
                '!' if self.match_char('=') => TokenKind::NotEqual,
                '!' => TokenKind::Bang,
                '=' if self.match_char('=') => TokenKind::EqualEqual,
                '=' => TokenKind::Equal,
                '<' if self.match_char('=') => TokenKind::LessEqual,
                '<' if self.match_char('<') => TokenKind::ShiftLeft,
                '<' => TokenKind::Less,
                '>' if self.match_char('=') => TokenKind::GreaterEqual,
                '>' if self.match_char('>') => TokenKind::ShiftRight,
                '>' => TokenKind::Greater,
                '&' if self.match_char('&') => TokenKind::AndAnd,
                '&' => TokenKind::Ampersand,
                '^' => TokenKind::Caret,
                '|' if self.match_char('|') => TokenKind::OrOr,
                '|' => TokenKind::Pipe,
                '?' if self.match_char('?') => TokenKind::NullCoalesce,
                '?' => TokenKind::Question,
                ':' => TokenKind::Colon,
                '(' => TokenKind::LeftParen,
                ')' => TokenKind::RightParen,
                '{' => TokenKind::LeftBrace,
                '}' => TokenKind::RightBrace,
                '[' => TokenKind::LeftBracket,
                ']' => TokenKind::RightBracket,
                ',' => TokenKind::Comma,
                '.' => TokenKind::Dot,
                ';' => TokenKind::Semicolon,
                _ => {
                    self.diagnostics.push(Diagnostic::error(
                        "T0001",
                        format!("અમાન્ય અક્ષર: {ch:?}"),
                        Span::new(start, self.position()),
                    ));
                    continue;
                }
            };

            tokens.push(Token::new(token, start, self.position()));
        }

        let end = self.source.len();
        tokens.push(Token::new(TokenKind::Eof, end, end));
        (tokens, self.diagnostics)
    }

    fn identifier(&mut self, start: usize) -> TokenKind {
        while self.peek().is_some_and(is_identifier_continue) {
            self.advance();
        }

        let raw = &self.source[self.offset(start)..self.offset(self.position())];
        let normalized: String = raw.nfc().collect();

        if let Some((character, prototype)) = normalized
            .chars()
            .find_map(|character| confusable_ascii_prototype(character).map(|prototype| (character, prototype)))
        {
            self.diagnostics.push(Diagnostic::error(
                "T0011",
                format!(
                    "identifierમાં confusable Unicode અક્ષર {character:?} છે; ASCII prototype {prototype:?} છે"
                ),
                Span::new(self.offset(start), self.offset(self.position())),
            ));
        }

        keyword(&normalized).unwrap_or_else(|| TokenKind::Identifier(normalized))
    }

    fn number(&mut self, start: usize, first: char) -> TokenKind {
        let mut malformed = false;

        if first == '0' {
            if matches!(self.peek(), Some('x' | 'X')) {
                self.advance();
                let digits_start = self.position();
                while self
                    .peek()
                    .is_some_and(|c| c.is_ascii_alphanumeric() || c == '_')
                {
                    self.advance();
                }
                let digits =
                    &self.source[self.offset(digits_start)..self.offset(self.position())];
                if !valid_digit_sequence(digits, |c| c.is_ascii_hexdigit()) {
                    malformed = true;
                }
                if self.peek().is_some_and(is_identifier_start) {
                    malformed = true;
                }
                if malformed {
                    self.report_invalid_number(start);
                }
                return TokenKind::Integer(self.lexeme(start).to_owned());
            }

            if matches!(self.peek(), Some('b' | 'B')) {
                self.advance();
                let digits_start = self.position();
                while self
                    .peek()
                    .is_some_and(|c| c.is_ascii_alphanumeric() || c == '_')
                {
                    self.advance();
                }
                let digits =
                    &self.source[self.offset(digits_start)..self.offset(self.position())];
                if !valid_digit_sequence(digits, |c| matches!(c, '0' | '1')) {
                    malformed = true;
                }
                if self.peek().is_some_and(is_identifier_start) {
                    malformed = true;
                }
                if malformed {
                    self.report_invalid_number(start);
                }
                return TokenKind::Integer(self.lexeme(start).to_owned());
            }

            if matches!(self.peek(), Some('o' | 'O')) {
                self.advance();
                let digits_start = self.position();
                while self
                    .peek()
                    .is_some_and(|c| c.is_ascii_alphanumeric() || c == '_')
                {
                    self.advance();
                }
                let digits =
                    &self.source[self.offset(digits_start)..self.offset(self.position())];
                if !valid_digit_sequence(digits, |c| matches!(c, '0'..='7')) {
                    malformed = true;
                }
                if self.peek().is_some_and(is_identifier_start) {
                    malformed = true;
                }
                if malformed {
                    self.report_invalid_number(start);
                }
                return TokenKind::Integer(self.lexeme(start).to_owned());
            }
        }

        self.consume_digits(|c| c.is_ascii_digit() || c == '_');
        let integer_part = self.lexeme(start);
        if !valid_digit_sequence(integer_part, |c| c.is_ascii_digit()) {
            malformed = true;
        }

        let mut is_float = false;

        if self.peek() == Some('.') && self.peek_next().is_some_and(|c| c.is_ascii_digit()) {
            is_float = true;
            self.advance();
            let fraction_start = self.position();
            self.consume_digits(|c| c.is_ascii_digit() || c == '_');
            let fraction =
                &self.source[self.offset(fraction_start)..self.offset(self.position())];
            if !valid_digit_sequence(fraction, |c| c.is_ascii_digit()) {
                malformed = true;
            }
        }

        if matches!(self.peek(), Some('e' | 'E')) {
            is_float = true;
            self.advance();
            if matches!(self.peek(), Some('+' | '-')) {
                self.advance();
            }
            let exponent_start = self.position();
            self.consume_digits(|c| c.is_ascii_digit() || c == '_');
            let exponent =
                &self.source[self.offset(exponent_start)..self.offset(self.position())];
            if !valid_digit_sequence(exponent, |c| c.is_ascii_digit()) {
                malformed = true;
            }
        }

        if self.peek().is_some_and(is_identifier_start) {
            malformed = true;
        }

        if malformed {
            self.report_invalid_number(start);
        }

        let value = self.lexeme(start).to_owned();
        if is_float {
            TokenKind::Float(value)
        } else {
            TokenKind::Integer(value)
        }
    }

    fn report_invalid_number(&mut self, start: usize) {
        self.diagnostics.push(Diagnostic::error(
            "T0009",
            "અમાન્ય numeric literal",
            Span::new(self.offset(start), self.offset(self.position())),
        ));
    }

    fn string(&mut self, start: usize) -> TokenKind {
        let mut value = String::new();
        while let Some(ch) = self.advance() {
            match ch {
                '"' => return TokenKind::String(value),
                '\\' => match self.advance() {
                    Some('n') => value.push('\n'),
                    Some('r') => value.push('\r'),
                    Some('t') => value.push('\t'),
                    Some('"') => value.push('"'),
                    Some('\\') => value.push('\\'),
                    Some('0') => value.push('\0'),
                    Some(other) => {
                        self.diagnostics.push(Diagnostic::error(
                            "T0004",
                            format!("અમાન્ય string escape: \\{other}"),
                            Span::new(
                                self.offset(self.position().saturating_sub(2)),
                                self.offset(self.position()),
                            ),
                        ));
                    }
                    None => break,
                },
                '\n' | '\r' => {
                    self.diagnostics.push(Diagnostic::error(
                        "T0005",
                        "string literal બંધ થયું નથી",
                        Span::new(start, self.position()),
                    ));
                    break;
                }
                other => value.push(other),
            }
        }

        if self.at_end() {
            self.diagnostics.push(Diagnostic::error(
                "T0005",
                "string literal બંધ થયું નથી",
                Span::new(start, self.position()),
            ));
        }

        TokenKind::String(value)
    }

    fn character(&mut self, start: usize) -> TokenKind {
        let value = match self.advance() {
            Some('\\') => match self.advance() {
                Some('n') => '\n',
                Some('r') => '\r',
                Some('t') => '\t',
                Some('0') => '\0',
                Some('\\') => '\\',
                Some('\'') => '\'',
                Some(other) => {
                    self.diagnostics.push(Diagnostic::error(
                        "T0006",
                        format!("અમાન્ય character escape: \\{other}"),
                        Span::new(start, self.position()),
                    ));
                    other
                }
                None => '\0',
            },
            Some(ch) => ch,
            None => '\0',
        };

        if self.peek() == Some('\'') {
            self.advance();
        } else {
            self.diagnostics.push(Diagnostic::error(
                "T0007",
                "character literal બંધ થયું નથી",
                Span::new(start, self.position()),
            ));
        }

        TokenKind::Character(value)
    }

    fn skip_whitespace_and_comments(&mut self) -> bool {
        let mut skipped = false;

        loop {
            while self.peek().is_some_and(|c| c.is_whitespace()) {
                skipped = true;
                self.advance();
            }

            if self.peek() == Some('/') && self.peek_next() == Some('/') {
                skipped = true;
                self.advance();
                self.advance();
                while self.peek().is_some_and(|c| c != '\n' && c != '\r') {
                    self.advance();
                }
                continue;
            }

            if self.peek() == Some('/') && self.peek_next() == Some('*') {
                skipped = true;
                let start = self.position();
                self.advance();
                self.advance();
                let mut depth = 1usize;

                while depth > 0 && !self.at_end() {
                    if self.peek() == Some('/') && self.peek_next() == Some('*') {
                        self.advance();
                        self.advance();
                        depth += 1;
                    } else if self.peek() == Some('*') && self.peek_next() == Some('/') {
                        self.advance();
                        self.advance();
                        depth -= 1;
                    } else {
                        self.advance();
                    }
                }

                if depth != 0 {
                    self.diagnostics.push(Diagnostic::error(
                        "T0008",
                        "block comment બંધ થયું નથી",
                        Span::new(self.offset(start), self.source.len()),
                    ));
                }
                continue;
            }

            break;
        }

        skipped
    }

    fn consume_digits<F>(&mut self, mut predicate: F)
    where
        F: FnMut(char) -> bool,
    {
        while self.peek().is_some_and(&mut predicate) {
            self.advance();
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.index).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.index + 1).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.index += 1;
        Some(ch)
    }

    fn at_end(&self) -> bool {
        self.index >= self.chars.len()
    }

    fn position(&self) -> usize {
        self.index
    }

    fn offset(&self, position: usize) -> usize {
        self.offsets[position.min(self.offsets.len() - 1)]
    }

    fn lexeme(&self, start: usize) -> &str {
        &self.source[self.offset(start)..self.offset(self.position())]
    }
}

fn is_identifier_start(c: char) -> bool {
    c == '_' || is_xid_start(c)
}

fn is_identifier_continue(c: char) -> bool {
    c == '_' || is_xid_continue(c)
}

fn valid_digit_sequence<F>(text: &str, predicate: F) -> bool
where
    F: Fn(char) -> bool,
{
    !text.is_empty()
        && !text.starts_with('_')
        && !text.ends_with('_')
        && !text.contains("__")
        && text.chars().all(|c| c == '_' || predicate(c))
}

fn confusable_ascii_prototype(character: char) -> Option<char> {
    if character.is_ascii() || !character.is_alphanumeric() {
        return None;
    }

    let prototype = char_confusable_prototype(character)?;
    if prototype.len() == 1 {
        let candidate = prototype[0];
        if candidate.is_ascii_alphanumeric() {
            return Some(candidate);
        }
    }

    None
}

fn keyword(text: &str) -> Option<TokenKind> {
    let mut map = HashMap::new();
    for (key, kind) in [
        ("સ્થિર", TokenKind::Const),
        ("બદલ", TokenKind::Mut),
        ("કાર્ય", TokenKind::Fn),
        ("પરત", TokenKind::Return),
        ("જો", TokenKind::If),
        ("નહીં", TokenKind::Else),
        ("તો", TokenKind::Then),
        ("માટે", TokenKind::For),
        ("દરેક", TokenKind::Each),
        ("માં", TokenKind::In),
        ("જ્યારે", TokenKind::While),
        ("તોડો", TokenKind::Break),
        ("આગળ", TokenKind::Continue),
        ("આયાત", TokenKind::Import),
        ("માંથી", TokenKind::From),
        ("રૂપ", TokenKind::Type),
        ("પ્રયત્ન", TokenKind::Try),
        ("ભૂલ", TokenKind::Error),
        ("ફેંકો", TokenKind::Throw),
        ("સાચું", TokenKind::True),
        ("ખોટું", TokenKind::False),
        ("શૂન્ય", TokenKind::Null),
        ("async", TokenKind::Async),
        ("await", TokenKind::Await),
        ("ક્ષમતા", TokenKind::Capability),
        ("જાહેર", TokenKind::Public),
        ("ખાનગી", TokenKind::Private),
        ("મોડ્યુલ", TokenKind::Module),
    ] {
        map.insert(key, kind);
    }
    map.get(text).cloned()
}
