fn main() {
    let mut counter = 0;

    let result: i32 = loop{
        counter += 2;
        if counter == 10 {
            break counter * 2;
    }
};
    println!("The result is {result}");       
}
