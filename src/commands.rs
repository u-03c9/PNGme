use crate::{
    args::{DecodeArgs, EncodeArgs, Opts, PrintArgs, RemoveArgs, SubCommand},
    chunk::Chunk,
    chunk_type::ChunkType,
    png::Png,
    Error, Result,
};
use clap::Clap;
use std::{convert::TryFrom, fs, path::PathBuf, str::FromStr};

pub fn parse_args() -> Result<()> {
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
            if let Some(output_file) = args.output_file.clone() {
                println!("\toutput file:\t{}", output_file);
            } else {
                println!("\toutput file:\t{}", args.file_path.to_str().unwrap());
            }
            encode_chunk_type(args)?;
        }
        SubCommand::Decode(args) => {
            println!("decoding...");
            println!("\tchunk type:\t{}", args.chunk_type);
            println!(
                "\tsource file:\t{}",
                args.file_path.to_str().expect("invalid file path")
            );
            decode_chunk_type(args)?;
        }
        SubCommand::Remove(args) => {
            println!("removing...");
            println!("\tchunk type:\t{}", args.chunk_type);
            println!(
                "\tsource file:\t{}",
                args.file_path.to_str().expect("invalid file path")
            );
            remove_chunk_type(args)?;
        }
        SubCommand::Print(args) => {
            println!("printing...");
            println!(
                "\tsource file:\t{}",
                args.file_path.to_str().expect("invalid file path")
            );
            print_png(args)?;
        }
    }

    println!("done!");
    Ok(())
}

fn encode_chunk_type(args: EncodeArgs) -> Result<()> {
    let mut png = png_from_file_path(args.file_path.clone())?;
    let chunk_type = ChunkType::from_str(args.chunk_type.as_str())?;
    let chunk = Chunk::new(chunk_type, args.message.as_bytes().to_vec());
    png.append_chunk(chunk);

    if let Some(output_path) = args.output_file {
        fs::write(output_path, png.as_bytes())?;
    } else {
        fs::write(args.file_path, png.as_bytes())?;
    }
    Ok(())
}

fn decode_chunk_type(args: DecodeArgs) -> Result<()> {
    let png = png_from_file_path(args.file_path.clone())?;
    let chunk_type = ChunkType::from_str(args.chunk_type.as_str())?;
    if let Some(chunk) = png.chunk_by_type(chunk_type.to_string().as_str()) {
        let decoded_message = chunk.data_as_string()?;
        println!("decoded message: {}", decoded_message);
        Ok(())
    } else {
        Err(Error::from(format!(
            "Couldn't find the chunk type '{}' in the file {:?}",
            chunk_type, args.file_path
        )))
    }
}

fn remove_chunk_type(args: RemoveArgs) -> Result<()> {
    let mut png = png_from_file_path(args.file_path.clone())?;
    let chunk_type = ChunkType::from_str(args.chunk_type.as_str())?;
    let chunk = png.remove_chunk(chunk_type.to_string().as_str())?;

    fs::write(args.file_path, png.as_bytes())?;

    println!("{} has been removed", chunk);
    Ok(())
}

fn print_png(args: PrintArgs) -> Result<()> {
    let png = png_from_file_path(args.file_path)?;
    println!("{}", png);

    Ok(())
}

fn png_from_file_path(file_path: PathBuf) -> Result<Png> {
    if !file_path.exists() {
        return Err(Error::from("Couldn't find file!"));
    }
    let file_bytes = fs::read(file_path)?;
    Png::try_from(file_bytes.as_ref())
}
