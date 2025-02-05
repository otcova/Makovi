use super::*;
use derive_more::derive::{Deref, Display};
use derive_more::From;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::{Index, Range};

#[derive(Clone)]
pub enum Instruction {
    Call(Call),

    Assign {
        variable: VariableId,
        value: Variable,
    },

    Return(Variable),

    /// Each `IfStart` must be closed with an `IfEnd`.
    /// However, multiple `IfStart`s can be closed with a single `IfEnd(<amount of if starts>)`
    IfStart(Variable, RunIf),

    /// An if-else statement must be represended as follow:
    /// [IfStart(_), ..<then>.., Else, ..<else>.., IfEnd]
    Else,

    /// Closes the scope of n ifs
    IfEnd(usize),

    /// Every `LoopStart` instruction must be followed with a `LoopEnd`.
    LoopStart,

    /// Every `LoopBreak` must be inside a `LoopStart` / `LoopEnd` block.
    Break,

    LoopEnd,
}

pub struct InstructionDisplay<'code, 'r> {
    instruction: &'r Instruction,
    definition: &'r FnDefinition<'code>,
}

impl Display for InstructionDisplay<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.instruction {
            Instruction::Call(call) => {
                if let Some(var) = call.result {
                    write!(f, "{var} = ")?;
                }

                let args = self.definition.get_arguments(call).iter().format(", ");
                write!(f, "{}({args})", call.fn_id)
            }
            Instruction::Assign { variable, value } => write!(f, "{variable} = {value}"),
            Instruction::Return(variable) => write!(f, "return {variable}"),
            Instruction::IfStart(variable, run_if) => write!(f, "if {variable} is {run_if}"),
            Instruction::Else => write!(f, "else"),
            Instruction::IfEnd(n) => write!(f, "endif x{n}"),
            Instruction::LoopStart => write!(f, "loop"),
            Instruction::Break => write!(f, "break"),
            Instruction::LoopEnd => write!(f, "endloop"),
        }
    }
}

impl Instruction {
    pub fn display<'code, 'r>(
        &'r self,
        definition: &'r FnDefinition<'code>,
    ) -> InstructionDisplay<'code, 'r> {
        InstructionDisplay {
            instruction: self,
            definition,
        }
    }
}

#[derive(Display, Clone, Copy)]
pub enum RunIf {
    #[display("false")]
    False,
    #[display("true")]
    True,
}

#[derive(Clone)]
pub struct Call {
    /// If is_some, the function result is stored in the variable
    pub result: Option<VariableId>,
    pub fn_id: FnId,

    // TODO: Bench: Store only the offset and use `FnDefinition.parameters` for length
    // to reduce hir::Instruction size. Compere it with reducing usize types to u32
    arguments: Range<usize>,

    /// Id inside the FnDefinition.
    pub call_id: usize,
}

#[derive(Debug, From, Clone, Copy, Eq, PartialEq, Hash, Display)]
pub enum FnId {
    Extern(ExternFnId),
    Local(FnDefinitionId),
}

#[derive(Debug, Deref, From, Clone, Copy, Eq, PartialEq, Hash, Display, PartialOrd, Ord)]
#[display("extern_f{_0}")]
pub struct ExternFnId(usize);

#[derive(Debug, Deref, Clone, Copy, Eq, PartialEq, Hash, Display, PartialOrd, Ord)]
#[display("local_f{_0}")]
pub struct FnDefinitionId(usize);

/// A collection of defined functions
#[derive(Default, Display)]
#[display("{}", definitions.iter().enumerate()
    .format_with("\n", |(id, def), f| f(&format_args!("f{id} = {def}"))))]
pub struct ModuleDefinitions<'code> {
    pub(super) definitions: Vec<FnDefinitionStage<'code>>,

    /// key: Name of a defined function
    pub(super) definitions_map: HashMap<&'code str, FnId>,
}

#[derive(Display)]
#[display(
r#"fn {name}({}){}
"#,
(0..*parameters).format_with(", ", |id, f| f(&format_args!("v{id}"))),
instructions.iter().format_with("", |inst, f| f(&format_args!("\n    {}", inst.display(self))))
)]
pub struct FnDefinition<'a> {
    pub name: &'a str,
    pub parameters: usize,

    pub instructions: Vec<Instruction>,
    pub variables_metadata: Vec<VariableMetadata>,

    /// Shared buffer to store the arguments of the function calls defined inside
    /// this definition.
    arguments: Vec<Variable>,

    /// Total number of function calls inside the FnDefinition.
    /// Each Call is assigned an id in the range 0..call_id.
    /// This ids are used by the FnInstance.
    calls: usize,
}

