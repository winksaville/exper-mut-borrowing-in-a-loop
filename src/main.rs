pub struct Data {
    data: Vec<u8>,
    idx: usize,
    line: String,
    line_number: u32,
}

impl Data {
    pub fn new(data: Vec<u8>) -> Data {
        Data {
            data,
            idx: 0,
            line: String::new(),
            line_number: 0,
        }
    }

    pub fn read_line(&mut self) -> Option<&str> {
        self.line.clear();
        while self.idx < self.data.len() {
            let c = self.data[self.idx];
            self.idx += 1;
            if c == b'\n' {
                break;
            } else {
                self.line.push(c as char);
            }
        }

        if self.line.is_empty() {
            None
        } else {
            self.line_number += 1;
            Some(&self.line)
        }
    }

    pub fn read_line1(&mut self) -> bool {
        self.line.clear();
        while self.idx < self.data.len() {
            let c = self.data[self.idx];
            self.idx += 1;
            if c == b'\n' {
                self.line_number += 1;
                return true;
            } else {
                self.line.push(c as char);
            }
        }

        if self.line.is_empty() {
            false
        } else {
            self.line_number += 1;
            true
        }
    }

    pub fn by_ref(&mut self) -> &mut Data {
        self
    }
}

fn main() {
    fn gen_data(ln: i32) -> Vec<u8> {
        format!("line {}\nline {}", ln, ln + 1)
            .to_string()
            .as_bytes()
            .to_vec()
    }

    let mut ln = 1;
    // Does not compile
    //   $ cargo run
    //      Compiling exper-mut-borrowing-in-loop v0.1.0 (/home/wink/prgs/rust/myrepos/exper-mut-borrowing-in-loop)
    //   error[E0502]: cannot borrow `data.line_number` as immutable because it is also borrowed as mutable
    //     --> src/main.rs:91:28
    //      |
    //   92 |     while let Some(line) = data.read_line() {
    //      |                            ---- mutable borrow occurs here
    //   93 |         println!("{}: {}", data.line_number, line);
    //      |                            ^^^^^^^^^^^^^^^^  ---- mutable borrow later used here
    //      |                            |
    //      |                            immutable borrow occurs here
    //      |
    //      = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
    //
    //   For more information about this error, try `rustc --explain E0502`.
    //   error: could not compile `exper-mut-borrowing-in-loop` (bin "exper-mut-borrowing-in-loop") due to 1 previous error
    //let mut data = Data::new(gen_data(ln));
    //while let Some(line) = data.read_line() {
    //    println!("{}: {}", data.line_number, line);
    //}

    // if we just don't use the struct all is well
    let mut data = Data::new(gen_data(ln));
    while let Some(line) = data.read_line() {
        println!("{}", line);
    }

    // Or if we just ignore the output and use the struct all is well
    ln += 2;
    let mut data = Data::new(gen_data(ln));
    while data.read_line().is_some() {
        println!("{}: {}", data.line_number, data.line);
    }

    // Alternative just return a boolean and always use the struct
    ln += 2;
    let mut data = Data::new(gen_data(ln));
    while data.read_line1() {
        println!("{}: {}", data.line_number, data.line);
    }
}
