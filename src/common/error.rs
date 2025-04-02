use std::{ fmt::Display, io::{ stdout, Write } };
use super::token::Token;

mod terminal {
    pub(super) const ESC: &'static str = "\x1b[";
    pub(super) const RED: &'static str = "91m";
    pub(super) const GREEN: &'static str = "92m";
    pub(super) const YELLOW: &'static str = "93m";
    pub(super) const RESET: &'static str = "\x1b[m";
}

mod formatting {
    use crate::common::token::Token;
    use super::{ terminal, ErrorSeverity };

    /// Format line and offset
    /// Takes in a line and an offset and returns the formatting everything
    pub(super) fn format_line_offset(
        line: usize,
        offset: usize,
        source: &Vec<String>,
        path: &String,
        kind: &'static str,
        severity: &ErrorSeverity
    ) -> Option<(String, String)> {
        // Format the header
        let header = format!(
            "\n[{}] {}:{} {}{}{}{}:",
            severity,
            path,
            line,
            terminal::ESC,
            terminal::YELLOW,
            kind,
            terminal::RESET
        );

        // Get the content of the line from the source.
        let line_content = &source[line - 1];

        // Create the whitespace to align with the token's position
        let whitespace = " ".repeat(offset);
        let underline = "^";

        // Prepare the formatted error message with the highlighted line.
        let body = format!(
            "~\n~ {}\n~ {}{}{}{}{}",
            line_content,
            terminal::ESC,
            terminal::YELLOW,
            whitespace,
            underline,
            terminal::RESET
        );

        // Return the line content as part of the error message.
        Some((body, header))
    }

    /// Takes in a token and returns the formatted error header
    /// and it's formatted line content to be printed.
    pub(super) fn format_token(
        token: &Token,
        source: &Vec<String>,
        path: &String,
        kind: &'static str,
        severity: &ErrorSeverity
    ) -> Option<(String, String)> {
        // A wee bit of bounds checking
        if token.line == 0 || token.line > source.len() {
            return None;
        }

        // Format the header
        let header = format!(
            "\n[{}] {}:{} {}{}{}{}:",
            severity,
            path,
            token.line,
            terminal::ESC,
            terminal::YELLOW,
            kind,
            terminal::RESET
        );

        // Get the content of the line from the source.
        let line_content = &source[token.line - 1];

        // Create the whitespace to align with the token's position
        let whitespace = " ".repeat(token.offset);
        let underline = "^".repeat(token.lexeme.len());

        // Prepare the formatted error message with the highlighted line.
        let body = format!(
            "~\n~ {}\n~ {}{}{}{}{}",
            line_content,
            terminal::ESC,
            terminal::YELLOW,
            whitespace,
            underline,
            terminal::RESET
        );

        // Return the line content as part of the error message.
        Some((body, header))
    }
}

pub(crate) enum ErrorBase<'a> {
    /// Syntax error in the code.
    SyntaxError {
        token: Token<'a>,
    },

    /// When an unmeaninful expression is used as a statement
    InvalidStatement {
        token: Token<'a>,
    },

    /// Unterminated literal, this is a lexing error.
    UnterminatedLiteral {
        line: usize,
        offset: usize,
    },

    /// Expected a token but found something else
    ExpectedToken {
        line: usize,
        offset: usize,
        offender: Token<'a>,
    },

    /// Improperly formatted literal, this is a parsing error.
    /// This realistically shouldn't ever happen.
    ParseError {
        token: Token<'a>,
    },

    /// Any kind of operation between two incompatible types
    IncompatibleTypes {
        line: usize,
        offset: usize,
    },

    UnknownIdentifier {
        line: usize,
        offset: usize,
    },

    /// One of few lexer errors, illegal character found while tokenizing
    IllegalCharacter {
        line: usize,
        offset: usize,
    },
}

impl<'a> ErrorBase<'a> {
    /// Returns a formatted version of header and line content based on the error type
    pub(crate) fn formatted(
        &self,
        source: &Vec<String>,
        path: &String,
        severity: &ErrorSeverity
    ) -> Option<(String, String)> {
        match self {
            Self::SyntaxError { token } =>
                formatting::format_token(token, source, path, self.kind(), severity),
            Self::InvalidStatement { token } =>
                formatting::format_token(token, source, path, self.kind(), severity),
            Self::ParseError { token } =>
                formatting::format_token(token, source, path, self.kind(), severity),
            Self::IllegalCharacter { line, offset } =>
                formatting::format_line_offset(*line, *offset, source, path, self.kind(), severity),
            Self::UnterminatedLiteral { line, offset } =>
                formatting::format_line_offset(*line, *offset, source, path, self.kind(), severity),
            Self::ExpectedToken { line, offset, offender: _ } =>
                formatting::format_line_offset(*line, *offset, source, path, self.kind(), severity),
            Self::IncompatibleTypes { line, offset } =>
                formatting::format_line_offset(*line, *offset, source, path, self.kind(), severity),
            Self::UnknownIdentifier { line, offset } =>
                formatting::format_line_offset(*line, *offset, source, path, self.kind(), severity),
        }
    }

