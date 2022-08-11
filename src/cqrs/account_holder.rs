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
// use rql::mashup;
use crate::cqrs::event::*;
use std::collections::HashMap;
use crate::database;
use crate::database::event_schema::EventSchema;
// use rql::repr::Representation::HumanReadable;

#[allow(dead_code)]
static AGGREGATE_TYPE: &str = "AccountHolder";

// todo: this struct should probably be moved to Projections
#[derive(Serialize, Deserialize, Debug)]
pub struct AccountHolder {
  pub aggregate_id: String,
  pub full_name: String,
  pub social_security_number: String, 
  pub date_of_birth: String, 
  pub phone_number: String, 
  pub home_address: String,
}

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
pub fn update_account_holder(aggregate: AccountHolder, changes: HashMap<String, String>) -> Option<Event> {
    let latest_event = get_event_by_aggregate_id(aggregate.aggregate_id);

    match latest_event {
        Some(_) => {
            let event_name = "update_account_holder_info";
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

fn get_event_by_aggregate_id(aggregate_id: String) -> Option<Event> {
    // access event db
    let schema: EventSchema = database::event_schema::get_schema();
    let event_table = schema.event();

    let mut events = Vec::new();

    // fetch latest events by aggregate_id
    for event in event_table
            .wher(|event| event.aggregate_id == aggregate_id )
            .select(|event| event.clone() ) {
        events.push(event.data);
    }

    println!("events fetched from db: {:?}", &events);
    println!("latest event: {:?}", events.clone().into_iter().nth(0));

    if events.len() == 0 {
        println!("events from db was an empty list");
        None
    } else {
        println!("events from db was not an empty list");
        let latest = events.clone().into_iter().nth(0).unwrap().clone();
        Some(latest)
    }
}

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

        #[test]
        fn test_update_account_holder(){
            let account_holder = get_account_holder();

            let changes = HashMap::from([
                ("full_name".into(), "Emil Törnros".into())
            ]);
            let updated_event = update_account_holder(account_holder, changes);

            println!("Update AccountHolder, event: {:?}", updated_event.unwrap());
        }

        #[test]
        fn test_get_latest_event(){
            let aggregate_id = get_random_aggregate_id_from_db();

            let latest_event = get_event_by_aggregate_id(aggregate_id);

            // sometimes this panics, just run test again until it works
            assert_eq!(latest_event.unwrap().event_name, "new"); 
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

        fn get_random_aggregate_id_from_db() -> String {
            let db = setup();
            let table = db.event();

            let events: Vec<_> =
                table
                .select(|event| event.aggregate_id.clone() )
                .collect();

            events.into_iter().nth(0).unwrap()
        }

        fn get_account_holder() -> AccountHolder {
            let aggregate_id = get_random_aggregate_id_from_db();
            AccountHolder{
                aggregate_id: aggregate_id,
                full_name: "Isak Törnros".into(),
                social_security_number: "199306257255".into(),
                date_of_birth: "199306257255".into(),
                phone_number: "0763154177".into(),
                home_address: "Nöbbelövs Torg 37, 22652 LUND".into(),
            }
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