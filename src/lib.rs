//! # Tinify Crate
//!
//! `tinify-rs` is a Rust client for the Tinify API.
//! Used for TinyPNG and TinyJPG. Tinify compresses your images intelligently.
//!
//! Read more at `https://tinify.com`
// --snip--

mod error;
mod client;
mod source;

pub use crate::client::Client;
pub use crate::source::Source;
pub use crate::error::TinifyException;

#[derive(Debug)]
pub struct Tinify {
  pub key: String,
}

pub enum ResizeMethod {
  Scale,
  Fit,
  Cover,
  Thumb,
}

impl Tinify {
  /// Create a new Tinify Object.
  ///
  /// # Examples
  ///
  /// ```
  /// use tinify::Tinify;
  /// 
  /// fn main() {
  ///   let tinify = Tinify::new();
  /// }
  /// ```
  pub fn new() -> Self {
    Self {
      key: String::new(),
    }
  }
  
  /// Set a Tinify Key.
  ///
  /// # Examples
  ///
  /// ```
  /// use tinify::Tinify;
  /// 
  /// fn main() {
  ///   let key = "tinify api key";
  ///   let tinify = Tinify::new().set_key(key);
  /// }
  /// ```
  pub fn set_key(mut self, key: &str) -> Self {
    self.key = key.to_string();
    self
  }

  /// Get a new Tinify Client.
  ///
  /// # Examples
  ///
  /// ```
  /// use tinify::{Tinify, TinifyException};
  /// 
  /// fn main() -> Result<(), :TinifyException> {
  ///   let key = "tinify api key";
  ///   let tinify = Tinify::new().set_key(key);
  ///   let client = tinify.get_client()?;
  ///   
  ///   Ok(())
  /// }
  /// ```
  pub fn get_client(&self) -> Result<Client, TinifyException> {
    if self.key.is_empty() {
      return Err(TinifyException::KeyException);
    }
    let client = Client::new(self.key.to_string());
  
    Ok(client)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use dotenv::dotenv;
  use std::env;

  #[test]
  #[should_panic(
    expected="Provide an API key with set_key(key) method"
  )]
  fn test_not_set_key() {
    let client = Tinify::new().get_client();
    if let Err(err) = client {
      panic!("{}", err.to_string());
    }
  }

  #[test]
  fn test_get_client() -> Result<(), TinifyException> {
    dotenv().ok();
    let key = match env::var("KEY") {
      Ok(key) => key,
      Err(_err) => panic!("No such file or directory."),
    };
    let _ = Tinify::new()
      .set_key(&key)
      .get_client()?;

    Ok(())
  }
}
