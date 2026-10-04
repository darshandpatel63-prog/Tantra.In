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
    assert!(matches!(tokens[7].kind, TokenKind::AndAnd));
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

#[test]
fn normalizes_identifier_spelling_to_nfc() {
    let source = "સ્થિર cafe\u{0301} = 1";
    let (tokens, diagnostics) = Lexer::new(source).lex();
    assert!(diagnostics.is_empty(), "{diagnostics:#?}");
    assert!(matches!(
        &tokens[1].kind,
        TokenKind::Identifier(name) if name == "café"
    ));
}

#[test]
fn rejects_uts39_ascii_confusable_identifier_characters() {
    let source = "સ્થિર раy = 1";
    let (_, diagnostics) = Lexer::new(source).lex();
    assert!(diagnostics.iter().any(|d| d.code == "T0011"));
}

#[test]
fn accepts_valid_numeric_literals_and_separators() {
    let source = "1_000 0xCA_FE 0b1010_0101 0o755_123 3.141_592 6.02e23";
    let (_, diagnostics) = Lexer::new(source).lex();
    assert!(diagnostics.is_empty(), "{diagnostics:#?}");
}

#[test]
fn rejects_malformed_numeric_literals() {
    for source in ["0x", "0b102", "0o89", "1__2", "1_e2", "1e+", "123abc"] {
        let (_, diagnostics) = Lexer::new(source).lex();
        assert!(
            diagnostics.iter().any(|d| d.code == "T0009"),
            "expected T0009 for {source:?}, got {diagnostics:#?}"
        );
    }
}

#[test]
fn validates_string_and_character_escapes() {
    let (_, string_diagnostics) = Lexer::new(r#""bad\q""#).lex();
    assert!(string_diagnostics.iter().any(|d| d.code == "T0004"));

    let (_, character_diagnostics) = Lexer::new(r"'\q'").lex();
    assert!(character_diagnostics.iter().any(|d| d.code == "T0006"));

    let (tokens, nul_diagnostics) = Lexer::new("'\0'").lex();
    assert!(nul_diagnostics.is_empty(), "{nul_diagnostics:#?}");
    assert!(matches!(tokens[0].kind, TokenKind::Character('\0')));
}

#[test]
fn parses_control_flow_and_error_handling_grammar() {
    let source = r#"
કાર્ય run(items: Array<પૂર્ણાંક>) {
    જ્યારે સાચું {
        જો સાચું {
            આગળ
        } નહીં જો ખોટું {
            તોડો
        } નહીં {
            ફેંકો "x"
        }
        પ્રયત્ન {
            await_value()
        } ભૂલ err {
            લખો(err)
        }
    }
}
"#;
    let (_, diagnostics) = tantra_compiler::parse_source(source);
    assert!(diagnostics.is_empty(), "{diagnostics:#?}");
}

#[test]
fn parses_conditional_and_compound_assignment() {
    let source = r#"
કાર્ય test() {
    બદલ x: પૂર્ણાંક = 1
    x += x > 0 ? 2 : 3
}
"#;
    let (tokens, lex_diagnostics) = Lexer::new(source).lex();
    assert!(lex_diagnostics.is_empty(), "{lex_diagnostics:#?}");
    assert!(tokens
        .iter()
        .any(|t| matches!(t.kind, TokenKind::PlusEqual)));

    let (_, diagnostics) = tantra_compiler::parse_source(source);
    assert!(diagnostics.is_empty(), "{diagnostics:#?}");
}

#[test]
fn preserves_unary_expression_source_span() {
    let source = "સ્થિર value = -x + y";
    let (program, diagnostics) = tantra_compiler::parse_source(source);
    assert!(diagnostics.is_empty(), "{diagnostics:#?}");
    let program = program.expect("program should parse");
    match &program.declarations[0] {
        Decl::Variable {
            value: Expr::Binary { left, .. },
            ..
        } => match left.as_ref() {
            Expr::Unary { span, .. } => {
                assert_eq!(*span, tantra_compiler::diagnostic::Span::new(24, 26))
            }
            other => panic!("expected unary expression, got {other:?}"),
        },
        other => panic!("unexpected AST: {other:?}"),
    }
}

#[test]
fn parses_async_await_and_dotted_capabilities() {
    let source = r#"
જાહેર async કાર્ય fetch() -> Result<શબ્દ, ApiError> ક્ષમતા network.read ક્ષમતા secrets.read {
    પરત await get()
}
"#;
    let (program, diagnostics) = tantra_compiler::parse_source(source);
    assert!(diagnostics.is_empty(), "{diagnostics:#?}");
    let program = program.expect("program should parse");

    match &program.declarations[0] {
        Decl::Function {
            async_,
            capabilities,
            body,
            ..
        } => {
            assert!(*async_);
            assert_eq!(
                capabilities,
                &vec!["network.read".to_owned(), "secrets.read".to_owned()]
            );
            assert!(matches!(
                body.statements.as_slice(),
                [tantra_compiler::parser::Stmt::Return(
                    Some(Expr::Await { .. }),
                    _
                )]
            ));
        }
        other => panic!("expected function declaration, got {other:?}"),
    }
}

#[test]
fn parses_all_compound_assignment_operators() {
    let source = r#"
કાર્ય test() {
    બદલ x: પૂર્ણાંક = 1
    x += 1
    x -= 1
    x *= 2
    x /= 2
    x %= 2
}
"#;
    let (tokens, lex_diagnostics) = Lexer::new(source).lex();
    assert!(lex_diagnostics.is_empty(), "{lex_diagnostics:#?}");
    for kind in [
        TokenKind::PlusEqual,
        TokenKind::MinusEqual,
        TokenKind::StarEqual,
        TokenKind::SlashEqual,
        TokenKind::PercentEqual,
    ] {
        assert!(
            tokens
                .iter()
                .any(|token| std::mem::discriminant(&token.kind) == std::mem::discriminant(&kind)),
            "missing compound assignment token: {kind:?}"
        );
    }

    let (_, diagnostics) = tantra_compiler::parse_source(source);
    assert!(diagnostics.is_empty(), "{diagnostics:#?}");
}

#[test]
fn parser_recovers_after_multiple_statement_errors() {
    let source = r#"
કાર્ય broken() {
    સ્થિર first: = 1;
    સ્થિર second: = 2;
}
"#;
    let (_, diagnostics) = tantra_compiler::parse_source(source);
    let errors = diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.code == "T1015")
        .count();
    assert_eq!(errors, 2, "{diagnostics:#?}");
}

