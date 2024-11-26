use std::env::args;

use crabscript::{context::Context, runtime::Runtime};
use miette::{miette, IntoDiagnostic};

fn main() -> miette::Result<()> {
    let mut args = args().skip(1);
    let file = args.next().ok_or(miette!("missing crabscript file"))?;
    let contents = std::fs::read_to_string(&file).into_diagnostic()?;
    let runtime = Runtime::parse(Context::init(), &file, contents)?;
    runtime.run()?;
    Ok(())
}
