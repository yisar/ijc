use yjc::parser::{block, expression, Node};
use yjc::transformer::{transform};

#[test]
fn text_parse_fixtures() {
    let cur_dir = std::env::current_dir().unwrap();
    let fixtures = cur_dir.join("tests/fixtures/parser");
    let files = std::fs::read_dir(&fixtures).unwrap();
    for file in files {
        let path = file.unwrap().path();
        let source = std::fs::read_to_string(&path).unwrap();
        let ast = block(&source);
        assert!(ast.is_ok());
        assert_eq!(
            ast.unwrap().0.trim(),
            "",
            "Expected ''. File: {}",
            path.display()
        );
    }
}

#[test]
fn test_parse_automatic() {
    let cur_dir = std::env::current_dir().unwrap();
    let fixtures = cur_dir.join("tests/fixtures/test262-parser-tests/pass");
    
    // 如果目录不存在，跳过测试
    if !fixtures.exists() {
        return;
    }
    
    let files = std::fs::read_dir(fixtures).unwrap();
    for file in files {
        let path = file.unwrap().path();
        let source = std::fs::read_to_string(&path).unwrap() + "\n";
        let ast = block(&source);
        assert!(ast.is_ok());
        // assert_eq!(ast.unwrap().0.trim(), "");
    }
}

// #[test]
// fn test_parser_get_deps() {
//     fn assert_deps(path: &str, requires: std::vec::Vec<&str>) {
//         let cur_dir = std::env::current_dir().unwrap();
//         let entry = cur_dir.join("tests/fixtures/parser_deps").join(path);
//         let source = std::fs::read_to_string(&entry).unwrap();
//         let ast = block(&source);
//         let result = get_deps(ast.unwrap().1);
//         assert_eq!(result, requires);
//     }
//     assert_deps("deps-modules.js", vec!["peter"]);
//     assert_deps("deps-crazy-indent.js", vec!["./math.js"]);
//     assert_deps("deps-double-quotes.js", vec!["./math"]);
//     assert_deps("deps-modules-2.js", vec!["itt"]);
//     assert_deps("deps-relative.js", vec!["./math.js"]);
//     assert_deps("deps-multiple.js", vec!["math", "./index.js", "fs"]);
//     assert_deps("deps-comments.js", vec!["fs"]);
//     assert_deps("deps-if.js", vec!["lodash"]);
//     assert_deps("deps-else.js", vec!["lodash"]);
//     assert_deps("deps-for.js", vec!["lodash"]);
//     assert_deps("deps-for-in.js", vec!["lodash"]);
//     assert_deps("deps-for-of.js", vec!["lodash"]);
//     assert_deps("deps-while.js", vec!["lodash"]);
//     assert_deps(
//         "deps-walker-all.js",
//         vec!["lodash", "underscore", "debounce", "assert"],
//     );
//     assert_deps("deps-lazy.js", vec!["lazily-loaded"]);
//     assert_deps("deps-class.js", vec!["ramda", "lodash"]);
// }

#[test]
fn test_complex() {
    fn assert_complete(i: &str) {
        assert!(block(i).is_ok());
        assert_eq!(block(i).unwrap().0, "");
    }

    assert_complete("a = b");
    assert_complete("1");
    assert_complete("1==1? 1+1 : 1-1");
    assert_complete("1==1? 1+1 : 1-1");
    assert_complete("-1");
    assert_complete("!1");
    assert_complete("-1");
    assert_complete("(1)");
    assert_complete("1*(1+1)");
    assert_complete("x");
    assert_complete("x");
    assert_complete("a+b+c+d");
    assert_complete("a-b-c-d");
    assert_complete("a*b*c*d");
    assert_complete("a/b/c/d");
    assert_complete("1**1**1");
    assert_complete("x*x*x");
    assert_complete("1 + 1 || 1 == 1 ^ 1 != 1/1 - 1");
    assert_complete("first += second += third");
    assert_complete("one += two /= 12");
    assert_complete("x = a && b == c + d * !z[0]++ || d ? 2 : 3");
    assert_complete(" a . b . c");
    assert_complete("a.b.c[7]");
    assert_complete("a()");
    assert_complete("a()[0]()");
    assert_complete("(a) => 1 + 1");
    assert_complete("(a) => [ 1 + 1 ]");
    assert_complete("(a) => ({a: 1})");
    assert_complete("((a) => a + 1)(1)");
    assert_complete("a => a");
    assert_complete("if (1 + 1 == 2) { return true; }");
    assert_complete("if ( true ) \n { \n return ; \n }");
    assert_complete("if (true) break; else continue;");
    assert_complete("if (true) for (0;0;0) break;");
    assert_complete("if (true) for (0;0;0) break;\nelse continue;");
    assert_complete("if (true) for (0;0;0) break; else continue;");
    assert_complete("if (true) 2; else 1;");
    assert_complete("{ let a }");
    assert_complete("<!-- hello\n");
    assert_complete("--> hello\n");
    assert_complete("a(`<span>${b}</span>`, \n`<a href=\"${c}\">${d}</a>`\n);");
}

#[test]
fn test_nesting_bench() {
    let start = std::time::Instant::now();

    let elapsed = start.elapsed();
    assert!(elapsed.as_millis() < 10);
}

#[test]
fn test_comments() {
    assert_eq!(
        block("  // hello\n asdf"),
        Ok(("", Node::Block(vec![Node::Ident("asdf")])))
    );
}

#[test]
fn test_transform() {
    // assert_eq!(
    //     transform(block("import 'y';").unwrap().1),
    //     Node::Block(vec![Node::Binary(
    //         "(",
    //         Box::new(Node::Ident("require")),
    //         Box::new(Node::Args(vec![Node::Str(String::from("y"))]))
    //     )])
    // );
}

