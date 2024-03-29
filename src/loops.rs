pub fn infinite(){
    let mut count = -6;
    println!("Infinite until stopped.");
    loop {
        count += 1;

        if count == 0 {
            println!("zero");

            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("Lets stop here");

            break;
        }
    }
}

pub fn fizzbuzz(){
    let mut n = 1;

    println!("This is a while loop\n");

    while n < 16 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        n += 1;
    }

    println!("\nThis is a for loop\n");

    for n in 1..16 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}
