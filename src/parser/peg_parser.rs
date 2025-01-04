use super::*;
use std::cell::RefCell;

peg::parser!(pub grammar parser(arena: &RefCell<&mut Ast<'input>>) for str {
    pub rule function() -> ExprPtr
        = [' ' | '\n']* "function" _ name:identifier_text() _
        "(" parameters:parameters_definition() ")" _
        "->" _ return_expr:identifier_definition() _
        "{" _ "\n" body:statements() _ "}" _ "\n" _
        { arena.borrow_mut().push(Expr::Function { name, parameters, return_expr, body }) }

    rule statements() -> ExprPtr = statements_list() / null_expr()
    rule statements_list() -> ExprPtr = statement:statement() next_statement:statements()
        { arena.borrow_mut().push(Expr::Statements(statement, next_statement)) }
    rule statement() -> ExprPtr = _ e:expression() _ "\n" { e }

    rule expression() -> ExprPtr =
        if_statement()
        / return_expr()
        / while_loop()
        / assignment()
        / binary_op()

    rule return_expr() -> ExprPtr = "return" _ expr:expression()
        { arena.borrow_mut().push(Expr::Return(expr)) }

    rule if_statement() -> ExprPtr = if_else() / if_else_if()

    rule if_else() -> ExprPtr =
        "if" _ condition:expression() _ "{" _ "\n"
            then_body:statements() _
        "}" _ "else" _ "{" _ "\n"
            else_body:statements() _
        "}"
        { arena.borrow_mut().push(Expr::IfElse { condition, then_body, else_body}) }

    rule if_else_if() -> ExprPtr =
        "if" _ condition:expression() _ "{" _ "\n"
            then_body:statements() _
        "}" _ "else" _ else_body:if_statement()
        { arena.borrow_mut().push(Expr::IfElse { condition, then_body, else_body }) }

    rule while_loop() -> ExprPtr =
        "while" _ condition:expression() _ "{" _ "\n" body:statements() _ "}"
        { arena.borrow_mut().push(Expr::WhileLoop { condition, body }) }

    rule assignment() -> ExprPtr
        = i:identifier_text() _ "=" _ e:expression() { arena.borrow_mut().push(Expr::Assign(i, e)) }

    rule binary_op() -> ExprPtr = precedence!{
        a:@ _ "==" _ b:(@) { arena.borrow_mut().push(Expr::Eq(a, b)) }
        a:@ _ "!=" _ b:(@) { arena.borrow_mut().push(Expr::Ne(a, b)) }
        a:@ _ "<"  _ b:(@) { arena.borrow_mut().push(Expr::Lt(a, b)) }
        a:@ _ "<=" _ b:(@) { arena.borrow_mut().push(Expr::Le(a, b)) }
        a:@ _ ">"  _ b:(@) { arena.borrow_mut().push(Expr::Gt(a, b)) }
        a:@ _ ">=" _ b:(@) { arena.borrow_mut().push(Expr::Ge(a, b)) }
        --
        a:@ _ "+" _ b:(@) { arena.borrow_mut().push(Expr::Add(a, b)) }
        a:@ _ "-" _ b:(@) { arena.borrow_mut().push(Expr::Sub(a, b)) }
        --
        a:@ _ "*" _ b:(@) { arena.borrow_mut().push(Expr::Mul(a, b)) }
        a:@ _ "/" _ b:(@) { arena.borrow_mut().push(Expr::Div(a, b)) }
        --
        a:@ _ "mod" _ "(" _ b:expression() _ ")" { arena.borrow_mut().push(Expr::Mod(a, b)) }
        a:@ _ "mod" _ b:(@) { arena.borrow_mut().push(Expr::Mod(a, b)) }
        --
        i:identifier_text() _ "(" args:parameters() ")" { arena.borrow_mut().push(Expr::Call(i, args)) }
        i:identifier() { i }
        l:literal() { l }
    }

    rule parameters() -> ExprPtr =  parameters_1()  / null_expr()
    rule parameters_1() -> ExprPtr = _ arg:expression() _ next:parameters_1_loop()
        { arena.borrow_mut().push(Expr::Parameters(arg, next)) }
    rule parameters_1_loop() -> ExprPtr = parameters_1_list()  / null_expr()
    rule parameters_1_list() -> ExprPtr = _ "," _ arg:expression() _ next:parameters_1_loop()
        { arena.borrow_mut().push(Expr::Parameters(arg, next)) }


    rule parameters_definition() -> ExprPtr =  parameters_definition_1()  / null_expr()
    rule parameters_definition_1() -> ExprPtr = _ arg:identifier_definition() _ next:parameters_definition_1_loop()
        { arena.borrow_mut().push(Expr::ParametersDefinition(arg, next)) }
    rule parameters_definition_1_loop() -> ExprPtr = parameters_definition_1_list()  / null_expr()
    rule parameters_definition_1_list() -> ExprPtr = _ "," _ arg:identifier_definition() _ next:parameters_definition_1_loop()
        { arena.borrow_mut().push(Expr::ParametersDefinition(arg, next)) }

    rule identifier() -> ExprPtr = i:identifier_text() { arena.borrow_mut().push(Expr::Identifier(i)) }
    rule identifier_definition() -> ExprPtr = i:identifier_text() { arena.borrow_mut().push(Expr::IdentifierDefinition(i)) }
    rule identifier_text() -> &'input str
        = quiet!{ n:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) { n } }
        / expected!("identifier")

    rule literal() -> ExprPtr
        = n:$(['0'..='9']+) { arena.borrow_mut().push(Expr::Literal(n)) }

    rule _() =  quiet!{[' ']*}
    rule null_expr() -> ExprPtr = _ { NULL_EXPR_PTR }
});
