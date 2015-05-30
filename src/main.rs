extern crate venice;
extern crate docopt;
extern crate rustc_serialize;

use std::fs::File;

use docopt::Docopt;

use venice::*;

static USAGE: &'static str = "
Usage: venice data-pack <file>
       venice server [options]
       venice --help

Options:
    -h, --help            Show this message.
    -p PORT, --port=PORT  The port to run the server on. [default: 8000]

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
        println!("server on {}", args.get_str("--port"));
    } else {
        unreachable!();
    }
}
