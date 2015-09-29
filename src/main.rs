use std::collections::HashMap;
use std::num;

#[derive(PartialEq, PartialOrd)]
struct Point {
    x : f64,
    y : f64,
}

struct Entry {
    position: Point,
    data: String,
}

type Id = i64;

struct Database {
    data: HashMap<Id, Entry>,
    lastId: Id,
}

fn distance(a: &Point, b: &Point) -> f64 {
    return distance_sq(a, b).sqrt();
}

fn distance_sq(a: &Point, b: &Point) -> f64 {
    return (a.x-b.x).powf(2.) + (a.y-b.y).powf(2.);
}

impl Database {
    fn new() -> Database {
        Database {
            data: HashMap::new(),
            lastId: 0
        }
    }

    fn insert(&mut self, e: Entry) {
        self.lastId += 1;
        self.data.insert(self.lastId, e);
    }

    fn in_radius_from(&self, from: Point, radius: f64) -> Vec<Id> {
        let radius_sq = radius.powf(2.);
        let mut items = Vec::new();
        for (id, entity) in self.data.iter() {
            if distance_sq(&from, &entity.position) < radius_sq {
                items.push(id.clone());
            }
        }
        return items;
    }
}

fn main() {
    println!("Hello, world!");

    let mut db = Database::new();
    db.insert(Entry{ position: Point{x: 0., y: 0.}, data:"origin".to_string()});

    for id in db.in_radius_from(Point{x: 0., y: 0.}, 10.) {
        println!("{}", id);
    }
}