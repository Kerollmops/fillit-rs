use std::io::{self, Read};

use fillit::parse_tetriminos;
use fillit::{Tetrimino::*, Position, VisualMap};

fn main() -> anyhow::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let tetriminos = parse_tetriminos(&buffer)?;
    eprintln!("{:#?}", tetriminos);

    let tetriminos = vec![
        (HorizontalBar, Position::new(0, 0)),
        (VerticalBar, Position::new(0, 1)),
    ];
    let map = VisualMap::new(tetriminos, 10);
    println!("{}", map);

    Ok(())
}
