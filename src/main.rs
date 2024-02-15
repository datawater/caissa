mod argsparse;
mod pgn;

fn main() -> std::io::Result<()> {
    let cli = argsparse::Cli::new();

    println!("{cli:?}");

    Ok(())
}
