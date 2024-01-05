# ctf-pwn
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
* [ ] Elf
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
let data: String = pipe.recvus_line().await?;
let data: AsciiString = pipe.recvas_line().await?;
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
let mut payload = Payload::new();
payload
    .recv_until("Name: ")
    .push("test")
    .send()
    .push_ansi_command(ansi::Down)
    .send()
    .push_ansi_command(ansi::Enter)
    .send()
    .recv_until("Something: ")
    .fill("A", 60)
    .send();

pipe.payload(&payload).await?;
```
