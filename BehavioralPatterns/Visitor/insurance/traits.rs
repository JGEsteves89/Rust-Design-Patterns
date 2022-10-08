pub trait SellableInsurances {
    fn sell_all_insurances(&self, building: &&dyn AcceptingInsurances) -> String;
    fn sell_residencial_insurances(&self, building: &&dyn AcceptingInsurances) -> String;
    fn sell_industry_insurances(&self, building: &&dyn AcceptingInsurances) -> String;
    fn sell_services_insurances(&self, building: &&dyn AcceptingInsurances) -> String;
}
pub trait AcceptingInsurances {
    fn get_valid_insurances(&self) -> Vec<String>;
    fn get_name(&self) -> String;
    fn be_visited(
        &self,
        building: &&dyn AcceptingInsurances,
        seller: &&dyn SellableInsurances,
    ) -> String;
}
