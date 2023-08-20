use crossterm::{
    cursor,
    event::{self, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::io::Write;
use std::time::Duration;
use std::{
    io::{self, BufRead},
    sync::mpsc::{self, Sender},
    thread::{self, JoinHandle},
};

use super::*;

enum ShellCommand {
    End,
    KeyPress(KeyEvent),
    Line(String),
}

impl<T> Pipe<T>
where
    T: Write + Read
{
    fn shell_line_sender(sender: Sender<ShellCommand>) -> JoinHandle<()> {
        thread::spawn(move || {
            let stdin = io::stdin();

            for line in stdin.lock().lines() {
                match line {
                    Ok(line) => {
                        //TODO: Return result
                        sender.send(ShellCommand::Line(line)).unwrap();
                    }
                    Err(_) => {}
                }
            }
            sender.send(ShellCommand::End).unwrap();
        })
    }

    fn shell_line_sender_keys(sender: Sender<ShellCommand>) -> JoinHandle<()> {
        thread::spawn(move || {
            terminal::enable_raw_mode().unwrap();
            loop {
                match event::read() {
                    Ok(event::Event::Key(k)) => {
                        if k.modifiers == KeyModifiers::CONTROL && k.code == KeyCode::Char('d') {
                            sender.send(ShellCommand::End).unwrap();
                            break;
                        } else {
                            sender.send(ShellCommand::KeyPress(k)).unwrap();
                        }
                    }
                    _ => {}
                }
            }
            terminal::disable_raw_mode().unwrap();
        })
    }

    pub fn interactive(&mut self, by_key: bool) {
        let (input_tx, input_rx) = mpsc::channel();
        let writer_thread: JoinHandle<()> = if by_key {
            Self::shell_line_sender_keys(input_tx)
        } else {
            Self::shell_line_sender(input_tx)
        };

        loop {
            match input_rx.try_recv() {
                Ok(ShellCommand::End) => {
                    break;
                }
                Ok(ShellCommand::Line(data)) => {
                    self.write_line(data).unwrap();
                }
                Ok(ShellCommand::KeyPress(key)) => {
                    if let KeyEventKind::Press = key.kind {
                        self.write_key(key.code).unwrap();
                    }
                }
                Err(mpsc::TryRecvError::Empty) => match self.read_buffer() {
                    Ok((buf, len)) => Self::encode_and_print_ascii(&buf[..len]),
                    Err(_) => {}
                },
                Err(_e) => {} // panic!("{_e:?}")
            }
        }

        //TODO: Return result
        writer_thread.join().unwrap();
    }

    fn encode_and_print_ascii(data: &[u8]) {
        data.iter().for_each(|&byte| {
            let c = byte as char;
            print!("{c}");
            // if c.is_ascii() && c.is_ascii_graphic() {
            //     print!("{c}");
            // } else {
            //     print!("\\x{:02X}", byte);
            // }
            io::stdout().flush().unwrap();
        });
    }
}
