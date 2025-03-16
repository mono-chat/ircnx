use crate::connections::user::User;

pub struct Member {
    user: User,
    modes: String, // Could be a Vec<char> or an enum (e.g. +q, +o, +v)
}
