use super::*;
pub mod parser {
    #[allow(unused_imports)]
    use super::*;
    type Input = str;
    type PositionRepr = <Input as ::peg::Parse>::PositionRepr;
    #[allow(unused_parens)]
    struct ParseState<'input> {
        _phantom: ::core::marker::PhantomData<(&'input ())>,
    }
    impl<'input> ParseState<'input> {
        fn new() -> ParseState<'input> {
            ParseState {
                _phantom: ::core::marker::PhantomData,
            }
        }
    }
    pub fn function<'input>(
        __input: &'input Input,
    ) -> ::core::result::Result<FunctionAST, ::peg::error::ParseError<PositionRepr>> {
        #![allow(non_snake_case, unused)]
        let mut __err_state = ::peg::error::ErrorState::new(::peg::Parse::start(__input));
        let mut __state = ParseState::new();
        match __parse_function(
            __input,
            &mut __state,
            &mut __err_state,
            ::peg::Parse::start(__input),
        ) {
            ::peg::RuleResult::Matched(__pos, __value) => {
                if ::peg::Parse::is_eof(__input, __pos) {
                    return Ok(__value);
                } else {
                    __err_state.mark_failure(__pos, "EOF");
                }
            }
            _ => (),
        }
        __state = ParseState::new();
        __err_state.reparse_for_error();
        match __parse_function(
            __input,
            &mut __state,
            &mut __err_state,
            ::peg::Parse::start(__input),
        ) {
            ::peg::RuleResult::Matched(__pos, __value) => {
                if ::peg::Parse::is_eof(__input, __pos) {
                    {
                        ::std::rt::begin_panic("Parser is nondeterministic: succeeded when reparsing for error position");
                    };
                    return Ok(__value);
                } else {
                    __err_state.mark_failure(__pos, "EOF");
                }
            }
            _ => (),
        }
        Err(__err_state.into_parse_error(__input))
    }
    fn __parse_function<'input>(
        __input: &'input Input,
        __state: &mut ParseState<'input>,
        __err_state: &mut ::peg::error::ErrorState,
        __pos: usize,
    ) -> ::peg::RuleResult<FunctionAST> {
        #![allow(non_snake_case, unused, clippy::redundant_closure_call)]
        {
            let __seq_res = {
                let mut __repeat_pos = __pos;
                loop {
                    let __pos = __repeat_pos;
                    let __step_res = match ::peg::ParseElem::parse_elem(__input, __pos) {
                        ::peg::RuleResult::Matched(__next, __ch) => match __ch {
                            ' ' | '\n' => ::peg::RuleResult::Matched(__next, ()),
                            _ => {
                                __err_state.mark_failure(__pos, "[' ' | '\\n']");
                                ::peg::RuleResult::Failed
                            }
                        },
                        ::peg::RuleResult::Failed => {
                            __err_state.mark_failure(__pos, "[' ' | '\\n']");
                            ::peg::RuleResult::Failed
                        }
                    };
                    match __step_res {
                        ::peg::RuleResult::Matched(__newpos, __value) => {
                            __repeat_pos = __newpos;
                        }
                        ::peg::RuleResult::Failed => {
                            break;
                        }
                    }
                }
                ::peg::RuleResult::Matched(__repeat_pos, ())
            };
            match __seq_res {
                ::peg::RuleResult::Matched(__pos, _) => {
                    match ::peg::ParseLiteral::parse_string_literal(__input, __pos, "function") {
                        ::peg::RuleResult::Matched(__pos, __val) => {
                            let __seq_res = match __parse__(__input, __state, __err_state, __pos) {
                                ::peg::RuleResult::Matched(pos, _) => {
                                    ::peg::RuleResult::Matched(pos, ())
                                }
                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                            };
                            match __seq_res {
                                ::peg::RuleResult::Matched(__pos, _) => {
                                    let __seq_res =
                                        __parse_identifier(__input, __state, __err_state, __pos);
                                    match __seq_res {
                                        ::peg::RuleResult::Matched(__pos, name) => {
                                            let __seq_res = match __parse__(
                                                __input,
                                                __state,
                                                __err_state,
                                                __pos,
                                            ) {
                                                ::peg::RuleResult::Matched(pos, _) => {
                                                    ::peg::RuleResult::Matched(pos, ())
                                                }
                                                ::peg::RuleResult::Failed => {
                                                    ::peg::RuleResult::Failed
                                                }
                                            };
                                            match __seq_res {
                                                ::peg::RuleResult::Matched(__pos, _) => {
                                                    match ::peg::ParseLiteral::parse_string_literal(
                                                        __input, __pos, "(",
                                                    ) {
                                                        ::peg::RuleResult::Matched(
                                                            __pos,
                                                            __val,
                                                        ) => {
                                                            let __seq_res = {
                                                                let mut __repeat_pos = __pos;
                                                                let mut __repeat_value = vec![];
                                                                loop {
                                                                    let __pos = __repeat_pos;
                                                                    let __pos = if __repeat_value
                                                                        .is_empty()
                                                                    {
                                                                        __pos
                                                                    } else {
                                                                        let __sep_res =
                                                                                                               match ::peg::ParseLiteral::parse_string_literal(__input,
                                                                                                                       __pos, ",") {
                                                                                                                   ::peg::RuleResult::Matched(__pos, __val) => {
                                                                                                                       ::peg::RuleResult::Matched(__pos, __val)
                                                                                                                   }
                                                                                                                   ::peg::RuleResult::Failed => {
                                                                                                                       __err_state.mark_failure(__pos, "\",\"");
                                                                                                                       ::peg::RuleResult::Failed
                                                                                                                   }
                                                                                                               };
                                                                        match __sep_res {
                                                                                                               ::peg::RuleResult::Matched(__newpos, _) => { __newpos }
                                                                                                               ::peg::RuleResult::Failed => break,
                                                                                                           }
                                                                    };
                                                                    let __step_res = {
                                                                        let __seq_res =
                                                                                                            match __parse__(__input, __state, __err_state, __pos) {
                                                                                                                ::peg::RuleResult::Matched(pos, _) =>
                                                                                                                    ::peg::RuleResult::Matched(pos, ()),
                                                                                                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                            };
                                                                        match __seq_res {
                                                                                                            ::peg::RuleResult::Matched(__pos, _) => {
                                                                                                                {
                                                                                                                    let __seq_res =
                                                                                                                        __parse_identifier(__input, __state, __err_state, __pos);
                                                                                                                    match __seq_res {
                                                                                                                        ::peg::RuleResult::Matched(__pos, i) => {
                                                                                                                            {
                                                                                                                                let __seq_res =
                                                                                                                                    match __parse__(__input, __state, __err_state, __pos) {
                                                                                                                                        ::peg::RuleResult::Matched(pos, _) =>
                                                                                                                                            ::peg::RuleResult::Matched(pos, ()),
                                                                                                                                        ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                    };
                                                                                                                                match __seq_res {
                                                                                                                                    ::peg::RuleResult::Matched(__pos, _) => {
                                                                                                                                        ::peg::RuleResult::Matched(__pos, (|| { i })())
                                                                                                                                    }
                                                                                                                                    ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                        ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                            ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                        }
                                                                    };
                                                                    match __step_res {
                                                                                                    ::peg::RuleResult::Matched(__newpos, __value) => {
                                                                                                        __repeat_pos = __newpos;
                                                                                                        __repeat_value.push(__value);
                                                                                                    }
                                                                                                    ::peg::RuleResult::Failed => { break; }
                                                                                                }
                                                                }
                                                                ::peg::RuleResult::Matched(
                                                                    __repeat_pos,
                                                                    __repeat_value,
                                                                )
                                                            };
                                                            match __seq_res {
                                                                                        ::peg::RuleResult::Matched(__pos, params_names) => {
                                                                                            match ::peg::ParseLiteral::parse_string_literal(__input,
                                                                                                    __pos, ")") {
                                                                                                ::peg::RuleResult::Matched(__pos, __val) => {
                                                                                                    {
                                                                                                        let __seq_res =
                                                                                                            match __parse__(__input, __state, __err_state, __pos) {
                                                                                                                ::peg::RuleResult::Matched(pos, _) =>
                                                                                                                    ::peg::RuleResult::Matched(pos, ()),
                                                                                                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                            };
                                                                                                        match __seq_res {
                                                                                                            ::peg::RuleResult::Matched(__pos, _) => {
                                                                                                                match ::peg::ParseLiteral::parse_string_literal(__input,
                                                                                                                        __pos, "->") {
                                                                                                                    ::peg::RuleResult::Matched(__pos, __val) => {
                                                                                                                        {
                                                                                                                            let __seq_res =
                                                                                                                                match __parse__(__input, __state, __err_state, __pos) {
                                                                                                                                    ::peg::RuleResult::Matched(pos, _) =>
                                                                                                                                        ::peg::RuleResult::Matched(pos, ()),
                                                                                                                                    ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                };
                                                                                                                            match __seq_res {
                                                                                                                                ::peg::RuleResult::Matched(__pos, _) => {
                                                                                                                                    match ::peg::ParseLiteral::parse_string_literal(__input,
                                                                                                                                            __pos, "(") {
                                                                                                                                        ::peg::RuleResult::Matched(__pos, __val) => {
                                                                                                                                            {
                                                                                                                                                let __seq_res =
                                                                                                                                                    {
                                                                                                                                                        let __seq_res =
                                                                                                                                                            match __parse__(__input, __state, __err_state, __pos) {
                                                                                                                                                                ::peg::RuleResult::Matched(pos, _) =>
                                                                                                                                                                    ::peg::RuleResult::Matched(pos, ()),
                                                                                                                                                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                                            };
                                                                                                                                                        match __seq_res {
                                                                                                                                                            ::peg::RuleResult::Matched(__pos, _) => {
                                                                                                                                                                {
                                                                                                                                                                    let __seq_res =
                                                                                                                                                                        __parse_identifier(__input, __state, __err_state, __pos);
                                                                                                                                                                    match __seq_res {
                                                                                                                                                                        ::peg::RuleResult::Matched(__pos, i) => {
                                                                                                                                                                            {
                                                                                                                                                                                let __seq_res =
                                                                                                                                                                                    match __parse__(__input, __state, __err_state, __pos) {
                                                                                                                                                                                        ::peg::RuleResult::Matched(pos, _) =>
                                                                                                                                                                                            ::peg::RuleResult::Matched(pos, ()),
                                                                                                                                                                                        ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                                                                    };
                                                                                                                                                                                match __seq_res {
                                                                                                                                                                                    ::peg::RuleResult::Matched(__pos, _) => {
                                                                                                                                                                                        ::peg::RuleResult::Matched(__pos, (|| { i })())
                                                                                                                                                                                    }
                                                                                                                                                                                    ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                        ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                            ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                                        }
                                                                                                                                                    };
                                                                                                                                                match __seq_res {
                                                                                                                                                    ::peg::RuleResult::Matched(__pos, return_name) => {
                                                                                                                                                        match ::peg::ParseLiteral::parse_string_literal(__input,
                                                                                                                                                                __pos, ")") {
                                                                                                                                                            ::peg::RuleResult::Matched(__pos, __val) => {
                                                                                                                                                                {
                                                                                                                                                                    let __seq_res =
                                                                                                                                                                        match __parse__(__input, __state, __err_state, __pos) {
                                                                                                                                                                            ::peg::RuleResult::Matched(pos, _) =>
                                                                                                                                                                                ::peg::RuleResult::Matched(pos, ()),
                                                                                                                                                                            ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                                                        };
                                                                                                                                                                    match __seq_res {
                                                                                                                                                                        ::peg::RuleResult::Matched(__pos, _) => {
                                                                                                                                                                            match ::peg::ParseLiteral::parse_string_literal(__input,
                                                                                                                                                                                    __pos, "{") {
                                                                                                                                                                                ::peg::RuleResult::Matched(__pos, __val) => {
                                                                                                                                                                                    {
                                                                                                                                                                                        let __seq_res =
                                                                                                                                                                                            match __parse__(__input, __state, __err_state, __pos) {
                                                                                                                                                                                                ::peg::RuleResult::Matched(pos, _) =>
                                                                                                                                                                                                    ::peg::RuleResult::Matched(pos, ()),
                                                                                                                                                                                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                                                                            };
                                                                                                                                                                                        match __seq_res {
                                                                                                                                                                                            ::peg::RuleResult::Matched(__pos, _) => {
                                                                                                                                                                                                match ::peg::ParseLiteral::parse_string_literal(__input,
                                                                                                                                                                                                        __pos, "\n") {
                                                                                                                                                                                                    ::peg::RuleResult::Matched(__pos, __val) => {
                                                                                                                                                                                                        {
                                                                                                                                                                                                            let __seq_res =
                                                                                                                                                                                                                __parse_statements(__input, __state, __err_state, __pos);
                                                                                                                                                                                                            match __seq_res {
                                                                                                                                                                                                                ::peg::RuleResult::Matched(__pos, statements) => {
                                                                                                                                                                                                                    {
                                                                                                                                                                                                                        let __seq_res =
                                                                                                                                                                                                                            match __parse__(__input, __state, __err_state, __pos) {
                                                                                                                                                                                                                                ::peg::RuleResult::Matched(pos, _) =>
                                                                                                                                                                                                                                    ::peg::RuleResult::Matched(pos, ()),
                                                                                                                                                                                                                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                                                                                                            };
                                                                                                                                                                                                                        match __seq_res {
                                                                                                                                                                                                                            ::peg::RuleResult::Matched(__pos, _) => {
                                                                                                                                                                                                                                match ::peg::ParseLiteral::parse_string_literal(__input,
                                                                                                                                                                                                                                        __pos, "}") {
                                                                                                                                                                                                                                    ::peg::RuleResult::Matched(__pos, __val) => {
                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                            let __seq_res =
                                                                                                                                                                                                                                                match __parse__(__input, __state, __err_state, __pos) {
                                                                                                                                                                                                                                                    ::peg::RuleResult::Matched(pos, _) =>
                                                                                                                                                                                                                                                        ::peg::RuleResult::Matched(pos, ()),
                                                                                                                                                                                                                                                    ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                                                                                                                                };
                                                                                                                                                                                                                                            match __seq_res {
                                                                                                                                                                                                                                                ::peg::RuleResult::Matched(__pos, _) => {
                                                                                                                                                                                                                                                    match ::peg::ParseLiteral::parse_string_literal(__input,
                                                                                                                                                                                                                                                            __pos, "\n") {
                                                                                                                                                                                                                                                        ::peg::RuleResult::Matched(__pos, __val) => {
                                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                                let __seq_res =
                                                                                                                                                                                                                                                                    match __parse__(__input, __state, __err_state, __pos) {
                                                                                                                                                                                                                                                                        ::peg::RuleResult::Matched(pos, _) =>
                                                                                                                                                                                                                                                                            ::peg::RuleResult::Matched(pos, ()),
                                                                                                                                                                                                                                                                        ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                                                                                                                                                    };
                                                                                                                                                                                                                                                                match __seq_res {
                                                                                                                                                                                                                                                                    ::peg::RuleResult::Matched(__pos, _) => {
                                                                                                                                                                                                                                                                        ::peg::RuleResult::Matched(__pos,
                                                                                                                                                                                                                                                                            (||
                                                                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                                                                            FunctionAST { name, params_names, return_name, statements }
                                                                                                                                                                                                                                                                                        })())
                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                    ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                        ::peg::RuleResult::Failed => {
                                                                                                                                                                                                                                                            __err_state.mark_failure(__pos, "\"\\n\"");
                                                                                                                                                                                                                                                            ::peg::RuleResult::Failed
                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                    ::peg::RuleResult::Failed => {
                                                                                                                                                                                                                                        __err_state.mark_failure(__pos, "\"}\"");
                                                                                                                                                                                                                                        ::peg::RuleResult::Failed
                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                }
                                                                                                                                                                                                                            }
                                                                                                                                                                                                                            ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                                                                                                        }
                                                                                                                                                                                                                    }
                                                                                                                                                                                                                }
                                                                                                                                                                                                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                                                                                            }
                                                                                                                                                                                                        }
                                                                                                                                                                                                    }
                                                                                                                                                                                                    ::peg::RuleResult::Failed => {
                                                                                                                                                                                                        __err_state.mark_failure(__pos, "\"\\n\"");
                                                                                                                                                                                                        ::peg::RuleResult::Failed
                                                                                                                                                                                                    }
                                                                                                                                                                                                }
                                                                                                                                                                                            }
                                                                                                                                                                                            ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                                                                        }
                                                                                                                                                                                    }
                                                                                                                                                                                }
                                                                                                                                                                                ::peg::RuleResult::Failed => {
                                                                                                                                                                                    __err_state.mark_failure(__pos, "\"{\"");
                                                                                                                                                                                    ::peg::RuleResult::Failed
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                        ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                            ::peg::RuleResult::Failed => {
                                                                                                                                                                __err_state.mark_failure(__pos, "\")\"");
                                                                                                                                                                ::peg::RuleResult::Failed
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                    ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                        ::peg::RuleResult::Failed => {
                                                                                                                                            __err_state.mark_failure(__pos, "\"(\"");
                                                                                                                                            ::peg::RuleResult::Failed
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                    ::peg::RuleResult::Failed => {
                                                                                                                        __err_state.mark_failure(__pos, "\"->\"");
                                                                                                                        ::peg::RuleResult::Failed
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                            ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                                ::peg::RuleResult::Failed => {
                                                                                                    __err_state.mark_failure(__pos, "\")\"");
                                                                                                    ::peg::RuleResult::Failed
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                    }
                                                        }
                                                        ::peg::RuleResult::Failed => {
                                                            __err_state
                                                                .mark_failure(__pos, "\"(\"");
                                                            ::peg::RuleResult::Failed
                                                        }
                                                    }
                                                }
                                                ::peg::RuleResult::Failed => {
                                                    ::peg::RuleResult::Failed
                                                }
                                            }
                                        }
                                        ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                    }
                                }
                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                            }
                        }
                        ::peg::RuleResult::Failed => {
                            __err_state.mark_failure(__pos, "\"function\"");
                            ::peg::RuleResult::Failed
                        }
                    }
                }
                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
            }
        }
    }
    fn __parse_statements<'input>(
        __input: &'input Input,
        __state: &mut ParseState<'input>,
        __err_state: &mut ::peg::error::ErrorState,
        __pos: usize,
    ) -> ::peg::RuleResult<Vec<ExprAst>> {
        #![allow(non_snake_case, unused, clippy::redundant_closure_call)]
        {
            let __seq_res = {
                let mut __repeat_pos = __pos;
                let mut __repeat_value = vec![];
                loop {
                    let __pos = __repeat_pos;
                    let __step_res = __parse_statement(__input, __state, __err_state, __pos);
                    match __step_res {
                        ::peg::RuleResult::Matched(__newpos, __value) => {
                            __repeat_pos = __newpos;
                            __repeat_value.push(__value);
                        }
                        ::peg::RuleResult::Failed => {
                            break;
                        }
                    }
                }
                ::peg::RuleResult::Matched(__repeat_pos, __repeat_value)
            };
            match __seq_res {
                ::peg::RuleResult::Matched(__pos, s) => ::peg::RuleResult::Matched(__pos, (|| s)()),
                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
            }
        }
    }
    fn __parse_statement<'input>(
        __input: &'input Input,
        __state: &mut ParseState<'input>,
        __err_state: &mut ::peg::error::ErrorState,
        __pos: usize,
    ) -> ::peg::RuleResult<ExprAst> {
        #![allow(non_snake_case, unused, clippy::redundant_closure_call)]
        {
            let __seq_res = match __parse__(__input, __state, __err_state, __pos) {
                ::peg::RuleResult::Matched(pos, _) => ::peg::RuleResult::Matched(pos, ()),
                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
            };
            match __seq_res {
                ::peg::RuleResult::Matched(__pos, _) => {
                    let __seq_res = __parse_expression(__input, __state, __err_state, __pos);
                    match __seq_res {
                        ::peg::RuleResult::Matched(__pos, e) => {
                            let __seq_res = match __parse__(__input, __state, __err_state, __pos) {
                                ::peg::RuleResult::Matched(pos, _) => {
                                    ::peg::RuleResult::Matched(pos, ())
                                }
                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                            };
                            match __seq_res {
                                ::peg::RuleResult::Matched(__pos, _) => {
                                    match ::peg::ParseLiteral::parse_string_literal(
                                        __input, __pos, "\n",
                                    ) {
                                        ::peg::RuleResult::Matched(__pos, __val) => {
                                            ::peg::RuleResult::Matched(__pos, (|| e)())
                                        }
                                        ::peg::RuleResult::Failed => {
                                            __err_state.mark_failure(__pos, "\"\\n\"");
                                            ::peg::RuleResult::Failed
                                        }
                                    }
                                }
                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                            }
                        }
                        ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                    }
                }
                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
            }
        }
    }
    fn __parse_expression<'input>(
        __input: &'input Input,
        __state: &mut ParseState<'input>,
        __err_state: &mut ::peg::error::ErrorState,
        __pos: usize,
    ) -> ::peg::RuleResult<ExprAst> {
        #![allow(non_snake_case, unused, clippy::redundant_closure_call)]
        {
            let __choice_res = __parse_if_else(__input, __state, __err_state, __pos);
            match __choice_res {
                ::peg::RuleResult::Matched(__pos, __value) => {
                    ::peg::RuleResult::Matched(__pos, __value)
                }
                ::peg::RuleResult::Failed => {
                    let __choice_res = __parse_while_loop(__input, __state, __err_state, __pos);
                    match __choice_res {
                        ::peg::RuleResult::Matched(__pos, __value) => {
                            ::peg::RuleResult::Matched(__pos, __value)
                        }
                        ::peg::RuleResult::Failed => {
                            let __choice_res =
                                __parse_assignment(__input, __state, __err_state, __pos);
                            match __choice_res {
                                ::peg::RuleResult::Matched(__pos, __value) => {
                                    ::peg::RuleResult::Matched(__pos, __value)
                                }
                                ::peg::RuleResult::Failed => {
                                    __parse_binary_op(__input, __state, __err_state, __pos)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    fn __parse_if_else<'input>(
        __input: &'input Input,
        __state: &mut ParseState<'input>,
        __err_state: &mut ::peg::error::ErrorState,
        __pos: usize,
    ) -> ::peg::RuleResult<ExprAst> {
        #![allow(non_snake_case, unused, clippy::redundant_closure_call)]
        {
            fn __infix_parse<T, S>(
                state: &mut S,
                err_state: &mut ::peg::error::ErrorState,
                min_prec: i32,
                lpos: usize,
                prefix_atom: &Fn(
                    usize,
                    &mut S,
                    &mut ::peg::error::ErrorState,
                    &Fn(usize, i32, &mut S, &mut ::peg::error::ErrorState) -> ::peg::RuleResult<T>,
                ) -> ::peg::RuleResult<T>,
                level_code: &Fn(
                    usize,
                    usize,
                    i32,
                    T,
                    &mut S,
                    &mut ::peg::error::ErrorState,
                    &Fn(usize, i32, &mut S, &mut ::peg::error::ErrorState) -> ::peg::RuleResult<T>,
                ) -> (T, ::peg::RuleResult<()>),
            ) -> ::peg::RuleResult<T> {
                let initial = {
                    prefix_atom(
                        lpos,
                        state,
                        err_state,
                        &(|pos, min_prec, state, err_state| {
                            __infix_parse(state, err_state, min_prec, pos, prefix_atom, level_code)
                        }),
                    )
                };
                if let ::peg::RuleResult::Matched(pos, mut infix_result) = initial {
                    let mut repeat_pos = pos;
                    loop {
                        let (val, res) = level_code(
                            repeat_pos,
                            lpos,
                            min_prec,
                            infix_result,
                            state,
                            err_state,
                            &(|pos, min_prec, state, err_state| {
                                __infix_parse(
                                    state,
                                    err_state,
                                    min_prec,
                                    pos,
                                    prefix_atom,
                                    level_code,
                                )
                            }),
                        );
                        infix_result = val;
                        if let ::peg::RuleResult::Matched(pos, ()) = res {
                            repeat_pos = pos;
                            continue;
                        }
                        break;
                    }
                    ::peg::RuleResult::Matched(repeat_pos, infix_result)
                } else {
                    ::peg::RuleResult::Failed
                }
            }
            __infix_parse(
                __state,
                __err_state,
                0,
                __pos,
                &(|__pos, __state, __err_state, __recurse| {
                    let __lpos = __pos;
                    if let ::peg::RuleResult::Matched(__pos, __v) =
                        match ::peg::ParseLiteral::parse_string_literal(__input, __pos, "if") {
                            ::peg::RuleResult::Matched(__pos, __val) => {
                                let __seq_res =
                                    match __parse__(__input, __state, __err_state, __pos) {
                                        ::peg::RuleResult::Matched(pos, _) => {
                                            ::peg::RuleResult::Matched(pos, ())
                                        }
                                        ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                    };
                                match __seq_res {
                                    ::peg::RuleResult::Matched(__pos, _) => {
                                        let __seq_res = __parse_expression(
                                            __input,
                                            __state,
                                            __err_state,
                                            __pos,
                                        );
                                        match __seq_res {
                                            ::peg::RuleResult::Matched(__pos, e) => {
                                                let __seq_res = match __parse__(
                                                    __input,
                                                    __state,
                                                    __err_state,
                                                    __pos,
                                                ) {
                                                    ::peg::RuleResult::Matched(pos, _) => {
                                                        ::peg::RuleResult::Matched(pos, ())
                                                    }
                                                    ::peg::RuleResult::Failed => {
                                                        ::peg::RuleResult::Failed
                                                    }
                                                };
                                                match __seq_res {
                                                                                        ::peg::RuleResult::Matched(__pos, _) => {
                                                                                            match ::peg::ParseLiteral::parse_string_literal(__input,
                                                                                                    __pos, "{") {
                                                                                                ::peg::RuleResult::Matched(__pos, __val) => {
                                                                                                    {
                                                                                                        let __seq_res =
                                                                                                            match __parse__(__input, __state, __err_state, __pos) {
                                                                                                                ::peg::RuleResult::Matched(pos, _) =>
                                                                                                                    ::peg::RuleResult::Matched(pos, ()),
                                                                                                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                            };
                                                                                                        match __seq_res {
                                                                                                            ::peg::RuleResult::Matched(__pos, _) => {
                                                                                                                match ::peg::ParseLiteral::parse_string_literal(__input,
                                                                                                                        __pos, "\n") {
                                                                                                                    ::peg::RuleResult::Matched(__pos, __val) => {
                                                                                                                        {
                                                                                                                            let __seq_res =
                                                                                                                                __parse_statements(__input, __state, __err_state, __pos);
                                                                                                                            match __seq_res {
                                                                                                                                ::peg::RuleResult::Matched(__pos, then_body) => {
                                                                                                                                    {
                                                                                                                                        let __seq_res =
                                                                                                                                            match __parse__(__input, __state, __err_state, __pos) {
                                                                                                                                                ::peg::RuleResult::Matched(pos, _) =>
                                                                                                                                                    ::peg::RuleResult::Matched(pos, ()),
                                                                                                                                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                            };
                                                                                                                                        match __seq_res {
                                                                                                                                            ::peg::RuleResult::Matched(__pos, _) => {
                                                                                                                                                match ::peg::ParseLiteral::parse_string_literal(__input,
                                                                                                                                                        __pos, "}") {
                                                                                                                                                    ::peg::RuleResult::Matched(__pos, __val) => {
                                                                                                                                                        {
                                                                                                                                                            let __seq_res =
                                                                                                                                                                match __parse__(__input, __state, __err_state, __pos) {
                                                                                                                                                                    ::peg::RuleResult::Matched(pos, _) =>
                                                                                                                                                                        ::peg::RuleResult::Matched(pos, ()),
                                                                                                                                                                    ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                                                };
                                                                                                                                                            match __seq_res {
                                                                                                                                                                ::peg::RuleResult::Matched(__pos, _) => {
                                                                                                                                                                    match ::peg::ParseLiteral::parse_string_literal(__input,
                                                                                                                                                                            __pos, "else") {
                                                                                                                                                                        ::peg::RuleResult::Matched(__pos, __val) => {
                                                                                                                                                                            {
                                                                                                                                                                                let __seq_res =
                                                                                                                                                                                    match __parse__(__input, __state, __err_state, __pos) {
                                                                                                                                                                                        ::peg::RuleResult::Matched(pos, _) =>
                                                                                                                                                                                            ::peg::RuleResult::Matched(pos, ()),
                                                                                                                                                                                        ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                                                                    };
                                                                                                                                                                                match __seq_res {
                                                                                                                                                                                    ::peg::RuleResult::Matched(__pos, _) => {
                                                                                                                                                                                        match ::peg::ParseLiteral::parse_string_literal(__input,
                                                                                                                                                                                                __pos, "{") {
                                                                                                                                                                                            ::peg::RuleResult::Matched(__pos, __val) => {
                                                                                                                                                                                                {
                                                                                                                                                                                                    let __seq_res =
                                                                                                                                                                                                        match __parse__(__input, __state, __err_state, __pos) {
                                                                                                                                                                                                            ::peg::RuleResult::Matched(pos, _) =>
                                                                                                                                                                                                                ::peg::RuleResult::Matched(pos, ()),
                                                                                                                                                                                                            ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                                                                                        };
                                                                                                                                                                                                    match __seq_res {
                                                                                                                                                                                                        ::peg::RuleResult::Matched(__pos, _) => {
                                                                                                                                                                                                            match ::peg::ParseLiteral::parse_string_literal(__input,
                                                                                                                                                                                                                    __pos, "\n") {
                                                                                                                                                                                                                ::peg::RuleResult::Matched(__pos, __val) => {
                                                                                                                                                                                                                    {
                                                                                                                                                                                                                        let __seq_res =
                                                                                                                                                                                                                            __parse_statements(__input, __state, __err_state, __pos);
                                                                                                                                                                                                                        match __seq_res {
                                                                                                                                                                                                                            ::peg::RuleResult::Matched(__pos, else_body) => {
                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                    let __seq_res =
                                                                                                                                                                                                                                        match __parse__(__input, __state, __err_state, __pos) {
                                                                                                                                                                                                                                            ::peg::RuleResult::Matched(pos, _) =>
                                                                                                                                                                                                                                                ::peg::RuleResult::Matched(pos, ()),
                                                                                                                                                                                                                                            ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                                                                                                                        };
                                                                                                                                                                                                                                    match __seq_res {
                                                                                                                                                                                                                                        ::peg::RuleResult::Matched(__pos, _) => {
                                                                                                                                                                                                                                            match ::peg::ParseLiteral::parse_string_literal(__input,
                                                                                                                                                                                                                                                    __pos, "}") {
                                                                                                                                                                                                                                                ::peg::RuleResult::Matched(__pos, __val) => {
                                                                                                                                                                                                                                                    ::peg::RuleResult::Matched(__pos,
                                                                                                                                                                                                                                                        (||
                                                                                                                                                                                                                                                                    { ExprAst::IfElse(Box::new(e), then_body, else_body) })())
                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                ::peg::RuleResult::Failed => {
                                                                                                                                                                                                                                                    __err_state.mark_failure(__pos, "\"}\"");
                                                                                                                                                                                                                                                    ::peg::RuleResult::Failed
                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                        ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                }
                                                                                                                                                                                                                            }
                                                                                                                                                                                                                            ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                                                                                                        }
                                                                                                                                                                                                                    }
                                                                                                                                                                                                                }
                                                                                                                                                                                                                ::peg::RuleResult::Failed => {
                                                                                                                                                                                                                    __err_state.mark_failure(__pos, "\"\\n\"");
                                                                                                                                                                                                                    ::peg::RuleResult::Failed
                                                                                                                                                                                                                }
                                                                                                                                                                                                            }
                                                                                                                                                                                                        }
                                                                                                                                                                                                        ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                                                                                    }
                                                                                                                                                                                                }
                                                                                                                                                                                            }
                                                                                                                                                                                            ::peg::RuleResult::Failed => {
                                                                                                                                                                                                __err_state.mark_failure(__pos, "\"{\"");
                                                                                                                                                                                                ::peg::RuleResult::Failed
                                                                                                                                                                                            }
                                                                                                                                                                                        }
                                                                                                                                                                                    }
                                                                                                                                                                                    ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                        ::peg::RuleResult::Failed => {
                                                                                                                                                                            __err_state.mark_failure(__pos, "\"else\"");
                                                                                                                                                                            ::peg::RuleResult::Failed
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                    ::peg::RuleResult::Failed => {
                                                                                                                                                        __err_state.mark_failure(__pos, "\"}\"");
                                                                                                                                                        ::peg::RuleResult::Failed
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                            ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                    ::peg::RuleResult::Failed => {
                                                                                                                        __err_state.mark_failure(__pos, "\"\\n\"");
                                                                                                                        ::peg::RuleResult::Failed
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                            ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                                ::peg::RuleResult::Failed => {
                                                                                                    __err_state.mark_failure(__pos, "\"{\"");
                                                                                                    ::peg::RuleResult::Failed
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                    }
                                            }
                                            ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                        }
                                    }
                                    ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                }
                            }
                            ::peg::RuleResult::Failed => {
                                __err_state.mark_failure(__pos, "\"if\"");
                                ::peg::RuleResult::Failed
                            }
                        }
                    {
                        return ::peg::RuleResult::Matched(__pos, __v);
                    }
                    if let ::peg::RuleResult::Matched(__pos, __v) =
                        match ::peg::ParseLiteral::parse_string_literal(__input, __pos, "if") {
                            ::peg::RuleResult::Matched(__pos, __val) => {
                                let __seq_res =
                                    match __parse__(__input, __state, __err_state, __pos) {
                                        ::peg::RuleResult::Matched(pos, _) => {
                                            ::peg::RuleResult::Matched(pos, ())
                                        }
                                        ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                    };
                                match __seq_res {
                                    ::peg::RuleResult::Matched(__pos, _) => {
                                        let __seq_res = __parse_expression(
                                            __input,
                                            __state,
                                            __err_state,
                                            __pos,
                                        );
                                        match __seq_res {
                                            ::peg::RuleResult::Matched(__pos, e) => {
                                                let __seq_res = match __parse__(
                                                    __input,
                                                    __state,
                                                    __err_state,
                                                    __pos,
                                                ) {
                                                    ::peg::RuleResult::Matched(pos, _) => {
                                                        ::peg::RuleResult::Matched(pos, ())
                                                    }
                                                    ::peg::RuleResult::Failed => {
                                                        ::peg::RuleResult::Failed
                                                    }
                                                };
                                                match __seq_res {
                                                                                        ::peg::RuleResult::Matched(__pos, _) => {
                                                                                            match ::peg::ParseLiteral::parse_string_literal(__input,
                                                                                                    __pos, "{") {
                                                                                                ::peg::RuleResult::Matched(__pos, __val) => {
                                                                                                    {
                                                                                                        let __seq_res =
                                                                                                            match __parse__(__input, __state, __err_state, __pos) {
                                                                                                                ::peg::RuleResult::Matched(pos, _) =>
                                                                                                                    ::peg::RuleResult::Matched(pos, ()),
                                                                                                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                            };
                                                                                                        match __seq_res {
                                                                                                            ::peg::RuleResult::Matched(__pos, _) => {
                                                                                                                match ::peg::ParseLiteral::parse_string_literal(__input,
                                                                                                                        __pos, "\n") {
                                                                                                                    ::peg::RuleResult::Matched(__pos, __val) => {
                                                                                                                        {
                                                                                                                            let __seq_res =
                                                                                                                                __parse_statements(__input, __state, __err_state, __pos);
                                                                                                                            match __seq_res {
                                                                                                                                ::peg::RuleResult::Matched(__pos, then_body) => {
                                                                                                                                    {
                                                                                                                                        let __seq_res =
                                                                                                                                            match __parse__(__input, __state, __err_state, __pos) {
                                                                                                                                                ::peg::RuleResult::Matched(pos, _) =>
                                                                                                                                                    ::peg::RuleResult::Matched(pos, ()),
                                                                                                                                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                            };
                                                                                                                                        match __seq_res {
                                                                                                                                            ::peg::RuleResult::Matched(__pos, _) => {
                                                                                                                                                match ::peg::ParseLiteral::parse_string_literal(__input,
                                                                                                                                                        __pos, "}") {
                                                                                                                                                    ::peg::RuleResult::Matched(__pos, __val) => {
                                                                                                                                                        {
                                                                                                                                                            let __seq_res =
                                                                                                                                                                match __parse__(__input, __state, __err_state, __pos) {
                                                                                                                                                                    ::peg::RuleResult::Matched(pos, _) =>
                                                                                                                                                                        ::peg::RuleResult::Matched(pos, ()),
                                                                                                                                                                    ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                                                };
                                                                                                                                                            match __seq_res {
                                                                                                                                                                ::peg::RuleResult::Matched(__pos, _) => {
                                                                                                                                                                    match ::peg::ParseLiteral::parse_string_literal(__input,
                                                                                                                                                                            __pos, "else") {
                                                                                                                                                                        ::peg::RuleResult::Matched(__pos, __val) => {
                                                                                                                                                                            {
                                                                                                                                                                                let __seq_res =
                                                                                                                                                                                    match __parse__(__input, __state, __err_state, __pos) {
                                                                                                                                                                                        ::peg::RuleResult::Matched(pos, _) =>
                                                                                                                                                                                            ::peg::RuleResult::Matched(pos, ()),
                                                                                                                                                                                        ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                                                                    };
                                                                                                                                                                                match __seq_res {
                                                                                                                                                                                    ::peg::RuleResult::Matched(__pos, _) => {
                                                                                                                                                                                        {
                                                                                                                                                                                            let __seq_res =
                                                                                                                                                                                                __parse_if_else(__input, __state, __err_state, __pos);
                                                                                                                                                                                            match __seq_res {
                                                                                                                                                                                                ::peg::RuleResult::Matched(__pos, else_body) => {
                                                                                                                                                                                                    ::peg::RuleResult::Matched(__pos,
                                                                                                                                                                                                        (||
                                                                                                                                                                                                                    {
                                                                                                                                                                                                                        ExprAst::IfElse(Box::new(e), then_body,
                                                                                                                                                                                                                            <[_]>::into_vec(Box::new([else_body])))
                                                                                                                                                                                                                    })())
                                                                                                                                                                                                }
                                                                                                                                                                                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                                                                            }
                                                                                                                                                                                        }
                                                                                                                                                                                    }
                                                                                                                                                                                    ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                        ::peg::RuleResult::Failed => {
                                                                                                                                                                            __err_state.mark_failure(__pos, "\"else\"");
                                                                                                                                                                            ::peg::RuleResult::Failed
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                    ::peg::RuleResult::Failed => {
                                                                                                                                                        __err_state.mark_failure(__pos, "\"}\"");
                                                                                                                                                        ::peg::RuleResult::Failed
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                            ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                    ::peg::RuleResult::Failed => {
                                                                                                                        __err_state.mark_failure(__pos, "\"\\n\"");
                                                                                                                        ::peg::RuleResult::Failed
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                            ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                                ::peg::RuleResult::Failed => {
                                                                                                    __err_state.mark_failure(__pos, "\"{\"");
                                                                                                    ::peg::RuleResult::Failed
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                    }
                                            }
                                            ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                        }
                                    }
                                    ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                }
                            }
                            ::peg::RuleResult::Failed => {
                                __err_state.mark_failure(__pos, "\"if\"");
                                ::peg::RuleResult::Failed
                            }
                        }
                    {
                        return ::peg::RuleResult::Matched(__pos, __v);
                    }
                    ::peg::RuleResult::Failed
                }),
                &(|__pos,
                   __lpos,
                   __min_prec,
                   mut __infix_result,
                   __state,
                   __err_state,
                   __recurse| { (__infix_result, ::peg::RuleResult::Failed) }),
            )
        }
    }
    fn __parse_while_loop<'input>(
        __input: &'input Input,
        __state: &mut ParseState<'input>,
        __err_state: &mut ::peg::error::ErrorState,
        __pos: usize,
    ) -> ::peg::RuleResult<ExprAst> {
        #![allow(non_snake_case, unused, clippy::redundant_closure_call)]
        match ::peg::ParseLiteral::parse_string_literal(__input, __pos, "while") {
            ::peg::RuleResult::Matched(__pos, __val) => {
                let __seq_res = match __parse__(__input, __state, __err_state, __pos) {
                    ::peg::RuleResult::Matched(pos, _) => ::peg::RuleResult::Matched(pos, ()),
                    ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                };
                match __seq_res {
                    ::peg::RuleResult::Matched(__pos, _) => {
                        let __seq_res = __parse_expression(__input, __state, __err_state, __pos);
                        match __seq_res {
                            ::peg::RuleResult::Matched(__pos, e) => {
                                let __seq_res =
                                    match __parse__(__input, __state, __err_state, __pos) {
                                        ::peg::RuleResult::Matched(pos, _) => {
                                            ::peg::RuleResult::Matched(pos, ())
                                        }
                                        ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                    };
                                match __seq_res {
                                    ::peg::RuleResult::Matched(__pos, _) => {
                                        match ::peg::ParseLiteral::parse_string_literal(
                                            __input, __pos, "{",
                                        ) {
                                            ::peg::RuleResult::Matched(__pos, __val) => {
                                                let __seq_res = match __parse__(
                                                    __input,
                                                    __state,
                                                    __err_state,
                                                    __pos,
                                                ) {
                                                    ::peg::RuleResult::Matched(pos, _) => {
                                                        ::peg::RuleResult::Matched(pos, ())
                                                    }
                                                    ::peg::RuleResult::Failed => {
                                                        ::peg::RuleResult::Failed
                                                    }
                                                };
                                                match __seq_res {
                                                                            ::peg::RuleResult::Matched(__pos, _) => {
                                                                                match ::peg::ParseLiteral::parse_string_literal(__input,
                                                                                        __pos, "\n") {
                                                                                    ::peg::RuleResult::Matched(__pos, __val) => {
                                                                                        {
                                                                                            let __seq_res =
                                                                                                __parse_statements(__input, __state, __err_state, __pos);
                                                                                            match __seq_res {
                                                                                                ::peg::RuleResult::Matched(__pos, loop_body) => {
                                                                                                    {
                                                                                                        let __seq_res =
                                                                                                            match __parse__(__input, __state, __err_state, __pos) {
                                                                                                                ::peg::RuleResult::Matched(pos, _) =>
                                                                                                                    ::peg::RuleResult::Matched(pos, ()),
                                                                                                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                            };
                                                                                                        match __seq_res {
                                                                                                            ::peg::RuleResult::Matched(__pos, _) => {
                                                                                                                match ::peg::ParseLiteral::parse_string_literal(__input,
                                                                                                                        __pos, "}") {
                                                                                                                    ::peg::RuleResult::Matched(__pos, __val) => {
                                                                                                                        ::peg::RuleResult::Matched(__pos,
                                                                                                                            (|| { ExprAst::WhileLoop(Box::new(e), loop_body) })())
                                                                                                                    }
                                                                                                                    ::peg::RuleResult::Failed => {
                                                                                                                        __err_state.mark_failure(__pos, "\"}\"");
                                                                                                                        ::peg::RuleResult::Failed
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                            ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    ::peg::RuleResult::Failed => {
                                                                                        __err_state.mark_failure(__pos, "\"\\n\"");
                                                                                        ::peg::RuleResult::Failed
                                                                                    }
                                                                                }
                                                                            }
                                                                            ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                        }
                                            }
                                            ::peg::RuleResult::Failed => {
                                                __err_state.mark_failure(__pos, "\"{\"");
                                                ::peg::RuleResult::Failed
                                            }
                                        }
                                    }
                                    ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                }
                            }
                            ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                        }
                    }
                    ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                }
            }
            ::peg::RuleResult::Failed => {
                __err_state.mark_failure(__pos, "\"while\"");
                ::peg::RuleResult::Failed
            }
        }
    }
    fn __parse_assignment<'input>(
        __input: &'input Input,
        __state: &mut ParseState<'input>,
        __err_state: &mut ::peg::error::ErrorState,
        __pos: usize,
    ) -> ::peg::RuleResult<ExprAst> {
        #![allow(non_snake_case, unused, clippy::redundant_closure_call)]
        {
            let __seq_res = __parse_identifier(__input, __state, __err_state, __pos);
            match __seq_res {
                ::peg::RuleResult::Matched(__pos, i) => {
                    let __seq_res = match __parse__(__input, __state, __err_state, __pos) {
                        ::peg::RuleResult::Matched(pos, _) => ::peg::RuleResult::Matched(pos, ()),
                        ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                    };
                    match __seq_res {
                        ::peg::RuleResult::Matched(__pos, _) => {
                            match ::peg::ParseLiteral::parse_string_literal(__input, __pos, "=") {
                                ::peg::RuleResult::Matched(__pos, __val) => {
                                    let __seq_res =
                                        match __parse__(__input, __state, __err_state, __pos) {
                                            ::peg::RuleResult::Matched(pos, _) => {
                                                ::peg::RuleResult::Matched(pos, ())
                                            }
                                            ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                        };
                                    match __seq_res {
                                        ::peg::RuleResult::Matched(__pos, _) => {
                                            let __seq_res = __parse_expression(
                                                __input,
                                                __state,
                                                __err_state,
                                                __pos,
                                            );
                                            match __seq_res {
                                                ::peg::RuleResult::Matched(__pos, e) => {
                                                    ::peg::RuleResult::Matched(
                                                        __pos,
                                                        (|| ExprAst::Assign(i, Box::new(e)))(),
                                                    )
                                                }
                                                ::peg::RuleResult::Failed => {
                                                    ::peg::RuleResult::Failed
                                                }
                                            }
                                        }
                                        ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                    }
                                }
                                ::peg::RuleResult::Failed => {
                                    __err_state.mark_failure(__pos, "\"=\"");
                                    ::peg::RuleResult::Failed
                                }
                            }
                        }
                        ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                    }
                }
                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
            }
        }
    }
    fn __parse_binary_op<'input>(
        __input: &'input Input,
        __state: &mut ParseState<'input>,
        __err_state: &mut ::peg::error::ErrorState,
        __pos: usize,
    ) -> ::peg::RuleResult<ExprAst> {
        #![allow(non_snake_case, unused, clippy::redundant_closure_call)]
        {
            fn __infix_parse<T, S>(
                state: &mut S,
                err_state: &mut ::peg::error::ErrorState,
                min_prec: i32,
                lpos: usize,
                prefix_atom: &Fn(
                    usize,
                    &mut S,
                    &mut ::peg::error::ErrorState,
                    &Fn(usize, i32, &mut S, &mut ::peg::error::ErrorState) -> ::peg::RuleResult<T>,
                ) -> ::peg::RuleResult<T>,
                level_code: &Fn(
                    usize,
                    usize,
                    i32,
                    T,
                    &mut S,
                    &mut ::peg::error::ErrorState,
                    &Fn(usize, i32, &mut S, &mut ::peg::error::ErrorState) -> ::peg::RuleResult<T>,
                ) -> (T, ::peg::RuleResult<()>),
            ) -> ::peg::RuleResult<T> {
                let initial = {
                    prefix_atom(
                        lpos,
                        state,
                        err_state,
                        &(|pos, min_prec, state, err_state| {
                            __infix_parse(state, err_state, min_prec, pos, prefix_atom, level_code)
                        }),
                    )
                };
                if let ::peg::RuleResult::Matched(pos, mut infix_result) = initial {
                    let mut repeat_pos = pos;
                    loop {
                        let (val, res) = level_code(
                            repeat_pos,
                            lpos,
                            min_prec,
                            infix_result,
                            state,
                            err_state,
                            &(|pos, min_prec, state, err_state| {
                                __infix_parse(
                                    state,
                                    err_state,
                                    min_prec,
                                    pos,
                                    prefix_atom,
                                    level_code,
                                )
                            }),
                        );
                        infix_result = val;
                        if let ::peg::RuleResult::Matched(pos, ()) = res {
                            repeat_pos = pos;
                            continue;
                        }
                        break;
                    }
                    ::peg::RuleResult::Matched(repeat_pos, infix_result)
                } else {
                    ::peg::RuleResult::Failed
                }
            }
            __infix_parse(
                __state,
                __err_state,
                0,
                __pos,
                &(|__pos, __state, __err_state, __recurse| {
                    let __lpos = __pos;
                    if let ::peg::RuleResult::Matched(__pos, __v) = {
                        let __seq_res = __parse_identifier(__input, __state, __err_state, __pos);
                        match __seq_res {
                            ::peg::RuleResult::Matched(__pos, i) => {
                                let __seq_res =
                                    match __parse__(__input, __state, __err_state, __pos) {
                                        ::peg::RuleResult::Matched(pos, _) => {
                                            ::peg::RuleResult::Matched(pos, ())
                                        }
                                        ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                    };
                                match __seq_res {
                                    ::peg::RuleResult::Matched(__pos, _) => {
                                        match ::peg::ParseLiteral::parse_string_literal(
                                            __input, __pos, "(",
                                        ) {
                                            ::peg::RuleResult::Matched(__pos, __val) => {
                                                let __seq_res = {
                                                    let mut __repeat_pos = __pos;
                                                    let mut __repeat_value = vec![];
                                                    loop {
                                                        let __pos = __repeat_pos;
                                                        let __pos = if __repeat_value.is_empty() {
                                                            __pos
                                                        } else {
                                                            let __sep_res =
                                                                                                               match ::peg::ParseLiteral::parse_string_literal(__input,
                                                                                                                       __pos, ",") {
                                                                                                                   ::peg::RuleResult::Matched(__pos, __val) => {
                                                                                                                       ::peg::RuleResult::Matched(__pos, __val)
                                                                                                                   }
                                                                                                                   ::peg::RuleResult::Failed => {
                                                                                                                       __err_state.mark_failure(__pos, "\",\"");
                                                                                                                       ::peg::RuleResult::Failed
                                                                                                                   }
                                                                                                               };
                                                            match __sep_res {
                                                                ::peg::RuleResult::Matched(
                                                                    __newpos,
                                                                    _,
                                                                ) => __newpos,
                                                                ::peg::RuleResult::Failed => break,
                                                            }
                                                        };
                                                        let __step_res = {
                                                            let __seq_res = match __parse__(
                                                                __input,
                                                                __state,
                                                                __err_state,
                                                                __pos,
                                                            ) {
                                                                ::peg::RuleResult::Matched(
                                                                    pos,
                                                                    _,
                                                                ) => ::peg::RuleResult::Matched(
                                                                    pos,
                                                                    (),
                                                                ),
                                                                ::peg::RuleResult::Failed => {
                                                                    ::peg::RuleResult::Failed
                                                                }
                                                            };
                                                            match __seq_res {
                                                                ::peg::RuleResult::Matched(
                                                                    __pos,
                                                                    _,
                                                                ) => {
                                                                    let __seq_res =
                                                                        __parse_expression(
                                                                            __input,
                                                                            __state,
                                                                            __err_state,
                                                                            __pos,
                                                                        );
                                                                    match __seq_res {
                                                                                                                        ::peg::RuleResult::Matched(__pos, e) => {
                                                                                                                            {
                                                                                                                                let __seq_res =
                                                                                                                                    match __parse__(__input, __state, __err_state, __pos) {
                                                                                                                                        ::peg::RuleResult::Matched(pos, _) =>
                                                                                                                                            ::peg::RuleResult::Matched(pos, ()),
                                                                                                                                        ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                    };
                                                                                                                                match __seq_res {
                                                                                                                                    ::peg::RuleResult::Matched(__pos, _) => {
                                                                                                                                        ::peg::RuleResult::Matched(__pos, (|| { e })())
                                                                                                                                    }
                                                                                                                                    ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                        ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                    }
                                                                }
                                                                ::peg::RuleResult::Failed => {
                                                                    ::peg::RuleResult::Failed
                                                                }
                                                            }
                                                        };
                                                        match __step_res {
                                                            ::peg::RuleResult::Matched(
                                                                __newpos,
                                                                __value,
                                                            ) => {
                                                                __repeat_pos = __newpos;
                                                                __repeat_value.push(__value);
                                                            }
                                                            ::peg::RuleResult::Failed => {
                                                                break;
                                                            }
                                                        }
                                                    }
                                                    ::peg::RuleResult::Matched(
                                                        __repeat_pos,
                                                        __repeat_value,
                                                    )
                                                };
                                                match __seq_res {
                                                                                        ::peg::RuleResult::Matched(__pos, args) => {
                                                                                            match ::peg::ParseLiteral::parse_string_literal(__input,
                                                                                                    __pos, ")") {
                                                                                                ::peg::RuleResult::Matched(__pos, __val) => {
                                                                                                    ::peg::RuleResult::Matched(__pos,
                                                                                                        (|| { ExprAst::Call(i, args) })())
                                                                                                }
                                                                                                ::peg::RuleResult::Failed => {
                                                                                                    __err_state.mark_failure(__pos, "\")\"");
                                                                                                    ::peg::RuleResult::Failed
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                    }
                                            }
                                            ::peg::RuleResult::Failed => {
                                                __err_state.mark_failure(__pos, "\"(\"");
                                                ::peg::RuleResult::Failed
                                            }
                                        }
                                    }
                                    ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                }
                            }
                            ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                        }
                    } {
                        return ::peg::RuleResult::Matched(__pos, __v);
                    }
                    if let ::peg::RuleResult::Matched(__pos, __v) = {
                        let __seq_res = __parse_identifier(__input, __state, __err_state, __pos);
                        match __seq_res {
                            ::peg::RuleResult::Matched(__pos, i) => {
                                ::peg::RuleResult::Matched(__pos, (|| ExprAst::Identifier(i))())
                            }
                            ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                        }
                    } {
                        return ::peg::RuleResult::Matched(__pos, __v);
                    }
                    if let ::peg::RuleResult::Matched(__pos, __v) = {
                        let __seq_res = __parse_literal(__input, __state, __err_state, __pos);
                        match __seq_res {
                            ::peg::RuleResult::Matched(__pos, l) => {
                                ::peg::RuleResult::Matched(__pos, (|| l)())
                            }
                            ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                        }
                    } {
                        return ::peg::RuleResult::Matched(__pos, __v);
                    }
                    ::peg::RuleResult::Failed
                }),
                &(|__pos,
                   __lpos,
                   __min_prec,
                   mut __infix_result,
                   __state,
                   __err_state,
                   __recurse| {
                    if 0i32 >= __min_prec {
                        if let ::peg::RuleResult::Matched(__pos, ()) = {
                            let __seq_res = match __parse__(__input, __state, __err_state, __pos) {
                                ::peg::RuleResult::Matched(pos, _) => {
                                    ::peg::RuleResult::Matched(pos, ())
                                }
                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                            };
                            match __seq_res {
                                ::peg::RuleResult::Matched(__pos, _) => {
                                    match ::peg::ParseLiteral::parse_string_literal(
                                        __input, __pos, "==",
                                    ) {
                                        ::peg::RuleResult::Matched(__pos, __val) => {
                                            let __seq_res = match __parse__(
                                                __input,
                                                __state,
                                                __err_state,
                                                __pos,
                                            ) {
                                                ::peg::RuleResult::Matched(pos, _) => {
                                                    ::peg::RuleResult::Matched(pos, ())
                                                }
                                                ::peg::RuleResult::Failed => {
                                                    ::peg::RuleResult::Failed
                                                }
                                            };
                                            match __seq_res {
                                                ::peg::RuleResult::Matched(__pos, _) => {
                                                    if let ::peg::RuleResult::Matched(__pos, b) =
                                                        __recurse(__pos, 0i32, __state, __err_state)
                                                    {
                                                        let a = __infix_result;
                                                        __infix_result = (|| {
                                                            ExprAst::Eq(Box::new(a), Box::new(b))
                                                        })(
                                                        );
                                                        ::peg::RuleResult::Matched(__pos, ())
                                                    } else {
                                                        ::peg::RuleResult::Failed
                                                    }
                                                }
                                                ::peg::RuleResult::Failed => {
                                                    ::peg::RuleResult::Failed
                                                }
                                            }
                                        }
                                        ::peg::RuleResult::Failed => {
                                            __err_state.mark_failure(__pos, "\"==\"");
                                            ::peg::RuleResult::Failed
                                        }
                                    }
                                }
                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                            }
                        } {
                            return (__infix_result, ::peg::RuleResult::Matched(__pos, ()));
                        }
                        if let ::peg::RuleResult::Matched(__pos, ()) = {
                            let __seq_res = match __parse__(__input, __state, __err_state, __pos) {
                                ::peg::RuleResult::Matched(pos, _) => {
                                    ::peg::RuleResult::Matched(pos, ())
                                }
                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                            };
                            match __seq_res {
                                ::peg::RuleResult::Matched(__pos, _) => {
                                    match ::peg::ParseLiteral::parse_string_literal(
                                        __input, __pos, "!=",
                                    ) {
                                        ::peg::RuleResult::Matched(__pos, __val) => {
                                            let __seq_res = match __parse__(
                                                __input,
                                                __state,
                                                __err_state,
                                                __pos,
                                            ) {
                                                ::peg::RuleResult::Matched(pos, _) => {
                                                    ::peg::RuleResult::Matched(pos, ())
                                                }
                                                ::peg::RuleResult::Failed => {
                                                    ::peg::RuleResult::Failed
                                                }
                                            };
                                            match __seq_res {
                                                ::peg::RuleResult::Matched(__pos, _) => {
                                                    if let ::peg::RuleResult::Matched(__pos, b) =
                                                        __recurse(__pos, 0i32, __state, __err_state)
                                                    {
                                                        let a = __infix_result;
                                                        __infix_result = (|| {
                                                            ExprAst::Ne(Box::new(a), Box::new(b))
                                                        })(
                                                        );
                                                        ::peg::RuleResult::Matched(__pos, ())
                                                    } else {
                                                        ::peg::RuleResult::Failed
                                                    }
                                                }
                                                ::peg::RuleResult::Failed => {
                                                    ::peg::RuleResult::Failed
                                                }
                                            }
                                        }
                                        ::peg::RuleResult::Failed => {
                                            __err_state.mark_failure(__pos, "\"!=\"");
                                            ::peg::RuleResult::Failed
                                        }
                                    }
                                }
                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                            }
                        } {
                            return (__infix_result, ::peg::RuleResult::Matched(__pos, ()));
                        }
                        if let ::peg::RuleResult::Matched(__pos, ()) = {
                            let __seq_res = match __parse__(__input, __state, __err_state, __pos) {
                                ::peg::RuleResult::Matched(pos, _) => {
                                    ::peg::RuleResult::Matched(pos, ())
                                }
                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                            };
                            match __seq_res {
                                ::peg::RuleResult::Matched(__pos, _) => {
                                    match ::peg::ParseLiteral::parse_string_literal(
                                        __input, __pos, "<",
                                    ) {
                                        ::peg::RuleResult::Matched(__pos, __val) => {
                                            let __seq_res = match __parse__(
                                                __input,
                                                __state,
                                                __err_state,
                                                __pos,
                                            ) {
                                                ::peg::RuleResult::Matched(pos, _) => {
                                                    ::peg::RuleResult::Matched(pos, ())
                                                }
                                                ::peg::RuleResult::Failed => {
                                                    ::peg::RuleResult::Failed
                                                }
                                            };
                                            match __seq_res {
                                                ::peg::RuleResult::Matched(__pos, _) => {
                                                    if let ::peg::RuleResult::Matched(__pos, b) =
                                                        __recurse(__pos, 0i32, __state, __err_state)
                                                    {
                                                        let a = __infix_result;
                                                        __infix_result = (|| {
                                                            ExprAst::Lt(Box::new(a), Box::new(b))
                                                        })(
                                                        );
                                                        ::peg::RuleResult::Matched(__pos, ())
                                                    } else {
                                                        ::peg::RuleResult::Failed
                                                    }
                                                }
                                                ::peg::RuleResult::Failed => {
                                                    ::peg::RuleResult::Failed
                                                }
                                            }
                                        }
                                        ::peg::RuleResult::Failed => {
                                            __err_state.mark_failure(__pos, "\"<\"");
                                            ::peg::RuleResult::Failed
                                        }
                                    }
                                }
                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                            }
                        } {
                            return (__infix_result, ::peg::RuleResult::Matched(__pos, ()));
                        }
                        if let ::peg::RuleResult::Matched(__pos, ()) = {
                            let __seq_res = match __parse__(__input, __state, __err_state, __pos) {
                                ::peg::RuleResult::Matched(pos, _) => {
                                    ::peg::RuleResult::Matched(pos, ())
                                }
                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                            };
                            match __seq_res {
                                ::peg::RuleResult::Matched(__pos, _) => {
                                    match ::peg::ParseLiteral::parse_string_literal(
                                        __input, __pos, "<=",
                                    ) {
                                        ::peg::RuleResult::Matched(__pos, __val) => {
                                            let __seq_res = match __parse__(
                                                __input,
                                                __state,
                                                __err_state,
                                                __pos,
                                            ) {
                                                ::peg::RuleResult::Matched(pos, _) => {
                                                    ::peg::RuleResult::Matched(pos, ())
                                                }
                                                ::peg::RuleResult::Failed => {
                                                    ::peg::RuleResult::Failed
                                                }
                                            };
                                            match __seq_res {
                                                ::peg::RuleResult::Matched(__pos, _) => {
                                                    if let ::peg::RuleResult::Matched(__pos, b) =
                                                        __recurse(__pos, 0i32, __state, __err_state)
                                                    {
                                                        let a = __infix_result;
                                                        __infix_result = (|| {
                                                            ExprAst::Le(Box::new(a), Box::new(b))
                                                        })(
                                                        );
                                                        ::peg::RuleResult::Matched(__pos, ())
                                                    } else {
                                                        ::peg::RuleResult::Failed
                                                    }
                                                }
                                                ::peg::RuleResult::Failed => {
                                                    ::peg::RuleResult::Failed
                                                }
                                            }
                                        }
                                        ::peg::RuleResult::Failed => {
                                            __err_state.mark_failure(__pos, "\"<=\"");
                                            ::peg::RuleResult::Failed
                                        }
                                    }
                                }
                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                            }
                        } {
                            return (__infix_result, ::peg::RuleResult::Matched(__pos, ()));
                        }
                        if let ::peg::RuleResult::Matched(__pos, ()) = {
                            let __seq_res = match __parse__(__input, __state, __err_state, __pos) {
                                ::peg::RuleResult::Matched(pos, _) => {
                                    ::peg::RuleResult::Matched(pos, ())
                                }
                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                            };
                            match __seq_res {
                                ::peg::RuleResult::Matched(__pos, _) => {
                                    match ::peg::ParseLiteral::parse_string_literal(
                                        __input, __pos, ">",
                                    ) {
                                        ::peg::RuleResult::Matched(__pos, __val) => {
                                            let __seq_res = match __parse__(
                                                __input,
                                                __state,
                                                __err_state,
                                                __pos,
                                            ) {
                                                ::peg::RuleResult::Matched(pos, _) => {
                                                    ::peg::RuleResult::Matched(pos, ())
                                                }
                                                ::peg::RuleResult::Failed => {
                                                    ::peg::RuleResult::Failed
                                                }
                                            };
                                            match __seq_res {
                                                ::peg::RuleResult::Matched(__pos, _) => {
                                                    if let ::peg::RuleResult::Matched(__pos, b) =
                                                        __recurse(__pos, 0i32, __state, __err_state)
                                                    {
                                                        let a = __infix_result;
                                                        __infix_result = (|| {
                                                            ExprAst::Gt(Box::new(a), Box::new(b))
                                                        })(
                                                        );
                                                        ::peg::RuleResult::Matched(__pos, ())
                                                    } else {
                                                        ::peg::RuleResult::Failed
                                                    }
                                                }
                                                ::peg::RuleResult::Failed => {
                                                    ::peg::RuleResult::Failed
                                                }
                                            }
                                        }
                                        ::peg::RuleResult::Failed => {
                                            __err_state.mark_failure(__pos, "\">\"");
                                            ::peg::RuleResult::Failed
                                        }
                                    }
                                }
                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                            }
                        } {
                            return (__infix_result, ::peg::RuleResult::Matched(__pos, ()));
                        }
                        if let ::peg::RuleResult::Matched(__pos, ()) = {
                            let __seq_res = match __parse__(__input, __state, __err_state, __pos) {
                                ::peg::RuleResult::Matched(pos, _) => {
                                    ::peg::RuleResult::Matched(pos, ())
                                }
                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                            };
                            match __seq_res {
                                ::peg::RuleResult::Matched(__pos, _) => {
                                    match ::peg::ParseLiteral::parse_string_literal(
                                        __input, __pos, ">=",
                                    ) {
                                        ::peg::RuleResult::Matched(__pos, __val) => {
                                            let __seq_res = match __parse__(
                                                __input,
                                                __state,
                                                __err_state,
                                                __pos,
                                            ) {
                                                ::peg::RuleResult::Matched(pos, _) => {
                                                    ::peg::RuleResult::Matched(pos, ())
                                                }
                                                ::peg::RuleResult::Failed => {
                                                    ::peg::RuleResult::Failed
                                                }
                                            };
                                            match __seq_res {
                                                ::peg::RuleResult::Matched(__pos, _) => {
                                                    if let ::peg::RuleResult::Matched(__pos, b) =
                                                        __recurse(__pos, 0i32, __state, __err_state)
                                                    {
                                                        let a = __infix_result;
                                                        __infix_result = (|| {
                                                            ExprAst::Ge(Box::new(a), Box::new(b))
                                                        })(
                                                        );
                                                        ::peg::RuleResult::Matched(__pos, ())
                                                    } else {
                                                        ::peg::RuleResult::Failed
                                                    }
                                                }
                                                ::peg::RuleResult::Failed => {
                                                    ::peg::RuleResult::Failed
                                                }
                                            }
                                        }
                                        ::peg::RuleResult::Failed => {
                                            __err_state.mark_failure(__pos, "\">=\"");
                                            ::peg::RuleResult::Failed
                                        }
                                    }
                                }
                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                            }
                        } {
                            return (__infix_result, ::peg::RuleResult::Matched(__pos, ()));
                        }
                    }
                    if 1i32 >= __min_prec {
                        if let ::peg::RuleResult::Matched(__pos, ()) = {
                            let __seq_res = match __parse__(__input, __state, __err_state, __pos) {
                                ::peg::RuleResult::Matched(pos, _) => {
                                    ::peg::RuleResult::Matched(pos, ())
                                }
                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                            };
                            match __seq_res {
                                ::peg::RuleResult::Matched(__pos, _) => {
                                    match ::peg::ParseLiteral::parse_string_literal(
                                        __input, __pos, "+",
                                    ) {
                                        ::peg::RuleResult::Matched(__pos, __val) => {
                                            let __seq_res = match __parse__(
                                                __input,
                                                __state,
                                                __err_state,
                                                __pos,
                                            ) {
                                                ::peg::RuleResult::Matched(pos, _) => {
                                                    ::peg::RuleResult::Matched(pos, ())
                                                }
                                                ::peg::RuleResult::Failed => {
                                                    ::peg::RuleResult::Failed
                                                }
                                            };
                                            match __seq_res {
                                                ::peg::RuleResult::Matched(__pos, _) => {
                                                    if let ::peg::RuleResult::Matched(__pos, b) =
                                                        __recurse(__pos, 1i32, __state, __err_state)
                                                    {
                                                        let a = __infix_result;
                                                        __infix_result = (|| {
                                                            ExprAst::Add(Box::new(a), Box::new(b))
                                                        })(
                                                        );
                                                        ::peg::RuleResult::Matched(__pos, ())
                                                    } else {
                                                        ::peg::RuleResult::Failed
                                                    }
                                                }
                                                ::peg::RuleResult::Failed => {
                                                    ::peg::RuleResult::Failed
                                                }
                                            }
                                        }
                                        ::peg::RuleResult::Failed => {
                                            __err_state.mark_failure(__pos, "\"+\"");
                                            ::peg::RuleResult::Failed
                                        }
                                    }
                                }
                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                            }
                        } {
                            return (__infix_result, ::peg::RuleResult::Matched(__pos, ()));
                        }
                        if let ::peg::RuleResult::Matched(__pos, ()) = {
                            let __seq_res = match __parse__(__input, __state, __err_state, __pos) {
                                ::peg::RuleResult::Matched(pos, _) => {
                                    ::peg::RuleResult::Matched(pos, ())
                                }
                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                            };
                            match __seq_res {
                                ::peg::RuleResult::Matched(__pos, _) => {
                                    match ::peg::ParseLiteral::parse_string_literal(
                                        __input, __pos, "-",
                                    ) {
                                        ::peg::RuleResult::Matched(__pos, __val) => {
                                            let __seq_res = match __parse__(
                                                __input,
                                                __state,
                                                __err_state,
                                                __pos,
                                            ) {
                                                ::peg::RuleResult::Matched(pos, _) => {
                                                    ::peg::RuleResult::Matched(pos, ())
                                                }
                                                ::peg::RuleResult::Failed => {
                                                    ::peg::RuleResult::Failed
                                                }
                                            };
                                            match __seq_res {
                                                ::peg::RuleResult::Matched(__pos, _) => {
                                                    if let ::peg::RuleResult::Matched(__pos, b) =
                                                        __recurse(__pos, 1i32, __state, __err_state)
                                                    {
                                                        let a = __infix_result;
                                                        __infix_result = (|| {
                                                            ExprAst::Sub(Box::new(a), Box::new(b))
                                                        })(
                                                        );
                                                        ::peg::RuleResult::Matched(__pos, ())
                                                    } else {
                                                        ::peg::RuleResult::Failed
                                                    }
                                                }
                                                ::peg::RuleResult::Failed => {
                                                    ::peg::RuleResult::Failed
                                                }
                                            }
                                        }
                                        ::peg::RuleResult::Failed => {
                                            __err_state.mark_failure(__pos, "\"-\"");
                                            ::peg::RuleResult::Failed
                                        }
                                    }
                                }
                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                            }
                        } {
                            return (__infix_result, ::peg::RuleResult::Matched(__pos, ()));
                        }
                    }
                    if 2i32 >= __min_prec {
                        if let ::peg::RuleResult::Matched(__pos, ()) = {
                            let __seq_res = match __parse__(__input, __state, __err_state, __pos) {
                                ::peg::RuleResult::Matched(pos, _) => {
                                    ::peg::RuleResult::Matched(pos, ())
                                }
                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                            };
                            match __seq_res {
                                ::peg::RuleResult::Matched(__pos, _) => {
                                    match ::peg::ParseLiteral::parse_string_literal(
                                        __input, __pos, "*",
                                    ) {
                                        ::peg::RuleResult::Matched(__pos, __val) => {
                                            let __seq_res = match __parse__(
                                                __input,
                                                __state,
                                                __err_state,
                                                __pos,
                                            ) {
                                                ::peg::RuleResult::Matched(pos, _) => {
                                                    ::peg::RuleResult::Matched(pos, ())
                                                }
                                                ::peg::RuleResult::Failed => {
                                                    ::peg::RuleResult::Failed
                                                }
                                            };
                                            match __seq_res {
                                                ::peg::RuleResult::Matched(__pos, _) => {
                                                    if let ::peg::RuleResult::Matched(__pos, b) =
                                                        __recurse(__pos, 2i32, __state, __err_state)
                                                    {
                                                        let a = __infix_result;
                                                        __infix_result = (|| {
                                                            ExprAst::Mul(Box::new(a), Box::new(b))
                                                        })(
                                                        );
                                                        ::peg::RuleResult::Matched(__pos, ())
                                                    } else {
                                                        ::peg::RuleResult::Failed
                                                    }
                                                }
                                                ::peg::RuleResult::Failed => {
                                                    ::peg::RuleResult::Failed
                                                }
                                            }
                                        }
                                        ::peg::RuleResult::Failed => {
                                            __err_state.mark_failure(__pos, "\"*\"");
                                            ::peg::RuleResult::Failed
                                        }
                                    }
                                }
                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                            }
                        } {
                            return (__infix_result, ::peg::RuleResult::Matched(__pos, ()));
                        }
                        if let ::peg::RuleResult::Matched(__pos, ()) = {
                            let __seq_res = match __parse__(__input, __state, __err_state, __pos) {
                                ::peg::RuleResult::Matched(pos, _) => {
                                    ::peg::RuleResult::Matched(pos, ())
                                }
                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                            };
                            match __seq_res {
                                ::peg::RuleResult::Matched(__pos, _) => {
                                    match ::peg::ParseLiteral::parse_string_literal(
                                        __input, __pos, "/",
                                    ) {
                                        ::peg::RuleResult::Matched(__pos, __val) => {
                                            let __seq_res = match __parse__(
                                                __input,
                                                __state,
                                                __err_state,
                                                __pos,
                                            ) {
                                                ::peg::RuleResult::Matched(pos, _) => {
                                                    ::peg::RuleResult::Matched(pos, ())
                                                }
                                                ::peg::RuleResult::Failed => {
                                                    ::peg::RuleResult::Failed
                                                }
                                            };
                                            match __seq_res {
                                                ::peg::RuleResult::Matched(__pos, _) => {
                                                    if let ::peg::RuleResult::Matched(__pos, b) =
                                                        __recurse(__pos, 2i32, __state, __err_state)
                                                    {
                                                        let a = __infix_result;
                                                        __infix_result = (|| {
                                                            ExprAst::Div(Box::new(a), Box::new(b))
                                                        })(
                                                        );
                                                        ::peg::RuleResult::Matched(__pos, ())
                                                    } else {
                                                        ::peg::RuleResult::Failed
                                                    }
                                                }
                                                ::peg::RuleResult::Failed => {
                                                    ::peg::RuleResult::Failed
                                                }
                                            }
                                        }
                                        ::peg::RuleResult::Failed => {
                                            __err_state.mark_failure(__pos, "\"/\"");
                                            ::peg::RuleResult::Failed
                                        }
                                    }
                                }
                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                            }
                        } {
                            return (__infix_result, ::peg::RuleResult::Matched(__pos, ()));
                        }
                    }
                    if 3i32 >= __min_prec {
                        if let ::peg::RuleResult::Matched(__pos, ()) = {
                            let __seq_res = match __parse__(__input, __state, __err_state, __pos) {
                                ::peg::RuleResult::Matched(pos, _) => {
                                    ::peg::RuleResult::Matched(pos, ())
                                }
                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                            };
                            match __seq_res {
                                ::peg::RuleResult::Matched(__pos, _) => {
                                    match ::peg::ParseLiteral::parse_string_literal(
                                        __input, __pos, "mod",
                                    ) {
                                        ::peg::RuleResult::Matched(__pos, __val) => {
                                            let __seq_res = match __parse__(
                                                __input,
                                                __state,
                                                __err_state,
                                                __pos,
                                            ) {
                                                ::peg::RuleResult::Matched(pos, _) => {
                                                    ::peg::RuleResult::Matched(pos, ())
                                                }
                                                ::peg::RuleResult::Failed => {
                                                    ::peg::RuleResult::Failed
                                                }
                                            };
                                            match __seq_res {
                                                ::peg::RuleResult::Matched(__pos, _) => {
                                                    match ::peg::ParseLiteral::parse_string_literal(
                                                        __input, __pos, "(",
                                                    ) {
                                                        ::peg::RuleResult::Matched(
                                                            __pos,
                                                            __val,
                                                        ) => {
                                                            let __seq_res = match __parse__(
                                                                __input,
                                                                __state,
                                                                __err_state,
                                                                __pos,
                                                            ) {
                                                                ::peg::RuleResult::Matched(
                                                                    pos,
                                                                    _,
                                                                ) => ::peg::RuleResult::Matched(
                                                                    pos,
                                                                    (),
                                                                ),
                                                                ::peg::RuleResult::Failed => {
                                                                    ::peg::RuleResult::Failed
                                                                }
                                                            };
                                                            match __seq_res {
                                                                ::peg::RuleResult::Matched(
                                                                    __pos,
                                                                    _,
                                                                ) => {
                                                                    let __seq_res =
                                                                        __parse_expression(
                                                                            __input,
                                                                            __state,
                                                                            __err_state,
                                                                            __pos,
                                                                        );
                                                                    match __seq_res {
                                                                                                                    ::peg::RuleResult::Matched(__pos, b) => {
                                                                                                                        {
                                                                                                                            let __seq_res =
                                                                                                                                match __parse__(__input, __state, __err_state, __pos) {
                                                                                                                                    ::peg::RuleResult::Matched(pos, _) =>
                                                                                                                                        ::peg::RuleResult::Matched(pos, ()),
                                                                                                                                    ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                                };
                                                                                                                            match __seq_res {
                                                                                                                                ::peg::RuleResult::Matched(__pos, _) => {
                                                                                                                                    match ::peg::ParseLiteral::parse_string_literal(__input,
                                                                                                                                            __pos, ")") {
                                                                                                                                        ::peg::RuleResult::Matched(__pos, __val) => {
                                                                                                                                            let a = __infix_result;
                                                                                                                                            __infix_result =
                                                                                                                                                (|| { ExprAst::Mod(Box::new(a), Box::new(b)) })();
                                                                                                                                            ::peg::RuleResult::Matched(__pos, ())
                                                                                                                                        }
                                                                                                                                        ::peg::RuleResult::Failed => {
                                                                                                                                            __err_state.mark_failure(__pos, "\")\"");
                                                                                                                                            ::peg::RuleResult::Failed
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                    ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                                }
                                                                }
                                                                ::peg::RuleResult::Failed => {
                                                                    ::peg::RuleResult::Failed
                                                                }
                                                            }
                                                        }
                                                        ::peg::RuleResult::Failed => {
                                                            __err_state
                                                                .mark_failure(__pos, "\"(\"");
                                                            ::peg::RuleResult::Failed
                                                        }
                                                    }
                                                }
                                                ::peg::RuleResult::Failed => {
                                                    ::peg::RuleResult::Failed
                                                }
                                            }
                                        }
                                        ::peg::RuleResult::Failed => {
                                            __err_state.mark_failure(__pos, "\"mod\"");
                                            ::peg::RuleResult::Failed
                                        }
                                    }
                                }
                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                            }
                        } {
                            return (__infix_result, ::peg::RuleResult::Matched(__pos, ()));
                        }
                        if let ::peg::RuleResult::Matched(__pos, ()) = {
                            let __seq_res = match __parse__(__input, __state, __err_state, __pos) {
                                ::peg::RuleResult::Matched(pos, _) => {
                                    ::peg::RuleResult::Matched(pos, ())
                                }
                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                            };
                            match __seq_res {
                                ::peg::RuleResult::Matched(__pos, _) => {
                                    match ::peg::ParseLiteral::parse_string_literal(
                                        __input, __pos, "mod",
                                    ) {
                                        ::peg::RuleResult::Matched(__pos, __val) => {
                                            let __seq_res = match __parse__(
                                                __input,
                                                __state,
                                                __err_state,
                                                __pos,
                                            ) {
                                                ::peg::RuleResult::Matched(pos, _) => {
                                                    ::peg::RuleResult::Matched(pos, ())
                                                }
                                                ::peg::RuleResult::Failed => {
                                                    ::peg::RuleResult::Failed
                                                }
                                            };
                                            match __seq_res {
                                                ::peg::RuleResult::Matched(__pos, _) => {
                                                    if let ::peg::RuleResult::Matched(__pos, b) =
                                                        __recurse(__pos, 3i32, __state, __err_state)
                                                    {
                                                        let a = __infix_result;
                                                        __infix_result = (|| {
                                                            ExprAst::Mod(Box::new(a), Box::new(b))
                                                        })(
                                                        );
                                                        ::peg::RuleResult::Matched(__pos, ())
                                                    } else {
                                                        ::peg::RuleResult::Failed
                                                    }
                                                }
                                                ::peg::RuleResult::Failed => {
                                                    ::peg::RuleResult::Failed
                                                }
                                            }
                                        }
                                        ::peg::RuleResult::Failed => {
                                            __err_state.mark_failure(__pos, "\"mod\"");
                                            ::peg::RuleResult::Failed
                                        }
                                    }
                                }
                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                            }
                        } {
                            return (__infix_result, ::peg::RuleResult::Matched(__pos, ()));
                        }
                    }
                    (__infix_result, ::peg::RuleResult::Failed)
                }),
            )
        }
    }
    fn __parse_identifier<'input>(
        __input: &'input Input,
        __state: &mut ParseState<'input>,
        __err_state: &mut ::peg::error::ErrorState,
        __pos: usize,
    ) -> ::peg::RuleResult<String> {
        #![allow(non_snake_case, unused, clippy::redundant_closure_call)]
        {
            let __choice_res = {
                __err_state.suppress_fail += 1;
                let res = {
                    let __seq_res = {
                        let str_start = __pos;
                        match match ::peg::ParseElem::parse_elem(__input, __pos) {
                            ::peg::RuleResult::Matched(__next, __ch) => match __ch {
                                'a'..='z' | 'A'..='Z' | '_' => {
                                    let __pos = __next;
                                    {
                                        {
                                            let __seq_res = {
                                                let mut __repeat_pos = __pos;
                                                loop {
                                                    let __pos = __repeat_pos;
                                                    let __step_res =
                                                        match ::peg::ParseElem::parse_elem(
                                                            __input, __pos,
                                                        ) {
                                                            ::peg::RuleResult::Matched(
                                                                __next,
                                                                __ch,
                                                            ) => match __ch {
                                                                'a'..='z'
                                                                | 'A'..='Z'
                                                                | '0'..='9'
                                                                | '_' => {
                                                                    ::peg::RuleResult::Matched(
                                                                        __next,
                                                                        (),
                                                                    )
                                                                }
                                                                _ => {
                                                                    __err_state.mark_failure(__pos,
                                                                                                            "['a'..='z' | 'A'..='Z' | '0'..='9' | '_']");
                                                                    ::peg::RuleResult::Failed
                                                                }
                                                            },
                                                            ::peg::RuleResult::Failed => {
                                                                __err_state.mark_failure(__pos,
                                                                                                    "['a'..='z' | 'A'..='Z' | '0'..='9' | '_']");
                                                                ::peg::RuleResult::Failed
                                                            }
                                                        };
                                                    match __step_res {
                                                        ::peg::RuleResult::Matched(
                                                            __newpos,
                                                            __value,
                                                        ) => {
                                                            __repeat_pos = __newpos;
                                                        }
                                                        ::peg::RuleResult::Failed => {
                                                            break;
                                                        }
                                                    }
                                                }
                                                ::peg::RuleResult::Matched(__repeat_pos, ())
                                            };
                                            match __seq_res {
                                                ::peg::RuleResult::Matched(__pos, _) => {
                                                    ::peg::RuleResult::Matched(__pos, ())
                                                }
                                                ::peg::RuleResult::Failed => {
                                                    ::peg::RuleResult::Failed
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => {
                                    __err_state
                                        .mark_failure(__pos, "['a'..='z' | 'A'..='Z' | '_']");
                                    ::peg::RuleResult::Failed
                                }
                            },
                            ::peg::RuleResult::Failed => {
                                __err_state.mark_failure(__pos, "['a'..='z' | 'A'..='Z' | '_']");
                                ::peg::RuleResult::Failed
                            }
                        } {
                            ::peg::RuleResult::Matched(__newpos, _) => ::peg::RuleResult::Matched(
                                __newpos,
                                ::peg::ParseSlice::parse_slice(__input, str_start, __newpos),
                            ),
                            ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                        }
                    };
                    match __seq_res {
                        ::peg::RuleResult::Matched(__pos, n) => {
                            ::peg::RuleResult::Matched(__pos, (|| n.to_owned())())
                        }
                        ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                    }
                };
                __err_state.suppress_fail -= 1;
                res
            };
            match __choice_res {
                ::peg::RuleResult::Matched(__pos, __value) => {
                    ::peg::RuleResult::Matched(__pos, __value)
                }
                ::peg::RuleResult::Failed => {
                    __err_state.mark_failure(__pos, ("identifier"));
                    ::peg::RuleResult::Failed
                }
            }
        }
    }
    fn __parse_literal<'input>(
        __input: &'input Input,
        __state: &mut ParseState<'input>,
        __err_state: &mut ::peg::error::ErrorState,
        __pos: usize,
    ) -> ::peg::RuleResult<ExprAst> {
        #![allow(non_snake_case, unused, clippy::redundant_closure_call)]
        {
            let __choice_res = {
                let __seq_res = {
                    let str_start = __pos;
                    match {
                        let mut __repeat_pos = __pos;
                        let mut __repeat_value = vec![];
                        loop {
                            let __pos = __repeat_pos;
                            let __step_res = match ::peg::ParseElem::parse_elem(__input, __pos) {
                                ::peg::RuleResult::Matched(__next, __ch) => match __ch {
                                    '0'..='9' => ::peg::RuleResult::Matched(__next, ()),
                                    _ => {
                                        __err_state.mark_failure(__pos, "['0'..='9']");
                                        ::peg::RuleResult::Failed
                                    }
                                },
                                ::peg::RuleResult::Failed => {
                                    __err_state.mark_failure(__pos, "['0'..='9']");
                                    ::peg::RuleResult::Failed
                                }
                            };
                            match __step_res {
                                ::peg::RuleResult::Matched(__newpos, __value) => {
                                    __repeat_pos = __newpos;
                                    __repeat_value.push(__value);
                                }
                                ::peg::RuleResult::Failed => {
                                    break;
                                }
                            }
                        }
                        if __repeat_value.len() >= 1 {
                            ::peg::RuleResult::Matched(__repeat_pos, ())
                        } else {
                            ::peg::RuleResult::Failed
                        }
                    } {
                        ::peg::RuleResult::Matched(__newpos, _) => ::peg::RuleResult::Matched(
                            __newpos,
                            ::peg::ParseSlice::parse_slice(__input, str_start, __newpos),
                        ),
                        ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                    }
                };
                match __seq_res {
                    ::peg::RuleResult::Matched(__pos, n) => {
                        ::peg::RuleResult::Matched(__pos, (|| ExprAst::Literal(n.to_owned()))())
                    }
                    ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                }
            };
            match __choice_res {
                ::peg::RuleResult::Matched(__pos, __value) => {
                    ::peg::RuleResult::Matched(__pos, __value)
                }
                ::peg::RuleResult::Failed => {
                    match ::peg::ParseLiteral::parse_string_literal(__input, __pos, "&") {
                        ::peg::RuleResult::Matched(__pos, __val) => {
                            let __seq_res =
                                __parse_identifier(__input, __state, __err_state, __pos);
                            match __seq_res {
                                ::peg::RuleResult::Matched(__pos, i) => ::peg::RuleResult::Matched(
                                    __pos,
                                    (|| ExprAst::GlobalDataAddr(i))(),
                                ),
                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                            }
                        }
                        ::peg::RuleResult::Failed => {
                            __err_state.mark_failure(__pos, "\"&\"");
                            ::peg::RuleResult::Failed
                        }
                    }
                }
            }
        }
    }
    fn __parse__<'input>(
        __input: &'input Input,
        __state: &mut ParseState<'input>,
        __err_state: &mut ::peg::error::ErrorState,
        __pos: usize,
    ) -> ::peg::RuleResult<()> {
        #![allow(non_snake_case, unused, clippy::redundant_closure_call)]
        {
            __err_state.suppress_fail += 1;
            let res = {
                let mut __repeat_pos = __pos;
                loop {
                    let __pos = __repeat_pos;
                    let __step_res = match ::peg::ParseElem::parse_elem(__input, __pos) {
                        ::peg::RuleResult::Matched(__next, __ch) => match __ch {
                            ' ' => ::peg::RuleResult::Matched(__next, ()),
                            _ => {
                                __err_state.mark_failure(__pos, "[' ']");
                                ::peg::RuleResult::Failed
                            }
                        },
                        ::peg::RuleResult::Failed => {
                            __err_state.mark_failure(__pos, "[' ']");
                            ::peg::RuleResult::Failed
                        }
                    };
                    match __step_res {
                        ::peg::RuleResult::Matched(__newpos, __value) => {
                            __repeat_pos = __newpos;
                        }
                        ::peg::RuleResult::Failed => {
                            break;
                        }
                    }
                }
                ::peg::RuleResult::Matched(__repeat_pos, ())
            };
            __err_state.suppress_fail -= 1;
            res
        }
    }
}
