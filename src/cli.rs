use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use sdcx::constraints::Constraints;
use sdcx::errors::Report;
use sdcx::file_db::FileDb;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};

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
    Fmt(Fmt),

    /// Check input file
    Check(Check),

    /// Dump elements of input file
    Dump(Dump),
}

#[derive(Debug, Parser)]
struct Fmt {
    /// SDC file
    file: PathBuf,

    /// Output file
    #[arg(short, long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct Check {
    /// SDC file
    file: PathBuf,

    /// Force SDC version
    #[arg(long)]
    force_version: Option<String>,
}

#[derive(Debug, Parser)]
struct Dump {
    /// SDC file
    file: PathBuf,

    /// Show clock
    #[arg(long)]
    clock: bool,
}

// ---------------------------------------------------------------------------------------------------------------------
// Functions
// ---------------------------------------------------------------------------------------------------------------------

fn read_file(file: &Path) -> Result<String> {
    let file_name = file.display();
    let f = File::open(file).with_context(|| format!("could not open file: {file_name}"))?;
    let mut reader = BufReader::new(f);
    let mut s = String::new();

    if file.extension().map(|x| x.to_str()) == Some(Some("gz")) {
        let mut buf = vec![];
        reader
            .read_to_end(&mut buf)
            .with_context(|| format!("could not read file: {file_name}"))?;
        let mut gz = GzDecoder::new(&*buf);
        gz.read_to_string(&mut s)
            .with_context(|| format!("could not decode gzip file: {file_name}"))?;
    } else {
        reader
            .read_to_string(&mut s)
            .with_context(|| format!("could not read file: {file_name}"))?;
    }

    Ok(s)
}

fn write_file(file: &Path, text: &str) -> Result<()> {
    let file_name = file.display();
    let mut f = File::create(file).with_context(|| format!("could not open file: {file_name}"))?;

    if file.extension().map(|x| x.to_str()) == Some(Some("gz")) {
        let mut encoder = GzEncoder::new(&f, Compression::default());
        write!(encoder, "{}", text)
            .with_context(|| format!("could not write file: {file_name}"))?;
    } else {
        write!(f, "{}", text).with_context(|| format!("could not write file: {file_name}"))?;
    }

    f.flush()?;
    Ok(())
}

fn with_report<T, U: Report>(
    x: Result<T, U>,
    files: &FileDb<String, &str>,
    msg: &str,
) -> Result<T> {
    match x {
        Ok(x) => Ok(x),
        Err(x) => {
            x.report(files)?;
            bail!("{}", msg)
        }
    }
}

fn format(opt: &Fmt) -> Result<()> {
    let s = read_file(&opt.file)?;

    let mut files = FileDb::new();
    files.add(opt.file.display().to_string(), s.as_str());

    let mut sdc = with_report(
        sdcx::Parser::parse(&s, &opt.file),
        &files,
        &format!("could not parse file: {}", opt.file.display()),
    )?;
    sdc.normalize();

    if let Some(output) = &opt.output {
        write_file(output, &format!("{}", sdc))?;
    } else {
        println!("{}", sdc);
    }
    Ok(())
}

fn check(opt: &Check) -> Result<()> {
    let s = read_file(&opt.file)?;

    let mut files = FileDb::new();
    files.add(opt.file.display().to_string(), s.as_str());

    let mut version = None;
    if let Some(force_version) = &opt.force_version {
        if let Ok(x) = force_version.as_str().try_into() {
            version = Some(x);
        } else {
            bail!("Unknown version: {force_version}")
        };
    }

    let sdc = with_report(
        sdcx::Parser::parse(&s, &opt.file),
        &files,
        &format!("could not parse file: {}", opt.file.display()),
    )?;

    for err in sdc.validate(version) {
        err.report(&files)?;
    }

    Ok(())
}

fn dump(opt: &Dump) -> Result<()> {
    let s = read_file(&opt.file)?;

    let mut files = FileDb::new();
    files.add(opt.file.display().to_string(), s.as_str());

    let sdc = with_report(
        sdcx::Parser::parse(&s, &opt.file),
        &files,
        &format!("could not parse file: {}", opt.file.display()),
    )?;

    let mut constraints: Constraints = sdc.into();

    if opt.clock {
        let clocks = with_report(
            constraints.clocks(),
            &files,
            &format!("could not interpret file: {}", opt.file.display()),
        )?;
        for clock in clocks {
            println!("{clock}");
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
        SubCommands::Fmt(x) => format(&x)?,
        SubCommands::Check(x) => check(&x)?,
        SubCommands::Dump(x) => dump(&x)?,
    }

    Ok(())
}
