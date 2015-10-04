pub mod database;
use database as db;

fn main() {
    println!("Hello, world!");

    let mut my_db = db::Database::new();
    my_db.insert(db::Entry{ position: db::Point{x: 0., y: 0.}, data:"origin".to_string()});

    for id in my_db.in_radius_from(db::Point{x: 0., y: 0.}, 10.) {
        println!("{}", id);
    }
}
