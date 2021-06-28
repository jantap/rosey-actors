pub use crate::{error, debug, info, warn};

#[macro_export]
macro_rules! error {
    ($($x:expr),*) => {
        log::error!("[{}] {}", LOGGING_MODULE, format!($($x),*));
    }
}

#[macro_export]
macro_rules! debug {
    ($($x:expr),*) => {
        log::debug!("[{}] {}", LOGGING_MODULE, format!($($x),*));
    }
}

#[macro_export]
macro_rules! info {
    ($($x:expr),*) => {
        log::info!("[{}] {}", LOGGING_MODULE, format!($($x),*));
    }
}

#[macro_export]
macro_rules! warn {
    ($($x:expr),*) => {
        log::warn!("[{}] {}", LOGGING_MODULE, format!($($x),*));
    }
}
