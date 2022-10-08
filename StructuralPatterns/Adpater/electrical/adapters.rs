use electrical::sockets::USSocket;
use electrical::traits::HasVoltage;

pub struct EUToUSAdapter<'a> {
    pub socket: &'a USSocket,
}

impl<'a> HasVoltage for EUToUSAdapter<'a> {
    fn get_voltage(&self) -> u8 {
        return self.socket.voltage * 2 - 20;
    }
}
