# ctf-pwn
[![crate](https://img.shields.io/crates/v/ctf-pwn.svg)](https://crates.io/crates/ctf-pwn)

Pwn utilities for Rust.

## Features

### Pipe
* [x] Converting TCP Stream or Process to Pipe
* [x] Conditional reading
* [x] Bridging pipe to stdout/stdin
* [x] Payload crafter
* [x] Ansi event support for ncurses like shells

### Shell
* [ ] Intel x86
* [ ] Amd x64
* [ ] Arm
* [ ] Risc-V

### Binary Parsing
* [x] Elf
* [ ] PE

## Examples

### Connecting to tcp stream
```rs
let mut pipe = TcpPipe::connect("127.0.0.1:1337").await?;
```

### Spawning new process
```rs
let mut pipe = ProcessPipe::from_app("ls").await?;

let mut pipe = ProcessPipe::from_app_args("ls", ["-l", "-a"]).await?;
```

### Generic loading
```rs
let mut pipe = Pipe::new(stdin(), stdout());
```

### Reading examples
```rs
let data: Vec<u8> = pipe.recv().await?;
let data: Vec<u8> = pipe.recv_until("Name:", false).await?;
let data: Vec<u8> = pipe.recv_until([0x01, 0x02, 0x03], false).await?;
let data: String = pipe.recv_line_utf8().await?;
let data: AsciiString = pipe.recv_line_ascii().await?;
```

### Regex
```rs
let data = pipe.recv_until(r"(Ok)|(Error)", true).await?;
let flag = pipe.recv_regex_utf8(r"HTB\{[^\}]+\}").await?;
```

### Interactive shell
```rs
pipe.interactive_shell().await?;
```

### Ncurses support

#### Sending ansi commands
```rs
pipe.write_ansi_command(ansi::Down).await?;
pipe.write_ansi_command(ansi::Right).await?;
pipe.write_ansi_command(ansi::Enter).await?;
```

#### Ansi event based interactive shell
```rs
pipe.interactive_ansi().await?;
```


### Payload
```rs
let payload = Payload::builder()
    .recv_until("> ", false)
    .push("1")
    .push("\n")
    .send()
    .recv_until("Insert card's serial number: ", false)
    .push_line("%4919x%7$hn")
    .send()
    .recv_regex_utf8(r"HTB\{[^\}]+\}")
    .build();

let flag = pipe.payload(payload).await?;
println!("{flag}");
```

### Elf
```rust
let elf = Elf::parse("app_path").await?;
let got: &HashMap<String, u64> = elf.got();
let plt: &HashMap<String, u64> = elf.plt();
let symbols: &HashMap<String, Symbol> = elf.symbols();
let dynamic_symbols: &HashMap<String, Symbol> = elf.dynamic_symbols();
```