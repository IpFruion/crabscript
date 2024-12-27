pub mod arg;
mod output;

use arg::ArgNotFoundErr;
use thiserror::Error;

use crate::context::Context;
pub use output::CommandOutput;

macro_rules! command_def {
    ($($other:path),*) => {
        pub trait Command: $($other +)* {
            type Output;

            fn run(&self, ctx: &mut Context) -> Result<Self::Output, CommandError>;
        }
    };
}

#[cfg(test)]
command_def!(downcast_rs::Downcast);
#[cfg(test)]
downcast_rs::impl_downcast!(Command assoc Output);

#[cfg(not(test))]
command_def!();

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
    pub(crate) fn with_short_hand<S: Into<Box<str>>>(mut self, s: S) -> Self {
        self.short_hand = Some(s.into());
        self
    }
}

impl From<std::io::Error> for CommandError {
    fn from(value: std::io::Error) -> Self {
        CommandError::new(value)
    }
}

impl From<ArgNotFoundErr> for CommandError {
    fn from(value: ArgNotFoundErr) -> Self {
        let arg = value.arg().to_string();
        CommandError::new(value).with_short_hand(arg)
    }
}
