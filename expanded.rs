#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod ast {

    use std::ops::Index;
    pub enum Expr {
        Literal(String),
        Identifier(String),
        Assign(String, ExprPtr),
        Eq(ExprPtr, ExprPtr),
        Ne(ExprPtr, ExprPtr),
        Lt(ExprPtr, ExprPtr),
        Le(ExprPtr, ExprPtr),
        Gt(ExprPtr, ExprPtr),
        Ge(ExprPtr, ExprPtr),
        Add(ExprPtr, ExprPtr),
        Sub(ExprPtr, ExprPtr),
        Mul(ExprPtr, ExprPtr),
        Div(ExprPtr, ExprPtr),
        Mod(ExprPtr, ExprPtr),
        IfElse(ExprPtr, Vec<Expr>, Vec<Expr>),
        WhileLoop(ExprPtr, Vec<Expr>),
        Call(String, Vec<Expr>),
        GlobalDataAddr(String),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Expr {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Expr::Literal(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Literal", &__self_0)
                }
                Expr::Identifier(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Identifier", &__self_0)
                }
                Expr::Assign(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(
                        f, "Assign", __self_0, &__self_1,
                    )
                }
                Expr::Eq(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(f, "Eq", __self_0, &__self_1)
                }
                Expr::Ne(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(f, "Ne", __self_0, &__self_1)
                }
                Expr::Lt(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(f, "Lt", __self_0, &__self_1)
                }
                Expr::Le(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(f, "Le", __self_0, &__self_1)
                }
                Expr::Gt(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(f, "Gt", __self_0, &__self_1)
                }
                Expr::Ge(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(f, "Ge", __self_0, &__self_1)
                }
                Expr::Add(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(f, "Add", __self_0, &__self_1)
                }
                Expr::Sub(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(f, "Sub", __self_0, &__self_1)
                }
                Expr::Mul(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(f, "Mul", __self_0, &__self_1)
                }
                Expr::Div(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(f, "Div", __self_0, &__self_1)
                }
                Expr::Mod(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(f, "Mod", __self_0, &__self_1)
                }
                Expr::IfElse(__self_0, __self_1, __self_2) => {
                    ::core::fmt::Formatter::debug_tuple_field3_finish(
                        f, "IfElse", __self_0, __self_1, &__self_2,
                    )
                }
                Expr::WhileLoop(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(
                        f,
                        "WhileLoop",
                        __self_0,
                        &__self_1,
                    )
                }
                Expr::Call(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(
                        f, "Call", __self_0, &__self_1,
                    )
                }
                Expr::GlobalDataAddr(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "GlobalDataAddr",
                        &__self_0,
                    )
                }
            }
        }
    }
    pub struct FunctionAST {
        pub name: String,
        pub params_names: Vec<String>,
        pub return_name: String,
        pub statements: Vec<Expr>,
    }
    type ExprPtr = Box<Expr>;
    pub struct Ast {
        data: Vec<Expr>,
    }
    impl Default for Ast {
        fn default() -> Self {
            Self {
                data: Vec::with_capacity(32),
            }
        }
    }
    impl Ast {
        pub fn push(&self, expr: Expr) -> ExprPtr {
            Box::new(expr)
        }
    }
}
pub mod ir {
    mod function_translator {
        use crate::ast::*;
        use cranelift::prelude::*;
        use cranelift_module::*;
        use std::collections::HashMap;
        /// A collection of state used for translating from toy-language AST nodes
        /// into Cranelift IR.
        pub struct FunctionTranslator<'a, M: Module> {
            pub int: types::Type,
            pub builder: FunctionBuilder<'a>,
            pub variables: HashMap<String, Variable>,
            pub module: &'a mut M,
        }
        impl<M: Module> FunctionTranslator<'_, M> {
            /// When you write out instructions in Cranelift, you get back `Value`s. You
            /// can then use these references in other instructions.
            pub fn translate_expr(&mut self, expr: Expr) -> Value {
                match expr {
                    Expr::Literal(literal) => {
                        let imm: i32 = literal.parse().unwrap();
                        self.builder.ins().iconst(self.int, i64::from(imm))
                    }
                    Expr::Add(lhs, rhs) => {
                        let lhs = self.translate_expr(*lhs);
                        let rhs = self.translate_expr(*rhs);
                        self.builder.ins().iadd(lhs, rhs)
                    }
                    Expr::Sub(lhs, rhs) => {
                        let lhs = self.translate_expr(*lhs);
                        let rhs = self.translate_expr(*rhs);
                        self.builder.ins().isub(lhs, rhs)
                    }
                    Expr::Mul(lhs, rhs) => {
                        let lhs = self.translate_expr(*lhs);
                        let rhs = self.translate_expr(*rhs);
                        self.builder.ins().imul(lhs, rhs)
                    }
                    Expr::Div(lhs, rhs) => {
                        let lhs = self.translate_expr(*lhs);
                        let rhs = self.translate_expr(*rhs);
                        self.builder.ins().udiv(lhs, rhs)
                    }
                    Expr::Mod(lhs, rhs) => {
                        let lhs = self.translate_expr(*lhs);
                        let rhs = self.translate_expr(*rhs);
                        self.builder.ins().urem(lhs, rhs)
                    }
                    Expr::Eq(lhs, rhs) => self.translate_icmp(IntCC::Equal, *lhs, *rhs),
                    Expr::Ne(lhs, rhs) => self.translate_icmp(IntCC::NotEqual, *lhs, *rhs),
                    Expr::Lt(lhs, rhs) => self.translate_icmp(IntCC::SignedLessThan, *lhs, *rhs),
                    Expr::Le(lhs, rhs) => {
                        self.translate_icmp(IntCC::SignedLessThanOrEqual, *lhs, *rhs)
                    }
                    Expr::Gt(lhs, rhs) => {
                        self.translate_icmp(IntCC::SignedGreaterThan, *lhs, *rhs)
                    }
                    Expr::Ge(lhs, rhs) => {
                        self.translate_icmp(IntCC::SignedGreaterThanOrEqual, *lhs, *rhs)
                    }
                    Expr::Call(name, args) => self.translate_call(name, args),
                    Expr::GlobalDataAddr(name) => self.translate_global_data_addr(name),
                    Expr::Identifier(name) => {
                        let variable = self.variables.get(&name).expect("variable not defined");
                        self.builder.use_var(*variable)
                    }
                    Expr::Assign(name, expr) => self.translate_assign(name, *expr),
                    Expr::IfElse(condition, then_body, else_body) => {
                        self.translate_if_else(*condition, then_body, else_body)
                    }
                    Expr::WhileLoop(condition, loop_body) => {
                        self.translate_while_loop(*condition, loop_body)
                    }
                }
            }
            fn translate_assign(&mut self, name: String, expr: Expr) -> Value {
                let new_value = self.translate_expr(expr);
                let variable = self.variables.get(&name).unwrap();
                self.builder.def_var(*variable, new_value);
                new_value
            }
            fn translate_icmp(&mut self, cmp: IntCC, lhs: Expr, rhs: Expr) -> Value {
                let lhs = self.translate_expr(lhs);
                let rhs = self.translate_expr(rhs);
                self.builder.ins().icmp(cmp, lhs, rhs)
            }
            fn translate_if_else(
                &mut self,
                condition: Expr,
                then_body: Vec<Expr>,
                else_body: Vec<Expr>,
            ) -> Value {
                let condition_value = self.translate_expr(condition);
                let then_block = self.builder.create_block();
                let else_block = self.builder.create_block();
                let merge_block = self.builder.create_block();
                self.builder.append_block_param(merge_block, self.int);
                self.builder
                    .ins()
                    .brif(condition_value, then_block, &[], else_block, &[]);
                self.builder.switch_to_block(then_block);
                self.builder.seal_block(then_block);
                let mut then_return = self.builder.ins().iconst(self.int, 0);
                for expr in then_body {
                    then_return = self.translate_expr(expr);
                }
                self.builder.ins().jump(merge_block, &[then_return]);
                self.builder.switch_to_block(else_block);
                self.builder.seal_block(else_block);
                let mut else_return = self.builder.ins().iconst(self.int, 0);
                for expr in else_body {
                    else_return = self.translate_expr(expr);
                }
                self.builder.ins().jump(merge_block, &[else_return]);
                self.builder.switch_to_block(merge_block);
                self.builder.seal_block(merge_block);
                let phi = self.builder.block_params(merge_block)[0];
                phi
            }
            fn translate_while_loop(
                &mut self,
                condition: Expr,
                loop_body: Vec<Expr>,
            ) -> Value {
                let header_block = self.builder.create_block();
                let body_block = self.builder.create_block();
                let exit_block = self.builder.create_block();
                self.builder.ins().jump(header_block, &[]);
                self.builder.switch_to_block(header_block);
                let condition_value = self.translate_expr(condition);
                self.builder
                    .ins()
                    .brif(condition_value, body_block, &[], exit_block, &[]);
                self.builder.switch_to_block(body_block);
                self.builder.seal_block(body_block);
                for expr in loop_body {
                    self.translate_expr(expr);
                }
                self.builder.ins().jump(header_block, &[]);
                self.builder.switch_to_block(exit_block);
                self.builder.seal_block(header_block);
                self.builder.seal_block(exit_block);
                self.builder.ins().iconst(self.int, 0)
            }
            fn translate_call(&mut self, name: String, args: Vec<Expr>) -> Value {
                let mut sig = self.module.make_signature();
                for _arg in &args {
                    sig.params.push(AbiParam::new(self.int));
                }
                sig.returns.push(AbiParam::new(self.int));
                let callee = self
                    .module
                    .declare_function(&name, Linkage::Import, &sig)
                    .expect("problem declaring function");
                let local_callee = self.module.declare_func_in_func(callee, self.builder.func);
                let mut arg_values = Vec::new();
                for arg in args {
                    arg_values.push(self.translate_expr(arg))
                }
                let call = self.builder.ins().call(local_callee, &arg_values);
                self.builder.inst_results(call)[0]
            }
            fn translate_global_data_addr(&mut self, name: String) -> Value {
                let sym = self
                    .module
                    .declare_data(&name, Linkage::Export, true, false)
                    .expect("problem declaring data object");
                let local_id = self.module.declare_data_in_func(sym, self.builder.func);
                let pointer = self.module.target_config().pointer_type();
                self.builder.ins().symbol_value(pointer, local_id)
            }
        }
    }
    use crate::ast::*;
    use codegen::Context;
    use cranelift::prelude::*;
    use cranelift_module::{FuncId, Linkage, Module};
    use function_translator::FunctionTranslator;
    use std::collections::HashMap;
    pub struct CodeIR<M: Module> {
        pub module: M,
        pub ctx: Context,
        pub builder_context: FunctionBuilderContext,
    }
    impl<M: Module> CodeIR<M> {
        pub fn load_function(&mut self, function: FunctionAST) -> Result<FuncId, String> {
            let int = types::I64;
            for _ in &function.params_names {
                self.ctx
                    .func
                    .signature
                    .params
                    .push(cranelift::prelude::AbiParam::new(int));
            }
            self.ctx.func.signature.returns.push(AbiParam::new(int));
            let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);
            let entry_block = builder.create_block();
            builder.append_block_params_for_function_params(entry_block);
            builder.switch_to_block(entry_block);
            builder.seal_block(entry_block);
            let variables = declare_variables(
                int,
                &mut builder,
                &function.params_names,
                &function.return_name,
                &function.statements,
                entry_block,
            );
            let mut trans = FunctionTranslator {
                int,
                builder,
                variables,
                module: &mut self.module,
            };
            for expr in function.statements {
                trans.translate_expr(expr);
            }
            let return_variable = trans.variables.get(&function.return_name).unwrap();
            let return_value = trans.builder.use_var(*return_variable);
            trans.builder.ins().return_(&[return_value]);
            trans.builder.finalize();
            let id = self
                .module
                .declare_function(&function.name, Linkage::Export, &self.ctx.func.signature)
                .map_err(|e| {
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(format_args!("Compilation error: {0}", e));
                        res
                    })
                })?;
            Ok(id)
        }
    }
    fn declare_variables(
        int: types::Type,
        builder: &mut FunctionBuilder,
        params: &[String],
        the_return: &str,
        stmts: &[Expr],
        entry_block: Block,
    ) -> HashMap<String, Variable> {
        let mut variables = HashMap::new();
        let mut index = 0;
        for (i, name) in params.iter().enumerate() {
            let val = builder.block_params(entry_block)[i];
            let var = declare_variable(int, builder, &mut variables, &mut index, name);
            builder.def_var(var, val);
        }
        let zero = builder.ins().iconst(int, 0);
        let return_variable =
            declare_variable(int, builder, &mut variables, &mut index, the_return);
        builder.def_var(return_variable, zero);
        for expr in stmts {
            declare_variables_in_stmt(int, builder, &mut variables, &mut index, expr);
        }
        variables
    }
    /// Recursively descend through the AST, translating all implicit
    /// variable declarations.
    fn declare_variables_in_stmt(
        int: types::Type,
        builder: &mut FunctionBuilder,
        variables: &mut HashMap<String, Variable>,
        index: &mut usize,
        expr: &Expr,
    ) {
        match *expr {
            Expr::Assign(ref name, _) => {
                declare_variable(int, builder, variables, index, name);
            }
            Expr::IfElse(ref _condition, ref then_body, ref else_body) => {
                for stmt in then_body {
                    declare_variables_in_stmt(int, builder, variables, index, stmt);
                }
                for stmt in else_body {
                    declare_variables_in_stmt(int, builder, variables, index, stmt);
                }
            }
            Expr::WhileLoop(ref _condition, ref loop_body) => {
                for stmt in loop_body {
                    declare_variables_in_stmt(int, builder, variables, index, stmt);
                }
            }
            _ => (),
        }
    }
    /// Declare a single variable declaration.
    fn declare_variable(
        int: types::Type,
        builder: &mut FunctionBuilder,
        variables: &mut HashMap<String, Variable>,
        index: &mut usize,
        name: &str,
    ) -> Variable {
        let var = Variable::new(*index);
        if !variables.contains_key(name) {
            variables.insert(name.into(), var);
            builder.declare_var(var, int);
            *index += 1;
        }
        var
    }
}
pub mod jit {
    use crate::ast::*;
    use crate::ir::*;
    use cranelift::prelude::*;
    use cranelift_jit::{JITBuilder, JITModule};
    use cranelift_module::{DataDescription, Linkage, Module};
    use std::slice;
    /// The basic JIT class.
    pub struct JIT {
        code: CodeIR<JITModule>,
        /// The data description, which is to data objects what `ctx` is to functions.
        data_description: DataDescription,
    }
    impl Default for JIT {
        fn default() -> Self {
            let mut flag_builder = settings::builder();
            flag_builder.set("use_colocated_libcalls", "false").unwrap();
            flag_builder.set("is_pic", "false").unwrap();
            let isa_builder = cranelift_native::builder().unwrap_or_else(|msg| {
                {
                    ::core::panicking::panic_fmt(format_args!(
                        "host machine is not supported: {0}",
                        msg
                    ));
                };
            });
            let isa = isa_builder
                .finish(settings::Flags::new(flag_builder))
                .unwrap();
            let builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());
            let module = JITModule::new(builder);
            let code = CodeIR {
                ctx: module.make_context(),
                builder_context: FunctionBuilderContext::new(),
                module,
            };
            Self {
                code,
                data_description: DataDescription::new(),
            }
        }
    }
    impl JIT {
        /// Compile a string in the toy language into machine code.
        pub fn compile_function(&mut self, function: FunctionAST) -> Result<*const u8, String> {
            let id = self.code.load_function(function)?;
            self.code
                .module
                .define_function(id, &mut self.code.ctx)
                .map_err(|e| {
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(format_args!("Compilation error: {0}", e));
                        res
                    })
                })?;
            self.code.module.clear_context(&mut self.code.ctx);
            self.code.module.finalize_definitions().unwrap();
            let code = self.code.module.get_finalized_function(id);
            Ok(code)
        }
        /// Create a zero-initialized data section.
        pub fn create_data(&mut self, name: &str, contents: Vec<u8>) -> Result<&[u8], String> {
            self.data_description.define(contents.into_boxed_slice());
            let id = self
                .code
                .module
                .declare_data(name, Linkage::Export, true, false)
                .map_err(|e| e.to_string())?;
            self.code
                .module
                .define_data(id, &self.data_description)
                .map_err(|e| e.to_string())?;
            self.data_description.clear();
            self.code.module.finalize_definitions().unwrap();
            let buffer = self.code.module.get_finalized_data(id);
            Ok(unsafe { slice::from_raw_parts(buffer.0, buffer.1) })
        }
    }
}
pub mod parser {
    mod peg_parser {
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
            ) -> ::core::result::Result<FunctionAST, ::peg::error::ParseError<PositionRepr>>
            {
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
                            match ::peg::ParseLiteral::parse_string_literal(
                                __input, __pos, "function",
                            ) {
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
                                            let __seq_res = __parse_identifier(
                                                __input,
                                                __state,
                                                __err_state,
                                                __pos,
                                            );
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
                                                                        match ::peg::ParseLiteral::parse_string_literal(__input,
                                                                                __pos, "(") {
                                                                            ::peg::RuleResult::Matched(__pos, __val) => {
                                                                                {
                                                                                    let __seq_res =
                                                                                        {
                                                                                            let mut __repeat_pos = __pos;
                                                                                            let mut __repeat_value = ::alloc::vec::Vec::new();
                                                                                            loop {
                                                                                                let __pos = __repeat_pos;
                                                                                                let __pos =
                                                                                                    if __repeat_value.is_empty() {
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
                                                                                                let __step_res =
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
                                                                                                match __step_res {
                                                                                                    ::peg::RuleResult::Matched(__newpos, __value) => {
                                                                                                        __repeat_pos = __newpos;
                                                                                                        __repeat_value.push(__value);
                                                                                                    }
                                                                                                    ::peg::RuleResult::Failed => { break; }
                                                                                                }
                                                                                            }
                                                                                            ::peg::RuleResult::Matched(__repeat_pos, __repeat_value)
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
                                                ::peg::RuleResult::Failed => {
                                                    ::peg::RuleResult::Failed
                                                }
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
            ) -> ::peg::RuleResult<Vec<Expr>> {
                #![allow(non_snake_case, unused, clippy::redundant_closure_call)]
                {
                    let __seq_res = {
                        let mut __repeat_pos = __pos;
                        let mut __repeat_value = ::alloc::vec::Vec::new();
                        loop {
                            let __pos = __repeat_pos;
                            let __step_res =
                                __parse_statement(__input, __state, __err_state, __pos);
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
                        ::peg::RuleResult::Matched(__pos, s) => {
                            ::peg::RuleResult::Matched(__pos, (|| s)())
                        }
                        ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                    }
                }
            }
            fn __parse_statement<'input>(
                __input: &'input Input,
                __state: &mut ParseState<'input>,
                __err_state: &mut ::peg::error::ErrorState,
                __pos: usize,
            ) -> ::peg::RuleResult<Expr> {
                #![allow(non_snake_case, unused, clippy::redundant_closure_call)]
                {
                    let __seq_res = match __parse__(__input, __state, __err_state, __pos) {
                        ::peg::RuleResult::Matched(pos, _) => ::peg::RuleResult::Matched(pos, ()),
                        ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                    };
                    match __seq_res {
                        ::peg::RuleResult::Matched(__pos, _) => {
                            let __seq_res =
                                __parse_expression(__input, __state, __err_state, __pos);
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
            ) -> ::peg::RuleResult<Expr> {
                #![allow(non_snake_case, unused, clippy::redundant_closure_call)]
                {
                    let __choice_res = __parse_if_else(__input, __state, __err_state, __pos);
                    match __choice_res {
                        ::peg::RuleResult::Matched(__pos, __value) => {
                            ::peg::RuleResult::Matched(__pos, __value)
                        }
                        ::peg::RuleResult::Failed => {
                            let __choice_res =
                                __parse_while_loop(__input, __state, __err_state, __pos);
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
            ) -> ::peg::RuleResult<Expr> {
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
                            &Fn(
                                usize,
                                i32,
                                &mut S,
                                &mut ::peg::error::ErrorState,
                            ) -> ::peg::RuleResult<T>,
                        ) -> ::peg::RuleResult<T>,
                        level_code: &Fn(
                            usize,
                            usize,
                            i32,
                            T,
                            &mut S,
                            &mut ::peg::error::ErrorState,
                            &Fn(
                                usize,
                                i32,
                                &mut S,
                                &mut ::peg::error::ErrorState,
                            ) -> ::peg::RuleResult<T>,
                        ) -> (T, ::peg::RuleResult<()>),
                    ) -> ::peg::RuleResult<T> {
                        let initial = {
                            prefix_atom(
                                lpos,
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
                                match ::peg::ParseLiteral::parse_string_literal(
                                    __input, __pos, "if",
                                ) {
                                    ::peg::RuleResult::Matched(__pos, __val) => {
                                        let __seq_res =
                                            match __parse__(__input, __state, __err_state, __pos) {
                                                ::peg::RuleResult::Matched(pos, _) => {
                                                    ::peg::RuleResult::Matched(pos, ())
                                                }
                                                ::peg::RuleResult::Failed => {
                                                    ::peg::RuleResult::Failed
                                                }
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
                                                                                                                                                                                                                                                                    { Expr::IfElse(Box::new(e), then_body, else_body) })())
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
                                                    ::peg::RuleResult::Failed => {
                                                        ::peg::RuleResult::Failed
                                                    }
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
                                match ::peg::ParseLiteral::parse_string_literal(
                                    __input, __pos, "if",
                                ) {
                                    ::peg::RuleResult::Matched(__pos, __val) => {
                                        let __seq_res =
                                            match __parse__(__input, __state, __err_state, __pos) {
                                                ::peg::RuleResult::Matched(pos, _) => {
                                                    ::peg::RuleResult::Matched(pos, ())
                                                }
                                                ::peg::RuleResult::Failed => {
                                                    ::peg::RuleResult::Failed
                                                }
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
                                                                                                                                                                                                                        Expr::IfElse(Box::new(e), then_body,
                                                                                                                                                                                                                            <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([else_body])))
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
                                                    ::peg::RuleResult::Failed => {
                                                        ::peg::RuleResult::Failed
                                                    }
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
                           __recurse| {
                            (__infix_result, ::peg::RuleResult::Failed)
                        }),
                    )
                }
            }
            fn __parse_while_loop<'input>(
                __input: &'input Input,
                __state: &mut ParseState<'input>,
                __err_state: &mut ::peg::error::ErrorState,
                __pos: usize,
            ) -> ::peg::RuleResult<Expr> {
                #![allow(non_snake_case, unused, clippy::redundant_closure_call)]
                match ::peg::ParseLiteral::parse_string_literal(__input, __pos, "while") {
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
                                    __parse_expression(__input, __state, __err_state, __pos);
                                match __seq_res {
                                    ::peg::RuleResult::Matched(__pos, e) => {
                                        let __seq_res =
                                            match __parse__(__input, __state, __err_state, __pos) {
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
                                                                                                                            (|| { Expr::WhileLoop(Box::new(e), loop_body) })())
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
            ) -> ::peg::RuleResult<Expr> {
                #![allow(non_snake_case, unused, clippy::redundant_closure_call)]
                {
                    let __seq_res = __parse_identifier(__input, __state, __err_state, __pos);
                    match __seq_res {
                        ::peg::RuleResult::Matched(__pos, i) => {
                            let __seq_res = match __parse__(__input, __state, __err_state, __pos) {
                                ::peg::RuleResult::Matched(pos, _) => {
                                    ::peg::RuleResult::Matched(pos, ())
                                }
                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                            };
                            match __seq_res {
                                ::peg::RuleResult::Matched(__pos, _) => {
                                    match ::peg::ParseLiteral::parse_string_literal(
                                        __input, __pos, "=",
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
                                                                (|| {
                                                                    Expr::Assign(i, Box::new(e))
                                                                })(
                                                                ),
                                                            )
                                                        }
                                                        ::peg::RuleResult::Failed => {
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
            ) -> ::peg::RuleResult<Expr> {
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
                            &Fn(
                                usize,
                                i32,
                                &mut S,
                                &mut ::peg::error::ErrorState,
                            ) -> ::peg::RuleResult<T>,
                        ) -> ::peg::RuleResult<T>,
                        level_code: &Fn(
                            usize,
                            usize,
                            i32,
                            T,
                            &mut S,
                            &mut ::peg::error::ErrorState,
                            &Fn(
                                usize,
                                i32,
                                &mut S,
                                &mut ::peg::error::ErrorState,
                            ) -> ::peg::RuleResult<T>,
                        ) -> (T, ::peg::RuleResult<()>),
                    ) -> ::peg::RuleResult<T> {
                        let initial = {
                            prefix_atom(
                                lpos,
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
                                let __seq_res =
                                    __parse_identifier(__input, __state, __err_state, __pos);
                                match __seq_res {
                                    ::peg::RuleResult::Matched(__pos, i) => {
                                        let __seq_res =
                                            match __parse__(__input, __state, __err_state, __pos) {
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
                                                    ::peg::RuleResult::Matched(__pos, __val) => {
                                                        let __seq_res = {
                                                            let mut __repeat_pos = __pos;
                                                            let mut __repeat_value =
                                                                ::alloc::vec::Vec::new();
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
                                                                                                                        __parse_expression(__input, __state, __err_state, __pos);
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
                                                                                                            }
                                                                                                            ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                                                                                        }
                                                                };
                                                                match __step_res {
                                                                    ::peg::RuleResult::Matched(
                                                                        __newpos,
                                                                        __value,
                                                                    ) => {
                                                                        __repeat_pos = __newpos;
                                                                        __repeat_value
                                                                            .push(__value);
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
                                                                                                        (|| { Expr::Call(i, args) })())
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
                                let __seq_res =
                                    __parse_identifier(__input, __state, __err_state, __pos);
                                match __seq_res {
                                    ::peg::RuleResult::Matched(__pos, i) => {
                                        ::peg::RuleResult::Matched(
                                            __pos,
                                            (|| Expr::Identifier(i))(),
                                        )
                                    }
                                    ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                }
                            } {
                                return ::peg::RuleResult::Matched(__pos, __v);
                            }
                            if let ::peg::RuleResult::Matched(__pos, __v) = {
                                let __seq_res =
                                    __parse_literal(__input, __state, __err_state, __pos);
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
                                                            if let ::peg::RuleResult::Matched(
                                                                __pos,
                                                                b,
                                                            ) = __recurse(
                                                                __pos,
                                                                0i32,
                                                                __state,
                                                                __err_state,
                                                            ) {
                                                                let a = __infix_result;
                                                                __infix_result = (|| {
                                                                    Expr::Eq(
                                                                        Box::new(a),
                                                                        Box::new(b),
                                                                    )
                                                                })(
                                                                );
                                                                ::peg::RuleResult::Matched(
                                                                    __pos,
                                                                    (),
                                                                )
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
                                                            if let ::peg::RuleResult::Matched(
                                                                __pos,
                                                                b,
                                                            ) = __recurse(
                                                                __pos,
                                                                0i32,
                                                                __state,
                                                                __err_state,
                                                            ) {
                                                                let a = __infix_result;
                                                                __infix_result = (|| {
                                                                    Expr::Ne(
                                                                        Box::new(a),
                                                                        Box::new(b),
                                                                    )
                                                                })(
                                                                );
                                                                ::peg::RuleResult::Matched(
                                                                    __pos,
                                                                    (),
                                                                )
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
                                                            if let ::peg::RuleResult::Matched(
                                                                __pos,
                                                                b,
                                                            ) = __recurse(
                                                                __pos,
                                                                0i32,
                                                                __state,
                                                                __err_state,
                                                            ) {
                                                                let a = __infix_result;
                                                                __infix_result = (|| {
                                                                    Expr::Lt(
                                                                        Box::new(a),
                                                                        Box::new(b),
                                                                    )
                                                                })(
                                                                );
                                                                ::peg::RuleResult::Matched(
                                                                    __pos,
                                                                    (),
                                                                )
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
                                                            if let ::peg::RuleResult::Matched(
                                                                __pos,
                                                                b,
                                                            ) = __recurse(
                                                                __pos,
                                                                0i32,
                                                                __state,
                                                                __err_state,
                                                            ) {
                                                                let a = __infix_result;
                                                                __infix_result = (|| {
                                                                    Expr::Le(
                                                                        Box::new(a),
                                                                        Box::new(b),
                                                                    )
                                                                })(
                                                                );
                                                                ::peg::RuleResult::Matched(
                                                                    __pos,
                                                                    (),
                                                                )
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
                                                            if let ::peg::RuleResult::Matched(
                                                                __pos,
                                                                b,
                                                            ) = __recurse(
                                                                __pos,
                                                                0i32,
                                                                __state,
                                                                __err_state,
                                                            ) {
                                                                let a = __infix_result;
                                                                __infix_result = (|| {
                                                                    Expr::Gt(
                                                                        Box::new(a),
                                                                        Box::new(b),
                                                                    )
                                                                })(
                                                                );
                                                                ::peg::RuleResult::Matched(
                                                                    __pos,
                                                                    (),
                                                                )
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
                                                            if let ::peg::RuleResult::Matched(
                                                                __pos,
                                                                b,
                                                            ) = __recurse(
                                                                __pos,
                                                                0i32,
                                                                __state,
                                                                __err_state,
                                                            ) {
                                                                let a = __infix_result;
                                                                __infix_result = (|| {
                                                                    Expr::Ge(
                                                                        Box::new(a),
                                                                        Box::new(b),
                                                                    )
                                                                })(
                                                                );
                                                                ::peg::RuleResult::Matched(
                                                                    __pos,
                                                                    (),
                                                                )
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
                                                            if let ::peg::RuleResult::Matched(
                                                                __pos,
                                                                b,
                                                            ) = __recurse(
                                                                __pos,
                                                                1i32,
                                                                __state,
                                                                __err_state,
                                                            ) {
                                                                let a = __infix_result;
                                                                __infix_result = (|| {
                                                                    Expr::Add(
                                                                        Box::new(a),
                                                                        Box::new(b),
                                                                    )
                                                                })(
                                                                );
                                                                ::peg::RuleResult::Matched(
                                                                    __pos,
                                                                    (),
                                                                )
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
                                                            if let ::peg::RuleResult::Matched(
                                                                __pos,
                                                                b,
                                                            ) = __recurse(
                                                                __pos,
                                                                1i32,
                                                                __state,
                                                                __err_state,
                                                            ) {
                                                                let a = __infix_result;
                                                                __infix_result = (|| {
                                                                    Expr::Sub(
                                                                        Box::new(a),
                                                                        Box::new(b),
                                                                    )
                                                                })(
                                                                );
                                                                ::peg::RuleResult::Matched(
                                                                    __pos,
                                                                    (),
                                                                )
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
                                                            if let ::peg::RuleResult::Matched(
                                                                __pos,
                                                                b,
                                                            ) = __recurse(
                                                                __pos,
                                                                2i32,
                                                                __state,
                                                                __err_state,
                                                            ) {
                                                                let a = __infix_result;
                                                                __infix_result = (|| {
                                                                    Expr::Mul(
                                                                        Box::new(a),
                                                                        Box::new(b),
                                                                    )
                                                                })(
                                                                );
                                                                ::peg::RuleResult::Matched(
                                                                    __pos,
                                                                    (),
                                                                )
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
                                                            if let ::peg::RuleResult::Matched(
                                                                __pos,
                                                                b,
                                                            ) = __recurse(
                                                                __pos,
                                                                2i32,
                                                                __state,
                                                                __err_state,
                                                            ) {
                                                                let a = __infix_result;
                                                                __infix_result = (|| {
                                                                    Expr::Div(
                                                                        Box::new(a),
                                                                        Box::new(b),
                                                                    )
                                                                })(
                                                                );
                                                                ::peg::RuleResult::Matched(
                                                                    __pos,
                                                                    (),
                                                                )
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
                                                                                        match ::peg::ParseLiteral::parse_string_literal(__input,
                                                                                                __pos, "(") {
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
                                                                                                                    __parse_expression(__input, __state, __err_state, __pos);
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
                                                                                                                                                (|| { Expr::Mod(Box::new(a), Box::new(b)) })();
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
                                                            if let ::peg::RuleResult::Matched(
                                                                __pos,
                                                                b,
                                                            ) = __recurse(
                                                                __pos,
                                                                3i32,
                                                                __state,
                                                                __err_state,
                                                            ) {
                                                                let a = __infix_result;
                                                                __infix_result = (|| {
                                                                    Expr::Mod(
                                                                        Box::new(a),
                                                                        Box::new(b),
                                                                    )
                                                                })(
                                                                );
                                                                ::peg::RuleResult::Matched(
                                                                    __pos,
                                                                    (),
                                                                )
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
                                                                                        match ::peg::ParseElem::parse_elem(__input, __pos) {
                                                                                            ::peg::RuleResult::Matched(__next, __ch) =>
                                                                                                match __ch {
                                                                                                    'a'..='z' | 'A'..='Z' | '0'..='9' | '_' =>
                                                                                                        ::peg::RuleResult::Matched(__next, ()),
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
                                            __err_state.mark_failure(
                                                __pos,
                                                "['a'..='z' | 'A'..='Z' | '_']",
                                            );
                                            ::peg::RuleResult::Failed
                                        }
                                    },
                                    ::peg::RuleResult::Failed => {
                                        __err_state
                                            .mark_failure(__pos, "['a'..='z' | 'A'..='Z' | '_']");
                                        ::peg::RuleResult::Failed
                                    }
                                } {
                                    ::peg::RuleResult::Matched(__newpos, _) => {
                                        ::peg::RuleResult::Matched(
                                            __newpos,
                                            ::peg::ParseSlice::parse_slice(
                                                __input, str_start, __newpos,
                                            ),
                                        )
                                    }
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
            ) -> ::peg::RuleResult<Expr> {
                #![allow(non_snake_case, unused, clippy::redundant_closure_call)]
                {
                    let __choice_res = {
                        let __seq_res = {
                            let str_start = __pos;
                            match {
                                let mut __repeat_pos = __pos;
                                let mut __repeat_value = ::alloc::vec::Vec::new();
                                loop {
                                    let __pos = __repeat_pos;
                                    let __step_res =
                                        match ::peg::ParseElem::parse_elem(__input, __pos) {
                                            ::peg::RuleResult::Matched(__next, __ch) => {
                                                match __ch {
                                                    '0'..='9' => {
                                                        ::peg::RuleResult::Matched(__next, ())
                                                    }
                                                    _ => {
                                                        __err_state
                                                            .mark_failure(__pos, "['0'..='9']");
                                                        ::peg::RuleResult::Failed
                                                    }
                                                }
                                            }
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
                                ::peg::RuleResult::Matched(__newpos, _) => {
                                    ::peg::RuleResult::Matched(
                                        __newpos,
                                        ::peg::ParseSlice::parse_slice(
                                            __input, str_start, __newpos,
                                        ),
                                    )
                                }
                                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                            }
                        };
                        match __seq_res {
                            ::peg::RuleResult::Matched(__pos, n) => ::peg::RuleResult::Matched(
                                __pos,
                                (|| Expr::Literal(n.to_owned()))(),
                            ),
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
                                        ::peg::RuleResult::Matched(__pos, i) => {
                                            ::peg::RuleResult::Matched(
                                                __pos,
                                                (|| Expr::GlobalDataAddr(i))(),
                                            )
                                        }
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
    }
    use crate::ast::*;
    impl FunctionAST {
        pub fn parse(code: &str) -> Result<Self, String> {
            peg_parser::parser::function(code).map_err(|e| {
                ::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(format_args!("Parsing error: {0}", e));
                    res
                })
            })
        }
    }
}
use ast::FunctionAST;
use jit::*;
pub struct MakoviJIT<In, Out> {
    jit: JIT,
    fn_ptr: fn(In) -> Out,
}
impl<In, Out> Default for MakoviJIT<In, Out> {
    fn default() -> Self {
        MakoviJIT {
            jit: JIT::default(),
            fn_ptr: |_| {
                ::core::panicking::panic_fmt(format_args!("Function not loaded!"));
            },
        }
    }
}
impl<In, Out> MakoviJIT<In, Out> {
    pub fn load_function(&mut self, code: &str) -> Result<(), String> {
        let function_ast = FunctionAST::parse(code)?;
        let ptr = self.jit.compile_function(function_ast)?;
        unsafe {
            self.fn_ptr = std::mem::transmute::<*const u8, fn(In) -> Out>(ptr);
        }
        Ok(())
    }
    pub fn run_code(&mut self, input: In) -> Out {
        (self.fn_ptr)(input)
    }
}
