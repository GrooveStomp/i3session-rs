extern crate i3ipc;

#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate serde;

use i3ipc::I3Connection;
use docopt::Docopt;

fn main() {
    let mut connection = I3Connection::connect().unwrap();

    println!("{}", connection.get_version().unwrap().human_readable);
}
