use electrical::traits::HasVoltage;
pub struct Computer {
    pub voltage: u8,
}

impl Computer {
    pub fn connect<T: HasVoltage>(&self, socket: &T) -> String {
        if self.voltage == socket.get_voltage() {
            return String::from("Successfull connection, no explosion....yet");
        }
        return String::from("Cannot connect, diferent voltage");
    }
}
