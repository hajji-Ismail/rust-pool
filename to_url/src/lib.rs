pub fn to_url(s: &str) -> String {
    let  str1 = s.to_string();
    str1.replace(" ","%20")

}