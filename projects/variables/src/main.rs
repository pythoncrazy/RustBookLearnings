fn main() {
    println!("Hello, world!");
    let mut x: i64 = 3; // The only use is to be printed, and I love type annotation, it's so nice
    println!("x is a variable, and is {x}");
    x = 4;
    println!("x is now: {x}");
    const PI: f64 = 3.14159; //It does go on forever, but nobody asked buddy. quit yappin buckaroo
    let radius: f64 = 5.0;
    let area: f64 = PI * f64::powf(radius, 2.0);
    println!("The area of a circle with radius 4 is: {area}");
}
