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
use websocket::Message;

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
        println!("starting server on {}", port);
        let server = websocket::Server::bind(("0.0.0.0", port)).unwrap();
        for connection in server {
            thread::spawn(move || {
                let req = connection.unwrap().read_request().unwrap();
                let resp = req.accept();
                let client = resp.send().unwrap();
                let (mut sender, mut receiver) = client.split();
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
                        Message::Pong(..) | Message::Binary(..) => {
                            elog!("received binary data");
                            let _ = sender.send_message(Message::Close(None));
                            return;
                        }
                    }
                }
            });
        }
    } else if args.get_bool("cli") {
        let port = args.get_str("--port").parse::<u16>().unwrap();
        let host = args.get_str("--host");
        let url = format!("ws://{}:{}", host, port);
        println!("connecting to {}", url);
        let url = websocket::client::request::Url::parse(&url).unwrap();
        let req = websocket::Client::connect(url).unwrap();
        let resp = req.send().unwrap();
        let () = resp.validate().unwrap();
        let client = resp.begin();
        let (mut sender, mut receiver) = client.split();
        thread::spawn(move || {
            for msg in receiver.incoming_messages::<Message>() {
                println!("{:?}", msg);
            }
        });
        loop {
            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();
            let msg = Message::Text(line);
            let () = sender.send_message(msg).unwrap();
        }
    } else {
        unreachable!();
    }
}
