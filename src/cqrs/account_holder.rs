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
    let account_holder_aggregate = AccountHolder{
                aggregate_id: aggregate_id,
                full_name: "Isak Törnros".into(),
                social_security_number: "199306257255".into(),
                date_of_birth: "199306257255".into(),
                phone_number: "0763154177".into(),
                home_address: "Nöbbelövs Torg 37, 22652 LUND".into(),
            };
    let changes = HashMap::from([("full_name".into(), "Emil Törnros")]);

    let new_event = update_account_holder(account_holder_aggregate, changes);

    // this returns:
    //   Event { 
    //       aggregate_id: "C3FFCB03-CDF6-0728-0974-B74DF906A5E6", 
    //       aggregate_version: 2, 
    //       event_name: "update_account_holder_info", 
    //       timestamp: "2022-08-11 08:18:53.001244 UTC", 
    //       metadata: {},
    //       deltas: {"full_name": "Emil Törnros"}, 
    //       aggregate_type: "AccountHolder" }

```
*/


use rql::prelude::*;
use crate::cqrs::event::*;
use std::collections::HashMap;
use crate::database;
use crate::database::event_schema::EventSchema;
use crate::projections::account_holder::AccountHolder;

#[allow(dead_code)]
static AGGREGATE_TYPE: &str = "AccountHolder";

#[allow(dead_code)]
pub fn create_new_account_holder(
        full_name: &str,
        social_security_number: &str,
        date_of_birth: &str,
        phone_number: &str,
        home_address: &str,
        ) -> Event {
    let metadata = HashMap::from([]);
    let deltas = HashMap::from([
        ("full_name".into(), full_name.into()),
        ("social_security_number".into(), social_security_number.into()),
        ("date_of_birth".into(), date_of_birth.into()),
        ("phone_number".into(), phone_number.into()),
        ("home_address".into(), home_address.into()),
    ]);
    let aggregate_type = String::from(AGGREGATE_TYPE);

    let event = Event::new(metadata, deltas, aggregate_type);
        
    event
}

#[allow(dead_code)]
pub fn update_account_holder(aggregate_id: String, changes: HashMap<String, String>, event_name: &str) -> Option<Event> {
    println!("update account holder aggregate_id: {}", aggregate_id);
    let latest_event = get_latest_event_by_aggregate_id(aggregate_id);

    match latest_event {
        Some(_) => {
            let metadata = HashMap::new();
        
            // Call method 'update' on latest event, generating a brand new and "updated" event based on the one passed in.
            let new_event = latest_event.unwrap().update(changes, metadata, event_name);
            return Some(new_event)
        },
        None => {
            return None
        }
    }

}
#[allow(dead_code)]
pub fn update_account_holder_info(aggregate: AccountHolder, changes: HashMap<String, String>) -> Option<Event> {
    let event_name = "update_account_holder_info";
    update_account_holder(aggregate.aggregate_id, changes, event_name)
}
#[allow(dead_code)]
pub fn update_account_holder_info_by_id(aggregate_id: String, changes: HashMap<String, String>) -> Option<Event> {
    let event_name = "update_account_holder_info";
    update_account_holder(aggregate_id, changes, event_name)
}
#[allow(dead_code)]
pub fn delete_account_holder(aggregate: AccountHolder) -> Option<Event> {
    let changes = HashMap::from([("deltas".into(), "deleted: true".into())]);
    let event_name = "delete_account_holder";
    update_account_holder(aggregate.aggregate_id, changes, event_name)
}
#[allow(dead_code)]
pub fn delete_account_holder_by_id(aggregate_id: String) -> Option<Event> {
    let changes = HashMap::from([("deltas".into(), "deleted: true".into())]);
    let event_name = "delete_account_holder";
    update_account_holder(aggregate_id, changes, event_name)
}

fn get_latest_event_by_aggregate_id(aggregate_id: String) -> Option<Event> {
    let events = get_events_by_id_and_type(aggregate_id, AGGREGATE_TYPE);

    println!("events fetched from db: {:?}", &events);
    println!("latest event: {:?}", events.clone().into_iter().nth(0));

    // check if any events were found, if yes, then take first in vec, else early-return a None.
    let latest = 
        if events.len() == 0 {
            println!("events from db was an empty list");
            return None
        } else {
            println!("events from db was not an empty list");
            Some(events[0].clone())
        };

    // check if latest event is already a delete-type event. In that case, return None.
    if latest.clone().unwrap().event_name == "delete_account_holder" {
        None
    } else {
        latest
    }
}

fn get_events_by_id_and_type(aggregate_id: String, aggregate_type: &str) -> Vec<Event> {
    let schema: EventSchema = database::event_schema::get_schema();
    let event_table = schema.event();

    let mut events = Vec::new();
    for event in event_table
            .wher(|event| 
                event.aggregate_id == aggregate_id 
                && event.aggregate_type == aggregate_type
            )
            .select(|event| event ) {
        events.push(event.data.clone());
    }

    events
}

// cargo test -- --nocapture
#[cfg(test)]
    mod tests {
        use super::*;

        fn setup() -> EventSchema{
            database::ruql::setup()
        }

        // using serial testing so that db is populated consistently, to avoid unexpected side effects
        // when the tests fire off in random order
        #[test]
        #[serial_test::serial]
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

        #[test]
        #[serial_test::serial]
        fn test_update_account_holder(){
            setup();
            let account_holder = get_account_holder();

            let changes = HashMap::from([
                ("full_name".into(), "Emil Törnros".into())
            ]);
            let updated_event = update_account_holder_info(account_holder, changes);

            let updated_event_copy = updated_event.clone().unwrap();

            println!("Update AccountHolder, event: {:?}", updated_event_copy);

            assert_eq!(updated_event_copy.event_name, "update_account_holder_info");
            assert_eq!(updated_event_copy.deltas.get("full_name").unwrap(), "Emil Törnros");
        }

        #[test]
        #[serial_test::serial]
        fn test_get_latest_event(){
            let aggregate_id = get_random_accountholder_id_from_db();

            let latest_event = get_latest_event_by_aggregate_id(aggregate_id);

            assert_eq!(latest_event.unwrap().event_name, "new"); 
        }

        #[test]
        #[serial_test::serial]
        fn test_delete_account_holder(){
            let account_holder = get_account_holder();

            let delete_event_1 = delete_account_holder_by_id(account_holder.aggregate_id.clone());
            let delete_event_2 = delete_account_holder(account_holder);

            // println!("delete_event_1: {:?}", delete_event_1);
            // println!("delete_event_2: {:?}", delete_event_2);

            assert_eq!(delete_event_1.unwrap().event_name, "delete_account_holder");
            assert_eq!(delete_event_2.unwrap().event_name, "delete_account_holder");
        }

        fn get_random_accountholder_id_from_db() -> String {
            let db = database::event_schema::get_schema();
            let table = db.event();

            let events: Vec<_> =
                table
                .wher(|event| event.aggregate_type == AGGREGATE_TYPE)
                .select(|event| event.aggregate_id.clone() )
                .collect();

            events.into_iter().nth(0).unwrap()
        }

        fn get_account_holder() -> AccountHolder {
            let aggregate_id = get_random_accountholder_id_from_db();
            AccountHolder{
                aggregate_id: aggregate_id,
                full_name: "Isak Törnros".into(),
                social_security_number: "199306257255".into(),
                date_of_birth: "199306257255".into(),
                phone_number: "0763154177".into(),
                home_address: "Nöbbelövs Torg 37, 22652 LUND".into(),
            }
        }
    }