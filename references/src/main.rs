fn main() {

    let mut x = 10;

    //let xr = &x;

    {
        let dom = &mut x; //dom is now is mutable reference

        *dom += 1;
        println!("dom is {}", dom);
    }


    println!("x is {}", x);


}
