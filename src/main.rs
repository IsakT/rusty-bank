mod cqrs;
mod database;
mod projections;
// use rql::prelude::*;
// use rql::mashup;

fn main() {
    println!("Hello, world! Foo");
    let _schema = database::event_schema::get_schema();

    // projections::account_holder::AccountHolder {

    // }
}
