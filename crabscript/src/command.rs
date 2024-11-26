use thiserror::Error;

use crate::context::Context;

pub trait Command {
    type Output;

    fn run(&self, ctx: &mut Context) -> Result<Self::Output, CommandError>;
}

#[derive(Debug)]
pub struct CommandOutput;

impl From<()> for CommandOutput {
    fn from(_value: ()) -> Self {
        CommandOutput
    }
}

#[derive(Error, Debug)]
#[error("{source}")]
pub struct CommandError {
    pub(crate) short_hand: Option<Box<str>>,
    #[source]
    source: Box<dyn std::error::Error + Send + Sync>,
}

impl CommandError {
    pub fn new<E: std::error::Error + Send + Sync + 'static>(err: E) -> Self {
        CommandError {
            source: Box::new(err),
            short_hand: None,
        }
    }
}

impl From<std::io::Error> for CommandError {
    fn from(value: std::io::Error) -> Self {
        CommandError::new(value)
    }
}
