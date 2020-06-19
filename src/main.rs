use std::io;

fn main() {
    println!(" ");
    println!(":::::::::::::: FIBONNACI ::::::::::::::");
    println!(" ");

    let n = get_position();
    let result = fibonnaci_number_at(n);

    println!("Fibonnaci number at the postion {} is: {}", n, result);
}

fn get_position() -> u64 {
    let position:u64;

    loop {
        println!("Enter the number position in the Fibonnaci sequence (n > 0):");
        println!(" ");

        let mut n:String = String::new();

        io::stdin().read_line(&mut n)
            .expect("Failed to read the input!");

        position = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!(" ");
                println!("Invalid value! Position must be greater than zero.");
                println!(" ");
                continue;
            }
        };

        println!(" ");
        println!("You have selected the Fibonnaci number at the position: {}", position);
        break;
    }

    position
}

fn fibonnaci_number_at(n:u64) -> u64 {
    if n == 0 {
        n
    } else {
        // recursion is nice, but it appears that it ends with stack overflow for larger values of n
        // simply too large number of recursions; for loop would probably work fine in that case
        n + fibonnaci_number_at(n - 1)
    }
}
