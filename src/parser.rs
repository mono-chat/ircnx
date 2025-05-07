use regex::Regex;
use std::{collections::HashMap, vec};

#[derive(Debug)]
pub struct IrcMessage {
    tags: Option<HashMap<String, IrcTag>>,
    source: Option<String>,
    command: String,
    parameters: Vec<String>,
}

// TODO: Currently, we parse tags even if the capability is not enabled.
// This is not ideal, as it adds unnecessary overhead. We should only parse tags if the capability is enabled.

pub fn parse_irc_message(message: &str) -> Result<IrcMessage, String> {
    let re = Regex::new(r"^ *(?:@(?<tags>[^ ]+) +)?(?::(?<source>[^ ]+) +)?(?<command>[^ ]+)(?: +(?<middle>[^: ][^ ]*(?: +[^: ][^ ]*)*))?(?: +:(?<trailing>.*))? *$").unwrap();
    let caps = re.captures(message).ok_or("Invalid message format")?;

    let tags = match caps.name("tags") {
        Some(m) => parse_irc_tags(m.as_str()),
        None => None,
    };

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
        tags,
        source,
        command,
        parameters,
    })
}

#[derive(Debug)]
pub struct IrcTag {
    client_prefix: Option<String>,
    key: String,
    key_name: String,
    value: Option<String>,
    vendor: Option<String>,
}

fn parse_irc_tags(tags_str: &str) -> Option<HashMap<String, IrcTag>> {
    let re = Regex::new(r"(?:(?<key>(?<client_prefix>\+)?(?:(?<vendor>[^=;/]*)/)?(?<key_name>[^=;]*))(?:=(?<value>[^;]*))?)+").unwrap();

    let mut tags = HashMap::new();
    let caps = re.captures_iter(tags_str);

    for tag in caps {
        let client_prefix = tag.name("client_prefix").map(|m| m.as_str().to_string());
        let key = tag.name("key").map(|m| m.as_str().to_string());
        let key_name = tag.name("key_name").map(|m| m.as_str().to_string());

        let value = tag
            .name("value")
            .map(|m| {
                let val = m.as_str();
                if val.is_empty() {
                    None // Normalize empty value to missing value.
                } else {
                    Some(unescape_irc_tag_value(val))
                }
            })
            .flatten();

        // Check key_name is valid (matches [A-Za-z0-9-]+).
        // TODO: Check if vendor is a valid DNS hostname.
        if let Some(key) = key {
            if key_name.as_ref().map_or(true, |k| {
                Regex::new(r"^[A-Za-z0-9-]+$").unwrap().is_match(k)
            }) {
                tags.insert(
                    key.clone(),
                    IrcTag {
                        client_prefix,
                        key: key,
                        key_name: key_name.unwrap_or_default(), // key_name should always be Some() when key is Some()
                        value: value,
                        vendor: tag.name("vendor").map(|m| m.as_str().to_string()),
                    },
                );
            }
        }
    }
    match tags.is_empty() {
        true => None,
        false => Some(tags),
    }
}

fn escape_irc_tag_value(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            ';' => "\\:".to_string(),
            ' ' => "\\s".to_string(),
            '\\' => "\\\\".to_string(),
            '\r' => "\\r".to_string(),
            '\n' => "\\n".to_string(),
            _ => c.to_string(),
        })
        .collect()
}

fn unescape_irc_tag_value(input: &str) -> String {
    let mut output = String::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c == '\\' {
            chars.next(); // Consume the backslash
            match chars.next() {
                Some(':') => output.push(';'),
                Some('s') => output.push(' '),
                Some('\\') => output.push('\\'),
                Some('r') => output.push('\r'),
                Some('n') => output.push('\n'),
                Some(other) => {
                    output.push('\\');
                    output.push(other);
                }
                None => break,
            }
        } else {
            output.push(c);
            chars.next();
        }
    }

    output
}
