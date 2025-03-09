use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct User {
    pub addr: std::net::SocketAddr,
}

pub type UserList = Arc<RwLock<HashMap<std::net::SocketAddr, User>>>;
