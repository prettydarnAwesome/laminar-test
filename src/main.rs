use std::net;
use std::io::stdin;
use std::str;

const SERVER: &str = "127.0.0.1:27016";

const SERVER_BIND: &str = "127.0.0.1:27016";
const LOCAL_BIND: &str = "127.0.0.1:27015";

fn client() {
    let stdin = stdin();

    let mut socket = net::UdpSocket::bind(LOCAL_BIND).expect("couldn't bind to address!");
    socket.connect(SERVER).expect("couldn't connect to server");

    loop {
        let mut s = String::new();
        stdin.read_line(&mut s);

        socket.send(s.as_bytes()).expect("couldn't send data");
    }

    // let mut buf = [0; 100];
    // match socket.recv(&mut buf) {
    //     Ok(received) => println!("received {} bytes {:?}", received, &buf[..received]),
    //     Err(e) => println!("recv function failed: {:?}", e),
    // }
}

fn server() {
    let mut socket = net::UdpSocket::bind(SERVER_BIND).expect("couldn't bind to address!");

    let mut buf = [0; 100];
    loop {
        match socket.recv(&mut buf) {
            Ok(received) => println!("received {} bytes {:?}", received, str::from_utf8(&buf[..received])),
            Err(e) => println!("recv function failed: {:?}", e),
        }
    }
}

fn main() {
    let stdin = stdin();

    println!("Please type in `server` or `client`.");

    let mut s = String::new();
    stdin.read_line(&mut s);

    if s.starts_with("s") {
        println!("Starting server..");
        server();
    } else {
        println!("Starting client..");
        client();
    }
}