pub mod prelude;
mod actor;
mod messages;
mod retry_strategy;
mod worker;
mod context;
mod logging;
mod error;

pub use actor::{Actor, ActorStatus, ActorType, ContinousActor, EventfulActor, BoxedEventfulResult};
pub use context::Ctx;
pub use error::ActorError;
pub use messages::BoxedMessage;
pub use retry_strategy::StrategyClosure;
use tokio::sync::mpsc::{
	unbounded_channel as channel, UnboundedReceiver as Receiver, UnboundedSender as Sender,
};
pub use worker::Worker;

static LOGGING_MODULE: &str = "ActorSystem";

pub fn new() -> (Ctx, Worker) {
	let (sx, rx) = channel();

	(sx.clone().into(), Worker::new(sx, rx))
}
