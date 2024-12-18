use clap::Parser;

use crate::command::Arg;

#[derive(Parser)]
#[command(name = "set")]
pub struct Set {
    arg: Arg<Box<str>>,
}
