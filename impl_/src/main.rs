struct Rectangle 
{
    width: u32,
    height: u32
}

impl Rectangle
{
    fn print_description(&self)
    {
        println!("Rectangle: {} x {}", self.width, self.height);
    }

    /*fn is_square(&self) -> bool {
        self.width == self.height;
    }*/
}

fn main() {
    
    let mut my_rect = Rectangle { width: 10, height: 5};

    // Rectangle: 10x 5
    my_rect.print_description();

    my_rect.height = 10;

    //println!("Rectangle is a square: {}", my_rect.is_square());

}