#[test]
fn preserves_original_unicode_identifier_span() {
    let source = "સ્થિર cafe\u{0301} = 1";
    let (tokens, diagnostics) = Lexer::new(source).lex();
    assert!(diagnostics.is_empty(), "{diagnostics:#?}");
    let start = "સ્થિર ".len();
    let end = "સ્થિર cafe\u{0301}".len();
    assert_eq!(
        tokens[1].span,
        tantra_compiler::diagnostic::Span::new(start, end)
    );
}

#[test]
fn preserves_nested_expression_source_span() {
    let source = "સ્થિર value = (foo[1] + -bar).baz";
    let (program, diagnostics) = tantra_compiler::parse_source(source);
    assert!(diagnostics.is_empty(), "{diagnostics:#?}");
    let program = program.expect("program should parse");
    match &program.declarations[0] {
        Decl::Variable {
            value: Expr::Member { span, .. },
            ..
        } => {
            let start = source.find("(foo").expect("nested expression start");
            assert_eq!(
                *span,
                tantra_compiler::diagnostic::Span::new(start, source.len())
            );
        }
        other => panic!("expected member expression, got {other:?}"),
    }
}

#[test]
fn parses_module_and_import_declarations() {
    let source = r#"
મોડ્યુલ ગણિત
આયાત ui.controls.button
"#;
    let (program, diagnostics) = tantra_compiler::parse_source(source);
    assert!(diagnostics.is_empty(), "{diagnostics:#?}");
    let program = program.expect("program should parse");
    assert_eq!(program.declarations.len(), 2);

    match &program.declarations[0] {
        Decl::Module { name, .. } => assert_eq!(name, "ગણિત"),
        other => panic!("expected module declaration, got {other:?}"),
    }

    match &program.declarations[1] {
        Decl::Import { path, .. } => {
            assert_eq!(
                path,
                &vec!["ui".to_owned(), "controls".to_owned(), "button".to_owned()]
            );
        }
        other => panic!("expected import declaration, got {other:?}"),
    }
}

