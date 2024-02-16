#![allow(dead_code)]

use lz4_flex::block::compress_prepend_size;
use pgn_reader::{BufferedReader, RawHeader, SanPlus, Skip, Visitor};
use shakmaty::{fen::Fen, CastlingMode, Chess, Position};
use std::fs::File;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PgnHeaders {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PgnGame {
    pub pos: Vec<Chess>,
    pub headers: Vec<PgnHeaders>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PositionTable {
    pub games: Vec<PgnGame>,
}

impl PositionTable {
    fn new() -> PositionTable {
        PositionTable { games: vec![] }
    }
}

impl Visitor for PositionTable {
    type Result = ();

    fn begin_game(&mut self) {
        self.games.push(PgnGame {
            pos: vec![Chess::new()],
            headers: vec![],
        });
    }

    fn header(&mut self, key: &[u8], value: RawHeader<'_>) {
        if key == b"FEN" {
            let fen = match Fen::from_ascii(value.as_bytes()) {
                Ok(fen) => fen,
                Err(err) => {
                    eprintln!("invalid fen header in game: {} ({:?})", err, value);
                    return;
                }
            };

            *(self.games.last_mut().unwrap().pos.last_mut().unwrap()) =
                match fen.into_position(CastlingMode::Chess960) {
                    Ok(pos) => pos,
                    Err(err) => {
                        eprintln!("Illegal fen header: {} ({:?})", err, value);
                        return;
                    }
                }
        }

        self.games.last_mut().unwrap().headers.push(PgnHeaders {
            key: compress_prepend_size(key),
            value: compress_prepend_size(value.0),
        });
    }

    fn end_headers(&mut self) -> Skip {
        Skip(false)
    }

    fn begin_variation(&mut self) -> Skip {
        eprintln!("Variations currently not supported.");
        Skip(false)
    }

    fn san(&mut self, san_plus: SanPlus) {
        let last_game = self.games.last_mut().unwrap();
        let san = san_plus.san.to_move(last_game.pos.last().unwrap()).unwrap();

        last_game.pos.push(last_game.pos.last().unwrap().clone());
        last_game.pos.last_mut().unwrap().play_unchecked(&san)
    }

    fn end_game(&mut self) -> Self::Result {}
}

impl PositionTable {
    pub fn from_pgn_database(game: String) -> Self {
        let mut se1f = Default::default();

        let file = File::open(game).unwrap();
        let mut reader = BufferedReader::new(Box::new(file));

        reader.read_all(&mut se1f).unwrap();
        se1f
    }
}
