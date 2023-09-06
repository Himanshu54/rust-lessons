fn main() {
    println!("Hello, world!");

    another_function(5, 'w');
}


fn another_function(value: i32, label: char){
    println!("Another Function");

    println!("Value : {value}, label: {label}");

    let y: i32 = {
        let x: i32 = 4;
        x + 1       // Rust Comments
    };
    println!("value of y : {y}");
}
