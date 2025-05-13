pub mod arg;
pub mod env;
pub mod error;
pub mod logging;
pub mod util;

pub type AppResult<A> = Result<A, error::AppError>;
pub use error::{AppError, AppError::Custom};
