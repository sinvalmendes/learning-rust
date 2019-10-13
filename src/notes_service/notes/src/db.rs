use std::ops::Deref;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};

use crate::model::{Note, NewNote};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

fn get_conn(pool: &PgPool) -> Result<PgPooledConnection, &'static str> {
    pool.get().map_err(|_| "Can't get connection")
}

pub fn create_note(note: Note, pool: &PgPool) -> Result<(), &'static str> {
    let new_note = NewNote { title: note.title, content: note.content};
    Note::insert(new_note, get_conn(pool)?.deref())
        .map(|_| ())
        .map_err(|_| "Error inserting task")
}
