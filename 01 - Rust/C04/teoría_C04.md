# OWNERSHIP 

dos granes grupos de lenguaje:
* Sistema: C

    Dejan al desarrollador la responsabilidad del manejo de memoria. Si había un puntero o una referencia, había que cerrarla porque sino había una fuga de memoria.

* Garbage collector: Javascript, JAva, Php
    
    Mientras se ejecuta el programa, el collector va liberando espacios de memoria que no se van utilizando.

Rust resuelve el manejo de la memoria de otra forma. Establece una serie de reglas que se verifican en el momento de la compilación. Las reglas de Ownership, que se implementan en el compilador para administrar la memoria. 

## Reglas de OWNERSHIP:

1. Cada valor en Rust tiene una variable que es su dueña (owner).
2. Solo puede haber un dueño a la vez.
3. Cuando un dueño sale del scope (ámbito donde la variable es válida) el valor es eliminado (dropped).

### Clone:
Los tipos de datos que se almacenan en el *heap*, como los *Strings* o *Vec*, no tienen la propiedad de *copy*, por lo que al ser reasignados, se cambia su propietario y la variable que los contenía es *dropped*. Por ejemplo:

    let s1 = String::from("hola"); 
    let s2 = s1; 
    println!("{s1}"); //ERROR

Se define un *String* en el *heap*, y se asigna su propiedad a la variable *s1*, que es un puntero en el *stack*, que apunta al espacio de memoria del *heap*. Luego, se cambia la propiedad del *String* al puntero *s2*, porque Rust no copia espacios de memoria del *heap* por defecto. Entonces, cuando el compilador detecta que *s1* ya no es propietaria de ningún dato, la descarta (dropped). Esta línea generará un error.
Para evitar el error anterior, si realmente se requiere que el mismo valor se asigne a dos variables, se debería utilizar el método .clone():

    let s2 = s1.clone() 

En este caso, se realiza una copia profunda, especificando que el dato *s1* debe ser clonado en otro espacio de memoria del *heap*, para que *s2* sea propietario de ese nuevo espacio.

### Copy trait:
Las variables que se almacenan en el *stack* porque son de un tamaño conocido (Char, bool, int, float o tuplas que contienen los tipos anteriores), tienen la propiedad de *copy*. Por lo cual, cuando su valor se asignan a otra variables, simplemente se copia dentro del *stack*, por razones de economía y rapidez de memoria. Es decir, que no cambia el ownership del valor, sino que cada variable ocupa su propio espacio de memoria en el *stack*. Por ejemplo:

    let i1:i32 = 5; 
    let i2 = i1; 
    println!("{i1}"); 
    
Esto sí funciona porque un integer es un tipo de dato simple con un tamaño fijo y conocido en tiempo de compilación, por lo que puede duplicarse en el *stack*. 

### Tuplas:
Las tuplas tenrán el *copy trait*, si cada uno de sus valores también lo tienen. De lo contrario, la tupla completa carecerá del *copy trait*. Por ejemplo:

    (i32, bool, char) //copy trait
    (i32, bool, char, &str) //copy trait
    (i32, bool, char, String) //no copy trait

## Reglas de borrowing
1. Si los préstamos son inmutables puede haber varios.
2. Solo puede haber un préstamo mutable. 
3. Si hay un préstamo mutable, no puedo tener un préstamo inmutable al mismo valor.

Los préstamos se realizan cuando un valor se pasa como referencia, utilizando la sintaxis de *&*. Por ejemplo:

    fn main() {
        let s1 = String::from("hola");
        impresora(s1);
        println!("{s1}"); //ERROR
    }

    fn impresora(texto:String){
        println!("Esta función imprime: {}", texto);
    }

En este caso, la función *impresora* recibe un dato tipo *String* y se asigna al parámetro *texto*. Entonces, cuando se la invoca, con *s1* como argumento, esta variable cede la propiedad del *String*, que es tomada por la variable *texto* dentro de la función. *s1* deja de ser una variable válida, por lo que al intentar imprimirla, retorna un error. Esto puede solucionarse mediante el *borrowing*, Por ejemplo, :

    fn main() {
        let s1 = String::from("hola");
        impresora(&s1);
        println!("{s1}");
    }

    fn impresora(texto:&String){
        println!("Esta función imprime: {}", texto);
    }