#[test]
fn parses_expression_precedence_and_associativity() {
    let source = r#"
કાર્ય test() {
    સ્થિર value = a + b * c ** d ?? e ? f : g
    x = y = z
}
"#;
    let (program, diagnostics) = tantra_compiler::parse_source(source);
    assert!(diagnostics.is_empty(), "{diagnostics:#?}");
    let program = program.expect("program should parse");

    match &program.declarations[0] {
        Decl::Function { body, .. } => {
            assert_eq!(body.statements.len(), 2);
            match &body.statements[0] {
                tantra_compiler::parser::Stmt::Decl(Decl::Variable {
                    value: Expr::Conditional { .. },
                    ..
                }) => {}
                other => panic!("expected conditional expression tree, got {other:?}"),
            }
            match &body.statements[1] {
                tantra_compiler::parser::Stmt::Expr(Expr::Binary {
                    op: TokenKind::Equal,
                    right,
                    ..
                }) => {
                    assert!(matches!(
                        right.as_ref(),
                        Expr::Binary {
                            op: TokenKind::Equal,
                            ..
                        }
                    ));
                }
                other => panic!("expected right-associative assignment, got {other:?}"),
            }
        }
        other => panic!("expected function declaration, got {other:?}"),
    }
}

#[test]
fn rejects_top_level_statements_in_v01() {
    let source = "લખો(સાચું)";
    let (program, diagnostics) = tantra_compiler::parse_source(source);
    assert!(program.is_none());
    assert!(
        diagnostics.iter().any(|d| d.code == "T1002"),
        "{diagnostics:#?}"
    );
}

#[test]
fn parser_recovers_at_following_control_statement_without_semicolon() {
    let source = r#"
કાર્ય broken() {
    સ્થિર first: = 1
    પ્રયત્ન {
        કામ()
    } ભૂલ err {
        ફેંકો err
    }
}
"#;
    let (_, diagnostics) = tantra_compiler::parse_source(source);
    let type_errors = diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.code == "T1015")
        .count();
    assert_eq!(type_errors, 1, "{diagnostics:#?}");
}

#[test]
fn parses_struct_type_fields_and_generic_parameters() {
    let source = r#"
રૂપ વ્યક્તિ<T> {
    નામ: શબ્દ
    ઉમર: પૂર્ણાંક
}
"#;
    let (program, diagnostics) = tantra_compiler::parse_source(source);
    assert!(diagnostics.is_empty(), "{diagnostics:#?}");
    let program = program.expect("program should parse");

    match &program.declarations[0] {
        Decl::Type {
            name,
            generic_params,
            fields,
            ..
        } => {
            assert_eq!(name, "વ્યક્તિ");
            assert_eq!(generic_params, &vec!["T".to_owned()]);
            assert_eq!(fields.len(), 2);
            assert_eq!(fields[0].name, "નામ");
            assert_eq!(fields[0].ty.name, "શબ્દ");
            assert_eq!(fields[1].name, "ઉમર");
            assert_eq!(fields[1].ty.name, "પૂર્ણાંક");
        }
        other => panic!("expected struct type declaration, got {other:?}"),
    }
}

#[test]
fn rejects_struct_type_missing_field_type() {
    let source = r#"
રૂપ વ્યક્તિ {
    નામ:
}
"#;
    let (_, diagnostics) = tantra_compiler::parse_source(source);
    assert!(
        diagnostics
            .iter()
            .any(|d| d.code == "T1015" || d.code == "T1036"),
        "{diagnostics:#?}"
    );
}

#[test]
fn parses_nested_generic_type_arguments() {
    let source = r#"
સ્થિર values: Array<Array<પૂર્ણાંક>> = []
"#;
    let (program, diagnostics) = tantra_compiler::parse_source(source);
    assert!(diagnostics.is_empty(), "{diagnostics:#?}");
    let program = program.expect("program should parse");

    match &program.declarations[0] {
        Decl::Variable { ty: Some(ty), .. } => {
            assert_eq!(ty.name, "Array");
            assert_eq!(ty.arguments.len(), 1);
            assert_eq!(ty.arguments[0].name, "Array");
            assert_eq!(ty.arguments[0].arguments.len(), 1);
            assert_eq!(ty.arguments[0].arguments[0].name, "પૂર્ણાંક");
        }
        other => panic!("expected variable declaration with nested generic type, got {other:?}"),
    }
}

