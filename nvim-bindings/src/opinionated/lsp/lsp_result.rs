use std::{error, fmt};

use mlua::prelude::LuaError;
use tokio::sync::oneshot::error::RecvError;

use super::protocol::ResponseError;

pub type LspResult<T> = std::result::Result<T, Error>;

/// Any error that can occur when interacting with a Neovim Lsp client (see `:h
/// vim.lsp.client`).
#[derive(Debug)]
pub enum Error {
    /// A call to the `request` function of a client returned `false`. This
    /// means the client has shutdown and all successive calls will return
    /// `false` as well. See `:h vim.lsp.client` for details.
    ClientShutdown,

    /// A request or notification sent to the Lsp server returned an error.
    ResponseError(ResponseError),

    /// `.await`ing the receiver of the `tokio::sync::oneshot` channel
    /// returned an error.
    ReceiverError(RecvError),

    /// .
    // NoResultOrError,

    /// A catchall for `mlua` errors.
    Lua(LuaError),
}

impl From<ResponseError> for Error {
    fn from(err: ResponseError) -> Self {
        Self::ResponseError(err)
    }
}

impl From<RecvError> for Error {
    fn from(err: RecvError) -> Self {
        Self::ReceiverError(err)
    }
}

impl From<LuaError> for Error {
    fn from(err: LuaError) -> Self {
        Self::Lua(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        match self {
            ClientShutdown => write!(f, "An Lsp client shut down"),

            ResponseError(err) => write!(
                f,
                "An Lsp server returned an error with code `{:?}` and \
                 message: '{}'",
                err.code, err.message
            ),

            ReceiverError(err) => write!(f, "{}", err),

            Lua(err) => write!(f, "{}", err),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;

        match self {
            ClientShutdown | ResponseError(_) => None,
            ReceiverError(err) => err.source(),
            Lua(err) => err.source(),
        }
    }
}
