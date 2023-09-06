fn main() {
    let number = 3;

    if number < 3 {
            println!("true");
    } else {
        println!("false");
    }

    foo_bar(15);

    let condition = true;

    let y = if condition {5} else {6};
    println!("value of y : {y}");
}

fn foo_bar(val: i32) {
    if val%3 == 0 {
        println!("Foo");
} else if val%5 == 0 {
    println!("Bar");
}}
