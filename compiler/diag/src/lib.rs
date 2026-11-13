use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub byte: usize,
    pub line: usize,
    pub column: usize,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            byte: 0,
            line: 1,
            column: 1,
        }
    }
}

impl Position {
    pub fn advance_char(self, ch: char) -> Self {
        if ch == '\n' {
            Self {
                byte: self.byte + ch.len_utf8(),
                line: self.line + 1,
                column: 1,
            }
        } else {
            Self {
                byte: self.byte + ch.len_utf8(),
                line: self.line,
                column: self.column + 1,
            }
        }
    }

    pub fn advance_bytes(self, len: usize) -> Self {
        Self {
            byte: self.byte + len,
            line: self.line,
            column: self.column + len,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    pub fn new(start: Position, end: Position) -> Self {
        debug_assert!(end.byte >= start.byte);
        Self { start, end }
    }

    pub fn empty(at: Position) -> Self {
        Self { start: at, end: at }
    }

    pub fn is_empty(self) -> bool {
        self.start == self.end
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub code: String,
    pub severity: Severity,
    pub message: String,
    pub span: Span,
    pub fix_hint: Option<String>,
}

impl Diagnostic {
    pub fn error(code: impl Into<String>, message: impl Into<String>, span: Span) -> Self {
        Self {
            code: code.into(),
            severity: Severity::Error,
            message: message.into(),
            span,
            fix_hint: None,
        }
    }

    pub fn with_fix_hint(mut self, fix_hint: impl Into<String>) -> Self {
        self.fix_hint = Some(fix_hint.into());
        self
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {:?} {}:{} {}",
            self.code, self.severity, self.span.start.line, self.span.start.column, self.message
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{Diagnostic, Position, Span};

    #[test]
    fn newline_advances_line_and_resets_column() {
        let position = Position::default().advance_char('a').advance_char('\n');

        assert_eq!(position.byte, 2);
        assert_eq!(position.line, 2);
        assert_eq!(position.column, 1);
    }

    #[test]
    fn span_empty_helper_is_consistent() {
        let point = Position::default().advance_bytes(3);
        let span = Span::empty(point);

        assert!(span.is_empty());
    }

    #[test]
    fn diagnostics_keep_fix_hints() {
        let span = Span::new(Position::default(), Position::default().advance_bytes(1));
        let diagnostic = Diagnostic::error("LEX002", "unterminated string literal", span)
            .with_fix_hint("Add a closing quote.");

        assert_eq!(diagnostic.fix_hint.as_deref(), Some("Add a closing quote."));
    }
}
