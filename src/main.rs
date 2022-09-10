use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use once_cell::sync::Lazy;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::{TcpListener, TcpStream},
    spawn,
};

static DICT: Lazy<Arc<Mutex<HashMap<String, String>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:6379").await.unwrap();

    loop {
        let (mut stream, _) = listener.accept().await.unwrap();
        spawn(async move {
            handle_client(&mut stream).await;
        });
    }
}

async fn handle_client(stream: &mut TcpStream) {
    let (rx, tx) = stream.split();
    let mut reader = BufReader::new(rx);
    let mut writer = BufWriter::new(tx);

    let mut commands = Vec::<String>::new();

    let mut line = String::new();

    loop {
        line.clear();
        commands.clear();

        let sz = reader.read_line(&mut line).await.unwrap();
        if sz == 0 {
            break;
        }

        let argsv: i32 = line[1..sz - 2].parse().unwrap();

        for _ in 0..argsv {
            reader.read_line(&mut line).await.unwrap();
            line.clear();
            reader.read_line(&mut line).await.unwrap();
            commands.push(line.trim_end().to_string().clone());
        }

        let result = parse_command(&commands);

        match result {
            Some(command) => {
                writer
                    .write(format!("${}\r\n{}\r\n", command.len(), command).as_bytes())
                    .await
                    .unwrap();
            }
            None => {
                writer.write(b"$-1\r\n").await.unwrap();
            }
        }

        writer.flush().await.unwrap();
    }
}

fn parse_command(command: &Vec<String>) -> Option<String> {
    match command[0].as_str() {
        "SET" => {
            let mut dict = DICT.lock().unwrap();
            dict.insert(command[1].clone(), command[2].clone());
            None
        }
        "GET" => {
            let dict = DICT.lock().unwrap();
            if let Some(value) = dict.get(&command[1]) {
                Some(value.to_owned())
            } else {
                None
            }
        }
        _ => {
            panic!("Unknown command");
        }
    }
}
