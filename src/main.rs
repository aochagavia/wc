#![feature(ascii_ctype)]

use std::ascii::AsciiExt;
use std::error::Error;
use std::io::{self, Read};

fn main() {
    let wc = run().expect("Failed to run");
    println!("{}", wc);
}

fn run() -> Result<u64, Box<Error>> {
    // Lock stdin before we start reading
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut counter = WordCounter::new();

    // Note: can you spot the performance pitfall of using stdin.lines()?
    let mut buffer = [0; 4096];
    loop {
        let len = stdin.read(&mut buffer).expect("Failed to read from stdin");
        if len == 0 {
            break;
        }

        counter.feed_bytes(&buffer[0..len]);
    }

    Ok(counter.get_wc())
}

struct WordCounter {
    word_count: u64,
    state: State
}

#[derive(Clone, Copy)]
enum State {
    ParsingWord,
    ParsingWhitespace
}

impl WordCounter {
    fn new() -> WordCounter {
        WordCounter {
            word_count: 0,
            state: State::ParsingWhitespace
        }
    }

    fn feed_bytes(&mut self, bytes: &[u8]) {
        // Note: we assume ASCII
        for &b in bytes {
            if b.is_ascii_whitespace() {
                // Whitespace seen!
                match self.state {
                    // If we were parsing a word, increase the word count
                    State::ParsingWord => {
                        self.word_count += 1;
                        self.state = State::ParsingWhitespace;
                    }
                    _ => {}
                }


            } else {
                // No whitespace!
                self.state = State::ParsingWord;
            }
        }
    }

    fn get_wc(&self) -> u64 {
        self.word_count
    }
}
