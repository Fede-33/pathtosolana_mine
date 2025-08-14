# CONTROL DE ERRORES
Se realiza a través del uso de dos *Enums*.

## OPTION:
Cuando un bloque de código puede tener un resultado o no, se controla mediante la declaración de un *Option* que excluya los resultados de error, es decir, que en vez de atrapar un error cuando sucede, se prevea y se predefina un comportamiento para cuando se presenten los factores que lo generen. Un ejemplo podría ser en una división cuando el denominador es 0:

    fn divide (num:f64, den: f64) -> Option<f64> {
        if den == 0.0 {
            None
        } else {
            Some( num / den )
        }
    }

    fn main() {
        let division = divide(10.0 , 0.0);
        match division {
            Some(x) => println!("Resultado {}",x),
            None => println!("No se puede dividir por 0")
        }      
    }

En el ejemplo se declara el retorno de la función como un *Option* de f64, y se define que si el denominador es 0, el retorno es None. Esto evita que se produzca el error de división por 0. Luego, se evalua el retorno de la función *divide* con un match y se definen los comportamientos.  

## RESULT:
Es de uso más específico, ya que puede retornar dos valores, **OK(T)** o **Err(E)**, donde T es valor de retorno exitoso, y E es el la identificación de un posible error. El ejemplo anterior podría definirse cómo:

    fn divide2 (num:f64, den:f64) -> Result<f64, String> {
        if den == 0.0 {
            Err ("No se puede dividir por 0".to_string())
        } else {
            Ok(num / den)
        }
    }

    fn main() {
        let division2 = divide2(10.0 , 2.0);
        match division2 {
            Ok (x) => println!("Resultado: {}", x),
            Err (x) => println!("{}", x)
        }   
    }
Nuevamente, la función retorna un *enum*, en este caso *Result* sobre el que se puede hacer *pattern matching* para mostrar un resultado exitoso o un mensaje de error.

### Ejemplo 1:

    use std::fs::file;
    use std::io::{self, Read};

    fn leer_archivo () -> Result <i32, io::Error> {
        let f = File::open("numero.txt");

        let mut f = match f{
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => {
                match s.trim().parse::<i32>(){
                    Ok(num:i32) => Ok(num),
                    Err(_) => Err(io::Error::new(io::ErrorKind::InvalidData, "No se pudo convertir en i32")),
                }
            }
            Err(e:Error) => err(e),
        }
    }

El bloque de código anterior realiza lo siguiente:
* Importa la estructura **fs::file** y el rasgo **Read** del módulo **io**.
* Define la función *leer_archivo* que no toma argumentos y retorna un **Result** con un *i32* o un error definido en el módulo importado.
* Se intenta abrir un archivo *numero.txt* y asignar su contenido a la variable *f*. **File::open** es un **Result** que retorna el contenido del archivo o un código de error. 
* Se define nuevamente la variable *f*, esta vez mutable, para iniciar una expresión **match** que evalúe el retorno del intento de abrir el archivo:
    * Si el resultado es exitoso **Ok(file)=>file** retorna *file*.
    * Si el resultado es error **Err(e) => return Err(e)** retorna el error *e*.
