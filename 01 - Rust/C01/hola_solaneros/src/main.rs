use std::env;
use std::process;

fn main() {

    println!("Hello, world!");

    let variable = 5; // Declaración básica de variable inmutable
    
    // Strings formats
    println!("{}", variable);
    println!("{variable}");

    let mut var_mutable = 5;// Declaración de variable mutable.
    println!("{var_mutable}");
    var_mutable = 4;
    println!("{var_mutable}");

    let var:i8 = 10; // Declaración de variable explicitando tipo de dato (Optimización de memoria)
    let pote = var.pow(2); //Elevado al cuadrado
    println!("{pote}");


    // Comparar con cambio de tipo de variable:
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No se propocionó la edad como argumento.");
        process::exit(1)
    }
    
    let edad: u8 = match args[1].parse::<u8>() {
        Ok(num) => num,
        Err(_) => {
            println!("{} no es una edad válida.", args[1]);
            process::exit(1)
        }
    };
        
    if edad >= 18 {
        println!("Es mayor de edad.");
    } else {
        println!("Es menor de edad.");
    }
    
}