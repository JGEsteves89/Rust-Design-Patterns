use electrical::traits::HasVoltage;
pub struct USSocket {
    pub voltage: u8,
}

impl HasVoltage for USSocket {
    fn get_voltage(&self) -> u8 {
        return self.voltage;
    }
}
