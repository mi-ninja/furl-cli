use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about=None, arg_required_else_help(true))]
pub struct FurlCliArgs {
    /// url to download the file from
    #[arg()]
    pub url: String,

    /// output directory, defaults to the current directory
    #[arg(short, long, default_value_t = String::from("."))]
    pub out: String,

    /// output filename, defaults to the filename in the url
    #[arg(short, long)]
    pub filename: Option<String>,

    /// Number of threads, defaults to 8, maximum allowed 255
    #[arg(short, long, default_value_t = 8)]
    pub threads: u8,
}
