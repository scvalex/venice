extern crate venice;
extern crate docopt;
extern crate websocket;

use std::fs::File;
use std::thread;

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
                let mut client = resp.send().unwrap();
                let msg = Message::Text("Hello".to_string());
                let _ = client.send_message(msg).unwrap();
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
        resp.validate();
        let mut client = resp.begin();
        let msg : Message = client.recv_message().unwrap();
        println!("received {:?}", msg);
    } else {
        unreachable!();
    }
}
