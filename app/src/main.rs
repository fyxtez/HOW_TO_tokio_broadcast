    use central::event::new_event_bus;

    #[tokio::main]
    async fn main() {
        let bus = new_event_bus();

        tokio::spawn(publisher::run(bus.clone()));
        tokio::spawn(listener1::run(bus.clone()));
        tokio::spawn(listener2::run(bus.clone()));

        futures::future::pending::<()>().await;
    }
