use sdl2::video::WindowBuildError;
use sdl2::IntegerOrSdlError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Can't not read rom")]
    RomErr(#[from] std::io::Error),
    #[error("Opcode {0} error")]
    Opcode(String),
    #[error("SDLERROR: {0}")]
    SdlError(String),
    #[error("Windows Builder Error")]
    WindowBuildError(#[from] WindowBuildError),
    #[error("Canvas Builder Error")]
    CavansBuilderError(#[from] IntegerOrSdlError),
}

pub type Result<T> = std::result::Result<T, Error>;
