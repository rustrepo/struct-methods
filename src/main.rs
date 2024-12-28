
//Struct is defined like this, remember struct keyword is snake case, and name is Camel case.
//within the struct, one can define different types of variables.

struct Rectangle {
    height: u8,
    width: u8,
}

//usually we don't implement functions directly on struct, instead use Methods. 
//methods ensures that a dedicated function is handling the struct. 
//this enhances the readability, configurability and ensure type safety in Rust.

impl Rectangle { //methods are defined using keyword impl and followed by the name of Struct.

    fn area(&self) -> u8 { //Always, the parameter used by method is self type.
        self.width * self.height
    }

     //in some cases a function inside method can accept multiple parameters, but all of the must be of same type.
    fn compare(&self, second_rect: Rectangle) -> bool {

        self.width > second_rect.width

    }


}

fn main () {
    //this is how we define a variable of Struct and initialise it with values.
    let rect1 = Rectangle {
        height: 7,
        width: 9,
    };

    let rect2 = Rectangle {
        height: 5,
        width: 8,
    };

    println!("Total area is {}", rect1.area()); //This is how we call a method with single self parameter.
    println!("Is the area of Rect 1 greater than rect 2 {}", rect1.compare(rect2));
    //this is how we call method with two parameters

}