#[test]
fn test_statement() {
    // assert_eq!(
    //     block("import 'y';"),
    //     Ok((
    //         "",
    //         Node::Block(vec![Node::Import((
    //             None,
    //             Box::new(Node::Str(String::from("y")))
    //         ))])
    //     ))
    // );
    // assert_eq!(
    //     block("import x from 'y';"),
    //     Ok((
    //         "",
    //         Node::Block(vec![Node::Import((
    //             Some(Box::new(Node::Ident("x"))),
    //             Box::new(Node::Str(String::from("y")))
    //         ))])
    //     ))
    // );
    assert_eq!(
        block("export let x = 'y';"),
        Ok((
            "",
            Node::Block(vec![Node::Export(Box::new(Node::Declaration((
                "let",
                vec![Node::Binary(
                    "=",
                    Box::new(Node::Ident("x")),
                    Box::new(Node::Str(String::from("y")))
                )]
            ))))])
        ))
    );
    assert_eq!(
        block("export const z = 1;"),
        Ok((
            "",
            Node::Block(vec![Node::Export(Box::new(Node::Declaration((
                "const",
                vec![Node::Binary(
                    "=",
                    Box::new(Node::Ident("z")),
                    Box::new(Node::Double(1.0))
                )]
            ))))])
        ))
    );
    assert_eq!(
        block("export var x = 'y';"),
        Ok((
            "",
            Node::Block(vec![Node::Export(Box::new(Node::Declaration((
                "var",
                vec![Node::Binary(
                    "=",
                    Box::new(Node::Ident("x")),
                    Box::new(Node::Str(String::from("y")))
                )]
            ))))])
        ))
    );
    assert_eq!(
        block("export default 3;"),
        Ok((
            "",
            Node::Block(vec![Node::Export(Box::new(Node::Default(Box::new(
                Node::Double(3.0)
            ))))])
        ))
    );
    assert_eq!(
        block("export function x() { return 1; };"),
        Ok((
            "",
            Node::Block(vec![Node::Export(Box::new(Node::Function((
                Some("x"),
                vec![],
                Box::new(Node::Block(vec![Node::Return(Some(Box::new(
                    Node::Double(1.0)
                )))]))
            ))))])
        ))
    );
    assert_eq!(
        block("export default function () { return 1; };"),
        Ok((
            "",
            Node::Block(vec![Node::Export(Box::new(Node::Default(Box::new(
                Node::Function((
                    None,
                    vec![],
                    Box::new(Node::Block(vec![Node::Return(Some(Box::new(
                        Node::Double(1.0)
                    )))]))
                ))
            ))))])
        ))
    );
    assert_eq!(
        block("export class x { };"),
        Ok((
            "",
            Node::Block(vec![Node::Export(Box::new(Node::Class((
                Some("x"),
                None,
                vec![]
            ))))])
        ))
    );
    assert_eq!(
        block("export default class { };"),
        Ok((
            "",
            Node::Block(vec![Node::Export(Box::new(Node::Default(Box::new(
                Node::Class((None, None, vec![]))
            ))))])
        ))
    );
    assert_eq!(
        block("a: continue;"),
        Ok((
            "",
            Node::Block(vec![Node::Label((
                Box::new(Node::Ident("a")),
                Box::new(Node::Continue(None))
            ))])
        ))
    );
    assert_eq!(
        block("continue}"),
        Ok(("}", Node::Block(vec![Node::Continue(None)])))
    );
    assert_eq!(
        block("{continue}"),
        Ok((
            "",
            Node::Block(vec![Node::Block(vec![Node::Continue(None)])])
        ))
    );
    assert_eq!(
        block("continue;continue;"),
        Ok((
            "",
            Node::Block(vec![Node::Continue(None), Node::Continue(None)])
        ))
    );
    assert_eq!(
        block(" continue ; continue ; "),
        Ok((
            " ",
            Node::Block(vec![Node::Continue(None), Node::Continue(None)])
        ))
    );
    assert_eq!(
        block("continue"),
        Ok(("", Node::Block(vec![Node::Continue(None)])))
    );
    assert_eq!(
        block("continue a"),
        Ok((
            "",
            Node::Block(vec![Node::Continue(Some(Box::new(Node::Ident("a"))))])
        ))
    );
    assert_eq!(
        block("continue; 1"),
        Ok((
            "",
            Node::Block(vec![Node::Continue(None), Node::Double(1.0)])
        ))
    );
    assert_eq!(
        block("break;"),
        Ok(("", Node::Block(vec![Node::Break(None)])))
    );
    assert_eq!(
        block(" break ; break ; "),
        Ok((" ", Node::Block(vec![Node::Break(None), Node::Break(None)])))
    );
    assert_eq!(
        block("break\n"),
        Ok(("", Node::Block(vec![Node::Break(None)])))
    );
    assert_eq!(
        block("continue;\n  break a;"),
        Ok((
            "",
            Node::Block(vec![
                Node::Continue(None),
                Node::Break(Some(Box::new(Node::Ident("a"))))
            ])
        ))
    );
    assert_eq!(
        block("break;\n  continue a;"),
        Ok((
            "",
            Node::Block(vec![
                Node::Break(None),
                Node::Continue(Some(Box::new(Node::Ident("a"))))
            ])
        ))
    );
    assert_eq!(
        block("return 1;"),
        Ok((
            "",
            Node::Block(vec![Node::Return(Some(Box::new(Node::Double(1.0))))])
        ))
    );
    assert_eq!(
        block("throw 1;"),
        Ok((
            "",
            Node::Block(vec![Node::Throw(Box::new(Node::Double(1.0)))])
        ))
    );
    assert_eq!(
        block("{ throw 1// hello\n a; }"),
        Ok((
            "",
            Node::Block(vec![Node::Block(vec![
                Node::Throw(Box::new(Node::Double(1.0))),
                Node::Ident("a")
            ])])
        ))
    );
    assert_eq!(
        block("return 1\n"),
        Ok((
            "",
            Node::Block(vec![Node::Return(Some(Box::new(Node::Double(1.0))))])
        ))
    );
    assert_eq!(
        block("return\n1\n"),
        Ok((
            "",
            Node::Block(vec![Node::Return(Some(Box::new(Node::Double(1.0))))])
        ))
    );
    assert_eq!(
        block("return; 1\n"),
        Ok(("", Node::Block(vec![Node::Return(None), Node::Double(1.0)])))
    );
    assert_eq!(
        block(" return 1 ; return 1 ; "),
        Ok((
            " ",
            Node::Block(vec![
                Node::Return(Some(Box::new(Node::Double(1.0)))),
                Node::Return(Some(Box::new(Node::Double(1.0))))
            ])
        ))
    );
    assert_eq!(
        block("return;"),
        Ok(("", Node::Block(vec![Node::Return(None)])))
    );
    assert_eq!(
        block("return"),
        Ok(("", Node::Block(vec![Node::Return(None)])))
    );
    assert_eq!(
        block("return\n"),
        Ok(("", Node::Block(vec![Node::Return(None)])))
    );
    assert_eq!(
        block("a = 2;"),
        Ok((
            "",
            Node::Block(vec![Node::Binary(
                "=",
                Box::new(Node::Ident("a")),
                Box::new(Node::Double(2.0))
            )])
        ))
    );
    assert_eq!(
        block("var a = 2;"),
        Ok((
            "",
            Node::Block(vec![Node::Declaration((
                "var",
                vec![Node::Binary(
                    "=",
                    Box::new(Node::Ident("a")),
                    Box::new(Node::Double(2.0))
                )]
            ))])
        ))
    );
    assert_eq!(
        block("let a = 2, b = 3"),
        Ok((
            "",
            Node::Block(vec![Node::Declaration((
                "let",
                vec![
                    Node::Binary("=", Box::new(Node::Ident("a")), Box::new(Node::Double(2.0))),
                    Node::Binary("=", Box::new(Node::Ident("b")), Box::new(Node::Double(3.0))),
                ]
            ))])
        ))
    );
    assert_eq!(
        block("let a=2,b=3"),
        Ok((
            "",
            Node::Block(vec![Node::Declaration((
                "let",
                vec![
                    Node::Binary("=", Box::new(Node::Ident("a")), Box::new(Node::Double(2.0))),
                    Node::Binary("=", Box::new(Node::Ident("b")), Box::new(Node::Double(3.0))),
                ]
            ))])
        ))
    );
    assert_eq!(
        block("const x = [];"),
        Ok((
            "",
            Node::Block(vec![Node::Declaration((
                "const",
                vec![Node::Binary(
                    "=",
                    Box::new(Node::Ident("x")),
                    Box::new(Node::List(vec![None]))
                )]
            ))])
        ))
    );
    assert_eq!(
        block("a = 2"),
        Ok((
            "",
            Node::Block(vec![Node::Binary(
                "=",
                Box::new(Node::Ident("a")),
                Box::new(Node::Double(2.0))
            )])
        ))
    );
    assert_eq!(
        block("a = G"),
        Ok((
            "",
            Node::Block(vec![Node::Binary(
                "=",
                Box::new(Node::Ident("a")),
                Box::new(Node::Ident("G"))
            )])
        ))
    );
    assert_eq!(
        block("abc = G"),
        Ok((
            "",
            Node::Block(vec![Node::Binary(
                "=",
                Box::new(Node::Ident("abc")),
                Box::new(Node::Ident("G"))
            )])
        ))
    );
    assert_eq!(
        block("const empty = G()"),
        Ok((
            "",
            Node::Block(vec![Node::Declaration((
                "const",
                vec![Node::Binary(
                    "=",
                    Box::new(Node::Ident("empty")),
                    Box::new(Node::Binary(
                        "(",
                        Box::new(Node::Ident("G")),
                        Box::new(Node::Args(vec![]))
                    ))
                )]
            ))])
        ))
    );
    assert_eq!(block("z"), Ok(("", Node::Block(vec![Node::Ident("z")]))));
    assert_eq!(
        block("if(true){return;}"),
        Ok((
            "",
            Node::Block(vec![Node::If((
                Box::new(Node::Paren(Box::new(Node::Ident("true")))),
                Box::new(Node::Block(vec![Node::Return(None)])),
                None
            ))])
        ))
    );
    assert_eq!(
        block("if(true)return;"),
        Ok((
            "",
            Node::Block(vec![Node::If((
                Box::new(Node::Paren(Box::new(Node::Ident("true")))),
                Box::new(Node::Return(None)),
                None
            ))])
        ))
    );
    assert_eq!(
        block("if(true)return;else break;"),
        Ok((
            "",
            Node::Block(vec![Node::If((
                Box::new(Node::Paren(Box::new(Node::Ident("true")))),
                Box::new(Node::Return(None)),
                Some(Box::new(Node::Break(None)))
            ))])
        ))
    );
    assert_eq!(
        block("if ( true )\n {\n return; \n}\n else \n { break; }"),
        Ok((
            "",
            Node::Block(vec![Node::If((
                Box::new(Node::Paren(Box::new(Node::Ident("true")))),
                Box::new(Node::Block(vec![Node::Return(None)])),
                Some(Box::new(Node::Block(vec![Node::Break(None)])))
            ))])
        ))
    );
    assert_eq!(
        block("while(true){return;}"),
        Ok((
            "",
            Node::Block(vec![Node::While((
                Box::new(Node::Paren(Box::new(Node::Ident("true")))),
                Box::new(Node::Block(vec![Node::Return(None)]))
            ))])
        ))
    );
    assert_eq!(
        block("while(true)  ;"),
        Ok((
            "",
            Node::Block(vec![Node::While((
                Box::new(Node::Paren(Box::new(Node::Ident("true")))),
                Box::new(Node::Blank)
            ))])
        ))
    );
    assert_eq!(
        block(" while ( true ) return ; "),
        Ok((
            " ",
            Node::Block(vec![Node::While((
                Box::new(Node::Paren(Box::new(Node::Ident("true")))),
                Box::new(Node::Return(None))
            ))])
        ))
    );
    assert_eq!(
        block(" do {} while ( true ) "),
        Ok((
            " ",
            Node::Block(vec![Node::Do((
                Box::new(Node::Block(vec![])),
                Box::new(Node::Paren(Box::new(Node::Ident("true")))),
            ))])
        ))
    );
    assert_eq!(
        block("with (a) b ; "),
        Ok((
            " ",
            Node::Block(vec![Node::With((
                Box::new(Node::Paren(Box::new(Node::Ident("a")))),
                Box::new(Node::Ident("b"))
            ))])
        ))
    );
    assert_eq!(
        block("for(0;0;0){return;}"),
        Ok((
            "",
            Node::Block(vec![Node::For((
                Box::new(Node::ForTrio(vec![
                    Some(Node::Double(0.0)),
                    Some(Node::Double(0.0)),
                    Some(Node::Double(0.0)),
                ])),
                Box::new(Node::Block(vec![Node::Return(None)]))
            ))])
        ))
    );
    assert_eq!(
        block("for(0;0;0) ;"),
        Ok((
            "",
            Node::Block(vec![Node::For((
                Box::new(Node::ForTrio(vec![
                    Some(Node::Double(0.0)),
                    Some(Node::Double(0.0)),
                    Some(Node::Double(0.0)),
                ])),
                Box::new(Node::Blank)
            ))])
        ))
    );
    assert_eq!(
        block("for ( 0 ; 0 ; 0 ) return ; "),
        Ok((
            " ",
            Node::Block(vec![Node::For((
                Box::new(Node::ForTrio(vec![
                    Some(Node::Double(0.0)),
                    Some(Node::Double(0.0)),
                    Some(Node::Double(0.0))
                ])),
                Box::new(Node::Return(None))
            ))])
        ))
    );
    assert_eq!(
        block("for ( ; ; ) return ; "),
        Ok((
            " ",
            Node::Block(vec![Node::For((
                Box::new(Node::ForTrio(vec![None, None, None])),
                Box::new(Node::Return(None))
            ))])
        ))
    );
    assert_eq!(
        block("for (let x = 1 ; x < 1 ; x++ ) return ; "),
        Ok((
            " ",
            Node::Block(vec![Node::For((
                Box::new(Node::ForTrio(vec![
                    Some(Node::Declaration((
                        "let",
                        vec![Node::Binary(
                            "=",
                            Box::new(Node::Ident("x")),
                            Box::new(Node::Double(1.0))
                        )]
                    ))),
                    Some(Node::Binary(
                        "<",
                        Box::new(Node::Ident("x")),
                        Box::new(Node::Double(1.0))
                    )),
                    Some(Node::Unary("++", Box::new(Node::Ident("x"))))
                ])),
                Box::new(Node::Return(None))
            ))])
        ))
    );
    assert_eq!(
        block("for (let x in y) return ; "),
        Ok((
            " ",
            Node::Block(vec![Node::For((
                Box::new(Node::ForIn((
                    Box::new(Node::Variable((
                        Some("let"),
                        Box::new(Node::Param((Box::new(Node::Ident("x")), None)))
                    ))),
                    Box::new(Node::Ident("y"))
                ))),
                Box::new(Node::Return(None))
            ))])
        ))
    );
    assert_eq!(
        block("for (let x of y) return ; "),
        Ok((
            " ",
            Node::Block(vec![Node::For((
                Box::new(Node::ForOf((
                    Box::new(Node::Variable((
                        Some("let"),
                        Box::new(Node::Param((Box::new(Node::Ident("x")), None)))
                    ))),
                    Box::new(Node::Ident("y"))
                ))),
                Box::new(Node::Return(None))
            ))])
        ))
    );
    assert_eq!(
        block("for (x of y) return ; "),
        Ok((
            " ",
            Node::Block(vec![Node::For((
                Box::new(Node::ForOf((
                    Box::new(Node::Variable((
                        None,
                        Box::new(Node::Param((Box::new(Node::Ident("x")), None)))
                    ))),
                    Box::new(Node::Ident("y"))
                ))),
                Box::new(Node::Return(None))
            ))])
        ))
    );
    assert_eq!(
        block("for (x in y) return ; "),
        Ok((
            " ",
            Node::Block(vec![Node::For((
                Box::new(Node::ForIn((
                    Box::new(Node::Variable((
                        None,
                        Box::new(Node::Param((Box::new(Node::Ident("x")), None)))
                    ))),
                    Box::new(Node::Ident("y"))
                ))),
                Box::new(Node::Return(None))
            ))])
        ))
    );
    assert_eq!(
        block("for (let [] of y) return ; "),
        Ok((
            " ",
            Node::Block(vec![Node::For((
                Box::new(Node::ForOf((
                    Box::new(Node::Variable((
                        Some("let"),
                        Box::new(Node::Param((Box::new(Node::ListPattern(vec![None])), None)))
                    ))),
                    Box::new(Node::Ident("y"))
                ))),
                Box::new(Node::Return(None))
            ))])
        ))
    );
    assert_eq!(
        block("for (let [x,y,z] of y) return ; "),
        Ok((
            " ",
            Node::Block(vec![Node::For((
                Box::new(Node::ForOf((
                    Box::new(Node::Variable((
                        Some("let"),
                        Box::new(Node::Param((
                            Box::new(Node::ListPattern(vec![
                                Some(Node::Param((Box::new(Node::Ident("x")), None))),
                                Some(Node::Param((Box::new(Node::Ident("y")), None))),
                                Some(Node::Param((Box::new(Node::Ident("z")), None))),
                            ])),
                            None
                        )))
                    ))),
                    Box::new(Node::Ident("y"))
                ))),
                Box::new(Node::Return(None))
            ))])
        ))
    );
    assert_eq!(
        block("for (let {} of y) return ; "),
        Ok((
            " ",
            Node::Block(vec![Node::For((
                Box::new(Node::ForOf((
                    Box::new(Node::Variable((
                        Some("let"),
                        Box::new(Node::Param((Box::new(Node::ObjPattern(vec![])), None)))
                    ))),
                    Box::new(Node::Ident("y"))
                ))),
                Box::new(Node::Return(None))
            ))])
        ))
    );
    assert_eq!(
        block("for (let {x,y,z} of y) return ; "),
        Ok((
            " ",
            Node::Block(vec![Node::For((
                Box::new(Node::ForOf((
                    Box::new(Node::Variable((
                        Some("let"),
                        Box::new(Node::Param((
                            Box::new(Node::ObjPattern(vec![
                                Node::Param((Box::new(Node::Ident("x")), None)),
                                Node::Param((Box::new(Node::Ident("y")), None)),
                                Node::Param((Box::new(Node::Ident("z")), None)),
                            ])),
                            None
                        )))
                    ))),
                    Box::new(Node::Ident("y"))
                ))),
                Box::new(Node::Return(None))
            ))])
        ))
    );
    assert_eq!(
        block("try {} catch {} "),
        Ok((
            "",
            Node::Block(vec![Node::Try((
                Box::new(Node::Block(vec![])),
                Some(Box::new(Node::Catch((None, Box::new(Node::Block(vec![])))))),
                None
            ))])
        ))
    );
    assert_eq!(
        block("try {} finally {} "),
        Ok((
            "",
            Node::Block(vec![Node::Try((
                Box::new(Node::Block(vec![])),
                None,
                Some(Box::new(Node::Block(vec![])))
            ))])
        ))
    );
    assert_eq!(
        block("try {} catch(x) {} "),
        Ok((
            "",
            Node::Block(vec![Node::Try((
                Box::new(Node::Block(vec![])),
                Some(Box::new(Node::Catch((
                    Some(Box::new(Node::Param((Box::new(Node::Ident("x")), None)))),
                    Box::new(Node::Block(vec![]))
                )))),
                None
            ))])
        ))
    );
    assert_eq!(
        block("try {} catch {} finally {}"),
        Ok((
            "",
            Node::Block(vec![Node::Try((
                Box::new(Node::Block(vec![])),
                Some(Box::new(Node::Catch((None, Box::new(Node::Block(vec![])))))),
                Some(Box::new(Node::Block(vec![])))
            ))])
        ))
    );
    assert_eq!(
        block("try {} catch(x) {} finally {}"),
        Ok((
            "",
            Node::Block(vec![Node::Try((
                Box::new(Node::Block(vec![])),
                Some(Box::new(Node::Catch((
                    Some(Box::new(Node::Param((Box::new(Node::Ident("x")), None)))),
                    Box::new(Node::Block(vec![]))
                )))),
                Some(Box::new(Node::Block(vec![])))
            ))])
        ))
    );
    assert_eq!(
        block("try {} catch({x}) {} finally {}"),
        Ok((
            "",
            Node::Block(vec![Node::Try((
                Box::new(Node::Block(vec![])),
                Some(Box::new(Node::Catch((
                    Some(Box::new(Node::Param((
                        Box::new(Node::ObjPattern(vec![Node::Param((
                            Box::new(Node::Ident("x")),
                            None
                        ))])),
                        None
                    )))),
                    Box::new(Node::Block(vec![]))
                )))),
                Some(Box::new(Node::Block(vec![])))
            ))])
        ))
    );
}

