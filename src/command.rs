#[derive(Debug)]
pub enum Command {
    ListUsers,
}

pub fn parse(str: &str) -> Option<Command> {
    let words: Vec<&str> = str.split(' ').filter(|str| str.len() > 0).collect();
    // CR scvalex: Use slice paterns when they become available in stable.
    if words.len() == 2 && words[0] == "list" && words[1] == "users" {
        Some(Command::ListUsers)
    } else {
        None
    }
}
