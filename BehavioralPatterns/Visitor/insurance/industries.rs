use insurance::traits::AcceptingInsurances;
use insurance::traits::SellableInsurances;
pub struct ShoesFactory;

impl AcceptingInsurances for ShoesFactory {
    fn get_valid_insurances(&self) -> Vec<String> {
        return vec!["fire".to_string(), "electrical".to_string()];
    }
    fn get_name(&self) -> String {
        return "Shoes Factory".to_string();
    }
    fn be_visited(
        &self,
        building: &&dyn AcceptingInsurances,
        seller: &&dyn SellableInsurances,
    ) -> String {
        return seller.sell_industry_insurances(building);
    }
}
