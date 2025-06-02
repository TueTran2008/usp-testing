use crate::usp_agent::uspa::UspError;
use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    // // -- Externals
    // #[from]
    // Io(std::io::Error), // as an example
    #[from]
    UspAgentError(UspError),
}

//Region: --Error Boilderplate
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
// end Region -- Error Boilderplat
