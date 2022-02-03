use crate::lexer::{Commands, Command};
use std::io;
use std::io::{ErrorKind, Read};
use std::ops::Add;

pub struct Interpreter<'a> {
    program: &'a Commands,
    arr: [u8; 30_000],
    pointer: usize,
    output: &'a mut dyn io::Write
}

impl<'a> Interpreter<'a> {
    pub fn new(commands: &'a mut Commands, output: &'a mut dyn io::Write) -> Self {
        Self { program: commands, arr: [0; 30_000], pointer: 0, output}
    }
    
    pub fn run(&mut self) {
        self.execute_commands(self.program.clone());
    }
    
    fn execute_commands(&mut self, commands: Commands) {
        commands.for_each(|command| {
            match command {
                Command::Increment => {
                    self.arr[self.pointer] += 1;
                }
                Command::Decrement => {
                    self.arr[self.pointer] -= 1;
                }
                Command::Right => {
                    self.pointer += 1;
                    if self.pointer >= 30_000 {
                        self.pointer -= 30_000;
                    }
                }
                Command::Left => {
                    if self.pointer == 0 {
                        self.pointer = 29_999;
                    } else {
                        self.pointer -= 1;
                    }
                }
                Command::Output => {
                    let _ = write!(self.output, "{}", self.arr[self.pointer] as char);
                }
                Command::Input => {
                    let mut buffer: [u8; 1] = [0];
                    if let Err(error) = io::stdin().read_exact(&mut buffer) {
                        if error.kind() != ErrorKind::UnexpectedEof {
                            panic!("Couldn't read stdin: {}", error)
                        }
                    }
                    self.arr[self.pointer] = buffer[0];
                }
                Command::Loop(commands) => {
                    self.execute_loop(&commands)
                }
                Command::StartLoop => {
                    panic!("Unexpected start loop token")
                }
                Command::EndLoop => {
                    panic!("Unexpected end loop token")
                }
                Command::Ignore => {
                
                }
            }
        })
    }
    
    fn execute_loop(&mut self, commands: &Commands) {
        loop {
            if self.arr[self.pointer] == 0 {
                break;
            }
            self.execute_commands(commands.clone());
        }
    }
}