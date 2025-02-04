use super::*;

#[derive(Default)]
pub struct FunctionScope<'a> {
    // TODO: Bench and test other data structures.
    // Reasons to use a linear search in Vec instead of a HashMap:
    // - Usually there are less than 20 variables,
    // a linear search could be faster than a HashMap lookup.
    // - Maintaining such HashMap would be more expencive.
    // - Usually, variables in are used more in the defined scope,
    // this means that we could expect to find the variable quicker with rfind.
    variables: Vec<VariableScope<'a>>,
}

struct VariableScope<'a> {
    name: &'a str,
    id: VariableId,
    nesting: usize,
}

impl<'a> FunctionScope<'a> {
    pub fn define_variable(&mut self, nesting: usize, name: &'a str, id: VariableId) {
        self.variables.push(VariableScope { name, id, nesting });
    }

    fn get_variable(&mut self, name: &str) -> Option<VariableId> {
        // TODO: Bench without `rev`
        self.variables
            .iter()
            .rev()
            .find(|var| var.name == name)
            .map(|var| var.id)
    }

    /// It will close the scope of all the variables of a nesting higher than
    /// the provided `nesting`
    pub fn set_nesting(&mut self, nesting: usize) {
        while let Some(var) = self.variables.last() {
            if var.nesting <= nesting {
                break;
            }
            self.variables.pop();
        }
    }
}

impl FunctionParser<'_, '_> {
    pub fn get_variable(&mut self, name: &str) -> Option<VariableId> {
        let variable = self.scope.get_variable(name);
        if let Some(id) = variable {
            self.function.use_variable(id);
        }
        variable
    }
}
