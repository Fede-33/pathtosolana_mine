fn main() {

    fn get_inflation(country: &str) -> Option<f64> {
        // llama a una API que retorna los datos (no todos los países informan la inflación)
        // if inflación existe
        match country {
            "AR" => Some(1.6), 
            _ => None,
        } //return implícito
    }

    let inflation: Option<f64> = get_inflation("AR"); 

    println!("Inflación: {}", inflation.unwrap_or(0.0)); //unwrap obtiene el valor asociado (Si es None, obtiene 0.0).

    let previous_value: f64 = 100.0;

    let current_value = previous_value
        * match inflation {
            Some(t) => 1.0 + (t / 100.0),
            None => 1.0,
        };
    println!("{current_value}");

    //Flujo de control:

    let age = 18;

    if age == 18 {
        println!("La persona tiene 18 años")
    } else if age < 18 {
        println!("La persona tiene menos de 18 años")
    } else {
        println!("La persona tiene más de 18 años")
    }

    match age {
        18 => println!("La persona tiene 18 años"),
        0..18 => println!("La persona tiene menos de 18 años"),
        _ => println!("La persona tiene más de 18 años"), //el _ es otherwise, para todos los valores no contemplados.
    }

    //Control de flujo para mostrar o no datos según Option 

    if let Some(x) = inflation {
        println!("{x}")
    }

    match inflation {
        //Evaluar la respuesta cuando no devuelve nada.
        Some(x) => println!("{x}"),
        _ => println!("No tiene dato"),
    }

    if let Some(x) = inflation {
        //hace lo mismo que la expresión anterior
        println!("{x}")
    }

}
