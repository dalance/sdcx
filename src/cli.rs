use anyhow::Result;
use clap::{Parser, Subcommand};
use sdcx;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

// ---------------------------------------------------------------------------------------------------------------------
// Opt
// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Parser)]
struct Opt {
    #[clap(subcommand)]
    subcommand: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    /// Format input file
    Fmt {
        /// SDC file
        file: PathBuf,

        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Check input file
    Check {},
}

// ---------------------------------------------------------------------------------------------------------------------
// Functions
// ---------------------------------------------------------------------------------------------------------------------

fn format(opt: &Opt) -> Result<()> {
    let SubCommands::Fmt {
        ref file,
        output: _,
    } = opt.subcommand
    else {
        return Ok(());
    };

    let f = File::open(file)?;
    let mut reader = BufReader::new(f);
    let mut s = String::new();
    reader.read_to_string(&mut s)?;

    let mut sdc = sdcx::parser::Parser::parse(&s, &file)?;
    sdc.normalize();
    println!("{}", sdc);

    Ok(())
}

// ---------------------------------------------------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------------------------------------------------

fn main() -> Result<()> {
    let opt: Opt = Parser::parse();

    match opt.subcommand {
        SubCommands::Fmt { .. } => format(&opt)?,
        SubCommands::Check { .. } => todo!(),
    }

    Ok(())
}
