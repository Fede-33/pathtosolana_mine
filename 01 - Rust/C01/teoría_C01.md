## RUST ES UN MEMORY SAFE LANGUAGE:
En lugar de asignarle al dev el manejo de memoria mediante un garbage collector, esto se resuelve a través del compilador mediante principios propios del lenguaje, que garantiza que no queden fugas de memoria, race conditions o referencias apuntando a espacios de memoria vacíos.

## TIPOS DE DATO:

* **INTEGER:** Números enteros.
    * **Con signo:** se definen comenzando con *i*: i8, i16, i32, i64, etc. 
    * **Sin signo:** se definen comenzando con *u*: u8, u16, u32, u64, etc.
* **FLOAT:** Números racionales. se definen con *f*: f32 y f64.
* **CHAR:** Caracteres únicos en unicode, como una letra o un emoji.
* **BOOL:** Toman valores de *true* o *false*, ocupan un Byte de memoria.

## PARÁMETROS SECUENCIALES:
Permite el ingreso de parámetros en la línea de comando cuando se ejecuta por consola, y esos parámetros recuperarlos dentro del código, mediante esta sintaxis:

    let args: Vec<String> = env::args().collect();
    dbg!(args);

Se estaría creando un Vector que recolecta los parámetros de consola. Los que se muestran mediante el macro *dbg!*. Otro ejemplo sería:

    let args: Vec<String> = env::args().collect();
    if &args[1] == "18" {
        println!("¿Querés que te felicite?")
    } else {
        println!("Tampoco te felicito")
    }

El código anterior evalúa una condición basándose en el parámetro incluido en la línea de comandos al compilar el programa.

## CONVERTIR TIPOS DE DATOS:
El ingreso de parámetros por línea de comandos siempre admite datos *String*, pero a través de la función *.parse()* puede modificarse este tipo de dato. Esta función retornará *Ok()* con el valor modificado si el proceso es correcto, o retornará *Err()* en caso de error. Para evitar cualquier tipo de excepción, es conveniente comparar el retorno de la función *.parse()* mediante la sentencia *Match*, devolviendo un mensaje de error que no detenga la ejecución o cause panico mediante una excepción:

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("No se propocionó la edad como argumento.");
        return;
    };
    
    let edad: u8 = match args[1].parse::<u8>() {
        Ok(num) => num,
        Err(_) => {
            println!("{} no es una edad válida.", args[1]);
            return;
        }
    };
        
    if edad >= 18 {
        println!("Podés escabiar.");
    } else {
        println!("También podés, pero yo no te dije nada.");
    }
