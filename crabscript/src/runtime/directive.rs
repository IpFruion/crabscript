use miette::SourceSpan;

use super::instruction::Instruction;

pub enum Directive {
    Label(Box<str>),
    Instruction {
        label: Option<Box<str>>,
        span: SourceSpan,
        instruction: Instruction,
        output: Option<Box<str>>,
    },
    Jump(Jump),
}

impl std::fmt::Debug for Directive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Instruction { .. } => f.debug_tuple("Instruction").finish(),
            _ => write!(f, "Unknown"),
        }
    }
}

pub struct Jump {
    jmp_type: JumpType,
    index: usize,
}

pub enum JumpType {
    Condition(Condition),
    Always,
}

pub struct Condition;
