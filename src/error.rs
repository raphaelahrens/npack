use std::io;
use std::path::Path;
use std::result::Result as StdResult;

use thiserror::Error as ThisError;

pub type Result<T> = StdResult<T, Error>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Io Error {0}")]
    Io(#[from] io::Error),
    #[error("Format")]
    Format,
    #[error("Git {0}")]
    Git(#[from] git2::Error),
    #[error("")]
    Editor,
    #[error("Fail to build plugin: {0}")]
    Build(String),
    #[error("Plugin is not installed")]
    PluginNotInstalled,
    #[error("NoPlugin")]
    NoPlugin,
    #[error("SkipLocal")]
    SkipLocal,
    #[error("{0}")]
    PluginInstalled(String),
    #[error("")]
    PackFile(String),
    #[error("Fail to copy directory: {0}")]
    CopyDir(#[from] std::path::StripPrefixError),
    #[error("Fail to copy directory: {0}")]
    WalkDir(#[from] walkdir::Error),
    #[error("" )]
    SaveYaml(#[from] yaml_rust::EmitError),
    #[error("")]
    LoadYaml(#[from] yaml_rust::ScanError),
    #[error("error executing as task")]
    TaskError(#[from] crate::task::TaskError),
}

impl Error {
    pub fn build<T: AsRef<str>>(s: T) -> Error {
        Error::Build(s.as_ref().to_string())
    }

    pub fn plugin_installed<T: AsRef<Path>>(s: T) -> Error {
        Error::PluginInstalled(format!("Plugin already installed under {:?}", s.as_ref()))
    }
}
