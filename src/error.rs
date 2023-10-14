use std::{error::Error as StdError, result::Result as StdResult};

pub type Error = Box<dyn StdError>;
pub type Result<T> = StdResult<T, Error>;
