fn main() {
    let x: u32 = 5u32;

    let y: u32 = {
        let x_squared: u32 = x * x;
        let x_cube: u32 = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    #[allow(unused_must_use)]
    let z: () = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
