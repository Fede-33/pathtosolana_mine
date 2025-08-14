use std::env;

fn main() {
   
    // Captura por argumento con método .unwrap() y evalúa con función
    let args: Vec<String> = env::args().collect();
    let age: u8 = args[1].parse().unwrap();
    arbitro(age);
    println!("{}", retorno(age));

    fn arbitro(edad:u8) {
        if edad >= 18 {
            println!("Es mayor de edad.");
        } else {
            println!("Es menor.");
        }
    }

    fn retorno(edad:u8) -> String{
        if edad >= 18 {
            "Es mayor de edad.".to_string()
        } else {
            "Es menor.".to_string()
        }
    }
    
    /* // Mostrar por consola los tres tipos de dato
    dbg!(age); //u8
    let ref_age:&str = &args[1];
    dbg!(ref_age); //&str
    let string_age:String = ref_age.to_string();
    dbg!(string_age); //String
    */
    
    // Concatenación:
    let texto:&'static str = "Edad: ";

    // let salida = texto + age; //(No permite concatenar &str + u8)
    // let salida = texto + string_age; //(No permite concatenar &str + String)
    // let salida = texto + ref_age; //(No permite concatenar &str + &str)
    // let salida:String = texto.to_string() + args[1]; //(No permite concatenar String + String)
    let salida:String = texto.to_string() + &args[1]; // (Permite concatenar String + &str)
    println!("{}", salida);

    

}
