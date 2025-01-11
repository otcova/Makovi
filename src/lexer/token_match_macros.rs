macro_rules! match_next {
    (
        match $lexer:ident.$fn:ident() {
            $($pat:pat => $then:expr$(,)?)*
        }
    ) => {
        match $lexer.$fn() {
            $($pat => $then,)*
                #[allow(unreachable_patterns)]
                (Some(Ok(found)), span) => {
                    return Err(CompilationError {
                        message: format!( "Unexpected token '{found:?}'"),
                        span,
                    })
                }
                #[allow(unreachable_patterns)]
                (Some(Err(token)), span) => {
                    return Err(CompilationError {
                        message: format!("Unknown token '{}'", token),
                        span,
                    })
                }
                #[allow(unreachable_patterns)]
                (None, span) => {
                    return Err(CompilationError {
                        message: "Unexpected end of file".to_owned(),
                        span,
                    })
                }
        }
    };
}

macro_rules! match_token {
    (
        match $lexer:ident.$fn:ident() {
            $($token:pat => $then:expr$(,)?)*
        }
    ) => {
        {
            #[allow(unused_imports)]
            use Token::*;
            match_next!(match $lexer.$fn() {
                $((Some(Ok($token)), _) => $then,)*
            })
        }
    };
}

macro_rules! expect_token {
    ($pat:pat, $token:expr) => {
        expect_token!($pat, _span, $token);
    };
    ($pat:pat, $span:ident, $token:expr) => {
        let token = $token;
        let (Some(Ok($pat)), $span) = token else {
            match token {
                (Some(Ok(found)), span) => {
                    return Err(CompilationError {
                        message: format!(
                            "Expected token '{}' but found '{found:?}'",
                            stringify!($pat)
                        ),
                        span,
                    })
                }
                (Some(Err(token)), span) => {
                    return Err(CompilationError {
                        message: format!("Unknown token {}", token),
                        span,
                    })
                }
                (None, span) => {
                    return Err(CompilationError {
                        message: format!(
                            "Expected token '{}' but reached end of file",
                            stringify!($pat)
                        ),
                        span,
                    })
                }
            }
        };
    };
}

pub(crate) use expect_token;
pub(crate) use match_next;
pub(crate) use match_token;