En este caso, la función *impresora* no recibe un dato de tipo *String*, sino una referencia a un string, es decir, un *&String*. Por lo tanto, la función se invoca con un préstamo (borrow), es decir, un argumento prestado de una variable. Esto evita el cambio de propiedad de la variable, y puede serguir utilizándose. 

### Borrow Checker:
Es parte del compilador, y controla que se cumplan las reglas de *borrowing*. También determina el tiempo de vida o inactividad de una referencia, ya sea *léxica* (definida por el usuario) o *NLL* (Non-Lexical Lifetime). En el segundo caso, el *borrow checker* determina que una referencia ya no está "activa" en el punto de su último uso.

### Referencias mutables e inmutables:
Las referencias inmutables son de solo lectura, por lo que no hay impedimentos lógicos para que pueda haber más de una. Es decir, que el valor no puede ser cambiado. En cambio, las referencias mutables tienen pemiso para modificar el dato alojado en la memoria, por eso, no puede haber otros préstamos cuando hay uno mutable, porque podría suceder que:
* Una referencia mutable modifique un valor que está siendo modificado por otra referencia mutable. Se produce un *data race*, es decir, que múltiples partes del código intentarían modificar el mismo espacio de memoria al mismo tiempo, llevando a resultados impredecibles.
* Una referencia inmutable intenta leer un valor que está siendo modificado por una referencia mutable. Esto es una condición de carrera clásica y puede llevar a bugs difíciles de depurar.

Por ejemplo:

    let mut s = String::from("Hola");

    let r1 = &s; // Referencia inmutable 1
    let r2 = &s; // Referencia inmutable 2 (permitido)

    println!("{} y {}", r1, r2); 

    let r3 = &mut s; 
    r3.push_str(" mundo!");
    println!("{}", r3);

De la *String* mutable declarada como *s* obtenemos dos refrencias inmutables *r1* y *r2*, que son utilizadas por última vez en la impresión. Entonces, el *borrow checker* determina que ese es el final de la vida de las referencias, y permite que se defina otra referencia mutable *r3* en el mismo *scope*. Si se intentara utilizar nuevamente alguna de las referencias inmutables, el *borrow checker* retornaría un error de compilación. 

### Des-referenciación:
Es el proceso de acceder al espacio de memoria al que apunta una referencia, donde reside el dato real para poder leerlo o modificarlo. Se realiza con la sintaxis de *\** asterisco:

    let x = 5; 
    let y = &x; 
    let z = *y; 

    println!("x = {}", x); 
    println!("y = {:?}", y); 
    println!("z = {}", z); 
    println!("*y = {}", *y); 

En este caso, *x* es una variable en el *stack*. *y* se define como una referencia que apunta a *x*. Y *z* toma la referencia *y*, pero al res-referenciarlo, recupera el valor del espacio de memoria y se lo asigna directamente a *z*. Al ser enteros y tener el *copy trait*, ahora *x* y *z* tienen ambos, un lugar de memoria en el *stack* en el que almacenan el mismo dato. Para las referencias mutables, el mecanismo es similar, pero con la particularidad de que, al desreferenciar, se concede autorización para modificar el dato original guardado en memoria:  

    let mut a = 10; 
    let b = &mut a; 
    *b += 1; 

    println!("a = {}", a); // Salida: a = 11
    println!("*b = {}", *b); // Salida: *b = 11
    
en el ejemplo se define una variable *a* mutable y una variable *b* que es una referencia mutable de *a*. Luego se le indica que incremente en uno, al *b* desreferenciado. Es decir, que vaya al espacio de memoria al que hace referencia *b* y lo modifique. De esta manera *a* pasa a ser 11.

**Desreferenciación implícta:**
En Rust se realiza una des-referenciación implícita (llamada "Deref coercion") cuando es conveniente. Esto sucede en varios contextos, como:

* Cuando pasas una referencia a una función que espera un tipo más general (por ejemplo, pasar un &String donde se espera un &str).
* Cuando usas el operador punto (.) en una referencia, Rust intenta desreferenciar automáticamente para acceder a los métodos del tipo al que se apunta.

Por ejemplo: 

    let s = String::from("hola"); 
    let r = &s; 
    r.len()
    
Se pueden llamar métodos de String directamente en *r* sin necesidad de escribir *(*r).len()* Rust sabe que debe des-referenciar *r* primero.
