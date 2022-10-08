use insurance::traits::AcceptingInsurances;
use insurance::traits::SellableInsurances;
pub struct Cafe;

impl AcceptingInsurances for Cafe {
    fn get_valid_insurances(&self) -> Vec<String> {
        return vec!["flood".to_string(), "electrical".to_string()];
    }
    fn get_name(&self) -> String {
        return "Cafe".to_string();
    }
    fn be_visited(
        &self,
        building: &&dyn AcceptingInsurances,
        seller: &&dyn SellableInsurances,
    ) -> String {
        return seller.sell_services_insurances(building);
    }
}
