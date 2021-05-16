fn main() {
    
    
    let my_string = String::from("How's it going? My name is Dom.");

    // length

    println!("Lenght: {}", my_string.len());

    // is empty?
    println!("String is empty? {}", my_string.is_empty());

    for token in my_string.split_whitespace()
    {
        println!("{}", token);
    }
}
