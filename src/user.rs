use crate::connection::Connection;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct User {
    pub addr: SocketAddr,
    pub nickname: Option<String>,
    pub connection: Connection,
    //pub hopcount: u32,
}

pub type UserList = Arc<RwLock<HashMap<SocketAddr, User>>>;
