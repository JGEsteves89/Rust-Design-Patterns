use insurance::traits::AcceptingInsurances;
use insurance::traits::SellableInsurances;
pub struct InsuranceSeller;

fn can_sell_this(acceptable: &Vec<String>, insurance: &String) -> bool {
    return acceptable.iter().any(|e| insurance.contains(e));
}
fn check_all_insurances(building: &&dyn AcceptingInsurances, acceptables: &Vec<String>) -> String {
    let acceptable = building.get_valid_insurances();
    for insurance in acceptables {
        if can_sell_this(&acceptable, &insurance) == false {
            return format!("Cannot sell insurance for {}!", insurance);
        }
    }
    return String::from("Sell went fine");
}

impl SellableInsurances for InsuranceSeller {
    fn sell_all_insurances(&self, building: &&dyn AcceptingInsurances) -> String {
        let all_insurances = vec![
            "fire".to_string(),
            "electrical".to_string(),
            "flood".to_string(),
        ];
        return check_all_insurances(building, &all_insurances);
    }
    fn sell_residencial_insurances(&self, building: &&dyn AcceptingInsurances) -> String {
        let all_insurances = vec!["fire".to_string(), "flood".to_string()];
        return check_all_insurances(building, &all_insurances);
    }
    fn sell_industry_insurances(&self, building: &&dyn AcceptingInsurances) -> String {
        let all_insurances = vec!["fire".to_string(), "electrical".to_string()];
        return check_all_insurances(building, &all_insurances);
    }
    fn sell_services_insurances(&self, building: &&dyn AcceptingInsurances) -> String {
        let all_insurances = vec!["electrical".to_string(), "flood".to_string()];
        return check_all_insurances(building, &all_insurances);
    }
}
