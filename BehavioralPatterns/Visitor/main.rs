mod insurance;
use insurance::industries::ShoesFactory;
use insurance::residencial::Home;
use insurance::seller::InsuranceSeller;
use insurance::services::Cafe;
use insurance::traits::AcceptingInsurances;
use insurance::traits::SellableInsurances;
fn main() {
    let mut city: Vec<&dyn AcceptingInsurances> = Vec::new();
    city.push(&Cafe);
    city.push(&Home);
    city.push(&ShoesFactory);

    let seller: Box<&dyn SellableInsurances> = Box::new(&InsuranceSeller);

    println!("");
    println!("Selling all insurances to every building:");
    for building in &city {
        println!(
            "Selling in {} : {}",
            building.get_name(),
            seller.sell_all_insurances(&building)
        )
    }

    println!("");
    println!("Visiting every building with a specific insurance:");
    for building in &city {
        println!(
            "Selling in {} : {}",
            building.get_name(),
            building.be_visited(building, &*seller)
        )
    }
}
