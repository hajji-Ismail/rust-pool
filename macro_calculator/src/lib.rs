
pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut cals = 0.0;
    let mut carbs = 0.0;
    let mut proteins = 0.0;
    let mut fats = 0.0;
    for food in foods {
        let cal_str = &food.calories.1[0..food.calories.1.len() - 4];
        let calf: f64 = cal_str.parse().expect("Invalid number");
        cals += food.nbr_of_portions * calf;
        carbs += food.carbs * food.nbr_of_portions;
        fats += food.fats * food.nbr_of_portions;
        proteins += food.proteins * food.nbr_of_portions;
    }
 
    
    let mut macro_calculator = json::JsonValue::new_object();
    macro_calculator["cals"]= (( cals* 100.0).round() / 100.0).into();
    macro_calculator["carbs"]= (( carbs* 100.0).round() / 100.0).into();
    macro_calculator["proteins"]= (( proteins* 100.0).round() / 100.0).into();
    macro_calculator["fats"]= (( fats* 100.0).round() / 100.0).into();
   
macro_calculator
  
   
}
