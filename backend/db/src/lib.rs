//! This module is responsible for facilitating interaction with the database.
//! Pools and Connections are defined which allow a pool to be specified at startup, and for routes to request a connection from the pool.
//! The files in this module contain functions that interact with the type specified by the filename.
//! These functions are analagous to stored procedures.


#![feature(use_extern_macros)]

#[macro_use]
extern crate db_proc_macros;
extern crate error;
extern crate wire;
extern crate auth as auth_lib;

#[macro_use]
extern crate diesel;
extern crate uuid;




//#[macro_use]
//extern crate serde_json;

extern crate slug;
extern crate rand;
extern crate chrono;
extern crate r2d2_diesel;
extern crate r2d2;

extern crate rocket;
extern crate identifiers;

#[macro_use(log)]
extern crate log;
extern crate simplelog;




use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
//use r2d2;

use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use error::ErrorFormatter;
use error::JoeResult;
use diesel::Identifiable;
// use diesel::Insertable;
// use diesel::Queryable;

mod diesel_extensions;

//pub mod auth;

//pub mod user;
//pub mod article;
//pub mod forum;
//pub mod thread;
//pub mod post;
//pub mod bucket;
//pub mod question;
//pub mod answer;
//pub mod chat;
//pub mod message;

mod calls;
pub use calls::*;

pub mod schema;

mod conversions;
//mod auth;


pub use user::User;
pub use article::Article;
pub use forum::{Forum, NewForum};
pub use thread::{Thread, NewThread};
pub use post::Post;
pub use bucket::Bucket;
pub use question::Question;
pub use answer::Answer;
pub use chat::Chat;
pub use message::Message;


/// Holds a bunch of connections to the database and hands them out to routes as needed.
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub const DATABASE_FILE: &'static str = env!("DATABASE_URL");

/// Initializes the pool.
pub fn init_pool() -> Pool {
    //    let config = r2d2::Config::default();
    let manager = ConnectionManager::<PgConnection>::new(DATABASE_FILE);
    r2d2::Pool::new(manager).expect(
        "db pool",
    )
}

/// Wrapper for PgConnection.
/// This type can be used in route methods to grab a DB connection from the managed pool.
pub struct Conn(r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl Conn {
    //    #[cfg(test)]
    pub fn new(pooled_connection: r2d2::PooledConnection<ConnectionManager<PgConnection>>) -> Conn {
        Conn(pooled_connection)
    }
}

impl Deref for Conn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Conn {
    type Error = ();

    // Gets the pool from the request and extracts a reference to a connection which is then wrapped in a Conn() and handed to route.
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Conn, ()> {
        let pool = match <State<Pool> as FromRequest>::from_request(request) {
            Outcome::Success(pool) => pool,
            Outcome::Failure(e) => return Outcome::Failure(e),
            Outcome::Forward(_) => return Outcome::Forward(()),
        };

        match pool.get() {
            Ok(conn) => Outcome::Success(Conn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}


pub trait Creatable<T> {
    fn create(insert: T, conn: &PgConnection) -> JoeResult<Self>
    where
        Self: Sized;
}

pub trait Retrievable<'a> {
    fn get_by_id(id: i32, conn: &PgConnection) -> JoeResult<Self>
    where
        Self: 'a + Sized,
        &'a Self: Identifiable;

    fn get_all(conn: &PgConnection) -> JoeResult<Vec<Self>>
    where
        Self: 'a + Sized,
        &'a Self: Identifiable;

    fn exists(id: i32, conn: &PgConnection) -> JoeResult<bool>
    where
        Self: 'a + Sized,
        &'a Self: Identifiable;

    // fn get_paginated(page_index: i64, page_size: i64, conn: &Conn) -> Result<Vec<Self>, WeekendAtJoesError>
    //     where
    //         Self: Sized;
}

trait Deletable<'a> {
    /// The delete operation will fail if any children exist: `ForeignKeyViolation`.
    /// A separate, safe-delete operation should be implemented that cleans up all children before this runs.
    fn delete_by_id(id: i32, conn: &PgConnection) -> JoeResult<Self>
    where
        Self: ErrorFormatter,
        Self: 'a + Sized,
        &'a Self: Identifiable;
}

/// Type tag that indicates that the tagged type can be created, retrieved, and deleted.
/// This collection of abilities means that it is safe to use in integration tests.
trait CRD<'a, T>
where
    Self: Creatable<T> + Retrievable<'a> + Deletable<'a>
{
}



use uuid::Uuid;

pub trait CreatableUuid<T> {
    fn create(insert: T, conn: &PgConnection) -> JoeResult<Self>
    where
        Self: Sized;
}

pub trait RetrievableUuid<'a> {
    fn get_by_uuid(id: Uuid, conn: &PgConnection) -> JoeResult<Self>
    where
        Self: 'a + Sized,
        &'a Self: Identifiable;

    fn get_all(conn: &PgConnection) -> JoeResult<Vec<Self>>
    where
        Self: 'a + Sized,
        &'a Self: Identifiable;

    fn exists(id: Uuid, conn: &PgConnection) -> JoeResult<bool>
    where
        Self: 'a + Sized,
        &'a Self: Identifiable;

    // fn get_paginated(page_index: i64, page_size: i64, conn: &Conn) -> Result<Vec<Self>, WeekendAtJoesError>
    //     where
    //         Self: Sized;
}

trait DeletableUuid<'a> {
    /// The delete operation will fail if any children exist: `ForeignKeyViolation`.
    /// A separate, safe-delete operation should be implemented that cleans up all children before this runs.
    fn delete_by_id(id: Uuid, conn: &PgConnection) -> JoeResult<Self>
    where
        Self: ErrorFormatter,
        Self: 'a + Sized,
        &'a Self: Identifiable;
}


