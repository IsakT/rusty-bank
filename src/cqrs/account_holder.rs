// use rql::prelude::*;
// use rql::mashup;
use crate::cqrs::event::*;
use std::collections::HashMap;

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
        ("name".into(), name.into()),
        ("social_security_number".into(), social_security_number.into()),
        ("date_of_birth".into(), date_of_birth.into()),
        ("phone_number".into(), phone_number.into()),
        ("home_address".into(), home_address.into()),
    ]);
    let aggregate_type = String::from(AGGREGATE_TYPE);

    let event = Event::new(metadata, deltas, aggregate_type);
        
    event
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

            println!("{:?}", event.event_name);
            println!("{:?}", event.deltas.get("name"));

            assert_eq!(event.event_name, "new");
            assert_eq!(event.aggregate_type, "AccountHolder");
            assert_eq!(event.deltas.get("name"), Some(&String::from(name)));
            assert_eq!(event.deltas.get("social_security_number"), Some(&String::from(social_security_number)));
            assert_eq!(event.deltas.get("date_of_birth"), Some(&String::from(date_of_birth)));
            assert_eq!(event.deltas.get("home_address"), Some(&String::from(home_address)));
        }

        fn test_update_account_holder(){
            // let event = test_data();


            // let updated_event = update_account_holder(event, changes);
        }

        fn test_delete_account_holder(){

        }

        fn test_get_account_holder(){

        }

        fn test_data() -> Event {
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

            event
          }
    }