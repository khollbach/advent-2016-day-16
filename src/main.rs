use std::io::{self, BufRead};

use anyhow::{Result, bail};

fn main() -> Result<()> {
    let pattern = parse_input(io::stdin().lock())?;

    let ck = checksum(&fill_disk(&pattern, 272));
    println!("{}", pattern_to_string(&ck));

    let ck = checksum(&fill_disk(&pattern, 35_651_584));
    println!("{}", pattern_to_string(&ck));

    let pat = fill_disk(&pattern, 272);
    draw_lines(&pat, 10., "out1.png")?;

    let pat = fill_disk(&pattern, 35_651_584);
    draw_lines(&pat, 0.1, "out2.png")?;

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

use draw::draw_lines;

mod draw {
    use tiny_skia::{Paint, PathBuilder, Pixmap, Stroke, Transform};

    use anyhow::{Context, Result};

    pub fn draw_lines(pat: &[bool], line_len: f32, filename: &str) -> Result<()> {
        /*
        idea:
        - one step fwd
        - rotate (0 left 1 right)
        (repeat until done)
        */

        /*
        details:
        - initial (/current) pos
        - update & draw lines as you go
        - output final thing to png
        */

        let path = {
            let mut curr_point = (250., 250.);
            let mut dir = (0., 1.); // (Up?)

            let mut pb = PathBuilder::new();
            pb.move_to(curr_point.0, curr_point.1);

            for &rot in pat {
                curr_point.0 += line_len * dir.0;
                curr_point.1 += line_len * dir.1;
                pb.line_to(curr_point.0, curr_point.1);

                if rot {
                    dir = cw(dir);
                } else {
                    dir = ccw(dir)
                }
            }
            pb.finish().context("pathbuilder finish")?
        };

        let mut out = Pixmap::new(500, 500).context("pixmap new")?;
        out.stroke_path(
            &path,
            &Paint::default(),
            &Stroke::default(),
            Transform::identity(),
            None,
        );
        out.save_png(filename)?;
        Ok(())
    }

    fn cw((x, y): (f32, f32)) -> (f32, f32) {
        (-y, x)
    }

    fn ccw((x, y): (f32, f32)) -> (f32, f32) {
        (y, -x)
    }
}
