use std::error::Error;

pub type ActorError = Box<dyn Error + Send + Sync + 'static>;
