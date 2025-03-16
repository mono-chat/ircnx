use crate::member::Member;

pub struct Channel {
    // key: Option<String>,
    // limit: Option<u32>,
    members: Vec<Member>,
    modes: String, // Could be a Vec<char> or an enum
    topic: Option<String>, // Could probably move to it's own struct if we want to store user/time   
}