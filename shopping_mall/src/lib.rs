pub mod mall;

pub use mall::*;

pub fn biggest_store(Mall: &mall::Mall) -> (String, mall::Store) {
    let mut max: u64 = 0;
    let mut storename: String = String::new();
    let mut floorn: String = String::new();

    for (floorname, floor) in Mall.floors.clone() {
        for (name, store) in floor.stores {
            if store.square_meters >= max {
                max = store.square_meters;
                storename = name;
                floorn = floorname.clone();
            }
        }
    }
    (
        storename.clone(),
        Mall.floors
            .get(&floorn)
            .unwrap()
            .stores
            .get(&storename)
            .unwrap()
            .clone(),
    )
}
pub fn highest_paid_employee(Mall: &mall::Mall) -> Vec<mall::Employee> {
       let mut max: f64 = 0.0;
    let mut employe_vec :  Vec<mall::Employee> = vec![];
    for (_, floor) in Mall.floors.clone() {
        for (_, store) in floor.stores {
            for (_, employe) in store.employees {
                if employe.salary >= max {
                    max = employe.salary;
                }
            }
        }
    }
    for (_, floor) in Mall.floors.clone() {
        for (_, store) in floor.stores {
            for (_, employe) in store.employees {
                if employe.salary == max {

                     employe_vec.push(employe);
                }
            }
        }
    }
  employe_vec
}
pub fn nbr_of_employees(Mall: &mall::Mall) -> usize {
    let mut nmb_of_employer = Mall.guards.len() ;
    for (_, floor) in Mall.floors.clone() {
        for (_, store) in floor.stores {
            nmb_of_employer += store.employees.len();
        }
    }

    nmb_of_employer
}
pub fn check_for_securities(Mall: &mut mall::Mall, guard: Vec<(String, Guard)>) {
    let mut nmb_of_guard = Mall.guards.len();
    let mut Mall_square_meter: u64 = 0;
    for (_,floor) in Mall.floors.clone(){
        Mall_square_meter += floor.size_limit ;

    }
  for g in guard {
    if Mall_square_meter  as usize / 200 > nmb_of_guard   {
       
        Mall.hire_guard(g.0, g.1);
        nmb_of_guard += 1 ;

    }else {
        return
    }
  }

}
pub fn cut_or_raise(Mall: &mut mall::Mall) {
    for (_, floor) in Mall.floors.iter_mut() {
        for (_, store) in floor.stores.iter_mut() {
            for (_, employ) in store.employees.iter_mut() {
                if employ.working_hours.1 - employ.working_hours.0 >= 10 {
                    employ.raise( employ.salary * 0.1);
                } else {
                    employ.cut(employ.salary * 0.1);
                }
            }
        }
    }
}



