use anyhow::{Context, Result};
use std::{fs::File, io::BufRead};
use structopt::StructOpt;

/// Command Line arguments in one struct
#[derive(Debug, StructOpt)]
struct Cli {
    /// Pattern to look for
    pattern: String,

    /// Path to file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    // let content = std::fs::read_to_string(&args.path).expect("could not readfile");
    let file = File::open(&args.path)
        .with_context(|| format!("Could not find file `{}`", args.path.display()))?;
    let reader = std::io::BufReader::new(file);
    let lines = reader.lines().map(|l| l.expect("Error reading line"));
    grrs::find_matches(lines, &args.pattern, &mut std::io::stdout())
        .with_context(|| format!("Error reading file `{}`", args.path.display()))?;

    Ok(())
}
