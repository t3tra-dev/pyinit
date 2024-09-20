use pyinit::{Result, PyInitArgs, PyInit};

fn main() -> Result<()> {
    let args: PyInitArgs = clap::Parser::parse();
    PyInit::from_interaction_or_args(args)?.run()?;
    Ok(())
}
