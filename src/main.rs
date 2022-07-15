mod cqrs;
mod database;
// use rql::prelude::*;
// use rql::mashup;

fn main() {
    println!("Hello, world! Foo");
    let _schema = database::event_schema::get_schema();


}
