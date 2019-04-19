pub fn basic_types(){
    const NUM:u32 = 1234;
    println!("Constant: {:?}", NUM );
}
pub fn tuple() {
    let tup = (500, 6.4, -20);

    let (_x, _y, _z) = tup;

    println!("The value of y is: {}", _z);
}
pub fn shadowing(){
    //shadowing
    let x = 3;

    let x = x + 2;

    let x = x * 2;

    println!("The shadowed value is {}", x);
}
