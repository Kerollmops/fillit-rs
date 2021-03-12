use std::io::{self, Read};

use fillit::parse_tetriminos;

fn main() -> anyhow::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let tetriminos = parse_tetriminos(&buffer)?;
    eprintln!("{:#?}", tetriminos);

    Ok(())
}
