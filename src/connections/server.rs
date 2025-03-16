use crate::channels::channel::Channel;

use super::user::User;


pub struct Server {
    channels: Vec<Channel>,
    users: Vec<User>,
}