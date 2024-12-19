use std::env::args;

use crabscript::{
    context::Context,
    runtime::{source::SourceCode, Runtime},
};
use miette::{miette, IntoDiagnostic};

fn main() -> miette::Result<()> {
    let mut args = args().skip(1);
    let file = args.next().ok_or(miette!("missing crabscript file"))?;
    let contents = std::fs::read_to_string(&file).into_diagnostic()?;
    let src = SourceCode::named(&file, contents);
    let runtime = Runtime::parse(Context::init(), src)?;
    runtime.run()?;
    Ok(())
}
