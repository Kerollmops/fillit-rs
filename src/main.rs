use std::io::{self, Read};

use fillit::{parse_tetriminos, find_best_fit};

fn main() -> anyhow::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let tetriminos = parse_tetriminos(&buffer)?;

    match find_best_fit(&tetriminos) {
        Some(map) => print!("{}", map),
        None => eprintln!("Cannot construct a valid map"),
    }

    Ok(())
}
