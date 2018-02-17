use schema::threads;
use chrono::NaiveDateTime;
use db::user::User;
use db::forum::Forum;
use error::WeekendAtJoesError;
use db::Conn;
use std::ops::Deref;
use db::handle_diesel_error;
use diesel;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::BelongingToDsl;
use diesel::ExpressionMethods;

#[derive( Debug, Clone, Identifiable, Associations, Queryable)]
#[belongs_to(User, foreign_key = "author_id")]
#[belongs_to(Forum, foreign_key = "forum_id")]
#[table_name="threads"]
pub struct Thread {
    /// Primary Key
    pub id: i32,
    /// Foreign Key to which the thread belongs to.
    pub forum_id: i32,
    /// Foreign Kay of the user who created the thread.
    pub author_id: i32,
    /// Timestamp of when the thread was created.
    pub created_date: NaiveDateTime,
    /// If the thread is locked, then it cannot be edited, nor can any of its posts.
    pub locked: bool,
    /// If the thread is archived, then it cannot be seen by non-moderators
    pub archived: bool,
    /// The title of the thread will be shown on think in the forum that will take you to the thread, as well as at the top of the thread's page.
    pub title: String
}


#[derive(Serialize, Deserialize, Insertable, Debug)]
#[table_name="threads"]
pub struct NewThread {
    pub forum_id: i32,
    pub author_id: i32,
    pub created_date: NaiveDateTime,
    pub locked: bool,
    pub archived: bool,
    pub title: String
}

impl Thread {

    /// Creates a new Thread.
    pub fn create_thread(new_thread: NewThread, conn: &Conn) -> Result<Thread, WeekendAtJoesError> {
        use schema::threads;

        diesel::insert_into(threads::table)
            .values(&new_thread)
            .get_result(conn.deref())
            .map_err(|e| handle_diesel_error(e, "Thread"))
    }

    /// Locks the thread, preventing posting and editing
    // TODO consolidate this function and unlock_thread(), by specifiying an additional bool value.
    pub fn lock_thread(thread_id: i32, conn: &Conn) -> Result<Thread, WeekendAtJoesError>{
        use schema::threads;
        use schema::threads::dsl::*;
        diesel::update(threads::table)
            .filter(id.eq(thread_id))
            .set(locked.eq(true))
            .get_result(conn.deref())
            .map_err(|e| handle_diesel_error(e, "Thread"))
    }

    /// Unlocks the thread, allowing posting and editing again.
    pub fn unlock_thread(thread_id: i32, conn: &Conn) -> Result<Thread, WeekendAtJoesError>{
        use schema::threads;
        use schema::threads::dsl::*;
        diesel::update(threads::table)
            .filter(id.eq(thread_id))
            .set(locked.eq(false))
            .get_result(conn.deref())
            .map_err(|e| handle_diesel_error(e, "Thread"))
    }

    /// Archives the thread, preventing it from being seen in typical requests.
    pub fn archive_thread(thread_id: i32, conn: &Conn) -> Result<Thread, WeekendAtJoesError> {
        use schema::threads;
        use schema::threads::dsl::*;
        diesel::update(threads::table)
            .filter(id.eq(thread_id))
            .set(archived.eq(true))
            .get_result(conn.deref())
            .map_err(|e| handle_diesel_error(e, "Thread"))
    }

    /// Gets all of the most recent threads in a forum.
    /// Archived threads will not be included.
    // TODO add a step to enable pagination
    pub fn get_threads_in_forum(requested_forum_id: i32, num_threads: i64, conn: &Conn) -> Result<Vec<Thread>, WeekendAtJoesError> {
        use schema::threads::dsl::*;
        use db::forum::Forum;

        let forum: Forum = Forum::get_forum(requested_forum_id, conn)?;

        Thread::belonging_to(&forum)
            .filter(archived.eq(false)) // don't get archived threads
            .order(created_date)
            .limit(num_threads)
            .get_results(conn.deref())
            .map_err(|e| handle_diesel_error(e, "Thread"))
    }

    /// Currently this acts as a helper method for Post::get_root_post() and isn't intended to be exposed via api
    pub fn get_thread(thread_id: i32, conn: &Conn) -> Result<Thread, WeekendAtJoesError> {
        use schema::threads::dsl::*;

        // Gets the first thread that matches the id.
        threads
            .find(thread_id)
            .first::<Thread>(conn.deref())
            .map_err(|e| handle_diesel_error(e, "Thread"))
    }

}