#[derive(Clone, Copy)]
pub struct VariableMetadata {
    /// Index of the instruction at which the variable is defined
    pub defined: usize,

    /// Index of the instruction where the variable is last used
    pub last_use: usize,
}

#[derive(Display)]
pub enum FnDefinitionStage<'code> {
    /// A function with name is yet to be defined
    #[display("fn {name}(...) <not defined>")]
    ToDefine {
        name: &'code str,
    },
    Defined(FnDefinition<'code>),
}

impl<'code> ModuleDefinitions<'code> {
    /// Returns error if a function with the same identifier is already defined.
    pub fn define(&mut self, def: FnDefinition<'code>) -> Result<FnDefinitionId, ()> {
        // We do not register unamed functions to the HashMap
        if def.name.is_empty() {
            let id = FnDefinitionId(self.definitions.len());
            self.definitions.push(FnDefinitionStage::Defined(def));
            return Ok(id);
        }

        match self.definitions_map.entry(def.name) {
            std::collections::hash_map::Entry::Vacant(vacant) => {
                let id = FnDefinitionId(self.definitions.len());
                self.definitions.push(FnDefinitionStage::Defined(def));
                vacant.insert(id.into());
                Ok(id)
            }
            std::collections::hash_map::Entry::Occupied(entry) => match *entry.get() {
                FnId::Extern(_) => Err(()),
                FnId::Local(fn_definition_id) => {
                    let stage = &mut self.definitions[*fn_definition_id];
                    if matches!(stage, FnDefinitionStage::ToDefine { .. }) {
                        *stage = FnDefinitionStage::Defined(def);
                        Ok(fn_definition_id)
                    } else {
                        Err(())
                    }
                }
            },
        }
    }

    /// If the provided 'name' it's not registered, a new FnId will be assigned and returned.
    pub fn get_fn_id(&mut self, name: &'code str) -> FnId {
        match self.definitions_map.entry(name) {
            std::collections::hash_map::Entry::Vacant(vacant) => {
                let id = FnDefinitionId(self.definitions.len()).into();
                vacant.insert(id);
                self.definitions.push(FnDefinitionStage::ToDefine { name });
                id
            }
            std::collections::hash_map::Entry::Occupied(entry) => *entry.get(),
        }
    }
}

impl<'code> Index<FnDefinitionId> for ModuleDefinitions<'code> {
    type Output = FnDefinitionStage<'code>;
    fn index(&self, index: FnDefinitionId) -> &Self::Output {
        &self.definitions[*index]
    }
}

impl<'a> FnDefinition<'a> {
    /// Returns the total number of variables.
    pub fn variables(&self) -> usize {
        self.variables_metadata.len()
    }

    pub fn get_arguments(&self, call: &Call) -> &[Variable] {
        &self.arguments[call.arguments.clone()]
    }

    pub fn new(name: &'a str, parameters: usize) -> Self {
        let param_metadata = VariableMetadata {
            defined: 0,
            last_use: 0,
        };

        FnDefinition {
            name,
            parameters,
            instructions: Vec::new(),
            variables_metadata: vec![param_metadata; parameters],
            arguments: Vec::new(),
            calls: 0,
        }
    }

    /// Creates a new variable with the `defined` and `last_use`
    /// pointing to next instruction to be pushed.
    pub fn new_variable(&mut self) -> VariableId {
        let var_id = self.variables_metadata.len().into();
        self.variables_metadata.push(VariableMetadata {
            defined: self.instructions.len(),
            last_use: self.instructions.len(),
        });
        var_id
    }

    pub fn push(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn push_fn_call<A: IntoIterator<Item = Variable>>(
        &mut self,
        definition_id: FnId,
        args: A,
        result: Option<VariableId>,
    ) {
        let start = self.arguments.len();
        self.arguments.extend(args);
        let end = self.arguments.len();

        let call_id = self.calls;
        self.calls += 1;

        self.push(Instruction::Call(Call {
            arguments: start..end,
            fn_id: definition_id,
            result,
            call_id,
        }));
    }

    /// Updates the variable `last_use` metadata to be the next instruction to be pushed.
    pub fn use_variable(&mut self, id: VariableId) {
        self.variables_metadata[*id].last_use = self.instructions.len();
    }
}
