use std::io::{self, BufRead};

use anyhow::{Result, bail};

fn main() -> Result<()> {
    let pattern = parse_input(io::stdin().lock())?;

    let ck = checksum(&fill_disk(&pattern, 272));
    println!("{}", pattern_to_string(&ck));

    let ck = checksum(&fill_disk(&pattern, 35_651_584));
    println!("{}", pattern_to_string(&ck));

    Ok(())
}

/// e.g.:
/// 10000
fn parse_input(mut input: impl BufRead) -> Result<Vec<bool>> {
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;
    if buf.ends_with('\n') {
        buf.pop();
    }

    buf.chars()
        .map(|c| match c {
            '0' => Ok(false),
            '1' => Ok(true),
            _ => bail!("unknown char: {c:?}"),
        })
        .collect()
}

fn fill_disk(pattern: &[bool], len: usize) -> Vec<bool> {
    let mut pattern = pattern.to_vec();

    while pattern.len() < len {
        let mut suffix = pattern.clone();
        for bit in &mut suffix {
            *bit ^= true;
        }
        suffix.reverse();

        pattern.push(false);

        pattern.extend_from_slice(&suffix);
    }

    pattern.truncate(len);
    pattern
}

fn checksum(pattern: &[bool]) -> Vec<bool> {
    if pattern.is_empty() {
        return vec![];
    }

    let mut checksum = pattern.to_vec();
    while checksum.len() % 2 == 0 {
        checksum = checksum.chunks(2).map(|pair| pair[0] == pair[1]).collect();
    }
    checksum
}

fn pattern_to_string(pattern: &[bool]) -> String {
    pattern
        .iter()
        .map(|bit| if *bit { '1' } else { '0' })
        .collect()
}
