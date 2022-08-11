use rql::prelude::*;
use rql::mashup;
use crate::cqrs::event::*;

schema! {
  pub EventSchema {
    event: Event,
  }
}

pub fn get_schema() -> EventSchema {
    let schema = EventSchema::new("test_database_example", HumanReadable).unwrap();

    return schema
}

// cargo test -- --nocapture
#[cfg(test)]
    mod tests {
        // use std::collections::HashMap;
        use crate::database;
        use super::*;

        // #[test]
        // #[serial_test::serial]
        // fn insert_an_event() {
        //     let db = EventSchema::new("test_database_example", HumanReadable).unwrap();
        //     let event = test_data();
        //     let event2 = test_data();
        //     let event3 = test_data();
        //     let event4 = test_data();
        //     let event5 = test_data();

        //     // delete all events
        //     db.event_mut().delete_where(|_| true);

        //     // insert event
        //     let res = db.event_mut().insert(event);
        //     db.event_mut().insert(event2);
        //     db.event_mut().insert(event3);
        //     db.event_mut().insert(event4);
        //     db.event_mut().insert(event5);

        //     println!("{}", res);

        //     assert_eq!(res.to_string().len(), 32);
        // }

        #[test]
        #[serial_test::serial]
        fn read_events() {
            let db = setup();
            let table = db.event();

            let event_names: Vec<String> =
                table
                    .wher(|event| event.event_name.contains("new") )
                    .select(|event| event.event_name.clone())
                    .collect();

            println!("event names: {:?}", event_names);

            let event_count = table.rows().count();
            // println!("event count: {:?}", event_count);

            let event_list: Vec<_> = table.rows().collect();
            // println!("row list: {:?}", event_list);

            assert_eq!(event_count, 8);
            assert_eq!(event_list.len(), 8);
            assert_eq!(event_names.len(), 8);
        }

        fn setup() -> EventSchema{
            database::ruql::setup()
        }

        // fn test_data() -> Event {
        //   let metadata = HashMap::from([
        //       ("a".into(), "1".into()),
        //       ("b".into(), "2".into()),
        //       ("c".into(), "3".into()),
        //   ]);
        //   let deltas = HashMap::from([
        //       ("a".into(), "1".into()),
        //       ("b".into(), "2".into()),
        //       ("c".into(), "3".into()),
        //   ]);
        //   let aggregate_type = String::from("AggregateType");

        //   let event = Event::new(metadata, deltas, aggregate_type);

        //   event
        // }

    }