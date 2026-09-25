use crate::diagnostic::{Diagnostic, Span};
use crate::token::{Token, TokenKind};

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub declarations: Vec<Decl>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Decl {
    Variable {
        mutable: bool,
        name: String,
        ty: Option<TypeRef>,
        value: Expr,
        span: Span,
    },
    Function {
        public: bool,
        async_: bool,
        name: String,
        params: Vec<Param>,
        return_type: Option<TypeRef>,
        capabilities: Vec<String>,
        body: Block,
        span: Span,
    },
    Import {
        path: Vec<String>,
        span: Span,
    },
    Type {
        name: String,
        span: Span,
    },
    Module {
        name: String,
        span: Span,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub name: String,
    pub ty: TypeRef,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeRef {
    pub name: String,
    pub arguments: Vec<TypeRef>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Stmt>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Decl(Decl),
    Return(Option<Expr>, Span),
    If {
        condition: Expr,
        then_block: Block,
        else_branch: Option<Box<Stmt>>,
        span: Span,
    },
    While {
        condition: Expr,
        body: Block,
        span: Span,
    },
    ForEach {
        name: String,
        iterable: Expr,
        body: Block,
        span: Span,
    },
    Break(Span),
    Continue(Span),
    Expr(Expr),
    Empty(Span),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Identifier(String, Span),
    Integer(String, Span),
    Float(String, Span),
    String(String, Span),
    Character(char, Span),
    Boolean(bool, Span),
    Null(Span),
    Unary {
        op: TokenKind,
        expr: Box<Expr>,
        span: Span,
    },
    Binary {
        left: Box<Expr>,
        op: TokenKind,
        right: Box<Expr>,
        span: Span,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
        span: Span,
    },
    Member {
        object: Box<Expr>,
        member: String,
        span: Span,
    },
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
        span: Span,
    },
    Grouped(Box<Expr>, Span),
    Array(Vec<Expr>, Span),
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    diagnostics: Vec<Diagnostic>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            diagnostics: Vec::new(),
        }
    }

    pub fn parse(mut self) -> (Option<Program>, Vec<Diagnostic>) {
        let mut declarations = Vec::new();

        while !self.check(&TokenKind::Eof) {
            match self.declaration() {
                Some(decl) => declarations.push(decl),
                None => self.synchronize(),
            }
        }

        let program = if self
            .diagnostics
            .iter()
            .any(|d| d.severity == crate::diagnostic::Severity::Error)
        {
            None
        } else {
            Some(Program { declarations })
        };

        (program, self.diagnostics)
    }

    fn declaration(&mut self) -> Option<Decl> {
        let public = self.match_kind(&TokenKind::Public);
        let private = self.match_kind(&TokenKind::Private);
        let _ = private;

        if self.match_kind(&TokenKind::Const) {
            return self.variable(false);
        }
        if self.match_kind(&TokenKind::Mut) {
            return self.variable(true);
        }
        if self.match_kind(&TokenKind::Async) {
            if self.match_kind(&TokenKind::Fn) {
                return self.function(public, true);
            }
            self.error_here("T1001", "async પછી કાર્ય expected");
            return None;
        }
        if self.match_kind(&TokenKind::Fn) {
            return self.function(public, false);
        }
        if self.match_kind(&TokenKind::Import) {
            return self.import_decl();
        }
        if self.match_kind(&TokenKind::Type) {
            return self.type_decl();
        }
        if self.match_kind(&TokenKind::Module) {
            return self.module_decl();
        }

        self.error_here("T1002", "અહીં declaration expected");
        None
    }

    fn variable(&mut self, mutable: bool) -> Option<Decl> {
        let name_token = self.consume_identifier("T1003", "variable name expected")?;
        let name = self.identifier_text(&name_token);
        let ty = if self.match_kind(&TokenKind::Colon) {
            Some(self.type_ref()?)
        } else {
            None
        };
        self.consume(
            &TokenKind::Equal,
            "T1004",
            "variable declarationમાં '=' expected",
        );
        let value = self.expression()?;
        let end = self.optional_semicolon_end(value_span(&value).end);
        Some(Decl::Variable {
            mutable,
            name,
            ty,
            value,
            span: Span::new(name_token.span.start, end),
        })
    }

    fn function(&mut self, public: bool, async_: bool) -> Option<Decl> {
        let name_token = self.consume_identifier("T1005", "function name expected")?;
        let name = self.identifier_text(&name_token);
        self.consume(
            &TokenKind::LeftParen,
            "T1006",
            "function parameters માટે '(' expected",
        );
        let mut params = Vec::new();

        if !self.check(&TokenKind::RightParen) {
            loop {
                let token = self.consume_identifier("T1007", "parameter name expected")?;
                let ty = {
                    self.consume(
                        &TokenKind::Colon,
                        "T1008",
                        "parameter type માટે ':' expected",
                    );
                    self.type_ref()?
                };
                params.push(Param {
                    name: self.identifier_text(&token),
                    ty,
                    span: Span::new(token.span.start, self.previous().span.end),
                });

                if !self.match_kind(&TokenKind::Comma) {
                    break;
                }
            }
        }

        self.consume(
            &TokenKind::RightParen,
            "T1009",
            "function parameters માટે ')' expected",
        );

        let return_type = if self.match_kind(&TokenKind::Arrow) {
            Some(self.type_ref()?)
        } else {
            None
        };

        let mut capabilities = Vec::new();
        while self.match_kind(&TokenKind::Capability) {
            let token = self.consume_identifier("T1010", "capability name expected")?;
            capabilities.push(self.identifier_text(&token));
        }

        let body = self.block()?;
        Some(Decl::Function {
            public,
            async_,
            name,
            params,
            return_type,
            capabilities,
            span: Span::new(name_token.span.start, body.span.end),
            body,
        })
    }

    fn import_decl(&mut self) -> Option<Decl> {
        let start = self.previous().span.start;
        let first = self.consume_identifier("T1011", "import path expected")?;
        let mut path = vec![self.identifier_text(&first)];

        while self.match_kind(&TokenKind::Dot) {
            let part = self.consume_identifier("T1012", "import path component expected")?;
            path.push(self.identifier_text(&part));
        }

        let end = self.optional_semicolon_end(self.previous().span.end);
        Some(Decl::Import {
            path,
            span: Span::new(start, end),
        })
    }

    fn type_decl(&mut self) -> Option<Decl> {
        let name = self.consume_identifier("T1013", "type name expected")?;
        let span = name.span;
        if self.match_kind(&TokenKind::LeftBrace) {
            let mut depth = 1;
            while depth > 0 && !self.check(&TokenKind::Eof) {
                if self.match_kind(&TokenKind::LeftBrace) {
                    depth += 1;
                } else if self.match_kind(&TokenKind::RightBrace) {
                    depth -= 1;
                } else {
                    self.advance();
                }
            }
        }
        Some(Decl::Type {
            name: self.identifier_text(&name),
            span: Span::new(span.start, self.previous().span.end),
        })
    }

    fn module_decl(&mut self) -> Option<Decl> {
        let token = self.consume_identifier("T1014", "module name expected")?;
        let span = Span::new(self.previous().span.start, self.previous().span.end);
        Some(Decl::Module {
            name: self.identifier_text(&token),
            span,
        })
    }

    fn type_ref(&mut self) -> Option<TypeRef> {
        let token = self.consume_identifier("T1015", "type name expected")?;
        let start = token.span.start;
        let name = self.identifier_text(&token);
        let mut arguments = Vec::new();

        if self.match_kind(&TokenKind::Less) {
            loop {
                arguments.push(self.type_ref()?);
                if !self.match_kind(&TokenKind::Comma) {
                    break;
                }
            }
            self.consume(
                &TokenKind::Greater,
                "T1016",
                "generic type માટે '>' expected",
            );
        }

        Some(TypeRef {
            name,
            arguments,
            span: Span::new(start, self.previous().span.end),
        })
    }

    fn block(&mut self) -> Option<Block> {
        let open = self.consume(&TokenKind::LeftBrace, "T1017", "'{' expected");
        let start = open
            .as_ref()
            .map_or(self.peek().span.start, |t| t.span.start);
        let mut statements = Vec::new();

        while !self.check(&TokenKind::RightBrace) && !self.check(&TokenKind::Eof) {
            if let Some(stmt) = self.statement() {
                statements.push(stmt);
            } else {
                self.synchronize();
            }
        }

        let close = self.consume(&TokenKind::RightBrace, "T1018", "'}' expected");
        let end = close.map_or(self.previous().span.end, |t| t.span.end);
        Some(Block {
            statements,
            span: Span::new(start, end),
        })
    }

    fn statement(&mut self) -> Option<Stmt> {
        if self.check(&TokenKind::Const)
            || self.check(&TokenKind::Mut)
            || self.check(&TokenKind::Public)
            || self.check(&TokenKind::Private)
            || self.check(&TokenKind::Async)
            || self.check(&TokenKind::Fn)
            || self.check(&TokenKind::Import)
            || self.check(&TokenKind::Type)
            || self.check(&TokenKind::Module)
        {
            return self.declaration().map(Stmt::Decl);
        }

        if self.match_kind(&TokenKind::Return) {
            let start = self.previous().span.start;
            if self.check(&TokenKind::RightBrace) || self.check(&TokenKind::Semicolon) {
                let end = self.optional_semicolon_end(self.previous().span.end);
                return Some(Stmt::Return(None, Span::new(start, end)));
            }
            let expr = self.expression()?;
            let end = self.optional_semicolon_end(value_span(&expr).end);
            return Some(Stmt::Return(Some(expr), Span::new(start, end)));
        }

        if self.match_kind(&TokenKind::If) {
            return self.if_statement();
        }

        if self.match_kind(&TokenKind::While) {
            let start = self.previous().span.start;
            let condition = self.expression()?;
            let body = self.block()?;
            return Some(Stmt::While {
                condition,
                body: body.clone(),
                span: Span::new(start, body.span.end),
            });
        }

        if self.match_kind(&TokenKind::For) {
            let start = self.previous().span.start;
            self.consume(&TokenKind::Each, "T1019", "'દરેક' expected");
            let name = self.consume_identifier("T1020", "loop variable expected")?;
            self.consume(&TokenKind::In, "T1021", "'માં' expected");
            let iterable = self.expression()?;
            let body = self.block()?;
            return Some(Stmt::ForEach {
                name: self.identifier_text(&name),
                iterable,
                body: body.clone(),
                span: Span::new(start, body.span.end),
            });
        }

        if self.match_kind(&TokenKind::Break) {
            let span = self.previous().span;
            let end = self.optional_semicolon_end(span.end);
            return Some(Stmt::Break(Span::new(span.start, end)));
        }

        if self.match_kind(&TokenKind::Continue) {
            let span = self.previous().span;
            let end = self.optional_semicolon_end(span.end);
            return Some(Stmt::Continue(Span::new(span.start, end)));
        }

        if self.match_kind(&TokenKind::LeftBrace) {
            self.current -= 1;
            let block = self.block()?;
            return Some(Stmt::Decl(Decl::Module {
                name: "<block>".to_owned(),
                span: block.span,
            }));
        }

        if self.check(&TokenKind::Semicolon) {
            let span = self.advance().span;
            return Some(Stmt::Empty(span));
        }

        let expr = self.expression()?;
        self.optional_semicolon_end(value_span(&expr));
        Some(Stmt::Expr(expr))
    }

    fn if_statement(&mut self) -> Option<Stmt> {
        let start = self.previous().span.start;
        let condition = self.expression()?;
        let then_block = self.block()?;

        let else_branch = if self.match_kind(&TokenKind::Else) {
            if self.match_kind(&TokenKind::If) {
                Some(Box::new(self.if_statement()?))
            } else {
                let block = self.block()?;
                Some(Box::new(Stmt::Expr(Expr::Grouped(
                    Box::new(Expr::Null(block.span)),
                    block.span,
                ))))
            }
        } else {
            None
        };

        let end = match &else_branch {
            Some(branch) => stmt_span(branch).end,
            None => then_block.span.end,
        };

        Some(Stmt::If {
            condition,
            then_block,
            else_branch,
            span: Span::new(start, end),
        })
    }

    fn expression(&mut self) -> Option<Expr> {
        self.parse_precedence(0)
    }

    fn parse_precedence(&mut self, min_prec: u8) -> Option<Expr> {
        let mut left = self.unary()?;

        loop {
            let Some((prec, right_assoc)) = binary_precedence(self.peek_kind()) else {
                break;
            };
            if prec < min_prec {
                break;
            }

            let op = self.advance().kind.clone();
            let next_min = if right_assoc { prec } else { prec + 1 };
            let right = self.parse_precedence(next_min)?;
            let span = Span::new(value_span(&left).start, value_span(&right).end);
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
                span,
            };
        }

        if min_prec == 0 && self.match_kind(&TokenKind::Equal) {
            let right = self.parse_precedence(0)?;
            let span = Span::new(value_span(&left).start, value_span(&right).end);
            left = Expr::Binary {
                left: Box::new(left),
                op: TokenKind::Equal,
                right: Box::new(right),
                span,
            };
        }

        Some(left)
    }

    fn unary(&mut self) -> Option<Expr> {
        if matches!(
            self.peek_kind(),
            TokenKind::Bang | TokenKind::Plus | TokenKind::Minus
        ) {
            let op = self.advance().kind.clone();
            let expr = self.unary()?;
            let span = Span::new(self.previous().span.start, value_span(&expr).end);
            return Some(Expr::Unary {
                op,
                expr: Box::new(expr),
                span,
            });
        }

        self.postfix()
    }

    fn postfix(&mut self) -> Option<Expr> {
        let mut expr = self.primary()?;

        loop {
            match self.peek_kind() {
                TokenKind::LeftParen => {
                    self.advance();
                    let mut args = Vec::new();
                    if !self.check(&TokenKind::RightParen) {
                        loop {
                            args.push(self.expression()?);
                            if !self.match_kind(&TokenKind::Comma) {
                                break;
                            }
                        }
                    }
                    let close = self.consume(&TokenKind::RightParen, "T1022", "')' expected")?;
                    let span = Span::new(value_span(&expr).start, close.span.end);
                    expr = Expr::Call {
                        callee: Box::new(expr),
                        args,
                        span,
                    };
                }
                TokenKind::Dot => {
                    self.advance();
                    let member = self.consume_identifier("T1023", "member name expected")?;
                    let span = Span::new(value_span(&expr).start, member.span.end);
                    expr = Expr::Member {
                        object: Box::new(expr),
                        member: self.identifier_text(&member),
                        span,
                    };
                }
                TokenKind::LeftBracket => {
                    self.advance();
                    let index = self.expression()?;
                    let close = self.consume(&TokenKind::RightBracket, "T1024", "']' expected")?;
                    let span = Span::new(value_span(&expr).start, close.span.end);
                    expr = Expr::Index {
                        object: Box::new(expr),
                        index: Box::new(index),
                        span,
                    };
                }
                _ => break,
            }
        }

        Some(expr)
    }

    fn primary(&mut self) -> Option<Expr> {
        let token = self.advance();
        match token.kind {
            TokenKind::Identifier(name) => Some(Expr::Identifier(name, token.span)),
            TokenKind::Integer(value) => Some(Expr::Integer(value, token.span)),
            TokenKind::Float(value) => Some(Expr::Float(value, token.span)),
            TokenKind::String(value) => Some(Expr::String(value, token.span)),
            TokenKind::Character(value) => Some(Expr::Character(value, token.span)),
            TokenKind::True => Some(Expr::Boolean(true, token.span)),
            TokenKind::False => Some(Expr::Boolean(false, token.span)),
            TokenKind::Null => Some(Expr::Null(token.span)),
            TokenKind::LeftParen => {
                let expr = self.expression()?;
                let close = self.consume(&TokenKind::RightParen, "T1025", "')' expected")?;
                Some(Expr::Grouped(
                    Box::new(expr),
                    Span::new(token.span.start, close.span.end),
                ))
            }
            TokenKind::LeftBracket => {
                let mut items = Vec::new();
                if !self.check(&TokenKind::RightBracket) {
                    loop {
                        items.push(self.expression()?);
                        if !self.match_kind(&TokenKind::Comma) {
                            break;
                        }
                    }
                }
                let close = self.consume(&TokenKind::RightBracket, "T1026", "']' expected")?;
                Some(Expr::Array(
                    items,
                    Span::new(token.span.start, close.span.end),
                ))
            }
            _ => {
                self.diagnostics.push(Diagnostic::error(
                    "T1027",
                    "expression expected",
                    token.span,
                ));
                None
            }
        }
    }

    fn synchronize(&mut self) {
        while !self.check(&TokenKind::Eof) {
            if self.previous().kind == TokenKind::Semicolon {
                return;
            }
            if matches!(
                self.peek_kind(),
                TokenKind::Const
                    | TokenKind::Mut
                    | TokenKind::Fn
                    | TokenKind::Import
                    | TokenKind::Type
                    | TokenKind::Module
                    | TokenKind::Return
                    | TokenKind::If
                    | TokenKind::While
                    | TokenKind::For
            ) {
                return;
            }
            self.advance();
        }
    }

    fn consume_identifier(&mut self, code: &'static str, message: &'static str) -> Option<Token> {
        if matches!(self.peek_kind(), TokenKind::Identifier(_)) {
            Some(self.advance())
        } else {
            self.error_here(code, message);
            None
        }
    }

    fn consume(
        &mut self,
        expected: &TokenKind,
        code: &'static str,
        message: &'static str,
    ) -> Option<Token> {
        if same_variant(self.peek_kind(), expected) {
            Some(self.advance())
        } else {
            self.error_here(code, message);
            None
        }
    }

    fn optional_semicolon_end(&mut self, fallback: usize) -> usize {
        if self.match_kind(&TokenKind::Semicolon) {
            self.previous().span.end
        } else {
            fallback
        }
    }

    fn error_here(&mut self, code: &'static str, message: &str) {
        self.diagnostics
            .push(Diagnostic::error(code, message, self.peek().span));
    }

    fn identifier_text(&self, token: &Token) -> String {
        match &token.kind {
            TokenKind::Identifier(name) => name.clone(),
            _ => String::new(),
        }
    }

    fn check(&self, expected: &TokenKind) -> bool {
        same_variant(self.peek_kind(), expected)
    }

    fn match_kind(&mut self, expected: &TokenKind) -> bool {
        if self.check(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn peek_kind(&self) -> &TokenKind {
        &self.peek().kind
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> Token {
        let token = self.tokens[self.current].clone();
        if !self.check(&TokenKind::Eof) {
            self.current += 1;
        }
        token
    }
}

fn same_variant(a: &TokenKind, b: &TokenKind) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

fn binary_precedence(kind: &TokenKind) -> Option<(u8, bool)> {
    Some(match kind {
        TokenKind::OrOr => (1, false),
        TokenKind::AndAnd => (2, false),
        TokenKind::Pipe => (3, false),
        TokenKind::Caret => (4, false),
        TokenKind::Ampersand => (5, false),
        TokenKind::EqualEqual | TokenKind::NotEqual => (6, false),
        TokenKind::Less | TokenKind::LessEqual | TokenKind::Greater | TokenKind::GreaterEqual => {
            (7, false)
        }
        TokenKind::ShiftLeft | TokenKind::ShiftRight => (8, false),
        TokenKind::Plus | TokenKind::Minus => (9, false),
        TokenKind::Star | TokenKind::Slash | TokenKind::Percent => (10, false),
        TokenKind::Power => (11, true),
        TokenKind::NullCoalesce => (0, false),
        _ => return None,
    })
}

fn value_span(expr: &Expr) -> Span {
    match expr {
        Expr::Identifier(_, span)
        | Expr::Integer(_, span)
        | Expr::Float(_, span)
        | Expr::String(_, span)
        | Expr::Character(_, span)
        | Expr::Boolean(_, span)
        | Expr::Null(span)
        | Expr::Unary { span, .. }
        | Expr::Binary { span, .. }
        | Expr::Call { span, .. }
        | Expr::Member { span, .. }
        | Expr::Index { span, .. }
        | Expr::Grouped(_, span)
        | Expr::Array(_, span) => *span,
    }
}

fn stmt_span(stmt: &Stmt) -> Span {
    match stmt {
        Stmt::Decl(Decl::Variable { span, .. })
        | Stmt::Decl(Decl::Function { span, .. })
        | Stmt::Decl(Decl::Import { span, .. })
        | Stmt::Decl(Decl::Type { span, .. })
        | Stmt::Decl(Decl::Module { span, .. })
        | Stmt::Return(_, span)
        | Stmt::Break(span)
        | Stmt::Continue(span)
        | Stmt::Empty(span) => *span,
        Stmt::If { span, .. } | Stmt::While { span, .. } | Stmt::ForEach { span, .. } => *span,
        Stmt::Expr(expr) => value_span(expr),
    }
}
