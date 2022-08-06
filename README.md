# Rusty-bank
A "hello world" style kind of app for me to learn the programming language Rust.

[WORK IN PROGRESS]

Rusty-bank is a simple bank app where users can handle their bank accounts, make transactions, etc.
Admin users have access to the entire transaction log and statistics of all the users and their activity.

Back-end and database is based on CQRS (Command and Query Responsibility Segregation) and eventsourcing principles, where the read model is separated from the write model, and where every action results in events and persisted indefinitely and immutably.
To make sense of all the events, projections aggregates the events into useful data and statistics.
