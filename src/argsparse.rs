#![allow(private_interfaces, dead_code)]

use std::env::args;

use std::path::Path;
use std::process::exit;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Cli {
    pub subcommand: SubCommand,
    pub state: State,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct State {
    pub convert_to: Option<ConvertTo>,
}

#[derive(Debug, Copy, Clone, Default, Eq)]
#[repr(u8)]
pub enum SubCommand {
    ConvertTo,
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
    println!("Version: {}", version::version!());
    println!("Authors: datawater\n");

    if subcommand == None {
        println!("Usage:\n\tcaissa <command> [arguments]");

        println!("Avaliable commands:");
        println!("\thelp:        prints this help message");
        println!("\tconvert_to   converts the provided files to the caissa format. run `caissa help convert_to` for further help");
    } else {
        match subcommand.as_ref().unwrap().as_str() {
            "convert_to" | "ct" | "t" => {
                println!("Usage:\n\tcaissa convert_to [-o output_file] [input files]");
            }

            _ => {
                eprintln!(
                    "Incorrect subcommand {}. Run `caissa --help` for usage",
                    subcommand.unwrap()
                );
                exit(1);
            }
        }
    }
}

impl Cli {
    pub fn new() -> Self {
        let mut se1f: Cli = Default::default();
        let mut args = args();

        let mut i = 0;

        args.nth(0);
        while i < args.len() {
            let arg = args.nth(0).unwrap();

            if se1f.subcommand == SubCommand::None && !arg.starts_with("--") {
                match arg.as_str() {
                    "help" | "h" => {
                        usage(args.nth(0));
                        exit(0);
                    }

                    "convert_to" | "to" | "t" => {
                        se1f.subcommand = SubCommand::ConvertTo;
                        se1f.state.convert_to = Some(Default::default());
                    }

                    _ => {
                        if Path::new(&arg).exists() {
                            i += 1;
                            continue;
                        }

                        eprintln!(
                            "Unknown subcommand `{arg}` provided. Run `caissa --help` for help."
                        );
                        exit(1);
                    }
                }

                i += 1;
                continue;
            }

            if arg == "-h" || arg == "--help" {
                usage(args.nth(0));
                exit(0);
            }

            if arg == "-o" || arg == "--output" {
                if se1f.subcommand == SubCommand::ConvertTo {
                    if se1f.state.convert_to.is_some() {
                        eprintln!(
                            "Output file provided more than once. Run `caissa --help` for help."
                        );
                        exit(1);
                    }

                    se1f.state.convert_to.as_mut().unwrap().outout_file = match args.nth(i + 1) {
                        Some(filename) => {
                            if !Path::new(&filename).exists() {
                                eprintln!("The provided output file path ({filename}) doesn't exist. Run `caissa --help` for help.");
                                exit(1);
                            }

                            Some(filename)
                        }

                        None => {
                            eprintln!("No input file provided after the output flag. Run `caissa --help` for help.");
                            exit(1);
                        }
                    };
                } else {
                    eprintln!(
                        "Ouput file supplied when it's not needed. Run `caissa --help` for help."
                    );
                    exit(1);
                }

                i += 1;
                continue;
            }

            if Path::new(&arg).exists() {
                if se1f.subcommand == SubCommand::ConvertTo {
                    se1f.state
                        .convert_to
                        .as_mut()
                        .unwrap()
                        .input_files
                        .push(arg.clone());
                } else {
                    eprintln!(
                        "Input file supplied when it's not needed. Run `caissa --help` for help."
                    );
                    exit(1);
                }

                i += 1;
                continue;
            }

            i += 1;
        }

        se1f
    }
}
