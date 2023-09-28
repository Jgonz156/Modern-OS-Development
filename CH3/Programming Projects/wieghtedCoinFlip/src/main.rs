use std::io;

fn main() {
    let n: u64 = get_unsigned_input();
    let k: u64 = get_unsigned_input();
    let p: f64 = get_float_input();
    
    println!("Recursive: {}", recursive_wieghted_coin_flip(n, k, p));
    println!("Dynamic  : {}", dynamic_wieghted_coin_flip(n, k, p));
}

fn get_float_input() -> f64 {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input = input.trim().to_string();
    input.parse::<f64>().unwrap()
}

fn get_unsigned_input() -> u64 {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input = input.trim().to_string();
    input.parse::<u64>().unwrap()
}

fn recursive_wieghted_coin_flip(n: u64, k: u64, p: f64) -> f64 {
    if n < k {
        return 0.0;
    }
    if n == k {
        return p.powf(k as f64);
    }
    return recursive_wieghted_coin_flip(n-1, k, p) + p.powf(k as f64)* (1.0 - p) * (1.0 - recursive_wieghted_coin_flip(n-k-1, k, p));
}

fn dynamic_wieghted_coin_flip(number_of_flips: u64, heads_in_a_row: u64, coin_weight: f64) -> f64 {
    let mut table: Vec<Vec<f64>> = vec![vec![0.0; number_of_flips as usize + 1]; heads_in_a_row as usize + 1];

    for n in 0..number_of_flips as usize + 1 {
        table[0][n] = 1.0;
    }

    for k in 0..heads_in_a_row as usize + 1 {
        table[k][0] = 0.0;
    }

    for k in 1..heads_in_a_row as usize + 1 {
        for n in 1..number_of_flips as usize + 1 {
            if n == k {
                table[k][n] = coin_weight.powf(k as f64);
                continue;
            } if n < k {
                table[k][n] = 0.0;
            } else {
                table[k][n] = table[k][n-1] + coin_weight.powf(k as f64) * (1.0 - coin_weight) * (1.0 - table[k][n-k-1]);
            }
        }
    }

    return table[heads_in_a_row as usize][number_of_flips as usize];
}