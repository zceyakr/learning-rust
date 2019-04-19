pub fn basic_types(){
    let x = 1;

    let y = 4.5;

    let boolean = 5 < 10;

    const CONSTANT:i32 = 55555;

    let character = 'a';

    let face = '\u{1F600}';

    println!("{:?}", (x, y, boolean, CONSTANT, character, face));

    let mut name = String::from("Zackery ");

    println!("Hello {}", name);

    name.push('\u{1F600}');

    println!("Hello {}", name);

    name.push_str(" Hitchcock");

    println!("Hello {}", name);

}

pub fn tuple(){
    let tup = (500, 6.4, -20);

    let (_x, _y, _z) = tup;

    println!("The value of y is: {}", _z);
}

pub fn shadowing(){
    //shadowing
    let x = 3;

    let x = x + 2;

    let x = x * 2;

    assert_eq!(x, 10);

    println!("The shadowed value is {}", x);
}
