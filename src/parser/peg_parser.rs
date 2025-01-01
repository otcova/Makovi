use super::*;
use crate::ast::*;
use std::cell::RefCell;

peg::parser!(pub grammar parser<'a>(arena: &ExprArena<'input>, buffers: &'a ReusablePool<VecExpr<'input>>) for str {
    pub rule function() -> FunctionAst<'input>
        = [' ' | '\n']* "function" _ name:identifier() _
        "(" params_names:((_ i:identifier() _ {i}) ** ",") ")" _
        "->" _
        "(" return_name:(_ i:identifier() _ {i}) ")" _
        "{" _ "\n"
        statements:statements()
        _ "}" _ "\n" _
        {
            FunctionAst {
                name,
                params_names,
                return_name,
                statements,
                // statements: statements.take_owned(),
            }
        }

    rule statements() -> VecExpr<'input>
        = s:(statement()*) { s }

    // rule statements() -> VecExpr<'input>
    //     = s:(statement()*) { VecExpr::from_vec(s) }

    // rule statements() -> VecExpr<'input> = s:statements_loop()
    //     { s.into_inner() }
    //
    // rule statements_loop() -> RefCell<VecExpr<'input>> = precedence! {
    //     s:statement() vec:statements_loop() { vec.borrow_mut().push(s); vec }
    //     // _ { RefCell::new(buffers.extract_buffer()) }
    //     _ { RefCell::new(VecExpr::new()) }
    // }


    // rule statements() -> Reused<'a, VecExpr<'input>> = s:statements_loop()
    //     { s.into_inner() }
    //
    // rule statements_loop() -> RefCell<Reused<'a, VecExpr<'input>>> = precedence! {
    //     s:statement() vec:statements_loop() { vec.borrow_mut().push(s); vec }
    //     _ { RefCell::new(buffers.take_buffer()) }
    // }

    rule statement() -> ExprAst<'input>
        = _ e:expression() _ "\n" { e }

    rule expression() -> ExprAst<'input>
        = if_else()
        / while_loop()
        / assignment()
        / binary_op()

    rule if_else() -> ExprAst<'input> = precedence! {
        "if" _ e:expression() _ "{" _ "\n"
            then_body:statements() _
        "}" _ "else" _ "{" _ "\n"
            else_body:statements() _
        "}"
        { ExprAst::IfElse(arena.push(e), arena.push_all(then_body.as_slice()), arena.push_all(else_body.as_slice())) }

        "if" _ e:expression() _ "{" _ "\n"
            then_body:statements() _
        "}" _ "else" _ else_body:if_else()
        { ExprAst::IfElseIf(arena.push(e), arena.push_all(then_body.as_slice()), arena.push(else_body)) }
    }

    rule while_loop() -> ExprAst<'input>
        = "while" _ e:expression() _ "{" _ "\n"
        loop_body:statements() _ "}"
        { ExprAst::WhileLoop(arena.push(e), arena.push_all(loop_body.as_slice())) }

    rule assignment() -> ExprAst<'input>
        = i:identifier() _ "=" _ e:expression() {ExprAst::Assign(i, arena.push(e))}

    rule binary_op() -> ExprAst<'input> = precedence!{
        a:@ _ "==" _ b:(@) { ExprAst::Eq(arena.push(a), arena.push(b)) }
        a:@ _ "!=" _ b:(@) { ExprAst::Ne(arena.push(a), arena.push(b)) }
        a:@ _ "<"  _ b:(@) { ExprAst::Lt(arena.push(a), arena.push(b)) }
        a:@ _ "<=" _ b:(@) { ExprAst::Le(arena.push(a), arena.push(b)) }
        a:@ _ ">"  _ b:(@) { ExprAst::Gt(arena.push(a), arena.push(b)) }
        a:@ _ ">=" _ b:(@) { ExprAst::Ge(arena.push(a), arena.push(b)) }
        --
        a:@ _ "+" _ b:(@) { ExprAst::Add(arena.push(a), arena.push(b)) }
        a:@ _ "-" _ b:(@) { ExprAst::Sub(arena.push(a), arena.push(b)) }
        --
        a:@ _ "*" _ b:(@) { ExprAst::Mul(arena.push(a), arena.push(b)) }
        a:@ _ "/" _ b:(@) { ExprAst::Div(arena.push(a), arena.push(b)) }
        --
        a:@ _ "mod" _ "(" _ b:expression() _ ")" { ExprAst::Mod(arena.push(a), arena.push(b)) }
        a:@ _ "mod" _ b:(@) { ExprAst::Mod(arena.push(a), arena.push(b)) }
        --
        i:identifier() _ "(" args:((_ e:expression() _ {e}) ** ",") ")" { ExprAst::Call(i, arena.push_vec(args)) }
        i:identifier() { ExprAst::Identifier(i) }
        l:literal() { l }
    }

    rule identifier() -> &'input str
        = quiet!{ n:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) { n } }
        / expected!("identifier")

    rule literal() -> ExprAst<'input>
        = n:$(['0'..='9']+) { ExprAst::Literal(n) }
        / "&" i:identifier() { ExprAst::GlobalDataAddr(i) }

    rule _() =  quiet!{[' ']*}
});
