use crate::token_def::*;
use crate::token_config::TokenizerConfig;
use std::io::{Error, Read};
use std::result::Result;

pub struct Tokenizer {
    ln: u32,
    col: u32,
    buff: String,
    current_state: Option<String>,
    config: Box<TokenizerConfig>,
    input_iter: Box<dyn Read>,
}

impl Tokenizer {
    // add code here
    pub fn new(config: TokenizerConfig, input_iter: Box<dyn Read>) -> Tokenizer {
        return Tokenizer {
            ln: 0,
            col: 0,
            buff: String::from(""),
            current_state: None,
            config : Box::new(config),
            input_iter,
        };
    }

    pub fn get_ln_col(&self) -> (u32, u32) {
        return (self.ln, self.col);
    }

    fn match_buff(&mut self) -> Option<String> {
        for expr in self.config.ignore.iter() {
            if expr.0.is_match(self.buff.as_str()) {
                self.buff.clear();
                return None;
            }
        }

        for token_def in self.config.tokens.iter() {
            for expr in &token_def.exprs {
                let buff_str = self.buff.as_str();
                if expr.0.is_match(buff_str) {
                    return Some(token_def.token_type.clone());
                }
            }
        }
        return None;
    }

    pub fn get_token(&mut self) -> Result<Option<Token>, Error> {
        let mut c = [0; 1];
        loop {
            // match buff here
            self.current_state = self.match_buff();
            let bytes_read = self.input_iter.read(&mut c)?;
            if bytes_read != 1 {
                break;
            }
            let char_read = c[0] as char;
            // push character onto buffer
            self.buff.push(char_read);

            if (char_read) == '\n' {
                self.ln += 1;
                self.col = 0;
            } else {
                self.col += 1;
            }

            // check if there is a transition to an accepting state
            let new_state = self.match_buff();

            if new_state.is_some() {
                self.current_state = new_state;
                continue;
            } else if self.current_state.is_some() {
                self.buff.pop();
                let tok = Token {
                    token_type: self.current_state.clone(),
                    token_value: self.buff.clone(),
                };
                self.current_state = None;
                self.buff = String::from(char_read);
                return Ok(Some(tok));
            }
        }

        if self.buff.len() > 0 {
            let tok = Token {
                token_type: self.match_buff(),
                token_value: self.buff.clone(),
            };
            self.buff = String::from("");
            self.current_state = None;
            return Ok(Some(tok));
        }

        return Ok(None);
    }
}