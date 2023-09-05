fn main() {
    let str1 = String::from("ruststudent");
    let str2 = String::from("rustclass");
    
    if compareString(str1.as_str(), str2.as_str()) {
        println!("前者的字典序更大");
    }
    else {
        println!("后者的字典序更大");
    }
}

fn compareString(x: &str, y: &str) -> bool{
    let mut i:i32;
    let mut xing = x.to_string();
    let mut ying = y.to_string();

    let mut length = xing.len();
    if length > ying.len() {
        length = ying.len();
    }

    for i in 0..length-1 {
        let xchar = xing.remove(0);
        let ychar = ying.remove(0);

        if xchar < ychar {
            return false; 
        }
        else if xchar > ychar {
            return true;
        }
    }

    return true;
}