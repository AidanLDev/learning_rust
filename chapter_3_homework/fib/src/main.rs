use std::io;

fn main() {
    /*
     *  1 1 2 3 5 8
     */
    let mut fib: Vec<u32> = Vec::new();
    fib.push(1);
    fib.push(1);

    println!("How many numbers of the fibananci sequence do you want to generate? (must be greater than 2 as it always starts 1, 1)");
    let mut num_of_fib = String::new();

    io::stdin()
        .read_line(&mut num_of_fib)
        .expect("Failed to read num_of_fib input from the user");

    let num_of_fib: u32 = num_of_fib.trim().parse().unwrap();

    println!("Getting the {num_of_fib} in the fibinaci sequence.");
    let mut index = 3;
    while index < num_of_fib + 1 {
        index = index + 1;

        let next_num = fib[fib.len() - 2] + fib[fib.len() - 1];

        fib.push(next_num);
    }

    for fib_number in fib {
        println!("{fib_number}");
    }
}
