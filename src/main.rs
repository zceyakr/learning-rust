mod loops;
mod data_types;
mod packages;

fn main() {
    println!("-------------------------");

    data_types::basic_types();

    println!("-------------------------");

    data_types::tuple();

    println!("-------------------------");

    data_types::shadowing();

    println!("-------------------------");

    loops::infinite();

    println!("-------------------------");

    loops::fizzbuzz();
    

    packages::ferris();

}
