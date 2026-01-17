use central::event::EventBus;

pub async fn run(bus: EventBus) {
    let mut rx = bus.subscribe();

    while let Ok(event) = rx.recv().await {
        println!("[LOGGER] {:?}", event);
    }
}