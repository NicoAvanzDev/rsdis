use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:6379").unwrap();

    loop {
        let (stream, _) = listener.accept().unwrap();
        thread::spawn(move || {
            handle_client(&stream);
        });
    }
}

fn handle_client(stream: &TcpStream) {
    let mut reader = BufReader::new(stream);
    let mut writer = BufWriter::new(stream);

    let mut commands = Vec::<String>::new();

    let mut line = String::new();

    loop {
        line.clear();
        commands.clear();

        let sz = reader.read_line(&mut line).unwrap();
        if sz == 0 {
            break;
        }

        let argsv: i32 = line[1..sz - 2].parse().unwrap();

        for _ in 0..argsv {
            reader.read_line(&mut line).unwrap();
            line.clear();
            reader.read_line(&mut line).unwrap();
            commands.push(line.clone());
        }

        let result = parse_command(&commands);

        match result {
            Some(command) => {
                writer
                    .write_all(format!("${}\r\n{}\r\n", command.len(), command).as_bytes())
                    .unwrap();
            }
            None => {
                writer.write_all(b"$-1\r\n").unwrap();
            }
        }

        writer.flush().unwrap();
    }
}

fn parse_command(command: &Vec<String>) -> Option<&str> {
    match command[0].trim_end() {
        "SET" => Some("OK"),
        "GET" => None,
        _ => {
            panic!("Unknown command");
        }
    }
}
