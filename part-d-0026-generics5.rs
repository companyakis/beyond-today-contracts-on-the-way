fn main() {

    let coor_data1 = Coordinates1 {x: 12, y: -5}; // i32, default
    
    let coor_data2: Coordinates1<u8>= Coordinates1 {x: 12, y: 45}; // u8

    let coor_data3 = Coordinates2 { x: 4.5, y: 11}; // f64, ,i32, default

    let coor_data4: Coordinates2<u16, f32> = Coordinates2 { x: 1200, y: 100.23}; // u16, f32

}

// types of all members are T

struct Coordinates1<T> {
    x: T,
    y: T
}

// members can have different types, T and U

struct Coordinates2<T, U> {
    x: T,
    y: U
}