* Se define una variable *s* que es un *String* mutable y vacío.
* Se evalúa, mediante una sentencia **match** el método **read_to_string** que se le puede aplicar a *f*, ya que es un archivo. El resultado de ese método se almacenaría en el *String* mutable *s*:
    * Si el resultado es exitoso **Ok(_)=>{** inicia un nuevo bloque de código:
        * Al contenido de *s* se lo intenta transformar a *i32* luego de eliminar los espacios en blanco mediante **trim().parse** toda esta operación es evaluada por una sentencia **match** que puede resultar:
            * Exitosa **Ok(num:i32)**, y en tal caso retorna **Ok(num)**.
            * Fallida **Err(_)**, entonces retorna un nuevo **io:Error** con el mensaje *"No se pudo convertir en i32"*.  
    * Si el resultado es error **Err(e) => return Err(e)** retorna el error *e*

### Ejemplo 2:

    fn leer_archivo_conciso () -> Result<i32, io:Error> {
        let mut f = File::open("numero.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        s.trim().parse::<i32>().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "No se pudo convertir en i32"))
    }

Utilizando las mismas librerías importadas que en el ejemplo anterior, pero realizando la operación de forma más concisa:
* La función *leer_archivo_conciso* se declara de la misma manera.
* Se intenta abrir el archivo *numero.txt* utilizando **File::open** y asignando el retorno a la variable mutable *f*, pero en este caso se agrega el operador **?**, que es el predefinido por Rust para el **pattern matching** de errores. Es decir que en caso de *Ok* desempaqueta el archivo y lo asigna a la variable, y en caso de *Err* retorna el código de error.
* Se define la variable mutable *s* como un *String* vacío.
* Se convierte el contenido de *f* en *String* y se asigna a la referencia mutable de *s*, pero nuevamente se agrega el operador **?** para controlar el éxito de la operación de conversión.
* La última línea intenta convertir el texto a número:
    * s.trim() Elimina los espacios en blanco al principio y al final de la cadena de texto.
    * parse::<i32>() Intenta convertir el texto restante a un número entero. Esta operación devuelve un Result<i32, ParseIntError>.
    * map_err(|_| ...): es un método que transforma el tipo de error, ya que *parse* devuelve un error de tipo **ParseIntError**, y la función solo puede devolver **io::Error**, 
    * La closure |_| recibe el error original y lo ignora con el gión bajo. Creando un nuevo **io::Error** con la descripción *"No se pudo convertir en i32"*.

# ARRAYS
Un tipo de dato que es una colección de valores con un tipo y extensión definida. Esta característica los hace rápidos, porque pueden almacenarse en el *stack*.

## DECLARACIÓN:
Por ejemplo, un *array* de tres números enteros se podría definir como:

    let nums: [i32:3] = [1,2,3];

El contenido de los *arrays* puede cambiar si se los define como mutables, pero siempre tendrán que cumplir con la condición en que fueron definidos, es decir mismo tipo y cantidad de datos:

    let mut nums: [i32:3] = [1,2,3];
    nums = [4,5,6];

Si se quiere inicializar un *array* con todos datos iguales, puede hacerse con la sintaxis de *;*, por ejemplo, una colección de cinco caracteres *a*:

    let letras_a: [char;5] = ['a';5];

Generalmente Rust interpreta el tipo y extensión del *array* y no hace falta especificarlo, por lo que lo siguiente funcionaría:

    let nums = [1,2,3];
    let letras_a = ['a';5];

## ACCESO:
Se realiza mediante índices comenzando por 0. Por ejemplo, para imprimir el primer número:

    println!(nums[0]);

Para leer cuantos elementos tiene un *array* puede usarse el método **len**:

    println!(nums.len())

## TIPOS DE DATO;
Los *arrays* pueden contener cualquier tipo de dato, simple o complejo. Sin embargo, en caso de que los datos que contengan sean propios de almacenar en el *heap* el *array* entonces contendrá los punteros a esos espacios de memoria. Por ejemplo, un *array* de *structs*, en realidad almacenará en el *stack* las referencias a los espacios de memoria del *heap*

# VECTORES
Son conjuntos de datos del mismo tipo, pero de extensión variable. Esto los hace más versátiles que los *arrays*, pero deben almacenarse en el *heap* por lo que su acceso es ménos rápido.

## DECLARACIÓN:
Se definen mediante la macro **vec!** y usando una sintaxis de corchetes. El lenguaje puede inferir el tipo de dato que contendrá:

    let mut planetas = vec! ["Mercurio", "Venus", "Tierra", "Marte"];

## ACCESO:
Se realiza mediante índices comenzando por 0. Por ejemplo, para imprimir el primer planeta:

    println!(planetas[0]);

Para leer cuantos elementos tiene un *vector* puede usarse el método **len**:

    println!(planetas.len())

## MODIFICACIÓN:
Al ser conjuntos variables, admiten métodos modificar la cantidad de elementos. Por ejemplo, para agregar un dato al final:

    planetas.push("Júpiter");

Si se intenta quitar el último elemento, puede usarse el método **.pop**. Este retorna un *Option*, ya que si el *vector* está vacío retorna *None* y si la operación es exitosa retorna el elemento eliminado:

    let planeta_borrado = planetas.pop();
    match planeta_borrado{
        Some (x) => println!("Planeta {} eliminado", x),
        None => println!("No hay planetas que eliminar"),
    }

# PROPIEDADES DE ITERACIÓN
Ni los *vectores* como los *arrays* son iterables por definición, sino que se les debe asignar la propiedad específicamente, esto se logra mediante el método **.iter()**:

    let v = vec![1,2,3,4,5];
    v.iter().for_each(|x| println!("{}", x))

En el ejemplo anterior, se transforma en iterable el vector *v* y se le aplica el método **.for_each** que itera entre todos sus valores, asignándo cada uno a *x* e imprimiéndolos línea  a línea.

    let s = "Hola mundo".to_string();
    s.chars().map(|n| {
        println! ("{}", n);
    }).collect::<Vec<_>>();

El ejemplo anterior crea una variable *s* del tipo *String*. Luego se le aplica el método **chars()** que retorna un iterador sobre cada caracter que compone la cadena. Posteriormente el método **map()** le asigna cada iteración a la variable *n* y le ejecuta la función que está entre llaves (clausura), en este caso, imprimirlo línea a línea. Finalmente, el método **collect()** es la forma más común de consumir un iterador y recolectar todos sus elementos en una colección, especificándo **::<Vec<_>>:** se le dice al compilador en qué tipo de colección se deben agrupar los elementos. **Vec** se refiere a un vector, y el **_** deja que Rust infiera el tipo de dato. En este caso, como la clausura del map no devuelve nada explícitamente, su tipo de retorno es el tipo unitario (), por lo que el resultado de collect será un Vec<()>. El resultado de esta expresión se descarta al final de la línea.
