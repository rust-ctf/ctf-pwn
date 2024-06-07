// #[macro_export]
// macro_rules! pipe {
//     ($stream:expr) => {{
//         let pipe: Pipe<_, _> = $stream.into();
//         pipe
//     }};
// }

// #[macro_export]
// macro_rules! tcp_pipe {
//     ($input:expr) => {{
//         let stream = TcpStream::connect($input).await.unwrap();
//         let pipe: TcpStreamPipe = stream.into();
//         pipe
//     }};
// }

// #[macro_export]
// macro_rules! shell_pipe {
//     ($program:expr $(, $arg:expr)*) => {{
//         let mut command = Command::new($program);
//         $(
//             command.arg($arg);
//         )*

//         ProcessPipe::spawn_command(command).unwrap()
//     }};
// }

// #[macro_export]
// macro_rules! tcp_shell_pipe {
//     (true, $input:expr, $(, $arg:expr)*) => {{
//         tcp_pipe!($input)
//     }};
//     (false, $program:expr, $(, $arg:expr)*) => {{
//         file_pipe!($program $(, $arg)*)
//     }};
// }
