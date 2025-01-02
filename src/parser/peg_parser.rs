use crate::ast::*;

peg::parser!(pub grammar parser<'a>(arena: &'input Ast<'input>) for str {
    pub rule function() -> FunctionExpr<'input>
        = [' ' | '\n']* "function" _ name:identifier() _
        "(" params_names:((_ i:identifier() _ {i}) ** ",") ")" _
        "->" _
        "(" return_name:(_ i:identifier() _ {i}) ")" _
        "{" _ "\n"
        statements:statements()
        _ "}" _ "\n" _
        {
            FunctionExpr {
                name,
                params_names,
                return_name,
                // statements: arena,
                statements,
            }
        }

    rule statements() -> Vec<AstNode<'input>> = s:(statement()*) { s }
    rule statement() -> AstNode<'input> = _ e:expression() _ "\n" { e }

    // rule statements() = statement() statements() / _()
    // rule statement() = _ e:expression() _ "\n" { e }

    rule expression() -> AstNode<'input> =
        if_statement()
        / while_loop()
        / assignment()
        / binary_op()

    rule if_statement() -> AstNode<'input> = if_else() / if_else_if()

    rule if_else() -> AstNode<'input> =
        "if" _ e:expression() _ "{" _ "\n"
            statements() _
        "}" _ "else" _ "{" _ "\n"
            statements() _
        "}"
        { arena.push1(e, |e| Expr::IfElse(e, 4, 3)) }

    rule if_else_if() -> AstNode<'input> =
        "if" _ e:expression() _ "{" _ "\n"
            statements() _
        "}" _ "else" _ else_body:if_statement()
        { arena.push2(e, else_body, |e, else_body| Expr::IfElseIf(e, 2, else_body)) }

    rule while_loop() -> AstNode<'input> =
        "while" _ e:expression() _ "{" _ "\n" statements() _ "}"
        { arena.push1(e, |e| Expr::WhileLoop(e, 1)) }

    rule assignment() -> AstNode<'input>
        = i:identifier() _ "=" _ e:expression() { arena.push1(e, |e| Expr::Assign(i, e)) }

    rule binary_op() -> AstNode<'input> = precedence!{
        a:@ _ "==" _ b:(@) { arena.push2(a, b, |a, b|Expr::Eq(a, b)) }
        a:@ _ "!=" _ b:(@) { arena.push2(a, b, |a, b|Expr::Ne(a, b)) }
        a:@ _ "<"  _ b:(@) { arena.push2(a, b, |a, b|Expr::Lt(a, b)) }
        a:@ _ "<=" _ b:(@) { arena.push2(a, b, |a, b|Expr::Le(a, b)) }
        a:@ _ ">"  _ b:(@) { arena.push2(a, b, |a, b|Expr::Gt(a, b)) }
        a:@ _ ">=" _ b:(@) { arena.push2(a, b, |a, b|Expr::Ge(a, b)) }
        --
        a:@ _ "+" _ b:(@) { arena.push2(a, b, |a, b|Expr::Add(a, b)) }
        a:@ _ "-" _ b:(@) { arena.push2(a, b, |a, b|Expr::Sub(a, b)) }
        --
        a:@ _ "*" _ b:(@) { arena.push2(a, b, |a, b|Expr::Mul(a, b)) }
        a:@ _ "/" _ b:(@) { arena.push2(a, b, |a, b|Expr::Div(a, b)) }
        --
        a:@ _ "mod" _ "(" _ b:expression() _ ")" { arena.push2(a, b, |a, b|Expr::Mod(a, b)) }
        a:@ _ "mod" _ b:(@) { arena.push2(a, b, |a, b|Expr::Mod(a, b)) }
        --
        i:identifier() _ "(" ((_ expression() _ ) ** ",") ")" { arena.push(Expr::Call(i, 7)) }
        i:identifier() { arena.push(Expr::Identifier(i)) }
        l:literal() { l }
    }

    rule identifier() -> &'input str
        = quiet!{ n:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) { n } }
        / expected!("identifier")

    rule literal() -> AstNode<'input>
        = n:$(['0'..='9']+) { arena.push(Expr::Literal(n)) }
        / "&" i:identifier() { arena.push(Expr::GlobalDataAddr(i)) }

    rule _() =  quiet!{[' ']*}
});
