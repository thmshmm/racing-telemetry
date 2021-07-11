use std::net::UdpSocket;
use std::thread;

mod forza;
mod utils;

const BIND_ARRD: &str = "0.0.0.0:7878";

fn main() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind(BIND_ARRD)?;
        println!("Listening on {}", BIND_ARRD);

        let mut buf = [0; forza::MESSAGE_SIZE];

        loop {
            match socket.recv_from(&mut buf) {
                Ok((amt, _src)) => {
                    thread::spawn(move || {
                        let data = forza::parse(&mut buf[..amt]);
                        println!(
                            "Speed: {}km/h, Engine RPM: {}, Gear: {}",
                            (data.speed * 3.6) as i32,
                            (data.current_engine_rpm) as i32,
                            data.gear
                        )
                    });
                }
                Err(err) => {
                    eprintln!("Err: {}", err);
                    break;
                }
            }
        }
    }
    Ok(())
}
