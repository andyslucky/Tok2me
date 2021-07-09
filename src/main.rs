use libtok2me::tokenizer::Tokenizer;
use libtok2me::token_config::TokenizerConfig;
use std::result::Result;
use std::fs::File;
use std::io::stdin;
use std::error::Error as IError;

fn main() -> Result<(), Box<dyn IError>> {
    let conf: TokenizerConfig = TokenizerConfig::from_file(File::open("C:/Users/USER/Documents/GitHub/tok/lex.yaml")?)?;
    let mut tokenizer = Tokenizer::new(Box::new(conf), Box::new(stdin()));
    loop {
        let token = tokenizer.get_token()?;
        if let Some(tok) = token {
            let tok_type = tok.token_type.unwrap_or(String::from("None"));
            if tok_type == "LF" {
                println!("{}\t\\n", tok_type);
            } else {
                println!("{}\t{}", tok_type, tok.token_value);
            }

            // println!("Tokenizer pos {:?}",tokenizer.get_ln_col());
        } else {
            break;
        }
    }
    Ok(())
}
