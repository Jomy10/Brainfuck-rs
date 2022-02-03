use std::fmt::Formatter;
use std::str::Chars;

/// Remove comments
pub fn pre_lex(input: &str) -> String {
    let mut chars = input.chars().collect::<Vec<char>>();
    chars.retain(|c| matches!(c, '+' | '-' | '>' | '<' | '.' | ',' | '[' | ']'));
    return chars.iter().map(|c| c).collect::<String>();
}

/// Brainfuck Lexer. Converts string input to [`Commands`](struct@Commands).
pub struct Lexer<'a> {
    chars: Chars<'a>
}

impl<'a> Lexer<'a> {
    pub fn new(program: &'a str) -> Self {
        Lexer { chars: program.chars() }
    }

    pub fn parse(&mut self) -> Commands {
        let mut commands = Commands::new();
        loop {
            if let Some(command) = self.parse_next() {
                if command == Command::EndLoop {
                    panic!("Unexpected closing loop character.\nTIP: have you forgotten a `[`?")
                } else {
                    commands.add(command);
                }
            } else {
                break;
            }
        }

        commands
    }

    /// Parses the next character, returning the corresponding command.
    fn parse_next(&mut self) -> Option<Command> {
        if let Some(char) = self.chars.next() {
            let command = match char {
                '+' => Command::Increment,
                '-' => Command::Decrement,
                '<' => Command::Left,
                '>' => Command::Right,
                '.' => Command::Output,
                ',' => Command::Input,
                '[' => self.parse_loop(),
                ']' => Command::EndLoop,
                _ => Command::Ignore
            };
            Some(command)
        } else {
            None
        }
    }

    fn parse_loop(&mut self) -> Command {
        let mut commands = Commands::new();
        loop {
            if let Some(command) = self.parse_next() {
                if command == Command::EndLoop {
                    break;
                } else {
                    commands.add(command);
                }
            } else {
                panic!("Compilation error: unexpected end-of-input in loop.\nTIP: You may be missing a closing `]`.");
            }
        }

        Command::Loop(commands)
    }
}

#[derive(PartialEq, Clone)]
pub enum Command {
    Increment,
    Decrement,
    Left,
    Right,
    Output,
    Input,
    Loop(Commands),
    StartLoop,
    EndLoop,
    Ignore
}

impl std::fmt::Debug for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Command::Increment => "+",
            Command::Decrement => "-",
            Command::Left => "<",
            Command::Right => ">",
            Command::Output => ".",
            Command::Input => ",",
            Command::Loop(_) => "L",
            Command::StartLoop => "[",
            Command::EndLoop => "]",
            Command::Ignore => "#"
        })
    }
    
}

#[derive(PartialEq, Debug, Clone)]
pub struct Commands {
    pub commands: Vec<Command>,
    index: usize
}

impl Commands {
    pub fn new() -> Self {
        Commands { commands: Vec::new(), index: 0 } 
    }

    pub fn add(&mut self, command: Command) {
        self.commands.push(command);
    }
}

impl<'a> Iterator for Commands {
    type Item = Command;

    // TODO: return a reference instead with a streaming iterator (https://stackoverflow.com/a/30422716/14874405)
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(val) = self.commands.get(self.index.increment()) {
            Some(val.clone())
        } else {
            None
        }
    }
}

pub trait Incrementer {
    /// Increments a value and returns the previous value
    fn increment(&mut self) -> Self;
}

impl Incrementer for usize {
    fn increment(&mut self) -> usize {
        // Not ideal, but oh well
        let i = self.clone();
        *self += 1;
        i
    }
}