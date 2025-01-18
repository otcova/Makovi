macro_rules! match_token {
    ( match $token:ident { $($pat:pat => $then:expr$(,)?)* }) => {
        {
        use Token::*;
        let $token = $token;
        match $token.expect_token()? {
            $($pat => $then,)*

            #[allow(unreachable_patterns)]
            found => {
                return Err(CompilationError {
                    message: format!( "Unexpected token '{found:?}'"),
                    span: $token.span,
                })
            }
        }
        }
    };

    ( match $self:ident.$token:ident.$fn:ident() { $($pat:pat => $then:expr$(,)?)* }) => {
        {
        let token = $self.$token.$fn();
        match_token! { match token { $($pat => $then,)* }}
        }
    };
}

pub(crate) use match_token;
