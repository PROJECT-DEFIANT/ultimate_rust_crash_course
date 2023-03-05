pub fn inspect(str :&String) -> bool {
    println!("You have provided {}", str);
    if str.ends_with("s") {
        println!("This is a plural noun");
        return true
    }
    println!("This is a singular noun");
    return false
    
}


pub fn change(str: &mut String) -> &mut String{
    println!("You have provided {}", str);
    if !str.ends_with("s") {
        str.push_str("s")
    }
    return str;
}

pub fn eat(str: String) -> bool {
    return str.starts_with("b") && str.contains("a")
}


pub fn bedazzle(str: &mut String) -> &String {
    *str = "sparkly".to_string();
    return str;
}