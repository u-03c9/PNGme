use clap::{AppSettings, Clap};
use std::path::PathBuf;

const CHUNK_TYPE_INFO: &str = "A valid chunk type\n\
    (check the PNG specs for more info)\n\
    http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html";

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
pub enum SubCommand {
    #[clap(about = "Encodes a message to a chunk type to a file")]
    Encode(EncodeArgs),

    #[clap(about = "Decodes a message from a chunk type in a file")]
    Decode(DecodeArgs),

    #[clap(about = "Removes a chunk type from a file")]
    Remove(RemoveArgs),

    #[clap(about = "Prints a file")]
    Print(PrintArgs),
}

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct EncodeArgs {
    #[clap(about = "A PNG file to encode the message into")]
    pub file_path: PathBuf,

    #[clap(about = CHUNK_TYPE_INFO)]
    pub chunk_type: String,

    #[clap(about = "The message you want to encode")]
    pub message: String,

    #[clap(about = "(Optional) Specifiy the output location")]
    pub output_file: Option<String>,
}

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct DecodeArgs {
    #[clap(about = "A PNG file to decode a message from")]
    pub file_path: PathBuf,

    #[clap(about = CHUNK_TYPE_INFO)]
    pub chunk_type: String,
}

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct RemoveArgs {
    #[clap(about = "A PNG file to remove a chunk type from")]
    pub file_path: PathBuf,
    #[clap(about = CHUNK_TYPE_INFO)]
    pub chunk_type: String,
}

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct PrintArgs {
    #[clap(about = "A PNG file to print")]
    pub file_path: PathBuf,
}