#[test]
fn preserves_right_shift_operator_after_nested_generic_fix() {
    let source = r#"
કાર્ય shift() {
    સ્થિર value = left >> right
}
"#;
    let (program, diagnostics) = tantra_compiler::parse_source(source);
    assert!(diagnostics.is_empty(), "{diagnostics:#?}");
    let program = program.expect("program should parse");

    match &program.declarations[0] {
        Decl::Function { body, .. } => match &body.statements[..] {
            [tantra_compiler::parser::Stmt::Decl(Decl::Variable {
                value: Expr::Binary { op, .. },
                ..
            })] => {
                assert_eq!(*op, TokenKind::ShiftRight);
            }
            other => panic!("expected right-shift expression, got {other:?}"),
        },
        other => panic!("expected function declaration, got {other:?}"),
    }
}

#[test]
fn parses_standalone_block_statement_as_block_ast() {
    let source = r#"
કાર્ય test() {
    {
        સ્થિર value: પૂર્ણાંક = 1
    }
}
"#;
    let (program, diagnostics) = tantra_compiler::parse_source(source);
    assert!(diagnostics.is_empty(), "{diagnostics:#?}");
    let program = program.expect("program should parse");

    match &program.declarations[0] {
        Decl::Function { body, .. } => match &body.statements[..] {
            [tantra_compiler::parser::Stmt::Block(block)] => {
                assert_eq!(block.statements.len(), 1);
            }
            other => panic!("expected standalone block statement, got {other:?}"),
        },
        other => panic!("expected function declaration, got {other:?}"),
    }
}

#[test]
fn parses_else_block_as_block_statement() {
    let source = r#"
કાર્ય test(value: પૂર્ણાંક) {
    જો value {
        પરત
    } નહીં {
        પરત
    }
}
"#;
    let (program, diagnostics) = tantra_compiler::parse_source(source);
    assert!(diagnostics.is_empty(), "{diagnostics:#?}");
    let program = program.expect("program should parse");

    match &program.declarations[0] {
        Decl::Function { body, .. } => match &body.statements[..] {
            [tantra_compiler::parser::Stmt::If { else_branch, .. }] => {
                assert!(matches!(
                    else_branch.as_deref(),
                    Some(tantra_compiler::parser::Stmt::Block(_))
                ));
            }
            other => panic!("expected if statement, got {other:?}"),
        },
        other => panic!("expected function declaration, got {other:?}"),
    }
}

#[test]
fn preserves_else_if_chain_as_nested_if_ast() {
    let source = r#"
કાર્ય classify(value: પૂર્ણાંક) {
    જો value > 10 {
        પરત
    } નહીં જો value > 0 {
        પરત
    } નહીં {
        પરત
    }
}
"#;
    let (program, diagnostics) = tantra_compiler::parse_source(source);
    assert!(diagnostics.is_empty(), "{diagnostics:#?}");
    let program = program.expect("program should parse");

    match &program.declarations[0] {
        Decl::Function { body, .. } => match &body.statements[..] {
            [tantra_compiler::parser::Stmt::If { else_branch, .. }] => {
                let nested = else_branch.as_deref().expect("expected else-if branch");
                match nested {
                    tantra_compiler::parser::Stmt::If {
                        else_branch: nested_else,
                        ..
                    } => {
                        assert!(matches!(
                            nested_else.as_deref(),
                            Some(tantra_compiler::parser::Stmt::Block(_))
                        ));
                    }
                    other => panic!("expected nested else-if AST, got {other:?}"),
                }
            }
            other => panic!("expected outer if statement, got {other:?}"),
        },
        other => panic!("expected function declaration, got {other:?}"),
    }
}

