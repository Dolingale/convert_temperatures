use std::io;

fn main() {
    println!("Select the conversion : \n
        1. Fahrenteit to Celsius
        2. Celsius to Fahrenteit");

    let mut conversion = String::new();

    io::stdin()
        .read_line(&mut conversion)
        .expect("Failed to read line");

    let conversion: u32 = conversion.trim().parse().unwrap();

    if conversion != 1 && conversion != 2 {
        println!("This value is not correct, only 1 and 2 are available");
    }
    else {
        println!("Enter the temperature you want to convert");

        let mut temperature = String::new();

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: f32 = temperature.trim().parse().unwrap();

        if conversion == 1
        {
            let result: f32 = (temperature - 32.0) * 5.0/9.0;
            println!("{}°F is {}°C", temperature, result);
        }
        else {
            let result: f32 = (temperature * 9.0/5.0) + 32.0;
            println!("{}°C is {}°F", temperature, result);
        };
    }
}
