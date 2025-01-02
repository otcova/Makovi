use crate::ast::*;

peg::parser!(pub grammar parser<'a>(arena: &'input Ast<'input>) for str {
    pub rule function() -> FunctionExpr<'input>
        = [' ' | '\n']* "function" _ name:identifier() _
        "(" params_names:((_ i:identifier() _ {i}) ** ",") ")" _
        "->" _
        "(" return_name:(_ i:identifier() _ {i}) ")" _
        "{" _ "\n"
        statements()
        _ "}" _ "\n" _
        {
            FunctionExpr {
                name,
                params_names,
                return_name,
                statements: arena,
            }
        }

    // rule statements() -> VecExpr<'input>
    //     = s:(statement()*) { s }
    //
    // rule statements() -> VecExpr<'input> = s:statements_loop()
    //     { s.into_inner() }
    // rule statements_loop() -> RefCell<VecExpr<'input>> =
    //     statements_iter() / statements_last()
    // rule statements_iter() -> RefCell<VecExpr<'input>> =
    //     s:statement() vec:statements_loop() { vec.borrow_mut().push(s); vec }
    // rule statements_last() -> RefCell<VecExpr<'input>> =
    //     _ { RefCell::new(VecExpr::new()) }

    rule statements() = statement() statements() / _()
    rule statement() = _ e:expression() _ "\n" { arena.push(e); }

    rule expression() -> Expr<'input> =
        if_statement()
        / while_loop()
        / assignment()
        / binary_op()

    rule if_statement() -> Expr<'input> = if_else() / if_else_if()

    rule if_else() -> Expr<'input> =
        "if" _ e:expression() _ "{" _ "\n"
            statements() _
        "}" _ "else" _ "{" _ "\n"
            statements() _
        "}"
        { Expr::IfElse(arena.push(e), 4, 3) }

    rule if_else_if() -> Expr<'input> =
        "if" _ e:expression() _ "{" _ "\n"
            statements() _
        "}" _ "else" _ else_body:if_statement()
        { Expr::IfElseIf(arena.push(e), 2, arena.push(else_body)) }

    rule while_loop() -> Expr<'input> =
        "while" _ e:expression() _ "{" _ "\n" statements() _ "}"
        { Expr::WhileLoop(arena.push(e), 1) }

    rule assignment() -> Expr<'input>
        = i:identifier() _ "=" _ e:expression() {Expr::Assign(i, arena.push(e))}

    rule binary_op() -> Expr<'input> = precedence!{
        a:@ _ "==" _ b:(@) { Expr::Eq(arena.push(a), arena.push(b)) }
        a:@ _ "!=" _ b:(@) { Expr::Ne(arena.push(a), arena.push(b)) }
        a:@ _ "<"  _ b:(@) { Expr::Lt(arena.push(a), arena.push(b)) }
        a:@ _ "<=" _ b:(@) { Expr::Le(arena.push(a), arena.push(b)) }
        a:@ _ ">"  _ b:(@) { Expr::Gt(arena.push(a), arena.push(b)) }
        a:@ _ ">=" _ b:(@) { Expr::Ge(arena.push(a), arena.push(b)) }
        --
        a:@ _ "+" _ b:(@) { Expr::Add(arena.push(a), arena.push(b)) }
        a:@ _ "-" _ b:(@) { Expr::Sub(arena.push(a), arena.push(b)) }
        --
        a:@ _ "*" _ b:(@) { Expr::Mul(arena.push(a), arena.push(b)) }
        a:@ _ "/" _ b:(@) { Expr::Div(arena.push(a), arena.push(b)) }
        --
        a:@ _ "mod" _ "(" _ b:expression() _ ")" { Expr::Mod(arena.push(a), arena.push(b)) }
        a:@ _ "mod" _ b:(@) { Expr::Mod(arena.push(a), arena.push(b)) }
        --
        i:identifier() _ "(" args:((_ e:expression() _ {e}) ** ",") ")" { Expr::Call(i, arena.push_vec(args)) }
        i:identifier() { Expr::Identifier(i) }
        l:literal() { l }
    }

    rule identifier() -> &'input str
        = quiet!{ n:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) { n } }
        / expected!("identifier")

    rule literal() -> Expr<'input>
        = n:$(['0'..='9']+) { Expr::Literal(n) }
        / "&" i:identifier() { Expr::GlobalDataAddr(i) }

    rule _() =  quiet!{[' ']*}
});
