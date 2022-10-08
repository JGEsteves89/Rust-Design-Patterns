use aviation::airport::Airport;
use aviation::traits::Landable;
pub struct Tower<'a> {
    pub airport: &'a mut Airport,
    pub current_time: u16,
}

impl Tower<'_> {
    pub fn land(&mut self, aircraft: &dyn Landable) -> String {
        if self.current_time < self.airport.landing_schedule {
            self.current_time = self.airport.landing_schedule
        }

        let result = format!("at {}: ", self.current_time)
            + &self.airport.set_landing(aircraft, self.current_time);
        self.current_time = self.current_time + aircraft.get_landing_time();
        return result;
    }
}
