fn main() {
    let tup = (92.8, 76, 100000);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);

    let _a = [1, 2, 3, 4, 5];

    let _b: [i32; 5] = [1, 2, 3, 4, 5];

    let c = [99; 5];

    let first = c[2];
    let second = c[3];

    println!("first: {}, second: {}", first, second);    
}
