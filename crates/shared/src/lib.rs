pub mod env;
pub mod error;
pub mod logging;

pub type Rlt<A> = Result<A, error::SharedErr>;
pub use error::{SharedErr, SharedErr::Custom};
