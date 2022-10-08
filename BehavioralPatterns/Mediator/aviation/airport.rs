use aviation::traits::Landable;
pub struct Airport {
    pub landing_schedule: u16,
}

impl Airport {
    pub fn set_landing(&mut self, aircraft: &dyn Landable, landing_time: u16) -> String {
        if landing_time >= self.landing_schedule {
            self.landing_schedule = landing_time + aircraft.get_landing_time();
            return String::from("Landing OK");
        }
        return String::from("Landing cannot be scheduled!!!");
    }
}
