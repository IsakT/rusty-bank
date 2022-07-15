/// Implementation of the Event struct.
/// 
/// Handles all the GUID, timestamps, aggregate_versions in the methods.
/// 
/// Example:
/// ```
///     let metadata = HashMap::from([("a".into(), "1".into())]);
///     let deltas = HashMap::from([("c".into(), "3".into())]);
///     let aggregate_type = String::from("AggregateType");
///     let event = Event::new(metadata, deltas, aggregate_type);
/// ```

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
    // Generates necessary values
    let utc = Utc::now().to_string();
    let guid = GUID::rand().to_string();
    let aggregate_version = 1;
    let event_name = "new";

    let event = new_event(guid, aggregate_version, event_name, utc, metadata, deltas, aggregate_type);

    event
  }
}

/// Constructs the event
fn new_event(aggregate_id: String,
  aggregate_version: u32,
  event_name: &str,
  timestamp: String, //DateTime<Utc>,
  metadata: HashMap<String, String>,
  deltas: HashMap<String, String>,
  aggregate_type: String) -> Event {
    Event{
      aggregate_id: aggregate_id,
      aggregate_version: aggregate_version,
      event_name: event_name.into(),
      timestamp: timestamp,
      metadata: metadata,
      deltas: deltas,
      aggregate_type: aggregate_type
    }
  }



// cargo test -- --nocapture
#[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn generate_new_event() {
          let event = test_data();
          let _event2 = test_data();
          let _event3 = test_data();

          println!("This is the first generated event{:?}", event);

          assert_eq!(event.timestamp.len(), 30);
          assert_eq!(event.aggregate_id.len(), 36);
          assert_eq!(event.event_name, "new");
          assert_eq!(event.metadata["a"], "1");
          assert_eq!(event.deltas["c"], "3");
          assert_eq!(event.aggregate_type, "AggregateType");
          assert_eq!(event.aggregate_version, 1);
        }

        // todo:
        // #[test]
        // fn generate_event_from_aggregate(){
        //   let event = Event::from_aggregate(aggregate, new_metadata, deltas, aggregate_type);
        // }

        fn test_data() -> Event {
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
          let aggregate_type = String::from("AggregateType");

          let event = Event::new(metadata, deltas, aggregate_type);

          println!("{:?}", event);

          event
        }
    }
