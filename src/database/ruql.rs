// use rql::prelude::*;

// #[derive(Serialize, Deserialize)]
// struct User {
//     name: String,
//     age: u8,
// }

// #[derive(Serialize, Deserialize)]
// struct Group {
//     name: String,
// }

// #[derive(Serialize, Deserialize)]
// struct Member {
//     user_id: Id<User>,
//     group_id: Id<Group>,
//     permission: bool,
// }

// schema! {
//     MySchema {
//         user: User,
//         group: Group,
//         member: Member,
//     }
// }

// // Create a new database with the previously defined schema
// // We pass a folder name for the database files as well as a representation type
// let db = MySchema::new("test_database_example", HumanReadable).unwrap();

// // Insert values into the database
// // Insertion returns the new row's id
// let dan   = db.user_mut().insert(User { name: "Dan".into(),   age: 25 });
// let steve = db.user_mut().insert(User { name: "Steve".into(), age: 39 });
// let mary  = db.user_mut().insert(User { name: "Mary".into(),  age: 31 });

// let admin  = db.group_mut().insert(Group { name: "Admin".into()       });
// let normal = db.group_mut().insert(Group { name: "Normal User".into() });

// db.member_mut().insert(Member { user_id: dan,   group_id: admin,  permission: true  });
// db.member_mut().insert(Member { user_id: steve, group_id: normal, permission: true  });
// db.member_mut().insert(Member { user_id: mary,  group_id: normal, permission: false });

// // Data can easily be looked up by id
// db.user_mut().get_mut(dan).unwrap().age += 1;
// let dan_age = db.user().get(dan).unwrap().age;
// assert_eq!(dan_age, 26);

// // Data can be selected from a table
// let ages: Vec<u8> = db.user().select(|user| user.age).collect();

// // Use `wher` to filter entries
// let can_run_for_president: Vec<String> =
//     db.user()
//         .wher(|user| user.age >= 35)
//         .select(|user| user.name.clone())
//         .collect();

// // Table intersections are done using `relate`
// // A function relating the tables is required
// for (user, permission) in db.user()
//     .relate(
//         &*db.member(),
//         |user, member| user.id == member.user_id && member.group_id == normal
//     )
//     .select(|(user, member)| (&user.data.name, member.permission)) {
//     println!("{} is a normal user with permission = {}", user, permission);
// }

// // Rows can be updated with `update`
// for mut user in db.user_mut().update() {
//     user.age += 1;
// }

// // Rows can be deleted in a few ways

// // By id
// db.user_mut().delete_one(steve);

// // With a where clause
// db.member_mut().delete_where(|member| member.permission);

// // With an iterator over ids
// db.user_mut().delete_iter(|_| vec![dan, mary]);

// // Changes to the database are automatically saved, so they can be loaded again
// let db_copy = MySchema::new("test_database_example", HumanReadable).unwrap();
// assert_eq!(db.user().len(),   db_copy.user().len()  );
// assert_eq!(db.group().len(),  db_copy.group().len() );
// assert_eq!(db.member().len(), db_copy.member().len());