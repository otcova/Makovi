macro_rules! match_next {
    (
        match $lexer:ident.$fn:ident() {
            $($pat:pat => $then:expr$(,)?)*
        }
    ) => {
        match $lexer.$fn() {
            $($pat => $then,)*
                #[allow(unreachable_patterns)]
                TokenResult { token: Some(Ok(found)), span, .. } => {
                    return Err(CompilationError {
                        message: format!( "Unexpected token '{found:?}'"),
                        span,
                    })
                }
                #[allow(unreachable_patterns)]
                TokenResult { token: Some(Err(())), span, slice } => {
                    return Err(CompilationError {
                        message: format!("Unknown token '{}'", slice),
                        span,
                    })
                }
                #[allow(unreachable_patterns)]
                TokenResult { token: None, span, .. } => {
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
            $(
                $token:ident
                $(($slice:ident))?
                => $then:expr$(,)?
            )*
        }
    ) => {
        match_next!(match $lexer.$fn() {
            $(TokenResult { token: Some(Ok($token)), $(slice: $slice,)? .. }=> $then,)*
        })
    };
}
pub(crate) use match_next;
pub(crate) use match_token;
