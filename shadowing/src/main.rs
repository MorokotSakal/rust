fn main() {
    
    
    let mut x = 10;

    {

        let x = 15; // temperary change the data to 15
    }

    let x = "X is a string";

    println!("x is {}", x);

    let x = true;

    println!("x is {}", x);
}
