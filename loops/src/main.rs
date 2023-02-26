fn main() {
    let mut counter = 0;
    'counting: loop {
        println!("count = {counter}");
        let mut remaining_numbers = 10;

        loop {
            println!("remaining numbers = {remaining_numbers}");
                if remaining_numbers == 8 { 
                    break;
        }
            if counter == 3 {
                break 'counting;
            }
            remaining_numbers -= 1;
    }
    
    counter += 1;
}
println!("final count = {counter}");
}

