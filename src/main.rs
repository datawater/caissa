use std::{error::Error, fs::File};
use std::io::Write;

use argsparse::SubCommand;
use pgn::PositionTable;

mod argsparse;
mod pgn;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = argsparse::Cli::new();

    if cli.subcommand == SubCommand::ConvertTo {
        let mut pt: PositionTable = Default::default();

        pt.from_game(cli.state.convert_to.as_ref().unwrap().input_file.clone());
    
        let output_file_path = cli.state.convert_to.as_ref().unwrap().outout_file.clone();

        let mut output_file = File::create(if output_file_path == None {
            "output.txt".to_string() // TODO: Change this to use a proper file path.
        } else {
            output_file_path.unwrap()  
        })?;

        writeln!(output_file, "{:?}", pt)?;
    }

    Ok(())
}
