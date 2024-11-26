use std::io::{BufWriter, Write};

use clap::Parser;

use crate::command::Command;

#[derive(Parser)]
#[command(name = "echo")]
pub struct Echo {
    args: Vec<String>,
}

impl Command for Echo {
    type Output = ();

    fn run(
        &self,
        _ctx: &mut crate::context::Context,
    ) -> Result<Self::Output, crate::command::CommandError> {
        let mut output = BufWriter::new(std::io::stdout());
        for arg in self.args.iter() {
            output.write_all(arg.as_bytes())?;
            output.write_all(b" ")?;
        }
        output.write_all(b"\n")?;
        output.flush()?;
        Ok(())
    }
}
