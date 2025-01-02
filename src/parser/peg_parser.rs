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

    rule statements() -> ExprPtr = statements_list() / empty_code_block()
    rule statements_list() -> ExprPtr = statement:statement() next_statement:statements()
        { arena.push(Expr::Statements(statement, next_statement)) }
    rule empty_code_block() -> ExprPtr = _ { NULL_EXPR_PTR }
    rule statement() -> ExprPtr = _ e:expression() _ "\n" { e }

    rule expression() -> ExprPtr =
        if_statement()
        / while_loop()
        / assignment()
        / binary_op()

    rule if_statement() -> ExprPtr = if_else() / if_else_if()

    rule if_else() -> ExprPtr =
        "if" _ condition:expression() _ "{" _ "\n"
            if_body:statements() _
        "}" _ "else" _ "{" _ "\n"
            else_body:statements() _
        "}"
        { arena.push(Expr::IfElse(condition, if_body, else_body)) }

    rule if_else_if() -> ExprPtr =
        "if" _ condition:expression() _ "{" _ "\n"
            body:statements() _
        "}" _ "else" _ if_else:if_statement()
        { arena.push(Expr::IfElseIf(condition, body, if_else)) }

    rule while_loop() -> ExprPtr =
        "while" _ e:expression() _ "{" _ "\n" body:statements() _ "}"
        { arena.push(Expr::WhileLoop(e, body)) }

    rule assignment() -> ExprPtr
        = i:identifier() _ "=" _ e:expression() { arena.push(Expr::Assign(i, e)) }

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
        i:identifier() _ "(" ((_ expression() _ ) ** ",") ")" { arena.push(Expr::Call(i, NULL_EXPR_PTR)) }
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
