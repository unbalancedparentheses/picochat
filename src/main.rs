// TODO
// when sending too much data, size gets printed. it is not beeing sent a u8, it is being sent as a string i think
// add nickname
// add colors to line
// check if use nanomsg, zeromq or similar
// add rooms, by default join lobby
// modularize
// print list of peers (exclude myself or print myself and state it is me)
// add /help
//     Help:          -h --help
//    Host IP:       -H --host {host IP}
//    Broadcast IP:  -B --broadcast {broadcast IP}
//    RPC Port:      -P --port {port}
// add encryption
// handle C-c https://github.com/Detegr/rust-ctrlc/blob/master/examples/readme_example.rs so that it returns 0
// if two instances are run, print address already in use
// check if tokyo is useful
// add one to one chat. creates a 1-1 room

use std::net::UdpSocket;
use std::io;
use std::io::Write;
use std::thread;
use std::time::Duration;
use std::process;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::SystemTime;
extern crate chrono;
use chrono::prelude::*;

fn main () {
    let peers = Arc::new(Mutex::new(HashMap::new()));

    let peers_sender = peers.clone();
    let sender = thread::spawn(move || {
        let socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");
        socket.set_broadcast(true).expect("set_broadcast call failed");

        print!(">>> ");
        io::stdout().flush().unwrap();

        loop {
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    input.pop();

                    process(&socket, input);
                    io::stdout().flush().unwrap();
                   // println!("peers {:?}", *peers_sender.lock().unwrap()); //TODO move this to the process of the sender when writting /peers
                }
                Err(error) =>
                    println!("error: {}", error),
            }
        }
    });

    let prober = thread::spawn(|| {
        let socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");
        socket.set_broadcast(true).expect("set_broadcast call failed");
        loop {
            let message = format!("P");
            socket.send_to(&message.into_bytes(), "255.255.255.255:4041").expect("couldn't send data");
            thread::sleep(Duration::from_millis(5000)); //TODO put in seconds
        }
    });

    let peers_rec = peers.clone();
    let receiver = thread::spawn(move || {
        let socket = UdpSocket::bind("0.0.0.0:4041").expect("couldn't bind to address");
        
        loop {
            let mut buffer = [0; 512];
            let (received_bytes, source) = socket.recv_from(&mut buffer)
                .expect("Didn't receive data");

            match std::str::from_utf8(&buffer[0..1]).unwrap() {
                "M" => {
                    println!("[{}] {}: {}", Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), source, std::str::from_utf8(&buffer[2..received_bytes]).unwrap());
                    io::stdout().flush().unwrap();
                    print!(">>> ");
                    io::stdout().flush().unwrap();
                }

                "P" => {
                    //println!("probe with source {}", source);
                    peers_rec.lock().unwrap().insert(source, SystemTime::now()); //TODO insert only seconds and IP
                    // clean hosts that have not received content for more than x seconds
                }

                _ =>  {
                }
            }
                
        }      
    });
    
    receiver.join().unwrap();
    sender.join().unwrap();
    prober.join().unwrap();
}

fn process(socket: &UdpSocket, input: String) {
    match input.as_ref() {
        "" => {
            print!(">>> ");
            io::stdout().flush().unwrap();            
        },

        "/quit" => {
            process::exit(0x0f00);
        },

        "/peers" => {
            //TODO print list of 
        },

        "/rooms" => {
        },
        
        "/help" => {
        },
        
        _ => {
            let size = input.len();
            let message = format!("M{}{}", size, input);
            socket.send_to(&message.into_bytes(), "255.255.255.255:4041").expect("couldn't send data");
        }
    }
    ()
}
