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
        loop {
            let socket = UdpSocket::bind("0.0.0.0:4041").expect("couldn't bind to address");
            let mut buf = [0; 10];
            let (_, _) = socket.recv_from(&mut buf)
                                                .expect("Didn't receive data");

            println!("{:?}", buf);
        }
    }); 
    
    child.join().unwrap();
    child2.join().unwrap();
}

fn process(socket: &UdpSocket, input: String) {
    match input.as_ref() {
        "" => {
            ()
        },
        "/probe" => {
            let message = b"P";
            socket.send_to(message, "255.255.255.255:4041").expect("couldn't send data");
            ()
        },
        _ => {
            let message = format!("{}{}", "M", input);
            socket.send_to(&message.into_bytes(), "255.255.255.255:4041").expect("couldn't send data");           
            ()
        }
    }
}
