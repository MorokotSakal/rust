fn main() {

    //let numbers = 30..50;

    //for i in numbers
    //{
        //println!("The number is {}", i);
    //}

    let animals = vec!["Rabbit", "Dog", "Cat"]; // create vector

    /*for a in animals.iter() // when using vector
    {
        println!("The animals name is {}", a);
    }*/

    for (index, a) in animals.iter().enumerate() // when using vector
    {
        println!("The index is {} and the animals name is {}", index, a);
    }


}
