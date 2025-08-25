fn main() {
    // to create a new value
    let mut cakes = 4;

    // to print the value
    println!("number of cakes: {cakes}" );

    //if you want specific formatting. p.e.
    let flowers: u8 = 5;

    /* WHY FORMATTING?
    Using `u8` (or any specific data type) in Rust has several advantages:
        1.Memory efficiency:
            A `u8` only takes up 1 byte (8 bits), so if you know your variable will 
            never be negative or greater than 255, you save memory compared to larger types like `u32` or `i32`.
         2. Safety:
            Rust prevents errors by forcing you to use the correct type. 
            If you declare a variable as `u8`, the compiler will warn you if you try 
            to assign a value outside the allowed range, helping you avoid hard-to-find bugs.
        3. Code clarity:
            Using specific types makes your code’s intent clearer. If you see `u8`, you know you’re only expecting small, positive values—like ages, quantities, bytes, etc.
        4. Interoperability:
            Many data formats and protocols (for example, binary files, images, networks) use bytes (`u8`) as the basic unit. Using `u8` makes it easier to work with this kind of data.

        Example:
        If you’re processing an image, each color can be represented as a value between 0 and 255. 
        Using `u8` is ideal for this.

        let red: u8 = 200;
        let green: u8 = 100;
        let blue: u8 = 50;
    */

    println!("number of flowers: {flowers}");

    // now, if you want sum of cakes and flowers
    let total = cakes + flowers as i32; // we need to cast flowers to i32 because cakes is i32
    println!("total items of cakes + flowers: {total}");  


    // if i want to change the value of cakes, make it mutable using 'mut' keyword
    cakes = 10; // now we can change the value
    println!("number of cakes after change: {cakes}" );
    

    

}
