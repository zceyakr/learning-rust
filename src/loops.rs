pub fn infinite(){
    let mut count = -6;
    println!("Infinite until stopped.");
    loop {
        count += 1;

        if count == 0 {
            println!("zero");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("Lets stop here");

            // Exit this loop
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

        // Increment counter
        n += 1;
    }

    println!("\nThis is a for loop\n");

    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..15 {
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
