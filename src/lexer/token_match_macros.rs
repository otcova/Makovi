macro_rules! match_token {
    (
        match $lexer:ident$(.$fn:ident())? {
            $($token:pat => $then:expr$(,)?)*
        }
    ) => {
        {
        let token = $lexer$(.$fn())?;
        match token.expect_token()? {
            $($token => $then,)*

            #[allow(unreachable_patterns)]
            found => {
                return Err(CompilationError {
                    message: format!( "Unexpected token '{found:?}'"),
                    span: token.span,
                })
            }
        }
        }
    };
}

pub(crate) use match_token;
