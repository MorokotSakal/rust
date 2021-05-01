fn main() {
    
    let tup1 = (20, "Rust", 3.4, 25, 30, 35);

    let tup2 = (20, "Rust", 3.4, false, (1,4,7)); //nested tuples

    let tup3 = (45, 6.7, "Computer");

    let (a, b, c) = tup3;


    println!("{}", tup1.2);

    println!("{}", (tup2.4).2);

    println!("a is {}", a);

    println!("b is {}", b);

    println!("c is {}", c);


}
