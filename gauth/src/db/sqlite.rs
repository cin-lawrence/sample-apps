#[cfg(feature = "server")]
use crate::models::User;

#[cfg(feature = "server")]
use rusqlite::Connection;
use std::sync::Mutex;

const SQLITE_USER_DATABASE: &str = "./users.db";

#[cfg(feature = "server")]
pub struct Sqlite {
    pub conn: Mutex<Connection>,
}

#[cfg(feature = "server")]
impl Sqlite {
    pub fn new() -> Self {
        Self {
            conn: Mutex::new(Connection::open(SQLITE_USER_DATABASE).unwrap()),
        }
    }

    pub fn create_table(&self) {
        let conn = self.conn.lock().unwrap();
        // Drop the table if it exists to handle schema changes
        let _ = conn.execute("DROP TABLE IF EXISTS users", []);
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY,
                first_name TEXT NOT NULL,
                last_name TEXT NOT NULL,
                email TEXT NOT NULL,
                password TEXT,
                provider TEXT NOT NULL
            )",
            [],
        )
        .unwrap();
    }

    pub fn create_google_user(&self, user: User) {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "INSERT INTO users (first_name, last_name, email, provider) VALUES (?, ?, ?, ?)",
            )
            .unwrap();

        stmt.execute(&[&user.given_name, &user.family_name, &user.email, "google"])
            .unwrap();
    }

    pub fn create_classic_user(
        &self,
        first_name: String,
        last_name: String,
        email: String,
        password_hash: String,
    ) {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("INSERT INTO users (first_name, last_name, email, password, provider) VALUES (?, ?, ?, ?, ?)")
            .unwrap();

        stmt.execute(&[&first_name, &last_name, &email, &password_hash, "classic"])
            .unwrap();
    }

    pub fn upsert_user(&self, user: User) {
        let conn = self.conn.lock().unwrap();
        let row = conn.query_row(
            "SELECT id, first_name, last_name FROM users WHERE email = ?1 AND provider = 'google'",
            &[&user.email],
            |row| {
                Ok((
                    row.get::<usize, usize>(0).unwrap(),
                    row.get::<usize, String>(1).unwrap(),
                    row.get::<usize, String>(2).unwrap(),
                ))
            },
        );

        match row {
            Ok(_user) => {
                println!("Login successful");
            }
            Err(ref _e) => {
                // Create user directly here to avoid deadlock
                let mut stmt = conn
                    .prepare("INSERT INTO users (first_name, last_name, email, provider) VALUES (?, ?, ?, ?)")
                    .unwrap();

                stmt.execute(&[&user.given_name, &user.family_name, &user.email, "google"])
                    .unwrap();
            }
        }
    }

    pub fn get_classic_user_by_email(
        &self,
        email: &String,
    ) -> Option<(usize, String, String, String)> {
        let conn = self.conn.lock().unwrap();
        let row = conn
            .query_row(
                "SELECT id, first_name, last_name, password FROM users WHERE email = ?1 AND provider = 'classic'",
                &[&email],
                |row| {
                    Ok(
                        (
                            row.get::<usize, usize>(0).unwrap(),
                            row.get::<usize, String>(1).unwrap(),
                            row.get::<usize, String>(2).unwrap(),
                            row.get::<usize, String>(3).unwrap(),
                        )
                    )
                }
            );
        match row {
            Ok(row) => Some(row),
            Err(ref _e) => None,
        }
    }

    pub fn check_user_email(&self, email: &String) -> Option<String> {
        let conn = self.conn.lock().unwrap();
        let row = conn.query_row(
            "SELECT email FROM users WHERE email = ?1 AND provider = 'classic'",
            &[&email],
            |row| row.get::<usize, String>(0),
        );

        match row {
            Ok(email) => Some(email),
            Err(ref _e) => None,
        }
    }
}
