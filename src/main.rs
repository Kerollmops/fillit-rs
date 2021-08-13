use std::io::{self, Read};

use fillit::{find_best_fit, parse_tetriminos};

fn main() -> anyhow::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let tetriminos = parse_tetriminos(&buffer)?;
    let map = find_best_fit(&tetriminos);

    print!("{}", map);

    Ok(())
}
