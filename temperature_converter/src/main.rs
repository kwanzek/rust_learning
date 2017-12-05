extern crate clap;

use clap::{Arg, App};

fn main() {
    let matches = App::new("TemperatureConverter")
        .version("0.1.0")
        .arg(Arg::with_name("from")
            .help("Convert from celcius or farenheit")
            .takes_value(true)
            .long("from")
            .short("f")
            .required(true))
        .arg(Arg::with_name("temperature")
            .help("Temperature value")
            .takes_value(true)
            .long("temperature")
            .short("t")
            .required(true))
        .get_matches();

    let temperature = matches.value_of("temperature").unwrap();
    let from = matches.value_of("from").unwrap();

    match from.as_ref() {
        "celcius" => {
            println!("Converted to {} farenheit!", convert_to_farenheit(temperature).to_string());
        },
        "farenheit" => {
            println!("Converted to {} celcius!", convert_to_celcius(temperature).to_string());
        },
        _ => {
            println!("Please enter celcius or farenheit");
        }
    }    
}

fn convert_to_farenheit(temperature: &str) -> f32 {
    let old_temp = temperature.parse::<f32>().unwrap();
    (old_temp * (9.0/5.0)) + 32.0
}

fn convert_to_celcius(temperature: &str) -> f32 {
    let old_temp = temperature.parse::<f32>().unwrap();
    (old_temp - 32.0) * (5.0/9.0)
}