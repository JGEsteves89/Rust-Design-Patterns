mod aviation;
use aviation::aircrafts::Airplane;
use aviation::aircrafts::Helicopter;
use aviation::aircrafts::Jet;
use aviation::airport::Airport;
use aviation::tower::Tower;
use aviation::traits::Landable;

fn main() {
    let mut schedule_landings: Vec<&dyn Landable> = Vec::new();
    schedule_landings.push(&Helicopter);
    schedule_landings.push(&Jet);
    schedule_landings.push(&Airplane);

    let mut airport = Airport {
        landing_schedule: 0,
    };

    println!("");
    println!("Land where the aircrafts has to know the previous aircraft:");
    let mut landing_time: u16 = 0;
    for aircraft in &schedule_landings {
        println!(
            "\tLanding {} at {}: {}",
            aircraft.get_name(),
            landing_time,
            airport.set_landing(*aircraft, landing_time)
        );
        landing_time = landing_time + 100;
    }

    println!("");
    println!("Land with a tower as a mediator:");
    let mut tower = Tower {
        airport: &mut airport,
        current_time: 0,
    };
    for aircraft in &schedule_landings {
        println!(
            "\tLanding {} {}",
            aircraft.get_name(),
            tower.land(*aircraft)
        );
    }
}
