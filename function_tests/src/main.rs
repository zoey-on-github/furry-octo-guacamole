fn main() {
    println!("damn.");
    new_function(5, 'f');
    print()
}

fn new_function(y:u64,f:char) { 
    println!("isn't code that executes not-top to bottom kinda weird to you, or is that just me?, also here's an variable: {y}, here's another: {f}");
    let z = {
        let e = 4;
        e + 2
    };
    println!("{z}")
}

fn five() -> u64 {
    5
}

fn print() {
    let s = five();
    println!("{s} is the number after 4");
}
