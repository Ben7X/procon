#[path = "src/args.rs"]
mod args;
#[path = "src/line.rs"]
mod line;
#[path = "src/node.rs"]
mod node;
#[path = "src/nodes.rs"]
mod nodes;
#[path = "src/property_file_reader.rs"]
mod property_file_reader;

use clap::CommandFactory;

fn main() -> std::io::Result<()> {
    let out_dir = std::path::PathBuf::from(
        std::env::var_os("OUT_DIR").ok_or_else(|| std::io::ErrorKind::NotFound)?,
    );
    let cmd = args::Args::command();

    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    std::fs::write(out_dir.join("head.1"), buffer)?;

    Ok(())
}
