use std::io;

fn main() {
    let mut temp = String::new();
    let mut temp_type = String::new();

    println!("What is the temperature you would like to convert (in numbers)");
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read users temp input");

    let temp: i32 = temp.trim().parse().unwrap();

    loop {
        println!("Would you like to convert the temp {temp}, to (f)arenheit or (c)elcious?");
        temp_type = "".to_string();
        io::stdin()
            .read_line(&mut temp_type)
            .expect("Failed to read users temp type input");

        temp_type = temp_type.trim().to_string();

        if temp_type == "f"
            || temp_type == "farenheit"
            || temp_type == "c"
            || temp_type == "celcious"
        {
            break;
        }
    }

    temp_type = temp_type.trim().to_string();

    println!("You will convert {temp} into {temp_type}");

    if temp_type == "f" || temp_type == "farenheit" {
        println!("Temp in farenheit is {}.", (temp * 9 / 5) + 32);
    } else {
        println!("Temp in Celcius is {}.", (temp - 32) * 5 / 9);
    }
}
