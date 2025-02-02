fn main() {

    let new_rectangle = Rectangle::new_rect(15 as f32, 20.0);

    let area = new_rectangle.rect_area_calculator();

    println!("Rectangle area: {area}") // Rectangle area: 300

}

struct Rectangle {
    w: f32,
    h: f32
}

impl Rectangle {

    // Associated function, no &self parameter!
    fn new_rect(w: f32, h: f32) -> Self {
        Rectangle { w, h }
    }

    fn rect_area_calculator(&self) -> f32 {
        self.w * self.h
    }
    
}

/*
If there is no self parameter in the method, the method is an associated function

Usually used to initialize instances, 
call the associated function new to create an instance corresponding to the structure

*/

