use crate::user::User;

pub async fn execute(user: &mut User, parts: &[&str]) {
    if let Some(nickname) = parts.get(1) {
        match user.nickname {
            Some(ref old_nickname) => {
                println!("User {} set their nickname to {}", old_nickname, nickname);
            }
            None => {
                // Registration?
                println!("User set their nickname to {}", nickname);
            }
        }
        user.nickname = Some(nickname.to_string());

        let nick_ack_message = format!("Nickname set to: {}\n", nickname);
        if let Err(e) = user.connection.write(nick_ack_message.as_bytes()).await {
            eprintln!(
                "Failed to send nickname acknowledgment to {}: {}",
                user.connection.addr, e
            );
        }
    } else {
        let error_message = "Error: No nickname provided. Usage: nick <nickname>\n";
        if let Err(e) = user.connection.write(error_message.as_bytes()).await {
            eprintln!("Failed to send error to {}: {}", user.connection.addr, e);
        }
    }
}
