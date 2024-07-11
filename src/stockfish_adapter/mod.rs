use std::env;
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

pub struct StockfishAdapter {
    process: Child,
    stdin: Option<ChildStdin>,
    stdout: Option<ChildStdout>,
}

impl StockfishAdapter {
    pub fn new() -> StockfishAdapter {
        let path = env::current_dir().unwrap();
        let mut child = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .arg("/C")
                .arg(path.join("windows.exe"))
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg("./ubuntu")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("failed to execute process")
        };

        println!("child pid: {}", child.id());

        let input = child.stdin.take().expect("failed to get stdin");
        let output = child.stdout.take().expect("failed to get stdout");

        StockfishAdapter {
            process: child,
            stdin: Some(input),
            stdout: Some(output),
        }
    }
    pub fn pid(&self) -> u32 {
        self.process.id()
    }
    pub fn kill(&mut self) {
        self.process.kill().expect("failed to kill process");
    }
    pub fn status(&mut self) -> bool {
        let sin = self.stdin.as_mut().unwrap();
        writeln!(sin, "isready").expect("failed to write to stdin");
        sin.flush().expect("failed to flush stdin");
        let sout = self.stdout.as_mut().unwrap();
        let reader = BufReader::new(sout);
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    if line.starts_with("readyok") {
                        return true;
                    }
                }
                Err(err) => eprintln!("Error reading line: {}", err),
            }
        }
        false
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
    pub fn legal_moves(&mut self) -> Vec<String> {
        let sout = self.stdout.as_mut().unwrap();
        let reader = BufReader::new(sout);
        let sin = self.stdin.as_mut().unwrap();
        writeln!(sin, "go perft 1").expect("failed to write to stdin");
        sin.flush();
        let mut legal_moves = Vec::new();
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    println!("{}", line);
                    if line.len() == 0 {
                        break;
                    }
                    let colon_index = line.find(":");
                    if colon_index.is_none() {
                        break;
                    }
                    let moves = &line[0..colon_index.unwrap()];
                    legal_moves.push(moves.to_string());
                }
                Err(err) => eprintln!("Error reading line: {}", err),
            }
        }
        legal_moves
    }
}
