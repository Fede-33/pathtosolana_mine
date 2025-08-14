mod network;
use network::{connect, disconnect};
use network::*;

fn main() {
    println!("Iniciando programa");
    network::connect();
    disconnect();
    connect();
    process();
}

