use std::io;

// Represents the Temperature Scale of the value
enum TemperatureScale{
    Celsius,
    Fahrenheit,
    Kelvin
}

struct Temperature{
    value:f64, //f64 because this will allow decimal values in
    scale:TemperatureScale,
}
//Create a Temperature instance with value and scale.
impl Temperature {
    fn new(value: f64, scale: TemperatureScale) -> Self {
        Self { value, scale }
    }

    fn to_celsius(&self) -> f64 {
        match self.scale {
            TemperatureScale::Celsius => self.value,
            TemperatureScale::Fahrenheit => (self.value - 32.0) * 5.0 / 9.0,
            TemperatureScale::Kelvin => self.value - 273.15,
        }
    }

    fn to_fahrenheit(&self) -> f64 {
        match self.scale {
            TemperatureScale::Celsius => (self.value * 9.0 / 5.0) + 32.0,
            TemperatureScale::Fahrenheit => self.value,
            TemperatureScale::Kelvin => ((self.value - 273.15) * 9.0 / 5.0) + 32.0,
        }
    }

    fn to_kelvin(&self) -> f64 {
        match self.scale {
            TemperatureScale::Celsius => self.value + 273.15,
            TemperatureScale::Fahrenheit => (self.value - 32.0) * 5.0 / 9.0 + 273.15,
            TemperatureScale::Kelvin => self.value,
        }
    }
}

//Prompting the user to enter the temperature, current scale and scale to be converted

    fn main() {
        println!("Welcome to the Temperature Convertor!");
        println!("Press Enter the Temperature Value!");

        //read input value and store it in a mutable string variable.

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let value: f64 = input.trim().parse().expect("Please type a valid number!");


        //Choose a Valid Temperature Scale

        println!("Please enter the scale of Temperature \n
        1. Celsius => Enter 1 \n
        2. Fahrenheit => Enter 2\n
        3. Kelvin => Enter 3\n");

        let mut scale_input = String::new();
        io::stdin().read_line(&mut scale_input).expect("Failed to read line");
        let scale_value = match scale_input.trim() {
            "1" => TemperatureScale::Celsius,
            "2" => TemperatureScale::Fahrenheit,
            "3" => TemperatureScale::Kelvin,
            _ => {
                println!("Please enter a valid scale input. Please restart the program");
                return;
            }
        };

        // Create a Temperature instance

        let input_temperature = Temperature::new(value, scale_value);

        //Display the conversions

        println!("\nConversions:");
        println!("Temperature in Celsius: {:.2} C", input_temperature.to_celsius());
        println!("Temperature in Fahrenheit: {:.2} Fa", input_temperature.to_fahrenheit());
        println!("Temperature in Kelvin: {:.2} K", input_temperature.to_kelvin());
        println!("--------------");
    }

