use std::io;
use std::convert::TryInto;

fn main() {
    println!(" ");
    println!(":::::::::::::: FIBONNACI ::::::::::::::");
    println!(" ");

    let n = get_position();
    let result = fibonnaci_number_at(n);
    let result_recursive = recursive_fibonnaci_number_at(n);

    println!("(for loop)  - Fibonnaci number at the postion {} is: {}", n, result);
    println!("(recursive) - Fibonnaci number at the postion {} is: {}", n, result_recursive);
}

fn get_position() -> u8 {
    let mut position:u128;

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

        if position > 186 {
            // max position value is 186
            // results with 332825110087067562321196029789634457848
            println!("Too large value! Please try again with lower value of n. (n <= 186)");
            continue;
        }

        println!(" ");
        println!("You have selected the Fibonnaci number at the position: {}", position);
        break;
    }

    position.try_into().unwrap()
}

fn fibonnaci_number_at(n:u8) -> u128 {
    let mut previous:u128 = 0;
    let mut current:u128 = 1;
    let mut temp:u128;

    if n == 0 {
        return 0;
    }

    for _number in 1..n {
        temp = current;
        current = previous + current;
        previous = temp;
    }

    current
}

fn recursive_fibonnaci_number_at(n:u8) -> u128 {
    fn calculate(target:u8, pos:u8, previous:u128, current:u128) -> u128 {
        if target == 0 {
            0
        } else if target == 1 {
            1
        } else if pos == target {
            previous + current
        } else if pos == 0 {
            calculate(target, pos + 1, 0, 0)
        } else if pos == 1 {
            calculate(target, pos + 1, 0, 1)
        } else {
            calculate(target, pos + 1, current, previous + current)
        }
    }

    calculate(n, 0, 0, 0)
}