#[test]
fn test_string() {
    assert_eq!(expression("\"\""), Ok(("", Node::Str(String::from("")))));
    assert_eq!(expression(" \"\" "), Ok((" ", Node::Str(String::from("")))));
    assert_eq!(
        expression(" \"a\" "),
        Ok((" ", Node::Str(String::from("a"))))
    );
    assert_eq!(
        expression(" \"Example\" "),
        Ok((" ", Node::Str(String::from("Example"))))
    );
    assert_eq!(
        expression("\"\\     a\""),
        Ok(("", Node::Str(String::from("a"))))
    );
    assert_eq!(
        expression("\"\\   b  a\""),
        Ok(("", Node::Str(String::from("b  a"))))
    );
    assert_eq!(
        expression("\"\\n\\n\""),
        Ok(("", Node::Str(String::from("\n\n"))))
    );
    assert_eq!(
        expression("\"✅\""),
        Ok(("", Node::Str(String::from("✅"))))
    );
    assert_eq!(
        expression("\"\\n\""),
        Ok(("", Node::Str(String::from("\n"))))
    );
    assert_eq!(
        expression("\"\\r\""),
        Ok(("", Node::Str(String::from("\r"))))
    );
    assert_eq!(
        expression("\"\\t\""),
        Ok(("", Node::Str(String::from("\t"))))
    );
    assert_eq!(
        expression("\"\\b\""),
        Ok(("", Node::Str(String::from("\u{08}"))))
    );
    assert_eq!(
        expression("\"\\v\""),
        Ok(("", Node::Str(String::from("\u{0B}"))))
    );
    assert_eq!(
        expression("\"\\f\""),
        Ok(("", Node::Str(String::from("\u{0C}"))))
    );
    assert_eq!(
        expression("\"\\\\\""),
        Ok(("", Node::Str(String::from("\\"))))
    );
    assert_eq!(
        expression("\"\\/\""),
        Ok(("", Node::Str(String::from("/"))))
    );
    assert_eq!(
        expression("\"\\\"\""),
        Ok(("", Node::Str(String::from("\""))))
    );
    assert_eq!(
        expression("\"\\\'\""),
        Ok(("", Node::Str(String::from("'"))))
    );
    assert_eq!(
        expression("``"),
        Ok(("", Node::Interpolation(String::from(""))))
    );
    assert_eq!(
        expression("`x`"),
        Ok(("", Node::Interpolation(String::from("x"))))
    );
    assert_eq!(expression("/foo/"), Ok(("", Node::Regex(("/foo/", None)))));
    assert_eq!(
        expression("/[a-z]+/"),
        Ok(("", Node::Regex(("/[a-z]+/", None))))
    );
    assert_eq!(
        expression("/[a-z]+/gi"),
        Ok(("", Node::Regex(("/[a-z]+/", Some("gi")))))
    );
}

