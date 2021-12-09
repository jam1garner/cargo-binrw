pub enum Error {
    BadIpAddr,
    NoPathFound,
    WriteIpDenied,
    NoCargoToml,
    BadCargoToml,
    NoTitleId,
    FailParseCargoStream,
    FailWriteNro,
    IoError(io::Error),
    CargoError(cargo_metadata::Error),
    ExitStatus(i32),
    GithubError,
    InvalidRepo,
    HostNotSupported,
    DownloadFailed,
    RustupNotFound,
    RustupLinkFailed,
}

pub type Result<T> = core::result::Result<T, Error>;

pub static BAD_IP_ADDR: &str = "\n\nCould not parse IP address: likely is not correctly formatted.";

impl From<cargo_metadata::Error> for Error {
    fn from(err: cargo_metadata::Error) -> Self {
        Self::CargoError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(_: reqwest::Error) -> Self {
        Self::DownloadFailed
    }
}
