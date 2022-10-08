mod electrical;
use electrical::adapters::EUToUSAdapter;
use electrical::appliances::Computer;
use electrical::sockets::USSocket;

fn main() {
    let pc = Computer { voltage: 220 };
    println!("Computer voltage: {}", pc.voltage);

    let socket = USSocket { voltage: 120 };
    println!("US socket voltage: {}", socket.voltage);

    println!("Try to connect directly: {}", pc.connect(&socket));

    let adapter = EUToUSAdapter { socket: &socket };
    println!("Try to connect with adapter: {}", pc.connect(&adapter));
}