    pub(crate) fn kind(&self) -> &'static str {
        match self {
            Self::SyntaxError { token: _ } => "Syntax Error",
            Self::ParseError { token: _ } => "Parse Error",
            Self::IllegalCharacter { line: _, offset: _ } => "Illegal Character",
            Self::UnterminatedLiteral { line: _, offset: _ } => "Unterminated Literal",
            Self::InvalidStatement { token: _ } => "Invalid Statement",
            Self::ExpectedToken { line: _, offset: _, offender: _ } => "Expected Token",
            Self::IncompatibleTypes { line: _, offset: _ } => "Incomaptible Types",
            Self::UnknownIdentifier { line: _, offset: _ } => "Unknown Identifier",
        }
    }
}

pub(crate) enum ErrorSeverity {
    Error = 0,
    Warning,
    Suggestion,
}

impl Display for ErrorSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (text, color) = match self {
            Self::Error => ("ERROR", terminal::RED),
            Self::Warning => ("WARNING", terminal::YELLOW),
            Self::Suggestion => ("SUGGESTION", terminal::GREEN),
        };
        write!(f, "{}{color}{text}{}", terminal::ESC, terminal::RESET)
    }
}

pub(crate) struct ChaoError<'a> {
    base: ErrorBase<'a>,
    severity: ErrorSeverity,
    can_compile: bool,
    msg: &'static str,
}

impl<'a> ChaoError<'a> {
    pub(crate) fn new(
        base: ErrorBase<'a>,
        severity: ErrorSeverity,
        can_compile: bool,
        msg: &'static str
    ) -> ChaoError<'a> {
        return ChaoError {
            base,
            severity,
            can_compile,
            msg,
        };
    }

    pub(crate) fn print(&self, reporter: &'a Reporter) {
        // Get the line content
        let (body, header) = self.base
            .formatted(reporter.source, reporter.path, &self.severity)
            .unwrap_or(("".to_string(), "".to_string()));

        // Dont let this error silently (for now)
        if body == "" || header == "" {
            eprintln!("ERROR BODY AND/OR HEADER ARE EMPTY!:\n  {}\n  {}", body, header);
            return;
        }

        let message = self.msg;
        write!(
            stdout(),
            "{header}\n{body}\n{}{}{message}{}",
            terminal::ESC,
            terminal::GREEN,
            terminal::RESET
        ).unwrap();

        match &self.base {
            ErrorBase::ExpectedToken { line: _, offset: _, offender } => {
                write!(
                    stdout(),
                    "{}{} found '{}' instead{}",
                    terminal::ESC,
                    terminal::GREEN,
                    offender.lexeme,
                    terminal::RESET
                ).unwrap();
            }
            _ => {}
        }

        // Flush all of this to stdout
        write!(stdout(), "\n").unwrap();
        stdout().flush().unwrap()
    }
}

pub(crate) struct Reporter<'a> {
    errors: Vec<ChaoError<'a>>,
    source: &'a Vec<String>,
    path: &'a String,
}

impl<'a> Reporter<'a> {
    pub(crate) fn new(source: &'a Vec<String>, path: &'a String) -> Reporter<'a> {
        return Reporter {
            errors: vec![],
            source,
            path,
        };
    }

    pub(crate) fn error(&mut self, base: ErrorBase<'a>, can_compile: bool, msg: &'static str) {
        self.errors.push(ChaoError::new(base, ErrorSeverity::Error, can_compile, msg));
    }

    pub(crate) fn dump(&mut self, mut errs: Vec<ChaoError<'a>>) {
        for e in errs.drain(0..) {
            self.errors.push(e)
        }
    }

    pub(crate) fn print_all(&mut self) {
        // Drain the errors and copy them into a new vec
        // This way we have ownership
        let errors: Vec<ChaoError<'a>> = self.errors.drain(0..).collect();
        errors.iter().for_each(|e| e.print(self));
    }
}
