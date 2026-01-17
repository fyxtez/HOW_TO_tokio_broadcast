use central::event::{EventBus, EventTag, TgEvent};
use tokio::time::{sleep, Duration};

pub async fn run(bus: EventBus) {
    let mut i = 0;

    loop {
        let tag = if i % 3 == 0 {
            EventTag::Token
        } else {
            EventTag::Other
        };

        let event = TgEvent {
            chat: "crypto_chat".into(),
            text: format!("message {}", i),
            tag,
        };

        let _ = bus.send(event);
        i += 1;

        sleep(Duration::from_millis(1000)).await;
    }
}
