/**
Implementation of the Event struct.

Handles all the GUID, timestamps, aggregate_versions in the methods.

Example:
```
    let metadata = HashMap::from([("a".into(), "1".into())]);
    let deltas = HashMap::from([("c".into(), "3".into())]);
    let aggregate_type = String::from("AggregateType");
    let event = Event::new(metadata, deltas, aggregate_type);
```
*/

extern crate guid_create;
use std::collections::HashMap;
use guid_create::GUID;
use chrono::prelude::*;
use rql::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
  pub aggregate_id: String,
  pub aggregate_version: u32,
  pub event_name: String,
  pub timestamp: String, // DateTime<Utc>,
  pub metadata: HashMap<String, String>,
  pub deltas: HashMap<String, String>,
  pub aggregate_type: String
}

impl Event {
  pub fn new(metadata: HashMap<String, String>, deltas: HashMap<String, String>, aggregate_type: String) -> Event {
    // timestamp as UTC to string
    let timestamp: String = Utc::now().to_string();
    // aggregate_id as a GUID
    let aggregate_id:String = GUID::rand().to_string();
    let aggregate_version: u32 = 1;
    let event_name: &str = "new";

    let event: Event = new_event(&aggregate_id, aggregate_version, event_name, timestamp, &metadata, deltas, &aggregate_type);

    event
  }
  /// Creates a new event based on the last event of the same aggregate in the event store.
  /// 'update' might be little misleading name, since a completely new event is generated.
  /// The name rather suggests an update of the aggregate, not of the event itself.
  pub fn update(&self, changes: HashMap<String, String>, metadata: HashMap<String, String>, event_name: &str) -> Event {
    // adds a new timestamp
    let timestamp: String = Utc::now().to_string();
    let aggregate_id: &String = &self.aggregate_id;
    // increments aggregate version, very important!
    let aggregate_version: u32 = &self.aggregate_version + 1;
    let metadata: HashMap<String, String> = metadata;
    let deltas: HashMap<String, String> = changes;
    let aggregate_type: &String = &self.aggregate_type;
    let event: Event = new_event(aggregate_id, aggregate_version, event_name, timestamp, &metadata, deltas, aggregate_type);

    event
  }
}

/// Constructs the event
fn new_event(aggregate_id: &String,
  aggregate_version: u32,
  event_name: &str,
  timestamp: String, //DateTime<Utc>,
  metadata: &HashMap<String, String>,
  deltas: HashMap<String, String>,
  aggregate_type: &String) -> Event {
    Event{
      aggregate_id: aggregate_id.clone(),
      aggregate_version: aggregate_version,
      event_name: event_name.into(),
      timestamp: timestamp,
      metadata: metadata.clone(),
      deltas: deltas,
      aggregate_type: aggregate_type.clone()
    }
  }



// cargo test -- --nocapture
#[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn generate_new_event() {
          let event: Event = create_new_event();
          let _event2: Event = create_new_event();
          let _event3: Event = create_new_event();

          println!("This is the first generated event{:?}", event);

          assert_eq!(event.timestamp.len(), 30);
          assert_eq!(event.aggregate_id.len(), 36);
          assert_eq!(event.event_name, "new");
          assert_eq!(event.metadata["a"], "1");
          assert_eq!(event.deltas["c"], "3");
          assert_eq!(event.aggregate_type, "AggregateType");
          assert_eq!(event.aggregate_version, 1);
        }

        #[test]
        fn generate_update_event() {
          let last_event: Event = create_new_event();
          let changes: HashMap<String, String> = 
            HashMap::from([("full_name".into(),"a new updated name".into())]);
          let metadata: HashMap<String, String> = HashMap::from([("a".into(), "1".into())]);
          let event_name: &str = "update";
          let updated_event: Event = last_event.update(changes, metadata, event_name);

          assert_eq!(updated_event.aggregate_version, 2u32);
          assert_eq!(updated_event.deltas["full_name"], "a new updated name");
          assert_eq!(updated_event.metadata["a"], "1");

          let new_changes: HashMap<String, String> = 
            HashMap::from([("full_name".into(),"a fried fire fox".into())]);
          
          let new_metadata: HashMap<String, String> = HashMap::from([("a".into(), "2".into())]);

          let updated_event_2: Event = updated_event.update(new_changes, new_metadata, event_name);

          assert_eq!(updated_event_2.aggregate_version, 3u32);
          assert_eq!(updated_event_2.deltas["full_name"], "a fried fire fox");
          assert_eq!(updated_event_2.metadata["a"], "2");
        }

        fn create_new_event() -> Event {
          let metadata: HashMap<String, String> = HashMap::from([
              ("a".into(), "1".into()),
              ("b".into(), "2".into()),
              ("c".into(), "3".into()),
          ]);
          let deltas: HashMap<String, String> = HashMap::from([
              ("a".into(), "1".into()),
              ("b".into(), "2".into()),
              ("c".into(), "3".into()),
          ]);
          let aggregate_type: String = String::from("AggregateType");

          let event: Event = Event::new(metadata, deltas, aggregate_type);

          println!("{:?}", event);

          event
        }
    }
