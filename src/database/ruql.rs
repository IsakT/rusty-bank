use crate::cqrs::event::*;
use std::collections::HashMap;
use crate::database;
use crate::database::event_schema::EventSchema;

pub fn setup() -> EventSchema{
    let db: EventSchema = database::event_schema::get_schema();

    // delete all events
    db.event_mut().delete_where(|_| true);

    let event1 = create_new_event("AccountHolder");
    let event2 = create_new_event("AccountHolder");
    let event3 = create_new_event("Message");
    let event4 = create_new_event("Message");
    let event5 = create_new_event("Transaction");
    let event6 = create_new_event("Transaction");
    let event7 = create_new_event("Session");
    let event8 = create_new_event("Session");
    

    // todo generate events from event.rs and insert
    // insert events
    db.event_mut().insert(event1);
    db.event_mut().insert(event2);
    db.event_mut().insert(event3);
    db.event_mut().insert(event4);
    db.event_mut().insert(event5);
    db.event_mut().insert(event6);
    db.event_mut().insert(event7);
    db.event_mut().insert(event8);

    db
}

fn create_new_event(aggregate_type: &str) -> Event {
    let metadata = HashMap::from([
        ("a".into(), "1".into()),
        ("b".into(), "2".into()),
        ("c".into(), "3".into()),
    ]);
    let deltas = HashMap::from([
        ("a".into(), "1".into()),
        ("b".into(), "2".into()),
        ("c".into(), "3".into()),
    ]);
    let aggregate_type = String::from(aggregate_type);

    let event = Event::new(metadata, deltas, aggregate_type);

    event
  }