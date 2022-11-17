use std::{net::{UdpSocket, Ipv4Addr, SocketAddrV4}, time::Duration, io::Error, thread, env};

fn send(size: usize) -> Result<(), Error> {
    let socket:UdpSocket = UdpSocket::bind("0.0.0.0:0")?;
    let addr:SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(255, 255, 255, 255), 3000);
    
    socket.set_read_timeout(Some(Duration::new(5, 0)))?;
    socket.set_broadcast(true)?;

    loop {
        match socket.send_to(&vec![255; size], addr) {
            Ok(n) => {
                if n != size {
                        println!( "Sent the wrong number of bytes")
                }
            },
            Err(e) => {
                println!("{}", e)
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Missing amount of threads to spawn");
        return;
    }

    let thread_count = args.get(1).unwrap().parse::<usize>();

    match thread_count {
        Ok(threads) => {
            if args.len() > 2 {
            let packet_size = args.get(2).unwrap().parse::<usize>();
                match packet_size {
                Ok(size) => {
                    for _ in 1..threads {
                        thread::spawn(move || send(size));
                    }
                    loop {}
                }
                Err(e) => {
                    println!("{}", e)
                }
                } } else {
                    for _ in 1..threads {
                        thread::spawn(|| send(508));
                    }
                    loop {}
                }
            } 
        Err(e) => {
            println!("{}", e)
        }
    }
}

