use std::{any::Any, fmt::Debug};

use super::{ActorError, actor::ActorItem};

pub type BoxedMessage = Box<dyn Any + Send + Sync + 'static>;

pub trait Message: Any + Send + Sync + Debug {}
impl<T> Message for T where T: Any + Send + Sync + Debug {}

pub enum SystemMessage {
	RegisterActor(ActorItem),
	StopActor(&'static str),
	StartActor(&'static str),
	SendMsg(&'static str, BoxedMessage),
	RestartActor(&'static str),
	ActorTaskFinished(&'static str, Option<Result<(), ActorError>>),
	Shutdown,
	// Broadcast(BoxedMessage),
}

pub struct Shutdown {}
