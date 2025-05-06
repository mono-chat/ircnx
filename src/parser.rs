use regex::Regex;
use std::{collections::HashMap, vec};

#[derive(Debug)]
pub struct IrcMessage {
    tags: Option<HashMap<String, String>>,
    source: Option<String>,
    command: String,
    parameters: Vec<String>,
}

// NOTE: We should only parse the tags if the capability has been negotiated.

pub fn parse_irc_message(message: &str) -> Result<IrcMessage, String> {
    let re = Regex::new(r"^ *(?:@(?<tags>[^ ]+) +)?(?::(?<source>[^ ]+) +)?(?<command>[^ ]+)(?: +(?<middle>[^: ][^ ]*(?: +[^: ][^ ]*)*))?(?: +:(?<trailing>.*))? *$").unwrap();
    let caps = re.captures(message).ok_or("Invalid message format")?;

    let source = match caps.name("source") {
        Some(m) => Some(m.as_str().to_string()),
        None => None,
    };

    let command = match caps.name("command") {
        Some(m) => m.as_str().to_string(),
        None => return Err("Command not found".to_string()),
    };

    let mut parameters = vec![];

    // Middle parameters are seperated by spaces, so we need to split them.
    if let Some(middle) = caps.name("middle") {
        middle.as_str().split(' ').for_each(|s| {
            if !s.is_empty() {
                // Ensure we don't add empty strings to the parameters vector (if multiple spaces are present)
                parameters.push(s.to_string());
            }
        });
    }

    // There can only be one trailing parameter, and it is the last part of the message, so we can just add it directly to the parameters vector
    if let Some(trailing) = caps.name("trailing") {
        parameters.push(trailing.as_str().to_string());
    }

    let parameters = parameters; // Shadowing, now immutable

    Ok(IrcMessage {
        tags: None,
        source,
        command,
        parameters,
    })
}
