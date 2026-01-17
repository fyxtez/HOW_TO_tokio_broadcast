use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub enum EventTag {
    Token,
    TradeSignal,
    Other,
}

#[derive(Debug, Clone)]
pub struct TgEvent {
    pub chat: String,
    pub text: String,
    pub tag: EventTag,
}

pub type EventBus = broadcast::Sender<TgEvent>;

pub fn new_event_bus() -> EventBus {
    let (tx, _) = broadcast::channel(1024);
    tx
}
