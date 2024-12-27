use clap::Parser;

use crate::command::{arg::Arg, Command};

#[derive(Parser)]
#[command(name = "set")]
pub struct Set {
    arg: Arg<String>,
}

impl Command for Set {
    type Output = String;

    fn run(
        &self,
        ctx: &mut crate::context::Context,
    ) -> Result<Self::Output, crate::command::CommandError> {
        Ok(self.arg.resolve(ctx)?.to_string())
    }
}
