use crate::context::Variable;

#[derive(Debug)]
pub enum CommandOutput {
    None,
    VariableValue(Variable),
}

impl From<()> for CommandOutput {
    fn from(_value: ()) -> Self {
        CommandOutput::None
    }
}

impl From<String> for CommandOutput {
    fn from(value: String) -> Self {
        CommandOutput::VariableValue(Variable::String(value))
    }
}

impl From<bool> for CommandOutput {
    fn from(value: bool) -> Self {
        CommandOutput::VariableValue(Variable::Boolean(value))
    }
}
