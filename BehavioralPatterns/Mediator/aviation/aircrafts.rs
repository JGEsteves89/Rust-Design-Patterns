use aviation::traits::Landable;
pub struct Helicopter;
pub struct Jet;
pub struct Airplane;

impl Landable for Helicopter {
    fn get_landing_time(&self) -> u16 {
        return 200;
    }
    fn get_name(&self) -> String {
        return String::from("Helicopter");
    }
}
impl Landable for Jet {
    fn get_landing_time(&self) -> u16 {
        return 90;
    }
    fn get_name(&self) -> String {
        return String::from("Jet");
    }
}
impl Landable for Airplane {
    fn get_landing_time(&self) -> u16 {
        return 120;
    }
    fn get_name(&self) -> String {
        return String::from("Airplane");
    }
}
