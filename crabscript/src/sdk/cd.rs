use std::path::PathBuf;

use clap::Parser;

use crate::{
    command::{Command, CommandError},
    context::Context,
};

#[derive(Parser)]
#[command(name = "cd")]
pub struct ChangeDirectory {
    dir: PathBuf,
}

impl Command for ChangeDirectory {
    type Output = ();

    fn run(&self, ctx: &mut Context) -> Result<Self::Output, CommandError> {
        ctx.cur_dir = Some(self.dir.clone());
        Ok(())
    }
}
