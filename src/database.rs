use std::collections::HashMap;

#[derive(PartialEq, PartialOrd)]
pub struct Point {
    pub x : f64,
    pub y : f64,
}

pub struct Entry {
    pub position: Point,
    pub data: String,
}

pub type Id = i64;

pub struct Database {
    data: HashMap<Id, Entry>,
    last_id: Id,
}

fn distance(a: &Point, b: &Point) -> f64 {
    return distance_sq(a, b).sqrt();
}

fn distance_sq(a: &Point, b: &Point) -> f64 {
    return (a.x-b.x).powf(2.) + (a.y-b.y).powf(2.);
}

impl Database {
    pub fn insert(&mut self, e: Entry) {
        self.last_id += 1;
        self.data.insert(self.last_id, e);
    }

    pub fn new() -> Database {
        Database {
            data: HashMap::new(),
            last_id: 0
        }
    }

    pub fn in_radius_from(&self, from: Point, radius: f64) -> Vec<Id> {
        let radius_sq = radius.powf(2.);
        let mut items = Vec::new();
        for (id, entity) in self.data.iter() {
            if distance_sq(&from, &entity.position) < radius_sq {
                items.push(id.clone());
            }
        }
        return items;
    }

    pub fn distance_between(&self, a: &Id, b: &Id) -> Option<f64> {
        let ma = self.data.get(a);
        let mb = self.data.get(b);

        match (ma,mb) {
            (Some(a), Some (b)) => Some(distance(&a.position, &b.position)),
            _ => None
        }
    }
}
