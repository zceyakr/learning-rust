mod data_types;
mod loops;
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
macro_rules! five_times {
    ($x:expr) => (5 * $x);
}

pub fn macros() {
    assert_eq!(25, five_times!(2 + 3));
}
