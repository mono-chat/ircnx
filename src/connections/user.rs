use crate::channels::channel::Channel;

pub struct User {
    // address: Address;
    awaymsg: Option<String>,
    channels: Vec<Channel>,
    hopcount: u32,
    hostname: String, // Passed in from another server, could be used for Virtual Host etc.
    nickname: String,
    username: String,
    realname: String,
    //userlevel - A | S | G | U?
    
}
