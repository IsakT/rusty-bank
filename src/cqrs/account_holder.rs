/**
Implementation of the AccountHolder type events.

Generates 'create', 'read', 'update', and 'delete' type events for AccountHolder.

# Example:

Generate event for a new AccountHolder:
```
    let name = "Isak Törnros".into();
    let social_security_number = "19930625-7255".into();
    let date_of_birth = "1993-06-25".into();
    let phone_number = "0763-154177".into();
    let home_address = "Nöbbelövs Torg 37, 22652 LUND, Sweden".into();
    let event = create_new_account_holder(
        name,
        social_security_number,
        date_of_birth,
        phone_number,
        home_address,
    );
```

Generate event for updating an existing AccountHolder aggregate:
```


```
*/


// use rql::prelude::*;
// use rql::mashup;
use crate::cqrs::event::*;
use std::collections::HashMap;
use crate::database::event_schema::*;
use rql::repr::Representation::HumanReadable;

static AGGREGATE_TYPE: &str = "AccountHolder";

pub fn create_new_account_holder(
        name: &str,
        social_security_number: &str,
        date_of_birth: &str,
        phone_number: &str,
        home_address: &str,
        ) -> Event {
    let metadata = HashMap::from([]);
    let deltas = HashMap::from([
        ("full_name".into(), name.into()),
        ("social_security_number".into(), social_security_number.into()),
        ("date_of_birth".into(), date_of_birth.into()),
        ("phone_number".into(), phone_number.into()),
        ("home_address".into(), home_address.into()),
    ]);
    let aggregate_type = String::from(AGGREGATE_TYPE);

    let event = Event::new(metadata, deltas, aggregate_type);
        
    event
}

// todo: implement Aggregate struct
pub fn update_account_holder(aggregate: Aggregate, changes: HashMap<String, String>) -> Event {
    // access event db
    let schema = get_schema();
    let table = schema.event();

    // fetch latest aggregate event

    // call method 'update' on latest event
    let new_event = latest_event.update(changes, metadata, event_name);

    new_event
}
// for x in my_map.keys() {
//     println!("{}", x);
//     println!("{}", my_map.get(x).unwrap());
// }

#[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_create_new_account_holder() {
            let full_name = "Isak Törnros".into();
            let social_security_number = "19930625-7255".into();
            let date_of_birth = "1993-06-25".into();
            let phone_number = "0763-154177".into();
            let home_address = "Nöbbelövs Torg 37, 22652 LUND, Sweden".into();

            let event = create_new_account_holder(
                full_name,
                social_security_number,
                date_of_birth,
                phone_number,
                home_address,
            );

            println!("{:?}", event.event_name);
            println!("{:?}", event.deltas.get("full_name"));

            assert_eq!(event.event_name, "new");
            assert_eq!(event.aggregate_type, "AccountHolder");
            assert_eq!(event.deltas.get("full_name"), Some(&String::from(full_name)));
            assert_eq!(event.deltas.get("social_security_number"), Some(&String::from(social_security_number)));
            assert_eq!(event.deltas.get("date_of_birth"), Some(&String::from(date_of_birth)));
            assert_eq!(event.deltas.get("home_address"), Some(&String::from(home_address)));
        }

        fn test_update_account_holder(){
            let db = setup();
            let table = db.event();

            let aggregate_id = 
            let aggregate_id = String::from(aggregate_id);

            let changes = HashMap::from([
                ("full_name".into(), "Foobar Svensson".into())
            ]);
            let updated_event = update_account_holder(aggregate_id, changes);
        }

        fn test_delete_account_holder(){

        }

        fn test_get_account_holder(){

        }

        fn setup() -> EventSchema{
            let db = EventSchema::new("test_database_example", HumanReadable).unwrap();

            // delete all events
            db.event_mut().delete_where(|_| true);

            let event = create_new_event();
            let event2 = create_new_event();
            let event3 = create_new_event();
            let event4 = create_new_event();
            let event5 = create_new_event();
            

            // todo generate events from event.rs and insert
            // insert events
            db.event_mut().insert(event);
            db.event_mut().insert(event2);
            db.event_mut().insert(event3);
            db.event_mut().insert(event4);
            db.event_mut().insert(event5);

            db
        }

        fn create_new_event() -> Event {
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
  
            event
          }
    }