use super::*;

peg::parser!(pub grammar parser(arena: &'input Ast<'input>) for str {
    pub rule function() -> ExprPtr
        = [' ' | '\n']* "function" _ name:identifier_text() _
        "(" params_names:parameters_definition() ")" _
        "->" _ return_name:identifier_definition() _
        "{" _ "\n" body:statements() _ "}" _ "\n" _
        {
            arena.push(Expr::Function(
                name,
                params_names,
                return_name,
                body,
            ))
        }

    rule statements() -> ExprPtr = statements_list() / null_expr()
    rule statements_list() -> ExprPtr = statement:statement() next_statement:statements()
        { arena.push(Expr::Statements(statement, next_statement)) }
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
        { arena.push(Expr::IfElse(condition, body, if_else)) }

    rule while_loop() -> ExprPtr =
        "while" _ e:expression() _ "{" _ "\n" body:statements() _ "}"
        { arena.push(Expr::WhileLoop(e, body)) }

    rule assignment() -> ExprPtr
        = i:identifier_text() _ "=" _ e:expression() { arena.push(Expr::Assign(i, e)) }

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
        i:identifier_text() _ "(" args:parameters() ")" { arena.push(Expr::Call(i, args)) }
        i:identifier() { i }
        l:literal() { l }
    }

    rule parameters() -> ExprPtr =  parameters_1()  / null_expr()
    rule parameters_1() -> ExprPtr = _ arg:expression() _ next:parameters_1_loop()
        { arena.push(Expr::Parameters(arg, next)) }
    rule parameters_1_loop() -> ExprPtr = parameters_1_list()  / null_expr()
    rule parameters_1_list() -> ExprPtr = _ "," _ arg:expression() _ next:parameters_1_loop()
        { arena.push(Expr::Parameters(arg, next)) }


    rule parameters_definition() -> ExprPtr =  parameters_definition_1()  / null_expr()
    rule parameters_definition_1() -> ExprPtr = _ arg:identifier_definition() _ next:parameters_definition_1_loop()
        { arena.push(Expr::ParametersDefinition(arg, next)) }
    rule parameters_definition_1_loop() -> ExprPtr = parameters_definition_1_list()  / null_expr()
    rule parameters_definition_1_list() -> ExprPtr = _ "," _ arg:identifier_definition() _ next:parameters_definition_1_loop()
        { arena.push(Expr::ParametersDefinition(arg, next)) }

    rule identifier() -> ExprPtr = i:identifier_text() { arena.push(Expr::Identifier(i)) }
    rule identifier_definition() -> ExprPtr = i:identifier_text() { arena.push(Expr::IdentifierDefinition(i)) }
    rule identifier_text() -> &'input str
        = quiet!{ n:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) { n } }
        / expected!("identifier")

    rule literal() -> ExprPtr
        = n:$(['0'..='9']+) { arena.push(Expr::Literal(n)) }
        / "&" i:identifier_text() { arena.push(Expr::GlobalDataAddr(i)) }

    rule _() =  quiet!{[' ']*}
    rule null_expr() -> ExprPtr = _ { NULL_EXPR_PTR }
});
