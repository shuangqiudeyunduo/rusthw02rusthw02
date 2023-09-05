struct Buffer<T>{
    vect:Vec<T>
}

impl <T:std::ops::Add<Output = T> + Copy> Buffer<T>{
    fn sum(&self) -> T{
        let mut result = self.vect[0];
        let length = self.vect.len();
        for i in 1..length {
            result = result + self.vect[i];
        }
        return result;
    }
}
fn main() {
    let b = Buffer{
        vect:[1.9,2.0,3.0,5.0,4.7].to_vec(),
    };
    println!("The sum is {}",b.sum());
}
