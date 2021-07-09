use crate::regex_wrapper::RegexWrapper;
use crate::token_def::TokenDef;
use serde::Deserialize;
use std::io::BufReader;
use serde_yaml;

/// Deserialized config file
#[derive(Debug, Deserialize)]
pub struct TokenizerConfig {
    pub ignore: Vec<RegexWrapper>,
    pub tokens: Vec<TokenDef>,
}

impl TokenizerConfig {
	pub fn new() -> TokenizerConfig {
		return TokenizerConfig{ignore:vec![],tokens:vec![]};
	}

	pub fn from_file(file : std::fs::File) -> serde_yaml::Result<TokenizerConfig> {
		let reader = BufReader::new(file);
		serde_yaml::from_reader(reader)
	}

	pub fn with_ignore(&mut self, ignore : Vec<RegexWrapper>) -> &mut Self {
		self.ignore = ignore;
		return self;
	}

	pub fn with_tokens(&mut self, tokens: Vec<TokenDef>) -> &mut Self {
		self.tokens = tokens;
		return self;
	}

}