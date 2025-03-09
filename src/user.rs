use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::connection::Connection;

#[derive(Debug)]
pub struct User {
    pub addr: std::net::SocketAddr,
    pub nickname: Option<String>,
    pub connection: Connection
}

pub type UserList<'a> = Arc<RwLock<HashMap<std::net::SocketAddr, &'a User>>>;
