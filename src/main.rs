mod koala;

use crate::koala::Koala;
use anyhow::{Context as _, anyhow};
use clap::{Parser, Subcommand};
use core::str::FromStr as _;
use std::fs;
use std::io::{Read as _, Write as _, stdin, stdout};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The four colors, in order, which should be used.
    #[arg(short, long)]
    colors: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Converts file(s) and replaces them.
    Replace { files: Vec<PathBuf> },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let colors = cli
        .colors
        .split(',')
        .map(|color| parse_col(color.trim_matches(' ')))
        .collect::<Result<Vec<u8>, _>>()?;

    #[expect(
        clippy::map_err_ignore,
        reason = "an error only occurs when there are not exactly 4 coclors, this text explains the error better"
    )]
    let colors = colors
        .try_into()
        .map_err(|_| anyhow!("Exactly four colors are required (separated by comma)"))?;

    match cli.command {
        None => {
            let mut file = Vec::with_capacity(10003);
            stdin()
                .read_to_end(&mut file)
                .context("Failed to read from STDIN")?;

            adapt(&mut file, colors)?;

            stdout()
                .write_all(&file)
                .context("Failed to write to STDOUT")?;
        }
        Some(Commands::Replace { files }) => {
            for file_name in files {
                println!("Converting {}", file_name.display());

                let mut file = fs::read(&file_name)
                    .with_context(|| format!("Failed to read {}", file_name.display()))?;

                adapt(&mut file, colors)
                    .with_context(|| format!("Failed to adapt {}", file_name.display()))?;

                fs::write(&file_name, file)
                    .with_context(|| format!("Failed to write {}", file_name.display()))?;
            }
        }
    }

    Ok(())
}

fn parse_col(col: &str) -> anyhow::Result<u8> {
    match u8::from_str(col) {
        Ok(col) if col < 16 => Ok(col),
        _ => Err(anyhow!("Invalid color: {col}")),
    }
}

fn adapt(file: &mut [u8], colors: [u8; 4]) -> anyhow::Result<()> {
    let mut koala =
        Koala::new(file).ok_or_else(|| anyhow!("Input file is not 10003 bytes long"))?;

    for (bitmap, screen, colram, bgcol) in koala.chars_mut() {
        fix_cols(bitmap, screen, colram, bgcol, colors)?;
    }

    koala.set_bgcolor(colors[0]);

    Ok(())
}

fn fix_cols(
    bitmap: &mut [u8; 8],
    screen: &mut u8,
    colram: &mut u8,
    bgcol: u8,
    out: [u8; 4],
) -> anyhow::Result<()> {
    let inp = [bgcol & 0xf, *screen >> 4, *screen & 0xf, *colram & 0xf];

    for b in bitmap {
        let mut o = 0;
        for i in (0..8).step_by(2) {
            let col = inp[((*b >> i) & 3) as usize];
            o |= (out.iter().position(|c| *c == col).ok_or_else(|| {
                anyhow!("found color {col}, which is not part of the given colors")
            })? as u8)
                << i;
        }
        *b = o;
    }

    *screen = (out[1] << 4) | out[2];
    *colram = out[3];

    Ok(())
}
