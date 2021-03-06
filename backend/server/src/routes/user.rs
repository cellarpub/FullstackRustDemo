use pool::Conn;
use rocket::Route;
use rocket_contrib::Json;
use routes::{
    convert_vector,
    Routable,
};
//use db::Retrievable;
use db::user::User;
use error::{
    BackendResult,
    Error,
};
use wire::user::*;
//use auth_lib::user_authorization::*;
use auth_lib::BannedSet;
use db::user::NewUser;
use identifiers::user::UserUuid;
use log::info;
use rocket::State;

use auth_lib::user_authorization::{
    AdminUser,
    NormalUser,
};

/// Gets basic info about an user.
/// Provided they know the id of the user, this information is available to anyone.
#[get("/<user_uuid>")]
fn get_user(user_uuid: UserUuid, conn: Conn) -> Result<Json<UserResponse>, Error> {
    User::get_user(user_uuid, &conn).map(UserResponse::from).map(Json)
}

/// Get all info about users.
/// This request is paginated. It will return vectors of users, 25 at a time.
/// This operation is only available to an admin.
#[get("/users/<index>")]
fn get_users(index: i32, _admin: AdminUser, conn: Conn) -> BackendResult<Json<Vec<FullUserResponse>>> {
    User::get_paginated(index, 25, &conn)
        .map(|x| x.0)
        .map(convert_vector)
        .map(Json)
}

/// Get all users with the specified role id.
/// This operation is only available to an admin.
#[get("/users_with_role/<role_id>")]
fn get_users_with_role(role_id: i32, _admin: AdminUser, conn: Conn) -> BackendResult<Json<Vec<UserResponse>>> {
    User::get_users_with_role(role_id.into(), &conn)
        .map(convert_vector)
        .map(Json)
}

/// It doesn't require any account to create a new user.
#[post("/", data = "<new_user>")]
pub fn create_user(new_user: Json<NewUserRequest>, conn: Conn) -> BackendResult<Json<UserResponse>> {
    let new_user: NewUser = new_user.into_inner().into();
    User::create_user(new_user, &conn).map(UserResponse::from).map(Json)
}

/// Allow a user to update their display name.
#[put("/", data = "<data>")]
fn update_user_display_name(
    data: Json<UpdateDisplayNameRequest>,
    _user: NormalUser,
    conn: Conn,
) -> BackendResult<Json<UserResponse>> {
    info!("updating user display name");
    let request: UpdateDisplayNameRequest = data.into_inner();

    let current_user_name = request.user_name;
    let new_display_name = request.new_display_name;

    User::update_user_display_name(current_user_name, new_display_name, &conn)
        .map(UserResponse::from)
        .map(Json)
}

/// Assigns a role to a user.
/// This operation is only available to an admin.
#[put("/assign_role", data = "<data>")]
fn assign_role(data: Json<UserRoleRequest>, _admin: AdminUser, conn: Conn) -> BackendResult<Json<UserResponse>> {
    User::add_role_to_user(data.uuid, data.user_role.into(), &conn)
        .map(UserResponse::from)
        .map(Json)
}

/// Ban the user. This prevents the user from being able to log in.
/// Because the user's identifier is immediately added to the banned set,
/// JWTs can cease to be validated as soon as the user is banned.
#[put("/ban/<user_uuid>")]
fn ban_user(
    user_uuid: UserUuid,
    _admin: AdminUser,
    banned_set: State<BannedSet>,
    conn: Conn,
) -> BackendResult<Json<UserResponse>> {
    // Set the banned state so the JWT resolvers can check for bans without checking a DB.
    banned_set.ban_user(user_uuid);

    User::set_ban_status(user_uuid, true, &conn)
        .map(UserResponse::from)
        .map(Json)
}

/// Unbans the user. This will allow them to log in again.
/// Because the user id is removed from the banned set,
/// any outstanding JWTs the banned user may have become viable again.
#[put("/unban/<user_uuid>")]
fn unban_user(
    user_uuid: UserUuid,
    _admin: AdminUser,
    banned_set: State<BannedSet>,
    conn: Conn,
) -> BackendResult<Json<UserResponse>> {
    banned_set.unban_user(&user_uuid);

    User::set_ban_status(user_uuid, false, &conn)
        .map(UserResponse::from)
        .map(Json)
}

// Export the ROUTES and their path
impl Routable for User {
    const ROUTES: &'static Fn() -> Vec<Route> = &|| {
        routes![
            create_user,
            update_user_display_name,
            get_user,
            get_users,
            get_users_with_role,
            assign_role,
            ban_user,
            unban_user // delete_user_by_name,
        ]
    };
    const PATH: &'static str = "/user/";
}
