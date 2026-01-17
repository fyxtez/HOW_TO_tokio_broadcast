use central::event::{EventBus, EventTag};

pub async fn run(bus: EventBus) {
    let mut rx = bus.subscribe();

    while let Ok(event) = rx.recv().await {
        if matches!(event.tag, EventTag::Token) {
            println!("[SNIPER] {}", event.text);
        }
    }
}