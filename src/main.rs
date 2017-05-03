use std::net::UdpSocket;
use std::io;
use std::io::Write;
use std::thread;

fn main () {
    let child = thread::spawn(move || {
        let socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");
        socket.set_broadcast(true).expect("set_broadcast call failed");

        print!("> ");
        io::stdout().flush().unwrap();

        loop {

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    input.pop();

                    process(&socket, input);
                    print!("> ");
                    io::stdout().flush().unwrap();
                }
                Err(error) => println!("error: {}", error),
            }
        }
    });

    let child2 = thread::spawn(move || {
        let socket = UdpSocket::bind("0.0.0.0:4041").expect("couldn't bind to address");
        loop {

            let mut buf_header = [0; 2];
            let (received_bytes, _source) = socket.recv_from(&mut buf_header)
                .expect("Didn't receive data");

            let mut left: u8  = std::str::from_utf8(&buf_header[1..2]).unwrap().parse().unwrap();
            
            while left > 0 {
                let mut buffer = [0; 1];
                let (received_bytes2, _source) = socket.recv_from(&mut buffer).expect("didn't receive data TODO");
                left = left - received_bytes2 as u8;
                println!("{}", left);
            }
            
            //println!("{}", std::str::from_utf8(&buf[0..received_bytes2]).unwrap());
            io::stdout().flush().unwrap();
        }
    });
    
    child.join().unwrap();
    child2.join().unwrap();
}

fn process(socket: &UdpSocket, input: String) {
    match input.as_ref() {
        "" => {
        },
        _ => {
            let size = input.len();
            println!("{}", size);
            let message = format!("{}{}{}", "M", size, input);
            socket.send_to(&message.into_bytes(), "255.255.255.255:4041").expect("couldn't send data");
        }
    }
    ()
}
