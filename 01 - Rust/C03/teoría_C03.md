## ENUMS
Tipo de dato complejo que te permite definir un conjunto de posibles variantes. Se utilizan cuando un valor puede ser una de varias cosas posibles, pero solo una a la vez. Los *enum* pueden ser variantes sin datos, como categorías o estados, o pueden ser variantes con datos asociados, que pueden ser de cualquier tipo.

### Option:
En Rust no existen los undefined ni los null, para evitar errores de programación, existe un tipo de valor *Option*. Es un *enum* que contiene *Some(t)* y None, donde *t* es el valor asociado, de retorno.

### Result:
Otro *enum* predefinido con el lenguaje es *Result* que contiene *Err(_)* u *OK(T)* donde *T* es el return de una operación. Error contendrá la información del error producido.

### Enum definidos:
Un ejemplo de un enum definido podría servir para representar los dos tipos de formato de número IP.

    enum IpAddr {
        V4 (u8,u8,u8,u8),
        V6 (String),
    }

Porque dependiendo de la IP que tenga un servidor y del protocolo, puede devolver distinto tipo de dato.

## CONTROL DE FLUJO

### If:
La sintaxis es:
    
    if age == 18 {
        println!("La persona tiene 18 años")
    } else if age < 18 {
        println!("La persona tiene menos de 18 años")
    } else {
        println!("La persona tiene más de 18 años")
    }

### Match:
Es una expresión exhaustiva, no se pueden dejar casos posibles por fuera de las opciones definidas. Se codifica mediante *pattern matching*, comparando un valor con una serie de patrones predefinidos y, si hay una coincidencia, ejecutar un bloque de código específico o extraer partes del valor. La sintaxis para hacer lo mismo que en el ejemplo anterior:

    match age {
        18 => println!("La persona tiene 18 años"),
        0..18 => println!("La persona tiene menos de 18 años"),
        _ => println!("La persona tiene más de 18 años"), 
    }

El valor *_* significa *otherwise*, y representa a todos los valores no contemplados, como un *else*.

### If let:
Patrón simplificado (syntactic sugar) para el *match*. En una evaluación de un *Option*, cuando solo interesa el dato asociado al *Some*, pero en caso de *None* no se realizará niguna acción, entonces *if let* es una forma simple de obtener el dato o continuar el flujo si no existe. El siguiente ejemplo evalúa si el valor de *inflation* existe, y en tal caso, lo almacena en *x*, y luego lo imprime.

    if let Some(x) = inflation {
        println!("{x}")
    }

## ITERACIONES

### Loop:
Es un ciclo infinito, por lo que será necesario introducir una condición que invoque un *break*. Por ejemplo, para imprimir los números del 0 al 20:

    let mut counter = 0;
    loop {
        println!("{counter}");
        counter += 1;
        if counter > 20{
            break
        }
    }

### For:
Es una iteración dentro de un conjunto de datos conocido. Por ejemplo, para imprimir los números del 0 al 20:

    for x in 0..=20{
        println!("{x}");
    }

### While:
Repetirá una línea de código mientras se cumpla una condición:

    let mut counter = 0;
    while counter <=20 {
        println!("{counter}");
        counter += 1;
    }
