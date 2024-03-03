use std::io::{stdin, BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::{env, thread};

pub struct stockfish_adapter {
    process: Child,
    stdin: Option<ChildStdin>,
    stdout: Option<ChildStdout>,
}

impl stockfish_adapter {
    pub fn new() -> stockfish_adapter {
        let path = env::current_dir().unwrap();
        let mut child = Command::new("cmd")
            .arg("/C")
            .arg(path.join("stockfish_15_1.exe"))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute child");

        println!("child pid: {}", child.id());

        let input = child.stdin.take().expect("failed to get stdin");
        let output = child.stdout.take().expect("failed to get stdout");

        stockfish_adapter {
            process: child,
            stdin: Some(input),
            stdout: Some(output),
        }
    }
    pub fn pid(&self) -> u32 {
        self.process.id()
    }
    pub fn newgame(&mut self) {
        let sin = self.stdin.as_mut().unwrap();
        writeln!(sin, "ucinewgame").expect("failed to write to stdin");
        sin.flush().expect("failed to flush stdin");
    }
    pub fn set_level(&mut self, level: u32) {
        let sin = self.stdin.as_mut().unwrap();
        writeln!(sin, "setoption name Skill Level value {}", level)
            .expect("failed to write to stdin");
        sin.flush().expect("failed to flush stdin");
    }
    pub fn set_fen(&mut self, fen: &str) {
        let sin = self.stdin.as_mut().unwrap();
        writeln!(sin, "position fen {}", fen).expect("failed to write to stdin");
        sin.flush().expect("failed to flush stdin");
    }
    fn go(&mut self, mut depth: u8) {
        if depth == 0 {
            depth = 5; //default depth
        }
        let sin = self.stdin.as_mut().unwrap();
        writeln!(sin, "go depth {}", depth).expect("failed to write to stdin");
        sin.flush().expect("failed to flush stdin");
    }
    pub fn bestmove(&mut self) -> String {
        self.go(8);
        let sout = self.stdout.as_mut().unwrap();
        let reader = BufReader::new(sout);
        let mut bestmove = String::new();
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    if line.starts_with("bestmove") {
                        let toks = line.split_whitespace().collect::<Vec<&str>>();
                        bestmove = toks[1].to_string();
                        break;
                    }
                }
                Err(err) => eprintln!("Error reading line: {}", err),
            }
        }
        bestmove
    }
}
