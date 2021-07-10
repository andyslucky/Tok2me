use clap::{AppSettings, Clap};
use libtok2me::token_config::TokenizerConfig;
use libtok2me::tokenizer::Tokenizer;
use std::error::Error as IError;
use std::fs::File;
use std::io::{stdin, Read};
use std::{result::Result, str::FromStr};

#[derive(Clap, Debug, Clone)]
#[clap(
    version = "1.0",
    author = "Andrew Strickland <andrewpstrickland@gmail.com>",
    about = "Maximal munch tokenizer. Reads a token definition document (required), and tokenizes an input file (optional) or stdin (default)."
)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(short, long, about = "YAML file providing token definitions")]
    token_file: FileWrapper,

    #[clap(
        short,
        long,
        about = "Input file to tokenize. If not provided STDIN will be used"
    )]
    input_file: Option<FileWrapper>,

    #[clap(short = 'n', long, about = "No comments in output")]
    disable_comments: bool,
}

impl Opts {
    pub fn get_input(&self) -> Result<Box<dyn Read>, std::io::Error> {
        return self
            .input_file
            .clone()
            .map_or(Ok(Box::new(stdin())), |f| Ok(Box::new(f.0.try_clone()?)));
    }

    pub fn get_token_file(&self) -> Result<File, std::io::Error> {
        return self.token_file.0.try_clone();
    }
}

#[derive(Debug)]
struct FileWrapper(File);

impl FromStr for FileWrapper {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f = File::open(s)?;
        Ok(FileWrapper(f))
    }
}

impl Clone for FileWrapper {
    fn clone(&self) -> Self {
        let f = self.0.try_clone().unwrap();
        return FileWrapper(f);
    }
}

fn write_comment(s: &str) {
    println!("# {}", s);
}


fn main() -> Result<(), Box<dyn IError>> {
    let opts = Opts::parse();
    let conf: TokenizerConfig = TokenizerConfig::from_file(opts.get_token_file()?)?;

    if !opts.disable_comments {
        write_comment("This is the tokenized output from tok2me.");
        write_comment("Lines beginning with '#' are comments and may be skipped!");
    }
    let mut tokenizer = Tokenizer::new(conf, opts.get_input()?);
    loop {
        let token = tokenizer.get_token()?;
        if let Some(tok) = token {
            let tok_type = tok.token_type.unwrap_or(String::from("None"));
            let mut val = tok.token_value.replace("\t", "\\t");
            val = val.replace("\r", "\\r");
            val = val.replace("\n", "\\n");
            println!("{}\t{}", tok_type, val);
        } else {
            break;
        }
    }
    Ok(())
}
