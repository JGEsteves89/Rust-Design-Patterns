use insurance::traits::AcceptingInsurances;
use insurance::traits::SellableInsurances;
pub struct Home;

impl AcceptingInsurances for Home {
    fn get_valid_insurances(&self) -> Vec<String> {
        return vec!["fire".to_string(), "flood".to_string()];
    }
    fn get_name(&self) -> String {
        return "Home".to_string();
    }
    fn be_visited(
        &self,
        building: &&dyn AcceptingInsurances,
        seller: &&dyn SellableInsurances,
    ) -> String {
        return seller.sell_residencial_insurances(building);
    }
}
