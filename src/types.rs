pub type Tokens = Vec<(String, String)>;
pub type Redirection = (String, String, String);

#[derive(Debug)]
pub struct Command {
    pub tokens: Tokens,
    pub redirects: Vec<Redirection>,
}

#[derive(Clone, Debug, Default)]
pub struct CommandResult {
    pub status: i32,
    pub stdout: String,
    pub stderr: String,
}

impl CommandResult {
    pub fn new() -> CommandResult {
        CommandResult {
            status: 0,
            stdout: String::new(),
            stderr: String::new(),
        }
    }
}
