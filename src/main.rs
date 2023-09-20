use chrono::{DateTime, TimeZone, Utc};
use uuid::Uuid;
use std::{
    collections::{BTreeMap, VecDeque},
    sync::Arc,
};

pub trait SnapshotTrait<T>
where
    T: Clone,
{
    // Creates a snapshot of the current state of the object.
    // Returns that represents the snapshot.
    fn create_snapshot(&self) -> &str;
    fn get_snapshot(tag: &str) -> T;
}

struct Row {
    data: [u8; 32],
}

impl Default for Row {
    fn default() -> Self {
        Self { data: [0; 32] }
    }
}

struct Client {
    id: u64,
    name: String,
}

impl Client {
    fn new(id: u64, name: String) -> Self {
        Self { id, name }
    }
}

static mut ID_COUNTER: u64 = 0;

struct DbEvent {
    id: u64,
    data: VecDeque<[u8; 32]>,
    who: Arc<Client>,
    when: u64,
    table_id: String,
}

impl DbEvent {
    fn new(db: Db, data: VecDeque<[u8; 32]>, who: Arc<Client>, table_id: String) -> Self {
        let when = Utc::now().timestamp_millis() as u64;

        unsafe {
            let id = ID_COUNTER;
            ID_COUNTER += 1;
            Self { id, data, who, when, table_id }
        }
    }
}

struct Db {
    tables: BTreeMap<String, Row>,
    clients: BTreeMap<String, Arc<Client>>,
}

impl Default for Db {
    fn default() -> Self {
        Self {
            tables: BTreeMap::default(),
            clients: BTreeMap::default(),
        }
    }
}

impl SnapshotTrait<u8> for Db {
    fn create_snapshot(&self) -> &str {
        todo!()
    }

    fn get_snapshot(tag: &str) -> u8 {
        todo!()
    }
}

impl Db {
    fn add_user(&mut self, client: Arc<Client>) {
        let id = Uuid::new_v4().to_string();
        self.clients.insert(id, client);
    }
}

fn main() {
    println!("Hello, world!");
}
