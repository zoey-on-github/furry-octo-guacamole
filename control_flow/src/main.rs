fn main() {
    let conditional_number = 6; 

    if conditional_number < 5 { 
        println!("condition was met");
    } else {
        println!("condition was not met");
    }

    let other_number = 4;
        if other_number % 3 == 0 {
            println!("number is disivible by 3");
        } else if other_number % 4 == 0 {
            println!("number is divisble by 4");
        } else if other_number % 6 == 0 {
            println!("number is divisble by 6");
        }
}