//
//pub mod testing {
//
//    use super::*;
//    use chrono::Utc;
//    use error::WeekendAtJoesError;
//    use db::user::*;
//    use db::forum::*;
//    use db::thread::*;
//    use wire::user::*;
//
//    #[allow(dead_code)]
//    /// Create a bunch of entries for every data type in the backend.
//    pub fn generate_test_fixtures(conn: &Conn) -> Result<(), WeekendAtJoesError> {
//
//        // Create User
//        let mut user: NewUser = NewUserRequest {
//            user_name: "Admin".into(),
//            display_name: "Admin".into(),
//            plaintext_password: "Admin".into(),
//        }.into();
//        user.roles.push(UserRole::Admin.into());
//        user.roles.push(
//            UserRole::Moderator.into(),
//        );
//        user.roles.push(
//            UserRole::Publisher.into(),
//        );
//        let user: User = User::create(user, conn)?;
//
//        // Create forums
//        let forum1: NewForum = NewForum {
//            title: "Joe Forum".to_string(),
//            description: "A Forum for All Things Joe."
//                .to_string(),
//        };
//        let forum1: Forum = Forum::create(forum1, conn)?;
//        let forum2: NewForum = NewForum {
//            title: "Off Topic".to_string(),
//            description: "A Forum for All Things Not Related to Joe."
//                .to_string(),
//        };
//        let forum2: Forum = Forum::create(forum2, conn)?;
//        let forums: Vec<Forum> = vec![forum1, forum2];
//
//        // Create Threads
//        let create_thread_fn = |forum: &Forum, user: &User, title: &str| {
//            let thread1: NewThread = NewThread {
//                forum_uuid: forum.uuid,
//                author_uuid: user.uuid,
//                created_date: Utc::now().naive_utc(),
//                locked: false,
//                archived: false,
//                title: title.to_string(),
//            };
//            Thread::create(thread1, &conn)
//        };
//
//        let thread_titles: Vec<&'static str> = vec!["Thread Title", "Another Thread", "Yet Another Thread"];
//
//        let mut threads: Vec<Thread> = vec![];
//        for forum in forums {
//            for thread_title in thread_titles.clone() {
//                threads.push(create_thread_fn(
//                    &forum,
//                    &user,
//                    thread_title,
//                )?)
//            }
//        }
//        //        let threads = threads; // remove mutability
//
//        return Ok(());
//    }
//
//
//}
//
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//    use db::user::*;
//    use db::article::*;
//    use wire::user::*;
//
//
//    #[test]
//    fn cascade_delete_test() {
//
//        let pool = init_pool();
//
//        let user_name: String = String::from("CascadeDeleteTest-UserName");
//
//        let conn = Conn::new(pool.get().unwrap());
//        let _ = User::delete_user_by_name(user_name.clone(), &conn);
//
//        let new_user: NewUser = NewUserRequest {
//            user_name: user_name.clone(),
//            display_name: String::from("DisplayName"),
//            plaintext_password: String::from("TestPassword"),
//        }.into();
//
//        let user = User::create(new_user, &conn).unwrap();
//
//        let new_article: NewArticle = NewArticle {
//            title: String::from("CascadeDeleteTest-ArticleTitle"),
//            slug: String::from("aah"),
//            body: String::from("body"),
//            author_id: user.id,
//        };
//
//        let _child_article: Article = Article::create(new_article, &conn)
//            .unwrap();
//
//        // Cascade delete should take care of the child article
//        assert!(
//            User::delete_by_id(user.id, &conn)
//                .is_ok(),
//            true
//        );
//    }
//
//    #[test]
//    fn create_without_dependencies() {
//
//        let pool = init_pool();
//        let conn = Conn::new(pool.get().unwrap());
//
//        let new_article: NewArticle = NewArticle {
//            title: String::from("CreateTest-ArticleTitle"),
//            slug: String::from("aah"),
//            body: String::from("body"),
//            author_id: 420420, // non-existent id
//        };
//
//        // Because the id of the author does not exist, creating a new article will fail.
//        assert!(
//            Article::create(new_article, &conn)
//                .is_err()
//        );
//
//    }
//}
