fn celsius_to_farenheit(x: f64)->f64{
        (x*9.0/5.0) + 32.0
}

fn fibonacci(x: i32)-> i32{
    if x == 0{
        0
    }else if x == 1 {
        1
    }else{
        fibonacci(x-1) + fibonacci(x-2)
    }
}



fn main() {
    let mut temp: f64 = 25.0;
    let mut number: i32 = 6;
    temp = celsius_to_farenheit(temp);
    println!("Temperature in farenheit: {temp}");
    number = fibonacci(number);
    println!("{number}");
}
