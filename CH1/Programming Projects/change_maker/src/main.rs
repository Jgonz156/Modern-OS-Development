/*
A small rust program that makes coin denominations from a given total.
 */
use std::io;
fn main() {
    println!("Enter a total amount of US Dollars: ");
    let total: u64 = get_input(); 
    make_change(total);
}

/*
Read an unsigned 64 bit integer from stdin.
 */
fn get_input() -> u64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse::<u64>().unwrap()
}

/*
Make change from a given total.
 */
fn make_change(mut money: u64) {
    let denominations: [u64; 4]= [25, 10, 5, 1];
    for denomination in denominations {
        let mut count = 0;
        while money >= denomination {
            money -= denomination;
            count += 1;
        }
        println!("{} x {} {}", denomination, count, match denomination {
            25 => "quarters",
            10 => "dimes",
            5 => "nickels",
            1 => "pennies",
            _ => "unknown"
        });
    }
}