pub mod client;
pub mod error;
pub mod schemas;

pub mod prelude {
    pub use crate::client::Client;
    pub use crate::schemas::*;
}
