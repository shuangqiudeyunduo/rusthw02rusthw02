fn main() {
    let mut str1:Vec<char> = vec!['a','b','c','d','e'];
    let mut i:u32;

    for i in addone(str1){
        print!("{} ",i);
    }
    println!();
}

fn addone (stradd:Vec<char>) -> Vec<char>{
    let result:Vec<char> = stradd.iter().map(|&c| char::from_u32(c as u32 + 1).unwrap()).collect();
    return result;
}