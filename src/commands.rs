use crate::args::{Opts, SubCommand};
use clap::Clap;

pub fn parse_args() {
    let opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Encode(args) => {
            println!("encoding...");
            println!("\tchunk type:\t{}", args.chunk_type);
            println!("\twith message:\t{}", args.message);
            println!(
                "\tsource file:\t{}",
                args.file_path.to_str().expect("invalid file path")
            );
            if let Some(output_file) = args.output_file {
                println!("\toutput file:\t{}", output_file);
            } else {
                println!("\toutput file:\toutput.png");
            }
        }
        SubCommand::Decode(args) => {
            println!("decoding...");
            println!("\tchunk type:\t{}", args.chunk_type);
            println!(
                "\tsource file:\t{}",
                args.file_path.to_str().expect("invalid file path")
            );
        }
        SubCommand::Remove(args) => {
            println!("removing...");
            println!("\tchunk type:\t{}", args.chunk_type);
            println!(
                "\tsource file:\t{}",
                args.file_path.to_str().expect("invalid file path")
            );
        }
        SubCommand::Print(args) => {
            println!("printing...");
            println!(
                "\tsource file:\t{}",
                args.file_path.to_str().expect("invalid file path")
            );
        }
    }
}
