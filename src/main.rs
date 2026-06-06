mod koala;

use crate::koala::Koala;
use anyhow::{Context as _, anyhow, bail};
use core::str::FromStr as _;
use std::env::args;
use std::io::{Read as _, Write as _, stdin, stdout};

fn main() -> anyhow::Result<()> {
    let args = args().collect::<Vec<String>>();

    let [_, col0, col1, col2, col3] = args.as_slice() else {
        bail!("Usage: {} <col0> <col1> <col2> <col3>", args[0]);
    };
    let colors = [
        parse_col(col0)?,
        parse_col(col1)?,
        parse_col(col2)?,
        parse_col(col3)?,
    ];

    let mut file = Vec::with_capacity(10003);
    stdin()
        .read_to_end(&mut file)
        .context("Failed to read from STDIN")?;

    let mut koala =
        Koala::new(&mut file).ok_or_else(|| anyhow!("Input file is not 10003 bytes long"))?;

    for (bitmap, screen, colram, bgcol) in koala.chars_mut() {
        fix_cols(bitmap, screen, colram, bgcol, colors)?;
    }

    koala.set_bgcol(colors[0]);

    stdout()
        .write_all(&file)
        .context("Failed to write to STDOUT")?;

    Ok(())
}

fn parse_col(col: &String) -> anyhow::Result<u8> {
    match u8::from_str(col.as_str()) {
        Ok(col) if col < 16 => Ok(col),
        _ => Err(anyhow!("Invalid color: {col}")),
    }
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
