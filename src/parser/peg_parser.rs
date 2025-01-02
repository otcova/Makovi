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

    rule statements() = statement() statements() / _()
    rule statement() = _ e:expression() _ "\n"

    rule expression() -> ExprPtr =
        if_statement()
        / while_loop()
        / assignment()
        / binary_op()

    rule if_statement() -> ExprPtr = if_else() / if_else_if()

    rule if_else() -> ExprPtr =
        "if" _ e:expression() _ "{" _ "\n"
            statements() _
        "}" _ "else" _ "{" _ "\n"
            statements() _
        "}"
        { arena.push(Expr::IfElse(e, 4, 3)) }

    rule if_else_if() -> ExprPtr =
        "if" _ e:expression() _ "{" _ "\n"
            statements() _
        "}" _ "else" _ else_body:if_statement()
        { arena.push(Expr::IfElseIf(e, 2, else_body)) }

    rule while_loop() -> ExprPtr =
        "while" _ e:expression() _ "{" _ "\n" statements() _ "}"
        { arena.push(Expr::WhileLoop(e, 1)) }

    rule assignment() -> ExprPtr
        = i:identifier() _ "=" _ e:expression() {arena.push(Expr::Assign(i, e))}

    rule binary_op() -> ExprPtr = precedence!{
        a:@ _ "==" _ b:(@) { arena.push(Expr::Eq(a, b)) }
        a:@ _ "!=" _ b:(@) { arena.push(Expr::Ne(a, b)) }
        a:@ _ "<"  _ b:(@) { arena.push(Expr::Lt(a, b)) }
        a:@ _ "<=" _ b:(@) { arena.push(Expr::Le(a, b)) }
        a:@ _ ">"  _ b:(@) { arena.push(Expr::Gt(a, b)) }
        a:@ _ ">=" _ b:(@) { arena.push(Expr::Ge(a, b)) }
        --
        a:@ _ "+" _ b:(@) { arena.push(Expr::Add(a, b)) }
        a:@ _ "-" _ b:(@) { arena.push(Expr::Sub(a, b)) }
        --
        a:@ _ "*" _ b:(@) { arena.push(Expr::Mul(a, b)) }
        a:@ _ "/" _ b:(@) { arena.push(Expr::Div(a, b)) }
        --
        a:@ _ "mod" _ "(" _ b:expression() _ ")" { arena.push(Expr::Mod(a, b)) }
        a:@ _ "mod" _ b:(@) { arena.push(Expr::Mod(a, b)) }
        --
        i:identifier() _ "(" ((_ expression() _ ) ** ",") ")" { arena.push(Expr::Call(i, 7)) }
        i:identifier() { arena.push(Expr::Identifier(i)) }
        l:literal() { l }
    }

    rule identifier() -> &'input str
        = quiet!{ n:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) { n } }
        / expected!("identifier")

    rule literal() -> ExprPtr
        = n:$(['0'..='9']+) { arena.push(Expr::Literal(n)) }
        / "&" i:identifier() { arena.push(Expr::GlobalDataAddr(i)) }

    rule _() =  quiet!{[' ']*}
});
