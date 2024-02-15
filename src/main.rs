mod argsparse;
mod pgn;

use pgn::PositionTable;
use std::{fs::File, io::Write};

fn main() -> std::io::Result<()> {
    let mut table: PositionTable = PositionTable { games: vec![] };
    table.from_game(std::env::args().nth(1).unwrap());

    let mut file = File::create("log.txt")?;
    let string = format!("{:?}", table);
    drop(table);
    file.write_all(string.as_bytes())?;

    Ok(())
}
