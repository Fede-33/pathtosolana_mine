use std::collections::HashMap;

fn main() {
    
    let duplicar = |x:f32| -> f32 { x * 2.0 }; 

    let saludar = |x| { format!("Hola {}", x) };

    println!("Resultado {}", duplicar(12.5));
    println!("{:?}", saludar("Pepe"));


    let numeros = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let dobles:Vec<f32> = numeros.iter().map(|&x| duplicar(x)).collect();
    println!("{:?}", dobles);

    let pares = numeros.iter().filter(|&x| x % 2.0 == 0.0).collect::<Vec<&f32>>();
    println!("{:?}", pares);

    let suma_impares = numeros.iter().filter(|&x| x % 2.0 != 0.0).sum::<f32>();
    println!("{:?}", suma_impares);

    let mut puntaje = HashMap::new();
    puntaje.insert("Ana", 10.0);
    puntaje.insert("Pepe", 9.0);
    puntaje.insert("Juan", 9.5);

    let punt_ana = puntaje.get("Ana");
    let punt_jose = puntaje.get("jose");

    println!("{:?}", puntaje);
    println!("{:?}", punt_ana);
    println!("{:?}", punt_jose);

    let puntaje_doble:HashMap<_,f32> = puntaje.iter().map(|(&k, &v)| (k, duplicar(v))).collect();
    println!("{:?}", puntaje_doble);

}