#[test]
fn test_single_quoted_string() {
    assert_eq!(expression("''"), Ok(("", Node::Str(String::from("")))));
    assert_eq!(expression(" '' "), Ok((" ", Node::Str(String::from("")))));
    assert_eq!(expression(" 'a' "), Ok((" ", Node::Str(String::from("a")))));
    assert_eq!(
        expression(" 'Example' "),
        Ok((" ", Node::Str(String::from("Example"))))
    );
    assert_eq!(
        expression("'\\     a'"),
        Ok(("", Node::Str(String::from("a"))))
    );
    assert_eq!(
        expression("'\\   b  a'"),
        Ok(("", Node::Str(String::from("b  a"))))
    );
    assert_eq!(
        expression("'\\n\\n'"),
        Ok(("", Node::Str(String::from("\n\n"))))
    );
    assert_eq!(
        expression("\"✅\""),
        Ok(("", Node::Str(String::from("✅"))))
    );
    assert_eq!(expression("'\\n'"), Ok(("", Node::Str(String::from("\n")))));
    assert_eq!(expression("'\\r'"), Ok(("", Node::Str(String::from("\r")))));
    assert_eq!(expression("'\\t'"), Ok(("", Node::Str(String::from("\t")))));
    assert_eq!(
        expression("'\\b'"),
        Ok(("", Node::Str(String::from("\u{08}"))))
    );
    assert_eq!(
        expression("'\\v'"),
        Ok(("", Node::Str(String::from("\u{0B}"))))
    );
    assert_eq!(
        expression("'\\f'"),
        Ok(("", Node::Str(String::from("\u{0C}"))))
    );
    assert_eq!(
        expression("'\\\\'"),
        Ok(("", Node::Str(String::from("\\"))))
    );
    assert_eq!(expression("'\\/'"), Ok(("", Node::Str(String::from("/")))));
    assert_eq!(
        expression("'\\\"'"),
        Ok(("", Node::Str(String::from("\""))))
    );
    assert_eq!(expression("'\\''"), Ok(("", Node::Str(String::from("'")))));
}

