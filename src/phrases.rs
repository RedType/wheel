use rand::prelude::*;
use std::{
  fs::File,
  io::{
    self,
    BufRead,
  },
  path::Path,
};

pub struct Phrase {
  pub category: String,
  pub text: String,
}

pub struct Topic(pub Vec<Phrase>);

pub enum ReadTopicError {
  IoError(io::Error),
  SyntaxError(usize),
}

impl From<io::Error> for ReadTopicError {
  fn from(value: io::Error) -> Self {
    Self::IoError(value)
  }
}

impl Topic {
  pub fn pick(&self) -> &Phrase {
    let mut rng = rand::rng();
    let i = rng.random_range(0..self.0.len());
    &self.0[i]
  }

  pub fn read<F: AsRef<Path>>(name: F) -> Result<Self, ReadTopicError> {
    let f = File::open(name)?;
    let reader = io::BufReader::new(f);
    let mut phrases = Vec::new();

    for (i, maybe_line) in reader.lines().enumerate() {
      let line = maybe_line?;
      let mut parts = line
        .split("--")
        .map(|s| s.trim().to_string());

      let Some(category) = parts.next() else {
        return Err(ReadTopicError::SyntaxError(i));
      };
      let Some(text) = parts.next() else {
        return Err(ReadTopicError::SyntaxError(i));
      };

      phrases.push(Phrase {
        category,
        text,
      });
    }

    Ok(Self(phrases))
  }
}
