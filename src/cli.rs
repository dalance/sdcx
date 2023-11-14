use anyhow::Result;
use clap::{Parser, Subcommand};
use flate2::read::GzDecoder;
use sdcx;
use sdcx::sdc::sdc_error::FileDb;
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

    if file.extension().map(|x| x.to_str()) == Some(Some("gz")) {
        let mut buf = vec![];
        reader.read_to_end(&mut buf)?;
        let mut gz = GzDecoder::new(&*buf);
        gz.read_to_string(&mut s)?;
    } else {
        reader.read_to_string(&mut s)?;
    }

    let mut files = FileDb::new();
    files.add(file.display().to_string(), s.as_str());

    let sdc = sdcx::parser::Parser::parse(&s, &file);

    match sdc {
        Ok(mut sdc) => {
            sdc.normalize();
            println!("{}", sdc);
        }
        Err(err) => {
            err.report(&files)?;
        }
    }

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