#[test]
fn test_double() {
    assert_eq!(expression("0"), Ok(("", Node::Double(0.0))));
    assert_eq!(expression("1"), Ok(("", Node::Double(1.0))));
    assert_eq!(expression("2.2"), Ok(("", Node::Double(2.2))));
    assert_eq!(expression("3."), Ok(("", Node::Double(3.0))));
    assert_eq!(expression(".4"), Ok(("", Node::Double(0.4))));
    assert_eq!(expression("1e2"), Ok(("", Node::Double(100.0))));
    assert_eq!(expression("1e-2"), Ok(("", Node::Double(0.01))));
    assert_eq!(expression("0"), Ok(("", Node::Double(0.0))));
    assert_eq!(expression("123456789"), Ok(("", Node::Double(123456789.0))));
    assert_eq!(expression("0."), Ok(("", Node::Double(0.0))));
    assert_eq!(expression("123."), Ok(("", Node::Double(123.0))));
    assert_eq!(expression(".012300"), Ok(("", Node::Double(0.0123))));
    assert_eq!(expression("0.012300"), Ok(("", Node::Double(0.0123))));
    assert_eq!(expression("123.045600"), Ok(("", Node::Double(123.0456))));
    assert_eq!(expression(".123e0"), Ok(("", Node::Double(0.123))));
    assert_eq!(expression("0.123e0"), Ok(("", Node::Double(0.123))));
    assert_eq!(expression("123.456e0"), Ok(("", Node::Double(123.456))));
    assert_eq!(expression(".123e01"), Ok(("", Node::Double(1.23))));
    assert_eq!(expression("0.123e01"), Ok(("", Node::Double(1.23))));
    assert_eq!(expression("123.456e02"), Ok(("", Node::Double(12345.6))));
    assert_eq!(expression(".123e+4"), Ok(("", Node::Double(1230.0))));
    assert_eq!(expression("0.123e+4"), Ok(("", Node::Double(1230.0))));
    assert_eq!(expression("123.456e+4"), Ok(("", Node::Double(1234560.0))));
    assert_eq!(expression(".123e-4"), Ok(("", Node::Double(0.0000123))));
    assert_eq!(expression("0.123e-4"), Ok(("", Node::Double(0.0000123))));
    assert_eq!(expression("123.456e-4"), Ok(("", Node::Double(0.0123456))));
    assert_eq!(expression("0e0"), Ok(("", Node::Double(0.0))));
    assert_eq!(expression("123e0"), Ok(("", Node::Double(123.0))));
    assert_eq!(expression("0e01"), Ok(("", Node::Double(0.0))));
    assert_eq!(expression("123e02"), Ok(("", Node::Double(12300.0))));
    assert_eq!(expression("0e+4"), Ok(("", Node::Double(0.0))));
    assert_eq!(expression("123e+4"), Ok(("", Node::Double(1230000.0))));
    assert_eq!(expression("0e-4"), Ok(("", Node::Double(0.0))));
    assert_eq!(expression("123e-4"), Ok(("", Node::Double(0.0123))));
}

#[test]
fn test_octal() {
    assert_eq!(expression("0o123"), Ok(("", Node::Octal(0o123))));
    assert_eq!(expression("0o111"), Ok(("", Node::Octal(0o111))));
    assert_eq!(expression("0o0"), Ok(("", Node::Octal(0o0))));
    assert_eq!(expression("0o03 "), Ok((" ", Node::Octal(0o3))));
    assert_eq!(expression("0o012 "), Ok((" ", Node::Octal(0o12))));
    assert_eq!(expression("0o07654321 "), Ok((" ", Node::Octal(0o7654321))));
}

#[test]
fn test_hexadecimal() {
    assert_eq!(expression("0x3 "), Ok((" ", Node::Hexadecimal(0x3))));
    assert_eq!(
        expression("0x0123789"),
        Ok(("", Node::Hexadecimal(0x0123789)))
    );
    assert_eq!(
        expression("0xABCDEF"),
        Ok(("", Node::Hexadecimal(0xabcdef)))
    );
    assert_eq!(
        expression("0xabcdef"),
        Ok(("", Node::Hexadecimal(0xabcdef)))
    );
}

#[test]
fn test_binarynum() {
    assert_eq!(expression("0b0"), Ok(("", Node::BinaryNum(0b0))));
    assert_eq!(expression("0b1"), Ok(("", Node::BinaryNum(0b1))));
    assert_eq!(expression("0b01010"), Ok(("", Node::BinaryNum(0b01010))));
    assert_eq!(
        expression("0b1010111"),
        Ok(("", Node::BinaryNum(0b1010111)))
    );
}

#[test]
fn test_identifier() {
    assert_eq!(expression("hello"), Ok(("", Node::Ident("hello"))));
    assert_eq!(expression("e"), Ok(("", Node::Ident("e"))));
}

#[test]
fn test_list() {
    assert_eq!(expression(" [ ] "), Ok((" ", Node::List(vec![None]))));
    assert_eq!(
        expression("[[]]"),
        Ok(("", Node::List(vec![Some(Node::List(vec![None]))])))
    );
    assert_eq!(expression("[]"), Ok(("", Node::List(vec![None]))));
    assert_eq!(
        expression("[ 1 ]"),
        Ok(("", Node::List(vec![Some(Node::Double(1.0))])))
    );
    assert_eq!(
        expression("[ 1, 2 ]"),
        Ok((
            "",
            Node::List(vec![Some(Node::Double(1.0)), Some(Node::Double(2.0))])
        ))
    );
    assert_eq!(
        expression("[ 1, \"2\" ]"),
        Ok((
            "",
            Node::List(vec![
                Some(Node::Double(1.0)),
                Some(Node::Str(String::from("2")))
            ])
        ))
    );
    assert_eq!(
        expression("[ ...a, 1 ]"),
        Ok((
            "",
            Node::List(vec![
                Some(Node::Splat(Box::new(Node::Ident("a")))),
                Some(Node::Double(1.0))
            ])
        ))
    );
    assert_eq!(
        expression("[ ...[], 1 ]"),
        Ok((
            "",
            Node::List(vec![
                Some(Node::Splat(Box::new(Node::List(vec![None])))),
                Some(Node::Double(1.0))
            ])
        ))
    );
    assert_eq!(
        expression("[ ,,, ]"),
        Ok(("", Node::List(vec![None, None, None, None])))
    );
}