#[test]
fn preserves_nested_control_flow_bodies_as_block_ast() {
    let source = r#"
કાર્ય nested(items: Array<પૂર્ણાંક>) {
    જ્યારે સાચું {
        માટે દરેક item માં items {
            જો item > 0 {
                પ્રયત્ન {
                    લખો(item)
                } ભૂલ err {
                    ફેંકો err
                }
            }
        }
    }
}
"#;
    let (program, diagnostics) = tantra_compiler::parse_source(source);
    assert!(diagnostics.is_empty(), "{diagnostics:#?}");
    let program = program.expect("program should parse");

    match &program.declarations[0] {
        Decl::Function { body, .. } => match &body.statements[..] {
            [tantra_compiler::parser::Stmt::While {
                body: while_body, ..
            }] => match &while_body.statements[..] {
                [tantra_compiler::parser::Stmt::ForEach { body: for_body, .. }] => {
                    match &for_body.statements[..] {
                        [tantra_compiler::parser::Stmt::If { then_block, .. }] => {
                            assert!(matches!(
                                then_block.statements.as_slice(),
                                [tantra_compiler::parser::Stmt::Try { .. }]
                            ));
                        }
                        other => panic!("expected nested if statement, got {other:?}"),
                    }
                }
                other => panic!("expected nested foreach statement, got {other:?}"),
            },
            other => panic!("expected nested while statement, got {other:?}"),
        },
        other => panic!("expected function declaration, got {other:?}"),
    }
}


#[test]
fn rejects_multi_character_literal_without_cascade() {
    let (tokens, diagnostics) = Lexer::new("'ab' c").lex();
    assert_eq!(
        diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.code == "T0007")
            .count(),
        1,
        "{diagnostics:#?}"
    );
    assert!(
        matches!(tokens[1].kind, TokenKind::Identifier(ref name) if name == "c"),
        "expected lexer recovery to resume after malformed character literal: {tokens:#?}"
    );
}

#[test]
fn parses_chained_postfix_expression() {
    let source = r#"
કાર્ય chain() {
    સ્થિર value = make(1)[0].name
}
"#;
    let (program, diagnostics) = tantra_compiler::parse_source(source);
    assert!(diagnostics.is_empty(), "{diagnostics:#?}");
    let program = program.expect("program should parse");

    match &program.declarations[0] {
        Decl::Function { body, .. } => match &body.statements[..] {
            [tantra_compiler::parser::Stmt::Decl(Decl::Variable {
                value: Expr::Member { object, .. },
                ..
            })] => match object.as_ref() {
                Expr::Index { object, .. } => {
                    assert!(matches!(object.as_ref(), Expr::Call { .. }));
                }
                other => panic!("expected indexed call expression, got {other:?}"),
            },
            other => panic!("expected variable declaration, got {other:?}"),
        },
        other => panic!("expected function declaration, got {other:?}"),
    }
}

#[test]
fn parses_multiple_and_nested_generic_arguments() {
    let source = r#"
સ્થિર values: Map<શબ્દ, Array<પૂર્ણાંક>> = []
"#;
    let (program, diagnostics) = tantra_compiler::parse_source(source);
    assert!(diagnostics.is_empty(), "{diagnostics:#?}");
    let program = program.expect("program should parse");

    match &program.declarations[0] {
        Decl::Variable { ty: Some(ty), .. } => {
            assert_eq!(ty.name, "Map");
            assert_eq!(ty.arguments.len(), 2);
            assert_eq!(ty.arguments[0].name, "શબ્દ");
            assert_eq!(ty.arguments[1].name, "Array");
            assert_eq!(ty.arguments[1].arguments[0].name, "પૂર્ણાંક");
        }
        other => panic!("expected typed variable declaration, got {other:?}"),
    }
}

#[test]
fn parses_complete_binary_operator_precedence_chain() {
    let source = r#"
કાર્ય operators() {
    સ્થિર value = a || b && c | d ^ e & f == g != h < i <= j > k >= l << m >> n + o - p * q / r % s ** t ?? u
}
"#;
    let (program, diagnostics) = tantra_compiler::parse_source(source);
    assert!(diagnostics.is_empty(), "{diagnostics:#?}");
    assert!(program.is_some(), "operator precedence chain should parse");
}

#[test]
fn conditional_expression_is_right_associative() {
    let source = r#"
સ્થિર value = a ? b : c ? d : e
"#;
    let (program, diagnostics) = tantra_compiler::parse_source(source);
    assert!(diagnostics.is_empty(), "{diagnostics:#?}");
    let program = program.expect("program should parse");

    match &program.declarations[0] {
        Decl::Variable {
            value:
                Expr::Conditional {
                    else_expr, ..
                },
            ..
        } => {
            assert!(matches!(else_expr.as_ref(), Expr::Conditional { .. }));
        }
        other => panic!("expected right-associative conditional expression, got {other:?}"),
    }
}
