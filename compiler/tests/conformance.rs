use tantra_compiler::{
    diagnostic::Severity,
    lexer::Lexer,
    parser::{Decl, Expr},
    token::TokenKind,
};

#[test]
fn lexes_core_gujarati_keywords_and_literals() {
    let source = r#"સ્થિર સંખ્યા: પૂર્ણાંક = 10
બદલ ગણતરી: પૂર્ણાંક = 0
સાચું ખોટું શૂન્ય
"#;

    let (tokens, diagnostics) = Lexer::new(source).lex();
    assert!(diagnostics.is_empty(), "{diagnostics:#?}");
    assert!(matches!(tokens[0].kind, TokenKind::Const));
    assert!(matches!(tokens[1].kind, TokenKind::Identifier(ref s) if s == "સંખ્યા"));
    assert!(matches!(tokens[3].kind, TokenKind::Identifier(ref s) if s == "પૂર્ણાંક"));
    assert!(matches!(tokens[5].kind, TokenKind::Integer(ref s) if s == "10"));
    assert!(matches!(tokens[6].kind, TokenKind::Mut));
    assert!(matches!(tokens[12].kind, TokenKind::True));
    assert!(matches!(tokens[13].kind, TokenKind::False));
    assert!(matches!(tokens[14].kind, TokenKind::Null));
}

#[test]
fn lexes_nested_block_comments_and_operators() {
    let source = "/* outer /* inner */ outer */ a ** 2 ?? b != c && d";
    let (tokens, diagnostics) = Lexer::new(source).lex();
    assert!(diagnostics.is_empty(), "{diagnostics:#?}");
    assert!(matches!(tokens[0].kind, TokenKind::Identifier(_)));
    assert!(matches!(tokens[1].kind, TokenKind::Power));
    assert!(matches!(tokens[3].kind, TokenKind::NullCoalesce));
    assert!(matches!(tokens[5].kind, TokenKind::AndAnd));
}

#[test]
fn parser_builds_variable_and_function_ast() {
    let source = r#"
સ્થિર સંખ્યા: પૂર્ણાંક = 10

જાહેર કાર્ય ઉમેરો(a: પૂર્ણાંક, b: પૂર્ણાંક) -> પૂર્ણાંક {
    પરત a + b
}
"#;

    let (program, diagnostics) = tantra_compiler::parse_source(source);
    assert!(diagnostics.is_empty(), "{diagnostics:#?}");
    let program = program.expect("program should parse");
    assert_eq!(program.declarations.len(), 2);

    match &program.declarations[0] {
        Decl::Variable { name, mutable, .. } => {
            assert_eq!(name, "સંખ્યા");
            assert!(!mutable);
        }
        other => panic!("unexpected declaration: {other:?}"),
    }

    match &program.declarations[1] {
        Decl::Function {
            name, public, body, ..
        } => {
            assert_eq!(name, "ઉમેરો");
            assert!(*public);
            assert_eq!(body.statements.len(), 1);
        }
        other => panic!("unexpected declaration: {other:?}"),
    }
}

#[test]
fn parser_reports_invalid_syntax_with_stable_code() {
    let source = "સ્થિર x: = 10";
    let (_, diagnostics) = tantra_compiler::parse_source(source);
    assert!(diagnostics
        .iter()
        .any(|d| d.code == "T1015" && d.severity == Severity::Error));
}

#[test]
fn parser_supports_calls_members_and_arrays() {
    let source = r#"
સ્થિર items = [1, 2, 3]
સ્થિર value = items[0]
સ્થિર text = ui.label("નમસ્તે")
"#;

    let (program, diagnostics) = tantra_compiler::parse_source(source);
    assert!(diagnostics.is_empty(), "{diagnostics:#?}");
    let program = program.expect("program should parse");

    match &program.declarations[2] {
        Decl::Variable {
            value: Expr::Call { .. },
            ..
        } => {}
        other => panic!("expected call expression, got {other:?}"),
    }
}

#[test]
fn unterminated_comment_is_diagnostic() {
    let (_, diagnostics) = Lexer::new("/* missing").lex();
    assert!(diagnostics.iter().any(|d| d.code == "T0008"));
}
