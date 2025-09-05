pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut initials: Vec<String> = vec![];
    for i in names {
        let name: Vec<&str> = i.split(' ').collect();
        let mut str1: String = Default::default();
        for nam in name {
            let initial: Vec<char> = nam.chars().collect();


            str1.push_str(&initial[0].to_string());
            str1.push_str(". ")

        }
       
          initials.push(str1.trim().to_string());
          str1.clear();
    }
    initials
}
