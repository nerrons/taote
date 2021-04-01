use rusqlite::{params, Connection, Result, NO_PARAMS};

#[derive(Debug)]
struct Album {
    id: i32,
    name: String,
    artist: String,
    rating: u8,
    review: String,
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE albums (
            id      INTEGER PRIMARY KEY,
            name    TEXT NOT NULL,
            artist  TEXT NOT NULL,
            rating  INTEGER,
            review  TEXT NOT NULL
        )",
        NO_PARAMS,
    )?;

    let album = Album {
        id: 0,
        name: "Our Puzzling Encounters Considered".to_string(),
        artist: "Psyopus".to_string(),
        rating: 6,
        review: "The guitarist Chris Arp is very worth listening to. Otherwise nothing special"
            .to_string(),
    };

    let mut insert_stmt =
        conn.prepare("INSERT INTO albums (name, artist, rating, review) VALUES (?, ?, ?, ?)")?;
    insert_stmt.execute(params![
        album.name,
        album.artist,
        album.rating,
        album.review
    ])?;

    let mut select_stmt = conn.prepare("SELECT id, name, artist, rating, review FROM albums")?;
    let albums_iter = select_stmt.query_map(NO_PARAMS, |row| {
        Ok(Album {
            id: row.get(0)?,
            name: row.get(1)?,
            artist: row.get(2)?,
            rating: row.get(3)?,
            review: row.get(4)?,
        })
    })?;

    for a in albums_iter {
        println!("Album: {:?}", a.unwrap());
    }

    Ok(())
}
