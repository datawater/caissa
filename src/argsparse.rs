#![allow(private_interfaces, dead_code)]

use std::env::args;

use std::process::exit;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Cli {
    pub subcommand: SubCommand,
}

#[derive(Debug, Clone, Default, Eq)]
#[repr(u8)]
pub enum SubCommand {
    ConvertTo(Option<ConvertTo>),
    #[default]
    None,
}

impl SubCommand {
    fn discriminant(&self) -> u8 {
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}

impl PartialEq for SubCommand {
    fn eq(&self, other: &Self) -> bool {
        self.discriminant() == other.discriminant()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct ConvertTo {
    pub input_files: Vec<String>,
    pub outout_file: Option<String>,
    pub compress_headers: bool,
}

fn usage(subcommand: Option<String>) {
    todo!();
}

impl Cli {
    pub fn new() -> Self {
        let mut se1f: Cli = Default::default();
        let mut args = args();

        let mut i = 0;
        while i < args.len() {
            let arg = args.nth(i).unwrap();

            if se1f.subcommand == SubCommand::None {
                match arg.as_str() {
                    "help" | "h" => {
                        usage(args.nth(i + 1));
                        exit(0);
                    },

                    "convert_to" | "to" | "t" => {
                        se1f.subcommand = SubCommand::ConvertTo(Default::default());
                    }
                
                    _ => {
                        eprintln!("Unknown subcommand `{arg}` provided. Run `caissa --help` for help.");
                        exit(1);
                    }
                }
            }

            if arg == "-h" || arg == "--help" {
                usage(None);
                exit(0);
            }

            if arg == "-o" || arg == "--output" {
                if se1f.subcommand != SubCommand::ConvertTo(None) {
                    eprintln!("Ouput file supplied when it's not needed. Run `caissa --help` for help.");
                    exit(1);
                }

                let SubCommand::ConvertTo(s) = &se1f.subcommand else {unreachable!()};
                let mut s = s.clone().unwrap();

                s.outout_file = match args.nth(i + 1) {
                    Some(filename) => {
                        if !Path::new(&filename).exists() {
                            eprintln!("The provided output file path ({filename}) doesn't exist. Run `caissa --help` for help.");    
                            exit(1);
                        }

                        Some(filename)
                    },

                    None => {
                       eprintln!("No input file provided after the output flag. Run `caissa --help` for help.");
                       exit(1); 
                    }
                };
            }

            i += 1;
        }

        se1f
    }
}
