use reqwest::StatusCode;
use std::process;
use std::fmt;

#[derive(Debug)]
pub enum TinifyException {
  NoKeyProvided,
  NoFileOrDirectory,
  KeyException,
  ClientException,
  ServerException,
  ConnectionException,
  UnknownException(StatusCode),
}

impl fmt::Display for TinifyException {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      TinifyException::NoKeyProvided => {
        write!(
          fmt,
          "Provide an API key with set_key(key) method",
        )
      },
      TinifyException::NoFileOrDirectory => {
        write!(
          fmt,
          "No such file or directory.",
        )
      },
      TinifyException::KeyException => {
        write!(
          fmt,
          "There was a problem with your API key or with your API account.",
        )
      }
      TinifyException::ClientException => {
        write!(
          fmt,
          "The request could not be completed because of a problem with the submitted data.",
        )
      }
      TinifyException::ServerException => {
        write!(
          fmt,
          "The request could not be completed because of a temporary problem with the Tinify API.",
        )
      }
      TinifyException::ConnectionException => {
        write!(
          fmt,
          "The request could not be sent because there was an issue connecting to the Tinify API.",
        )
      }
      TinifyException::UnknownException(status) => {
        write!(
          fmt,
          "The request could not be completed because of a unknown problem. HTTP status code: {}",
          status
        )
      }
    }
  }
}

pub fn exit_error(
  type_exception: TinifyException,
  status_code: &StatusCode,
) {
  eprintln!(
    "{} status: {}",
    type_exception,
    &status_code,
  );
  process::exit(1);
}
