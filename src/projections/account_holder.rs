use rql::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountHolder {
  pub aggregate_id: String,
  pub full_name: String,
  pub social_security_number: String, 
  pub date_of_birth: String, 
  pub phone_number: String, 
  pub home_address: String,
}

#[cfg(test)]
    mod tests {
        // use super::*;

        #[test]
        fn internal() {
            
        }
    }