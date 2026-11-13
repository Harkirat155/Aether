use aether_diag::{Diagnostic, Position, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub span: Span,
}

impl Token {
    fn new(kind: TokenKind, lexeme: String, span: Span) -> Self {
        Self { kind, lexeme, span }
    }

    fn error(label: impl Into<String>, span: Span) -> Self {
        Self::new(TokenKind::Error, label.into(), span)
    }

    fn eof(position: Position) -> Self {
        Self::new(TokenKind::Eof, String::new(), Span::empty(position))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Keyword(Keyword),
    Contextual(ContextualToken),
    Identifier,
    Integer,
    Float,
    String,
    Bool(bool),
    None,
    Operator(Operator),
    Delimiter(Delimiter),
    Error,
    Eof,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    Agent,
    Team,
    Role,
    Goal,
    Beliefs,
    Desires,
    Intentions,
    Memory,
    Capabilities,
    Tool,
    Workflow,
    Protocol,
    Reason,
    Plan,
    Reflect,
    Believe,
    Intend,
    Effects,
    Requires,
    Policy,
    Await,
    HumanGate,
    Fn,
    Struct,
    Enum,
    Trait,
    Impl,
    Match,
    If,
    Else,
    While,
    For,
    Loop,
    Break,
    Continue,
    Return,
    Use,
    Const,
    Let,
    Mut,
    Some,
    Result,
    Ok,
    Err,
    Async,
    Spawn,
    Where,
    As,
    In,
    SelfValue,
    Super,
    Module,
    Pub,
    Private,
    Probabilistic,
}

impl Keyword {
    fn from_ident(value: &str) -> Option<Self> {
        Some(match value {
            "agent" => Self::Agent,
            "team" => Self::Team,
            "role" => Self::Role,
            "goal" => Self::Goal,
            "beliefs" => Self::Beliefs,
            "desires" => Self::Desires,
            "intentions" => Self::Intentions,
            "memory" => Self::Memory,
            "capabilities" => Self::Capabilities,
            "tool" => Self::Tool,
            "workflow" => Self::Workflow,
            "protocol" => Self::Protocol,
            "reason" => Self::Reason,
            "plan" => Self::Plan,
            "reflect" => Self::Reflect,
            "believe" => Self::Believe,
            "intend" => Self::Intend,
            "effects" => Self::Effects,
            "requires" => Self::Requires,
            "policy" => Self::Policy,
            "await" => Self::Await,
            "human_gate" => Self::HumanGate,
            "fn" => Self::Fn,
            "struct" => Self::Struct,
            "enum" => Self::Enum,
            "trait" => Self::Trait,
            "impl" => Self::Impl,
            "match" => Self::Match,
            "if" => Self::If,
            "else" => Self::Else,
            "while" => Self::While,
            "for" => Self::For,
            "loop" => Self::Loop,
            "break" => Self::Break,
            "continue" => Self::Continue,
            "return" => Self::Return,
            "use" => Self::Use,
            "const" => Self::Const,
            "let" => Self::Let,
            "mut" => Self::Mut,
            "some" => Self::Some,
            "result" => Self::Result,
            "ok" => Self::Ok,
            "err" => Self::Err,
            "async" => Self::Async,
            "spawn" => Self::Spawn,
            "where" => Self::Where,
            "as" => Self::As,
            "in" => Self::In,
            "self" => Self::SelfValue,
            "super" => Self::Super,
            "module" => Self::Module,
            "pub" => Self::Pub,
            "private" => Self::Private,
            "Probabilistic" => Self::Probabilistic,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextualToken {
    AmpMut,
    Begin,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Arrow,
    FatArrow,
    PathSep,
    AndAnd,
    OrOr,
    LessEq,
    GreaterEq,
    EqEq,
    BangEq,
    Assign,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Less,
    Greater,
    Bang,
    Amp,
    Pipe,
    Dot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Delimiter {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
    Semi,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LexOutput {
    pub tokens: Vec<Token>,
    pub diagnostics: Vec<Diagnostic>,
}

pub fn lex(source: &str) -> LexOutput {
    lex_bytes(source.as_bytes())
}

pub fn lex_bytes(bytes: &[u8]) -> LexOutput {
    let mut output = LexOutput::default();
    let mut position = Position::default();
    let mut remainder = bytes;

    while !remainder.is_empty() {
        match std::str::from_utf8(remainder) {
            Ok(valid) => {
                position = lex_valid_segment(valid, position, &mut output);
                remainder = &[];
            }
            Err(error) => {
                let valid_up_to = error.valid_up_to();

                if valid_up_to > 0 {
                    let valid = std::str::from_utf8(&remainder[..valid_up_to])
                        .expect("utf-8 prefix should already be validated");
                    position = lex_valid_segment(valid, position, &mut output);
                }

                let invalid_len = error.error_len().unwrap_or(1);
                let start = position;
                position = position.advance_bytes(invalid_len);
                let span = Span::new(start, position);

                output.tokens.push(Token::error("<invalid-utf8>", span));
                output.diagnostics.push(
                    Diagnostic::error("LEX001", "invalid UTF-8 sequence", span)
                        .with_fix_hint("Remove or replace the invalid byte sequence."),
                );

                remainder = &remainder[valid_up_to + invalid_len..];
            }
        }
    }

    output.tokens.push(Token::eof(position));
    output
}

fn lex_valid_segment(source: &str, start: Position, output: &mut LexOutput) -> Position {
    let mut lexer = ValidLexer {
        source,
        index: 0,
        position: start,
        output,
    };

    lexer.run();
    lexer.position
}

struct ValidLexer<'a> {
    source: &'a str,
    index: usize,
    position: Position,
    output: &'a mut LexOutput,
}

impl<'a> ValidLexer<'a> {
    fn run(&mut self) {
        while let Some(ch) = self.peek_char() {
            if ch.is_whitespace() {
                self.bump_char();
                continue;
            }

            if self.starts_with("//") {
                self.lex_line_comment();
                continue;
            }

            if self.starts_with("/*") {
                self.lex_block_comment();
                continue;
            }

            if self.starts_with_contextual_amp_mut() {
                self.emit_fixed(4, TokenKind::Contextual(ContextualToken::AmpMut));
                continue;
            }

            if is_ident_start(ch) {
                self.lex_identifier_or_keyword();
                continue;
            }

            if ch.is_ascii_digit() {
                self.lex_number();
                continue;
            }

            if ch == '"' {
                self.lex_string();
                continue;
            }

            if self.lex_multi_char_operator() || self.lex_single_char_token() {
                continue;
            }

            let start = self.position;
            self.bump_char();
            let span = Span::new(start, self.position);
            self.output.tokens.push(Token::error("<invalid-token>", span));
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.source[self.index..].chars().next()
    }

    fn peek_second_char(&self) -> Option<char> {
        let mut chars = self.source[self.index..].chars();
        chars.next()?;
        chars.next()
    }

    fn bump_char(&mut self) -> Option<char> {
        let ch = self.peek_char()?;
        self.index += ch.len_utf8();
        self.position = self.position.advance_char(ch);
        Some(ch)
    }

    fn starts_with(&self, value: &str) -> bool {
        self.source[self.index..].starts_with(value)
    }

    fn emit_fixed(&mut self, len: usize, kind: TokenKind) {
        let start_index = self.index;
        let start = self.position;

        for _ in 0..len {
            self.bump_char();
        }

        let span = Span::new(start, self.position);
        let lexeme = self.source[start_index..self.index].to_owned();
        self.output.tokens.push(Token::new(kind, lexeme, span));
    }

    fn lex_line_comment(&mut self) {
        self.bump_char();
        self.bump_char();

        while let Some(ch) = self.peek_char() {
            if ch == '\n' {
                break;
            }

            self.bump_char();
        }
    }

    fn lex_block_comment(&mut self) {
        let start = self.position;
        let start_index = self.index;
        self.bump_char();
        self.bump_char();

        let mut depth = 1;

        while depth > 0 {
            if self.starts_with("/*") {
                self.bump_char();
                self.bump_char();
                depth += 1;
                continue;
            }

            if self.starts_with("*/") {
                self.bump_char();
                self.bump_char();
                depth -= 1;
                continue;
            }

            if self.bump_char().is_none() {
                let span = Span::new(start, self.position);
                let lexeme = self.source[start_index..self.index].to_owned();
                self.output.tokens.push(Token::error(lexeme, span));
                self.output.diagnostics.push(
                    Diagnostic::error("LEX003", "unterminated block comment", span)
                        .with_fix_hint("Add a closing */ for the block comment."),
                );
                return;
            }
        }
    }

    fn lex_identifier_or_keyword(&mut self) {
        let start = self.position;
        let start_index = self.index;
        self.bump_char();

        while let Some(ch) = self.peek_char() {
            if !is_ident_continue(ch) {
                break;
            }

            self.bump_char();
        }

        let span = Span::new(start, self.position);
        let lexeme = self.source[start_index..self.index].to_owned();

        let kind = match lexeme.as_str() {
            "true" => TokenKind::Bool(true),
            "false" => TokenKind::Bool(false),
            "none" => TokenKind::None,
            "begin" => TokenKind::Contextual(ContextualToken::Begin),
            "end" => TokenKind::Contextual(ContextualToken::End),
            _ => Keyword::from_ident(&lexeme)
                .map(TokenKind::Keyword)
                .unwrap_or(TokenKind::Identifier),
        };

        self.output.tokens.push(Token::new(kind, lexeme, span));
    }

    fn lex_number(&mut self) {
        let start = self.position;
        let start_index = self.index;
        let mut is_float = false;

        self.consume_digits();

        if self.peek_char() == Some('.') && self.peek_second_char().is_some_and(|ch| ch.is_ascii_digit()) {
            is_float = true;
            self.bump_char();
            self.consume_digits();
        }

        if matches!(self.peek_char(), Some('e' | 'E')) {
            let checkpoint = (self.index, self.position);
            self.bump_char();

            if matches!(self.peek_char(), Some('+' | '-')) {
                self.bump_char();
            }

            if self.peek_char().is_some_and(|ch| ch.is_ascii_digit()) {
                is_float = true;
                self.consume_digits();
            } else {
                self.index = checkpoint.0;
                self.position = checkpoint.1;
            }
        }

        if self.peek_char().is_some_and(is_ident_continue) {
            while let Some(ch) = self.peek_char() {
                if !is_ident_continue(ch) {
                    break;
                }

                self.bump_char();
            }

            let span = Span::new(start, self.position);
            let lexeme = self.source[start_index..self.index].to_owned();
            self.output.tokens.push(Token::error(lexeme, span));
            self.output.diagnostics.push(
                Diagnostic::error("LEX004", "invalid numeric literal suffix or format", span)
                    .with_fix_hint("Separate the number from the identifier or use a supported numeric form."),
            );
            return;
        }

        let span = Span::new(start, self.position);
        let lexeme = self.source[start_index..self.index].to_owned();
        let kind = if is_float {
            TokenKind::Float
        } else {
            TokenKind::Integer
        };

        self.output.tokens.push(Token::new(kind, lexeme, span));
    }

    fn lex_string(&mut self) {
        let start = self.position;
        let start_index = self.index;
        self.bump_char();

        while let Some(ch) = self.peek_char() {
            match ch {
                '"' => {
                    self.bump_char();
                    let span = Span::new(start, self.position);
                    let lexeme = self.source[start_index..self.index].to_owned();
                    self.output.tokens.push(Token::new(TokenKind::String, lexeme, span));
                    return;
                }
                '\\' => {
                    self.bump_char();
                    if self.peek_char().is_some() {
                        self.bump_char();
                    }
                }
                '\n' => break,
                _ => {
                    self.bump_char();
                }
            }
        }

        let span = Span::new(start, self.position);
        let lexeme = self.source[start_index..self.index].to_owned();
        self.output.tokens.push(Token::error(lexeme, span));
        self.output.diagnostics.push(
            Diagnostic::error("LEX002", "unterminated string literal", span)
                .with_fix_hint("Add a closing quote for the string literal."),
        );
    }

    fn lex_multi_char_operator(&mut self) -> bool {
        let pairs = [
            ("->", TokenKind::Operator(Operator::Arrow)),
            ("=>", TokenKind::Operator(Operator::FatArrow)),
            ("::", TokenKind::Operator(Operator::PathSep)),
            ("&&", TokenKind::Operator(Operator::AndAnd)),
            ("||", TokenKind::Operator(Operator::OrOr)),
            ("<=", TokenKind::Operator(Operator::LessEq)),
            (">=", TokenKind::Operator(Operator::GreaterEq)),
            ("==", TokenKind::Operator(Operator::EqEq)),
            ("!=", TokenKind::Operator(Operator::BangEq)),
        ];

        for (value, kind) in pairs {
            if self.starts_with(value) {
                self.emit_fixed(2, kind);
                return true;
            }
        }

        false
    }

    fn lex_single_char_token(&mut self) -> bool {
        let kind = match self.peek_char() {
            Some('(') => TokenKind::Delimiter(Delimiter::LeftParen),
            Some(')') => TokenKind::Delimiter(Delimiter::RightParen),
            Some('{') => TokenKind::Delimiter(Delimiter::LeftBrace),
            Some('}') => TokenKind::Delimiter(Delimiter::RightBrace),
            Some('[') => TokenKind::Delimiter(Delimiter::LeftBracket),
            Some(']') => TokenKind::Delimiter(Delimiter::RightBracket),
            Some(',') => TokenKind::Delimiter(Delimiter::Comma),
            Some(':') => TokenKind::Delimiter(Delimiter::Colon),
            Some(';') => TokenKind::Delimiter(Delimiter::Semi),
            Some('=') => TokenKind::Operator(Operator::Assign),
            Some('+') => TokenKind::Operator(Operator::Plus),
            Some('-') => TokenKind::Operator(Operator::Minus),
            Some('*') => TokenKind::Operator(Operator::Star),
            Some('/') => TokenKind::Operator(Operator::Slash),
            Some('%') => TokenKind::Operator(Operator::Percent),
            Some('<') => TokenKind::Operator(Operator::Less),
            Some('>') => TokenKind::Operator(Operator::Greater),
            Some('!') => TokenKind::Operator(Operator::Bang),
            Some('&') => TokenKind::Operator(Operator::Amp),
            Some('|') => TokenKind::Operator(Operator::Pipe),
            Some('.') => TokenKind::Operator(Operator::Dot),
            _ => return false,
        };

        self.emit_fixed(1, kind);
        true
    }

    fn consume_digits(&mut self) {
        while self.peek_char().is_some_and(|ch| ch.is_ascii_digit()) {
            self.bump_char();
        }
    }

    fn starts_with_contextual_amp_mut(&self) -> bool {
        if !self.starts_with("&mut") {
            return false;
        }

        self.source[self.index + 4..]
            .chars()
            .next()
            .is_none_or(|ch| !is_ident_continue(ch))
    }
}

fn is_ident_start(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphabetic()
}

fn is_ident_continue(ch: char) -> bool {
    is_ident_start(ch) || ch.is_ascii_digit()
}

#[cfg(test)]
mod tests {
    use super::{lex, lex_bytes, ContextualToken, Diagnostic, Keyword, Operator, TokenKind};

    fn diagnostic_codes(output: &[Diagnostic]) -> Vec<&str> {
        output.iter().map(|diagnostic| diagnostic.code.as_str()).collect()
    }

    #[test]
    fn lexes_keywords_literals_and_operators_with_spans() {
        let output = lex("agent worker &mut begin end true none -> <=");

        assert!(output.diagnostics.is_empty());
        assert_eq!(output.tokens[0].kind, TokenKind::Keyword(Keyword::Agent));
        assert_eq!(output.tokens[0].span.start.line, 1);
        assert_eq!(output.tokens[0].span.start.column, 1);
        assert_eq!(output.tokens[0].span.end.column, 6);
        assert_eq!(output.tokens[1].kind, TokenKind::Identifier);
        assert_eq!(output.tokens[2].kind, TokenKind::Contextual(ContextualToken::AmpMut));
        assert_eq!(output.tokens[3].kind, TokenKind::Contextual(ContextualToken::Begin));
        assert_eq!(output.tokens[4].kind, TokenKind::Contextual(ContextualToken::End));
        assert_eq!(output.tokens[5].kind, TokenKind::Bool(true));
        assert_eq!(output.tokens[6].kind, TokenKind::None);
        assert_eq!(output.tokens[7].kind, TokenKind::Operator(Operator::Arrow));
        assert_eq!(output.tokens[8].kind, TokenKind::Operator(Operator::LessEq));
        assert_eq!(output.tokens.last().map(|token| &token.kind), Some(&TokenKind::Eof));
    }

    #[test]
    fn reports_unterminated_string_literals() {
        let output = lex("\"unterminated");

        assert_eq!(diagnostic_codes(&output.diagnostics), vec!["LEX002"]);
        assert_eq!(output.tokens[0].kind, TokenKind::Error);
        assert!(!output.tokens[0].span.is_empty());
    }

    #[test]
    fn reports_nested_block_comment_failures() {
        let output = lex("/* outer /* inner */");

        assert_eq!(diagnostic_codes(&output.diagnostics), vec!["LEX003"]);
        assert_eq!(output.tokens[0].kind, TokenKind::Error);
    }

    #[test]
    fn reports_invalid_numeric_suffixes() {
        let output = lex("123abc");

        assert_eq!(diagnostic_codes(&output.diagnostics), vec!["LEX004"]);
        assert_eq!(output.tokens[0].kind, TokenKind::Error);
        assert_eq!(output.tokens[0].lexeme, "123abc");
    }

    #[test]
    fn recovers_invalid_utf8_sequences() {
        let output = lex_bytes(b"agent \xFF tool");

        assert_eq!(diagnostic_codes(&output.diagnostics), vec!["LEX001"]);
        assert_eq!(output.tokens[0].kind, TokenKind::Keyword(Keyword::Agent));
        assert_eq!(output.tokens[1].kind, TokenKind::Error);
        assert_eq!(output.tokens[2].kind, TokenKind::Keyword(Keyword::Tool));
        assert_eq!(output.tokens.last().map(|token| &token.kind), Some(&TokenKind::Eof));
    }

    #[test]
    fn rejects_non_ascii_identifier_starts() {
        let output = lex("éclair");

        assert_eq!(output.tokens[0].kind, TokenKind::Error);
        assert_eq!(output.tokens.last().map(|token| &token.kind), Some(&TokenKind::Eof));
    }
}