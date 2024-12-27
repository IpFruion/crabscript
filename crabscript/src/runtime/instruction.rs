use crate::{
    command::{Command, CommandError, CommandOutput},
    context::Context,
    sdk::{echo::Echo, set::Set},
};

pub enum Instruction {
    Set(Set),
    Echo(Echo),
    Generic(Box<dyn Command<Output = CommandOutput>>),
}

impl Instruction {
    #[inline]
    pub fn run(self, ctx: &mut Context) -> Result<CommandOutput, CommandError> {
        match self {
            Instruction::Set(v) => v.run(ctx).map(Into::into),
            Instruction::Echo(v) => v.run(ctx).map(Into::into),
            Instruction::Generic(g) => g.run(ctx),
        }
    }

    #[cfg(test)]
    pub fn as_command<T: Command + 'static>(&self) -> Option<&T>
    where
        T::Output: Into<CommandOutput>,
    {
        match self {
            Instruction::Generic(g) => g
                .downcast_ref::<GenericInstruction<T>>()
                .map(|generic| &generic.0),
            _ => None,
        }
    }
}

impl From<Box<dyn Command<Output = CommandOutput>>> for Instruction {
    fn from(value: Box<dyn Command<Output = CommandOutput>>) -> Self {
        Instruction::Generic(value)
    }
}

impl From<Set> for Instruction {
    fn from(value: Set) -> Self {
        Instruction::Set(value)
    }
}

impl From<Echo> for Instruction {
    fn from(value: Echo) -> Self {
        Instruction::Echo(value)
    }
}

/// A generic instruction that uses [Into<CommandOutput>] for conversion of the [Command]
pub struct GenericInstruction<C>(C);

impl<T, C: Command<Output = T>> GenericInstruction<C> {
    pub fn new(cmd: C) -> Self {
        GenericInstruction(cmd)
    }
}

impl<T: Into<CommandOutput>, C: Command<Output = T>> Command for GenericInstruction<C> {
    type Output = CommandOutput;

    #[inline]
    fn run(&self, ctx: &mut Context) -> Result<Self::Output, CommandError> {
        Ok(self.0.run(ctx)?.into())
    }
}
