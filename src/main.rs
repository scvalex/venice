#[macro_use]
extern crate venice;
extern crate docopt;
extern crate websocket;

use std::io;
use std::io::Write;
use std::thread;
use std::sync::{mpsc, Arc, Mutex, MutexGuard};
use std::sync::mpsc::channel;

use websocket::ws::receiver::Receiver;
use websocket::ws::sender::Sender;
use websocket::server::Connection;
use websocket::dataframe::DataFrame;
use websocket::client::Client;

use venice::*;
mod command;

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

fn channels_of_websocket<S, R>(client: Client<DataFrame, S, R>) ->
    (mpsc::Sender<websocket::Message>, mpsc::Receiver<String>)
    where S: 'static + Sender<DataFrame> + Send, R: 'static + Receiver<DataFrame> + Send
{
    use websocket::Message;
    let (mut sender, mut receiver) = client.split();
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
    let (tx_out, rx_out) = channel();
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
                    match tx_out.send(text) {
                        Ok(()) => (),
                        Err(_) => {
                            let _ = tx1.send(Message::Close(None));
                            return;
                        }
                    }
                }
                Message::Pong(..) | Message::Binary(..) => {
                    elog!("received unexpected message: {:?}", msg);
                    let _ = tx1.send(Message::Close(None));
                    return;
                }
            }
        }
    });
    (tx, rx_out)
}

fn handle_client_connection<R, W>(conn: Connection<R, W>, server: MutexGuard<Server>)
    where R: 'static + std::io::Read + Send, W: 'static + std::io::Write + Send
{
    use websocket::Message;
    use command::Command;
    let req = conn.read_request().unwrap();
    let resp = req.accept();
    let (tx, rx) = channels_of_websocket(resp.send().unwrap());
    loop {
        match rx.recv() {
            Err(_) => return,
            Ok(str) => {
                println!("Received: {}", str);
                let cmd = command::parse(&str);
                match cmd {
                    None => {
                        tx.send(Message::Text("unknown command".to_string())).unwrap();
                    }
                    Some(Command::ListUsers) => {
                        let users = server.users();
                        let users = format!("{:?}", users);
                        tx.send(Message::Text(users)).unwrap();
                    }
                }
            }
        }
    }
}

fn run_server(port: u16) {
    println!("starting server on {}", port);
    let server = Server::new();
    let server = Arc::new(Mutex::new(server));
    for conn in websocket::Server::bind(("0.0.0.0", port)).unwrap() {
        let server = server.clone();
        thread::spawn(move || {
            let server = server.lock().unwrap();
            handle_client_connection(conn.unwrap(), server);
        });
    }
}

fn run_cli(url: String) {
    use websocket::Message;
    println!("connecting to {}", url);
    let url = websocket::client::request::Url::parse(&url).unwrap();
    let req = websocket::Client::connect(url).unwrap();
    let resp = req.send().unwrap();
    let () = resp.validate().unwrap();
    let (tx, rx) = channels_of_websocket(resp.begin());
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let () = tx.send(Message::Text(line.trim().to_string())).unwrap();
        match rx.recv() {
            Err(_)  => return,
            Ok(str) => println!("{}", str),
        }
    }
}

fn main() {
    // CR scvalex: Docopt is ridiculously slow.  Replace it with something else.
    use docopt::Docopt;
    use std::fs::File;
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
