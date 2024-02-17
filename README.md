# Mutable borrowing in a loop

The compiler error didn't help me solve a bug I had. I have a struct
which maintains some data and its producing some strings as I call a
read_line method. The obvious thing was to have read_line return a
&str to the mutable string in the struct. But I couldn't do that
now I have a mutable and immutable reference to the same struct.

So this code:
```
    let mut data = Data::new(gen_data(ln));
    while let Some(line) = data.read_line() {
        println!("{}: {}", data.line_number, line);
    }
```
Would cause this error:
```
$ cargo run
   Compiling exper-mut-borrowing-in-loop v0.1.0 (/home/wink/prgs/rust/myrepos/exper-mut-borrowing-in-loop)
error[E0502]: cannot borrow `data.line_number` as immutable because it is also borrowed as mutable
  --> src/main.rs:91:28
   |
92 |     while let Some(line) = data.read_line() {
   |                            ---- mutable borrow occurs here
93 |         println!("{}: {}", data.line_number, line);
   |                            ^^^^^^^^^^^^^^^^  ---- mutable borrow later used here
   |                            |
   |                            immutable borrow occurs here
   |
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0502`.
error: could not compile `exper-mut-borrowing-in-loop` (bin "exper-mut-borrowing-in-loop") due to 1 previous error
```

But if I just don't use the struct in the loop
```
    let mut data = Data::new(gen_data(ln));
    while data.read_line().is_some() {
        println!("{}: {}", data.line_number, data.line);
    }
```
or like this
```
    let mut data = Data::new(gen_data(ln));
    while let Some(_) = data.read_line() {
        println!("{}: {}", data.line_number, data.line);
    }
```
There is no problem.

It seems to me the problem is the `Some(line)` more than using
`data.read_line()` or `data.line_number`. Also the compiler says
that `line` is a mutable borrow in the `println!` but I think
an `Option<&str>` is immutable, so I'm confused.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
