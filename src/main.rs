#[macro_use]
extern crate venice;
extern crate docopt;
extern crate websocket;

use std::io;
use std::io::Write;
use std::fs::File;
use std::thread;

use websocket::ws::receiver::Receiver;
use websocket::ws::sender::Sender;
use websocket::server::Connection;

use docopt::Docopt;

use venice::*;

static USAGE: &'static str = "
Usage: venice data-pack <file>
       venice server [options]
       venice cli [options]
       venice --help

Options:
    -h, --help            Show this message.
    -p PORT, --port=PORT  The server's port. [default: 8000]
    -h HOST, --host=HOST  The server's host. [default: localhost]

Commands:
    data-pack   Load and print a data pack.
    server      Run the server.
";

fn handle_client_connection<R, W>(conn: Connection<R, W>)
    where R: std::io::Read, W: std::io::Write
{
    use websocket::Message;
    let req = conn.read_request().unwrap();
    let resp = req.accept();
    let (mut sender, mut receiver) = resp.send().unwrap().split();
    for msg in receiver.incoming_messages::<Message>() {
        let msg = match msg {
            Ok(msg) => msg,
            Err(msg) => {
                elog!("connection error: {}", msg);
                let _ = sender.send_message(Message::Close(None));
                return;
            }
        };
        match msg {
            Message::Close(_) => {
                let _ = sender.send_message(Message::Close(None));
                return;
            }
            Message::Ping(data) => {
                let _ = sender.send_message(Message::Pong(data));
            }
            Message::Text(text) => {
                println!("received {:?}", text);
            }
            Message::Pong(_) | Message::Binary(_) => {
                elog!("received unexpected message: {:?}", msg);
                let _ = sender.send_message(Message::Close(None));
                return;
            }
        }
    }
}

fn run_server(port: u16) {
    println!("starting server on {}", port);
    let server = websocket::Server::bind(("0.0.0.0", port)).unwrap();
    for conn in server {
        thread::spawn(move || {
            handle_client_connection(conn.unwrap());
        });
    }
}

fn run_cli(url: String) {
    use std::sync::mpsc::channel;
    use websocket::Message;

    println!("connecting to {}", url);
    let url = websocket::client::request::Url::parse(&url).unwrap();
    let req = websocket::Client::connect(url).unwrap();
    let resp = req.send().unwrap();
    let () = resp.validate().unwrap();
    let (mut sender, mut receiver) = resp.begin().split();
    let (tx, rx) = channel();
    thread::spawn(move || {
        loop {
            let msg = match rx.recv() {
                Ok(msg) => msg,
                Err(_) => return,
            };
            match msg {
                Message::Close(_) => {
                    let _ = sender.send_message(msg);
                    return;
                }
                _ => (),
            }
            let () = sender.send_message(msg).unwrap();
        }
    });
    let tx1 = tx.clone();
    thread::spawn(move || {
        for msg in receiver.incoming_messages::<Message>() {
            let msg = match msg {
                Ok(msg) => msg,
                Err(msg) => {
                    elog!("connection error: {}", msg);
                    let _ = tx1.send(Message::Close(None));
                    return;
                }
            };
            match msg {
                Message::Close(_) => {
                    let _ = tx1.send(Message::Close(None));
                    return;
                }
                Message::Ping(data) => {
                    let _ = tx1.send(Message::Pong(data));
                }
                Message::Text(text) => {
                    println!("{}", text);
                }
                Message::Pong(..) | Message::Binary(..) => {
                    elog!("received unexpected message: {:?}", msg);
                    let _ = tx1.send(Message::Close(None));
                    return;
                }
            }
        }
    });
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let msg = Message::Text(line.trim().to_string());
        let () = tx.send(msg).unwrap();
    }
}

fn main() {
    let args =
        Docopt::new(USAGE)
        .and_then(|dopt| dopt.parse())
        .unwrap_or_else(|e| e.exit());
    if args.get_bool("data-pack") {
        let dp = DataPack::load(&mut File::open(args.get_str("<file>")).unwrap());
        println!("data_pack: {}", dp);
    } else if args.get_bool("server") {
        let port = args.get_str("--port").parse::<u16>().unwrap();
        run_server(port);
    } else if args.get_bool("cli") {
        let port = args.get_str("--port").parse::<u16>().unwrap();
        let host = args.get_str("--host");
        let url = format!("ws://{}:{}", host, port);
        run_cli(url);
    } else {
        unreachable!();
    }
}
