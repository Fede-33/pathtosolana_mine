
fn main() {
    
    let s1 = String::from("hola"); // Creamos s1 como referencia en el stack, de un espacio de memoria en el heap con el string "hola"

    let s2 = s1; //no se copia el valor, sino que se cambia el dueño del valor, se crea un nuevo espacio de memoria stack que apunta al mismo string en el heap

    // println!("{s1}"); esto falla porque s1 ya no es el dueño de la referencia, y ya no existe s1 en el stack, se borró cuando se mudó el ownership a s2

    let s1 = String::from("hola");
    let s2 = s1.clone(); //creo un nuevo puntero s2 en stack, pero también clono la memoria del heap, porque no puede haber dos referencias a lo mismo.

    let i1:i32 = 5; 

    let i2 = i1; 

    println!("{i1}"); //esto sí funciona porque un integer se aloja naturalmente en el stack y tiene la propiedad copy. No es una referencia.

    //toma_propiedad(s1); // Esta función toma propiedad del valor, por esto no puede imprimirlo, ya no es su propiedad

    toma_propiedad(&s1); //Funciona porque es una referencia a un string, se debe aclarar en la definición de la función que el parámetro es una referencia.

    toma_propiedad_int(i2); // funciona porque tiene la propiedad Copy

    recibo_referencia_inmutable(&s1);
    recibo_referencia_inmutable(&s1);
    recibo_referencia_inmutable(&s1);
    //recibo_referencia_mutable(&mut s1); // error, no puede suceder otro préstamo ya que hay una referencia inmuetable

    let texto = String::from("Hola mundo");
    let palabra = primera_palabra(&texto);
    println!("{palabra}");

    let mut s = String::from("Hola");

    let r1 = &s; // Referencia inmutable 1
    let r2 = &s; // Referencia inmutable 2 (permitido)

    println!("{} y {}", r1, r2); // Usamos r1 y r2 aquí, sus préstamos terminan.

    let r3 = &mut s; // Referencia mutable (ahora permitido porque r1 y r2 ya no están activas)
    r3.push_str(" mundo!");
    println!("{}", r3);
}

fn primera_palabra(s:&String)-> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn toma_propiedad(texto:&String){
    println!("adentro de la función: {}", texto)
}

fn toma_propiedad_int(entero:i32){
    println!("adentro de la función: {}", entero)
}

fn recibo_referencia_inmutable(texto: &String){
    println!("adentro de la función: {}", texto)
}

fn recibo_referencia_mutable(texto:&mut String){
    println!("adentro de la función: {}", texto)
}