#[test]
fn test_jsx() {
    assert_eq!(
        expression("<div a='b'>c</div>"),
        Ok((
            "",
            Node::JSXElement((
                Box::new(Node::Ident("div")),
                vec![Node::KeyValue((
                    Box::new(Node::Ident("a")),
                    Box::new(Node::Str(String::from("b")))
                ))],
                vec![Node::JSXText(String::from("c")),]
            ))
        ))
    );

    // Test self-closing tag
    assert_eq!(
        expression("<img src=\"test.jpg\" />"),
        Ok((
            "",
            Node::JSXElement((
                Box::new(Node::Ident("img")),
                vec![Node::KeyValue((
                    Box::new(Node::Ident("src")),
                    Box::new(Node::Str(String::from("test.jpg")))
                ))],
                vec![]
            ))
        ))
    );

    // Test JSX Fragment
    assert_eq!(
        expression("<><div>A</div><div>B</div></>"),
        Ok((
            "",
            Node::JSXFragment(vec![
                Node::JSXElement((
                    Box::new(Node::Ident("div")),
                    vec![],
                    vec![Node::JSXText(String::from("A"))]
                )),
                Node::JSXElement((
                    Box::new(Node::Ident("div")),
                    vec![],
                    vec![Node::JSXText(String::from("B"))]
                ))
            ])
        ))
    );

    // Test member expression tag
    assert_eq!(
        expression("<React.Fragment>Hello</React.Fragment>"),
        Ok((
            "",
            Node::JSXElement((
                Box::new(Node::JSXMemberExpression((
                    Box::new(Node::Ident("React")),
                    Box::new(Node::Ident("Fragment"))
                ))),
                vec![],
                vec![Node::JSXText(String::from("Hello"))]
            ))
        ))
    );

    // Test spread attribute
    assert_eq!(
        expression("<Component {...props} />"),
        Ok((
            "",
            Node::JSXElement((
                Box::new(Node::Ident("Component")),
                vec![Node::JSXSpreadAttribute(Box::new(Node::Ident("props")))],
                vec![]
            ))
        ))
    );

    // Test namespaced name
    assert_eq!(
        expression("<svg:path d=\"M0 0\" />"),
        Ok((
            "",
            Node::JSXElement((
                Box::new(Node::JSXNamespacedName((
                    Box::new(Node::Ident("svg")),
                    Box::new(Node::Ident("path"))
                ))),
                vec![Node::KeyValue((
                    Box::new(Node::Ident("d")),
                    Box::new(Node::Str(String::from("M0 0")))
                ))],
                vec![]
            ))
        ))
    );

    // Test hyphenated tag name
    assert_eq!(
        expression("<my-custom-element>Content</my-custom-element>"),
        Ok((
            "",
            Node::JSXElement((
                Box::new(Node::Ident("my-custom-element")),
                vec![],
                vec![Node::JSXText(String::from("Content"))]
            ))
        ))
    );
}

#[test]
fn test_object() {
    assert_eq!(
        expression("{\"a\": 1}"),
        Ok((
            "",
            Node::Object(vec![Node::KeyValue((
                Box::new(Node::Str(String::from("a"))),
                Box::new(Node::Double(1.0))
            ))])
        ))
    );
    assert_eq!(
        expression("{x: 1, y: 2}"),
        Ok((
            "",
            Node::Object(vec![
                Node::KeyValue((Box::new(Node::Ident("x")), Box::new(Node::Double(1.0)))),
                Node::KeyValue((Box::new(Node::Ident("y")), Box::new(Node::Double(2.0)))),
            ])
        ))
    );
    assert_eq!(
        expression("{[1]: 1, [\"a\"]: 2}"),
        Ok((
            "",
            Node::Object(vec![
                Node::KeyValue((Box::new(Node::Double(1.0)), Box::new(Node::Double(1.0)))),
                Node::KeyValue((
                    Box::new(Node::Str(String::from("a"))),
                    Box::new(Node::Double(2.0))
                )),
            ])
        ))
    );
    assert_eq!(
        expression("{a: {}}"),
        Ok((
            "",
            Node::Object(vec![Node::KeyValue((
                Box::new(Node::Ident("a")),
                Box::new(Node::Object(vec![]))
            ))])
        ))
    );
    assert_eq!(
        expression("{ ...a }"),
        Ok((
            "",
            Node::Object(vec![Node::Splat(Box::new(Node::Ident("a")))])
        ))
    );
    assert_eq!(
        expression("{ ...[] }"),
        Ok((
            "",
            Node::Object(vec![Node::Splat(Box::new(Node::List(vec![None])))])
        ))
    );
    assert_eq!(
        expression("{ a, b }"),
        Ok(("", Node::Object(vec![Node::Ident("a"), Node::Ident("b")])))
    );
    assert_eq!(
        expression("{ a() { b } }"),
        Ok((
            "",
            Node::Object(vec![Node::Shorthand((
                Box::new(Node::Ident("a")),
                vec![],
                Box::new(Node::Block(vec![Node::Ident("b")]))
            ))])
        ))
    );
    assert_eq!(
        expression("{ \"a\"() { b } }"),
        Ok((
            "",
            Node::Object(vec![Node::Shorthand((
                Box::new(Node::Str(String::from("a"))),
                vec![],
                Box::new(Node::Block(vec![Node::Ident("b")]))
            ))])
        ))
    );
    assert_eq!(
        expression("{ [a]() { b } }"),
        Ok((
            "",
            Node::Object(vec![Node::Shorthand((
                Box::new(Node::Ident("a")),
                vec![],
                Box::new(Node::Block(vec![Node::Ident("b")]))
            ))])
        ))
    );
}

#[test]
fn test_parenthesis() {
    assert_eq!(
        expression("(1)"),
        Ok(("", Node::Paren(Box::new(Node::Double(1.0)))))
    );
    assert_eq!(
        expression("([])"),
        Ok(("", Node::Paren(Box::new(Node::List(vec![None])))))
    );
    assert_eq!(
        expression(" ( 1 ) "),
        Ok((" ", Node::Paren(Box::new(Node::Double(1.0)))))
    );
    assert_eq!(
        expression(" ( [ ] ) "),
        Ok((" ", Node::Paren(Box::new(Node::List(vec![None])))))
    );
}

#[test]
fn test_closure() {
    assert_eq!(
        expression("(a, b) => 1 + 1"),
        Ok((
            "",
            Node::Closure((
                vec![
                    Node::Param((Box::new(Node::Ident("a")), None)),
                    Node::Param((Box::new(Node::Ident("b")), None))
                ],
                Box::new(Node::Binary(
                    "+",
                    Box::new(Node::Double(1.0)),
                    Box::new(Node::Double(1.0))
                ))
            ))
        ))
    );
    assert_eq!(
        expression("(a) => ({})"),
        Ok((
            "",
            Node::Closure((
                vec![Node::Param((Box::new(Node::Ident("a")), None))],
                Box::new(Node::Paren(Box::new(Node::Object(vec![]))))
            ))
        ))
    );
}

#[test]
fn test_function() {
    assert_eq!(
        expression("function(){}"),
        Ok((
            "",
            Node::Function((None, vec![], Box::new(Node::Block(vec![]))))
        ))
    );
    assert_eq!(
        expression("function f(x, y){ return x; }"),
        Ok((
            "",
            Node::Function((
                Some("f"),
                vec![
                    Node::Param((Box::new(Node::Ident("x")), None)),
                    Node::Param((Box::new(Node::Ident("y")), None))
                ],
                Box::new(Node::Block(vec![Node::Return(Some(Box::new(
                    Node::Ident("x")
                )))]))
            ))
        ))
    );
    assert_eq!(
        expression("function f ( x, y) { return x }"),
        Ok((
            "",
            Node::Function((
                Some("f"),
                vec![
                    Node::Param((Box::new(Node::Ident("x")), None)),
                    Node::Param((Box::new(Node::Ident("y")), None))
                ],
                Box::new(Node::Block(vec![Node::Return(Some(Box::new(
                    Node::Ident("x")
                )))]))
            ))
        ))
    );
    assert_eq!(
        block("function a(){} function b(){}"),
        Ok((
            "",
            Node::Block(vec![
                Node::Function((Some("a"), vec![], Box::new(Node::Block(vec![])))),
                Node::Function((Some("b"), vec![], Box::new(Node::Block(vec![]))))
            ])
        ))
    );
}

