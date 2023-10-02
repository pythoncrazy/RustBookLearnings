fn main() {
    println!("Hello, world!");
    let mut x: i64 = 3; // The only use is to be printed, and I love type annotation, it's so nice
    println!("x is a variable, and is {x}");
    x = 4;
    println!("x is now: {x}");

    //Section on Consts

    const PI: f64 = 3.14159; //It does go on forever, but nobody asked buddy. quit yappin buckaroo
    let radius: f64 = 5.0;
    let area: f64 = PI * f64::powf(radius, 2.0);
    println!("The area of a circle with radius 4 is: {area}");

    //Section on Booleans

    let t: bool = true;
    println!("The boolean is {t}"); // So it is possible to print the boolean directly, pretty cool
}
