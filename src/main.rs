use std::io::Write;
use std::path::Path;
use std::{error::Error, fs::File};

use argsparse::SubCommand;
use pgn::PositionTable;

mod argsparse;
mod pgn;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = argsparse::Cli::new();

    if cli.subcommand == SubCommand::ConvertTo {
        let pt = PositionTable::from_pgn_database(
            cli.state.convert_to.as_ref().unwrap().input_file.clone(),
        );

        let output_file_path = cli.state.convert_to.as_ref().unwrap().outout_file.clone();

        let mut output_file = File::create(if output_file_path.is_none() {
            Path::new(&cli.state.convert_to.as_ref().unwrap().input_file)
                .with_extension("")
                .to_str()
                .unwrap()
                .to_owned()
        } else {
            output_file_path.unwrap()
        })?;

        writeln!(output_file, "{:?}", pt)?;
    }

    Ok(())
}