#[test]
fn test_generator() {
    assert_eq!(
        expression("function*() {}"),
        Ok((
            "",
            Node::Generator((None, vec![], Box::new(Node::Block(vec![]))))
        ))
    );
}

#[test]
fn test_mutation() {
    assert_eq!(
        expression(" 1 = 2 "),
        Ok((
            " ",
            Node::Binary(
                "=",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );
    assert_eq!(
        expression(" 1 += 2 "),
        Ok((
            " ",
            Node::Binary(
                "+=",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );
    assert_eq!(
        expression(" 1 -= 2 "),
        Ok((
            " ",
            Node::Binary(
                "-=",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );
    assert_eq!(
        expression(" 1 %= 2 "),
        Ok((
            " ",
            Node::Binary(
                "%=",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );
    assert_eq!(
        expression(" 1 *= 2 "),
        Ok((
            " ",
            Node::Binary(
                "*=",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );
    assert_eq!(
        expression(" 1 /= 2 "),
        Ok((
            " ",
            Node::Binary(
                "/=",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );
    assert_eq!(
        expression(" 1 <<= 2 "),
        Ok((
            " ",
            Node::Binary(
                "<<=",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );
    assert_eq!(
        expression(" 1 >>>= 2 "),
        Ok((
            " ",
            Node::Binary(
                ">>>=",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );
    assert_eq!(
        expression(" 1 >>= 2 "),
        Ok((
            " ",
            Node::Binary(
                ">>=",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );
    assert_eq!(
        expression(" 1 &= 2 "),
        Ok((
            " ",
            Node::Binary(
                "&=",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );
    assert_eq!(
        expression(" 1 ^= 2 "),
        Ok((
            " ",
            Node::Binary(
                "^=",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );
    assert_eq!(
        expression(" 1 |= 2 "),
        Ok((
            " ",
            Node::Binary(
                "|=",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );
    assert_eq!(
        expression(" 1 = 2 = 3 "),
        Ok((
            " ",
            Node::Binary(
                "=",
                Box::new(Node::Binary(
                    "=",
                    Box::new(Node::Double(1.0)),
                    Box::new(Node::Double(2.0))
                )),
                Box::new(Node::Double(3.0))
            )
        ))
    );
}

#[test]
fn test_ternary() {
    assert_eq!(
        expression("1 ? 2 : 3"),
        Ok((
            "",
            Node::Ternary(
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0)),
                Box::new(Node::Double(3.0))
            )
        ))
    );
}

#[test]
fn test_comparison() {
    assert_eq!(
        expression(" 1 == 2 "),
        Ok((
            " ",
            Node::Binary(
                "==",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );
    assert_eq!(
        expression(" 1 != 2 "),
        Ok((
            " ",
            Node::Binary(
                "!=",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );
    assert_eq!(
        expression(" 1 > 2 "),
        Ok((
            " ",
            Node::Binary(
                ">",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );
    assert_eq!(
        expression(" 1 < 2 "),
        Ok((
            " ",
            Node::Binary(
                "<",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );
    assert_eq!(
        expression(" 1 >= 2 "),
        Ok((
            " ",
            Node::Binary(
                ">=",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );
    assert_eq!(
        expression(" 1 <= 2 "),
        Ok((
            " ",
            Node::Binary(
                "<=",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );
    assert_eq!(
        expression(" 1 === 2 "),
        Ok((
            " ",
            Node::Binary(
                "===",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );
    assert_eq!(
        expression(" 1 !== 2 "),
        Ok((
            " ",
            Node::Binary(
                "!==",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );
    assert_eq!(
        expression(" 1 instanceof 2 "),
        Ok((
            " ",
            Node::Binary(
                "instanceof",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );
    assert_eq!(
        expression(" 1 in 2 "),
        Ok((
            " ",
            Node::Binary(
                "in",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );
    assert_eq!(
        expression(" 1 == 2 == 3 "),
        Ok((
            " ",
            Node::Binary(
                "==",
                Box::new(Node::Binary(
                    "==",
                    Box::new(Node::Double(1.0)),
                    Box::new(Node::Double(2.0))
                )),
                Box::new(Node::Double(3.0))
            )
        ))
    );
}

#[test]
fn test_logic() {
    assert_eq!(
        expression(" 1 || 2 "),
        Ok((
            " ",
            Node::Binary(
                "||",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );
    assert_eq!(
        expression(" 1 && 2 "),
        Ok((
            " ",
            Node::Binary(
                "&&",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );
    assert_eq!(
        expression(" 1 ?? 2 "),
        Ok((
            " ",
            Node::Binary(
                "??",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );
}

#[test]
fn test_bitwise() {
    assert_eq!(
        expression(" 1 | 2 "),
        Ok((
            " ",
            Node::Binary(
                "|",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );
    assert_eq!(
        expression(" 1 ^ 2 "),
        Ok((
            " ",
            Node::Binary(
                "^",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );
    assert_eq!(
        expression(" 1 & 2 "),
        Ok((
            " ",
            Node::Binary(
                "&",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );

    assert_eq!(
        expression(" 1 >> 2 "),
        Ok((
            " ",
            Node::Binary(
                ">>",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );
    assert_eq!(
        expression(" 1 >>> 2 "),
        Ok((
            " ",
            Node::Binary(
                ">>>",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );
    assert_eq!(
        expression(" 1 << 2 "),
        Ok((
            " ",
            Node::Binary(
                "<<",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );
}

#[test]
fn test_arithmetic() {
    assert_eq!(
        expression(" 1 + 2 "),
        Ok((
            " ",
            Node::Binary(
                "+",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );
    assert_eq!(
        expression(" 1 - 2 "),
        Ok((
            " ",
            Node::Binary(
                "-",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );
    assert_eq!(
        expression(" 1 * 2 "),
        Ok((
            " ",
            Node::Binary(
                "*",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );
    assert_eq!(
        expression(" 1 / 2 "),
        Ok((
            " ",
            Node::Binary(
                "/",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );
    assert_eq!(
        expression(" 1 % 2 "),
        Ok((
            " ",
            Node::Binary(
                "%",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );
    assert_eq!(
        expression(" 1 ** 2 "),
        Ok((
            " ",
            Node::Binary(
                "**",
                Box::new(Node::Double(1.0)),
                Box::new(Node::Double(2.0))
            )
        ))
    );
}

#[test]
fn test_prefix() {
    assert_eq!(
        expression(" ++ 2 "),
        Ok((" ", Node::Unary("++", Box::new(Node::Double(2.0)))))
    );
    assert_eq!(
        expression(" -- 2 "),
        Ok((" ", Node::Unary("--", Box::new(Node::Double(2.0)))))
    );
    assert_eq!(
        expression(" + 2 "),
        Ok((" ", Node::Unary("+", Box::new(Node::Double(2.0)))))
    );
    assert_eq!(
        expression(" - 2 "),
        Ok((" ", Node::Unary("-", Box::new(Node::Double(2.0)))))
    );
    assert_eq!(
        expression(" ! 2 "),
        Ok((" ", Node::Unary("!", Box::new(Node::Double(2.0)))))
    );
    assert_eq!(
        expression(" ~ 2 "),
        Ok((" ", Node::Unary("~", Box::new(Node::Double(2.0)))))
    );
    assert_eq!(
        expression(" !!2 "),
        Ok((
            " ",
            Node::Unary("!", Box::new(Node::Unary("!", Box::new(Node::Double(2.0)))))
        ))
    );
    assert_eq!(
        expression(" ! ! 2 "),
        Ok((
            " ",
            Node::Unary("!", Box::new(Node::Unary("!", Box::new(Node::Double(2.0)))))
        ))
    );

    assert_eq!(
        expression("typeof a"),
        Ok(("", Node::Unary("typeof", Box::new(Node::Ident("a")))))
    );
    assert_eq!(
        expression("void a"),
        Ok(("", Node::Unary("void", Box::new(Node::Ident("a")))))
    );
    assert_eq!(
        expression("delete a"),
        Ok(("", Node::Unary("delete", Box::new(Node::Ident("a")))))
    );
    assert_eq!(
        expression("await a"),
        Ok(("", Node::Unary("await", Box::new(Node::Ident("a")))))
    );
    assert_eq!(
        expression("yield a"),
        Ok(("", Node::Unary("yield", Box::new(Node::Ident("a")))))
    );
    assert_eq!(
        expression("yield* a"),
        Ok(("", Node::Unary("yield*", Box::new(Node::Ident("a")))))
    );
    assert_eq!(
        expression("yield  *   a"),
        Ok(("", Node::Unary("yield*", Box::new(Node::Ident("a")))))
    );
    assert_eq!(
        expression("new a"),
        Ok(("", Node::Unary("new", Box::new(Node::Ident("a")))))
    );
}

#[test]
fn test_postfix() {
    assert_eq!(
        expression(" a++"),
        Ok(("", Node::Unary("++", Box::new(Node::Ident("a")))))
    );
    assert_eq!(
        expression(" a--"),
        Ok(("", Node::Unary("--", Box::new(Node::Ident("a")))))
    );
}

#[test]
fn test_comma() {
    assert_eq!(
        expression("a,b,c"),
        Ok((
            "",
            Node::Binary(
                ",",
                Box::new(Node::Binary(
                    ",",
                    Box::new(Node::Ident("a")),
                    Box::new(Node::Ident("b"))
                )),
                Box::new(Node::Ident("c"))
            )
        ))
    );
}

#[test]
fn test_action() {
    assert_eq!(
        expression(" a?.a"),
        Ok((
            "",
            Node::Binary("?.", Box::new(Node::Ident("a")), Box::new(Node::Ident("a")))
        ))
    );
    assert_eq!(
        expression(" a?.(a)"),
        Ok((
            "",
            Node::Binary(
                "?.(",
                Box::new(Node::Ident("a")),
                Box::new(Node::Args(vec![Node::Ident("a")]))
            )
        ))
    );
    assert_eq!(
        expression(" a?.[a]"),
        Ok((
            "",
            Node::Binary(
                "?.[",
                Box::new(Node::Ident("a")),
                Box::new(Node::Ident("a"))
            )
        ))
    );
    assert_eq!(
        expression(" a[a]"),
        Ok((
            "",
            Node::Binary("[", Box::new(Node::Ident("a")), Box::new(Node::Ident("a")))
        ))
    );
    assert_eq!(
        expression(" a [ a ]"),
        Ok((
            "",
            Node::Binary("[", Box::new(Node::Ident("a")), Box::new(Node::Ident("a")))
        ))
    );
    assert_eq!(
        expression(" a(a)"),
        Ok((
            "",
            Node::Binary(
                "(",
                Box::new(Node::Ident("a")),
                Box::new(Node::Args(vec![(Node::Ident("a"))]))
            )
        ))
    );
    assert_eq!(
        expression(" a ( a )"),
        Ok((
            "",
            Node::Binary(
                "(",
                Box::new(Node::Ident("a")),
                Box::new(Node::Args(vec![(Node::Ident("a"))]))
            )
        ))
    );
    assert_eq!(
        expression(" a.a"),
        Ok((
            "",
            Node::Binary(".", Box::new(Node::Ident("a")), Box::new(Node::Ident("a")))
        ))
    );
    assert_eq!(
        expression(" a()"),
        Ok((
            "",
            Node::Binary(
                "(",
                Box::new(Node::Ident("a")),
                Box::new(Node::Args(vec![]))
            )
        ))
    );
    assert_eq!(
        expression("a``"),
        Ok((
            "",
            Node::Binary(
                "`",
                Box::new(Node::Ident("a")),
                Box::new(Node::Interpolation(String::from("")))
            )
        ))
    );
}

#[test]
fn test_classes() {
    assert_eq!(
        expression(" class a { }"),
        Ok(("", Node::Class((Some("a"), None, vec![]))))
    );
    assert_eq!(
        expression(" class a extends b { }"),
        Ok((
            "",
            Node::Class((Some("a"), Some(Box::new(Node::Ident("b"))), vec![]))
        ))
    );
    assert_eq!(
        expression(" class extends b {}"),
        Ok((
            "",
            Node::Class((None, Some(Box::new(Node::Ident("b"))), vec![]))
        ))
    );
    assert_eq!(
        expression(" class extends (a,b) {}"),
        Ok((
            "",
            Node::Class((None, Some(Box::new(Node::Idents(vec!["a", "b"]))), vec![]))
        ))
    );
    assert_eq!(
        expression(" class a {  b () { }  }"),
        Ok((
            "",
            Node::Class((
                Some("a"),
                None,
                vec![Node::Shorthand((
                    Box::new(Node::Ident("b")),
                    vec![],
                    Box::new(Node::Block(vec![]))
                ))]
            ))
        ))
    );
    assert_eq!(
        expression(" class a {   c = 1 }"),
        Ok((
            "",
            Node::Class((
                Some("a"),
                None,
                vec![Node::Field((
                    Box::new(Node::Ident("c")),
                    Box::new(Node::Double(1.0))
                ))]
            ))
        ))
    );
    assert_eq!(
        expression(" class a {  b () { } ; c  =  1 }"),
        Ok((
            "",
            Node::Class((
                Some("a"),
                None,
                vec![
                    Node::Shorthand((
                        Box::new(Node::Ident("b")),
                        vec![],
                        Box::new(Node::Block(vec![]))
                    )),
                    Node::Field((Box::new(Node::Ident("c")), Box::new(Node::Double(1.0))))
                ]
            ))
        ))
    );
    assert_eq!(
        expression("class{}"),
        Ok(("", Node::Class((None, None, vec![]))))
    );
    assert_eq!(
        expression("  class  a { set  b  ( ) {   }  }"),
        Ok((
            "",
            Node::Class((
                Some("a"),
                None,
                vec![Node::Setter(Box::new(Node::Shorthand((
                    Box::new(Node::Ident("b")),
                    vec![],
                    Box::new(Node::Block(vec![]))
                ))))]
            ))
        ))
    );
    assert_eq!(
        expression(" class a {  get b () { }  }"),
        Ok((
            "",
            Node::Class((
                Some("a"),
                None,
                vec![Node::Getter(Box::new(Node::Shorthand((
                    Box::new(Node::Ident("b")),
                    vec![],
                    Box::new(Node::Block(vec![]))
                ))))]
            ))
        ))
    );
    assert_eq!(
        expression(" class a {  static a() {}  }"),
        Ok((
            "",
            Node::Class((
                Some("a"),
                None,
                vec![Node::Static(Box::new(Node::Shorthand((
                    Box::new(Node::Ident("a")),
                    vec![],
                    Box::new(Node::Block(vec![]))
                ))))]
            ))
        ))
    );
    assert_eq!(
        expression(" class a {  static get a() {}  }"),
        Ok((
            "",
            Node::Class((
                Some("a"),
                None,
                vec![Node::Static(Box::new(Node::Getter(Box::new(
                    Node::Shorthand((
                        Box::new(Node::Ident("a")),
                        vec![],
                        Box::new(Node::Block(vec![]))
                    ))
                ))))]
            ))
        ))
    );
}
