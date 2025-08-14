## HEAP - STACK:
Son los componentes que forman la memoria de un programa. 

### Stack (Pila): 
Es la memoria virtual que está reservada exclusivamente a la ejecución del programa.

* **Organización:** Es una región de memoria organizada de forma LIFO (Last In, First Out - Último en Entrar, Primero en Salir). Piensa en una pila de platos: el último plato que pones es el primero que quitas.

* **Almacenamiento:** Almacena variables locales de funciones, parámetros de funciones y la dirección de retorno de las llamadas a funciones.

* **Gestión:** La gestión de la memoria en la pila es automática. Cuando una función es llamada, se asigna un "marco de pila" para esa función, y cuando la función termina, ese marco se desasigna automáticamente.

* **Tamaño:** Tiene un tamaño limitado, que generalmente es determinado en tiempo de compilación. Si una función intenta usar más memoria de la disponible en la pila, se produce un error de "desbordamiento de pila" (stack overflow).

* **Velocidad:** Es muy rápida para asignar y desasignar memoria debido a su naturaleza organizada.

* **Ejemplo:** Cuando llamas a una función, sus variables locales se guardan en la pila. Cuando la función termina, esas variables se eliminan automáticamente.

### Heap (Montón): 
La memoria que el programa debe pedirle al sistema operativo para realizar ciertas tareas, que exceden el espacio de memoria del

* **Organización:** Es una región de memoria de propósito general donde la memoria se asigna y desasigna de forma dinámica, es decir, en tiempo de ejecución. No sigue un orden LIFO o FIFO.

* **Almacenamiento:** Almacena objetos y estructuras de datos que necesitan vivir más allá de la duración de una función específica, o cuya vida útil no puede determinarse en tiempo de compilación.

* **Gestión:** La gestión de la memoria en el heap es manual (en lenguajes como C/C++ se usa malloc/free o new/delete) o semiautomática (en lenguajes con recolección de basura como Java o Python). Esto significa que es responsabilidad del programador (o del recolector de basura) liberar la memoria cuando ya no se necesita. Si no se libera la memoria correctamente, puede haber "fugas de memoria" (memory leaks).

* **Tamaño:** Su tamaño es mucho mayor que el de la pila y puede crecer o encogerse dinámicamente según sea necesario. El límite es la cantidad de RAM disponible en el sistema.

* **Velocidad:** Es más lenta para asignar y desasignar memoria en comparación con la pila, ya que el sistema tiene que encontrar un bloque de memoria disponible que se ajuste a la solicitud.

* **Ejemplo:** Si creas un objeto grande o una lista dinámica en tu programa, es probable que se almacene en el heap para que pueda ser accedido desde diferentes partes del programa y persistir mientras sea necesario.

## STRING - &STR:
En Rust, cuando se define una variable que contendrá texto, es necesario diferenciar si será de tamaño estático o dinámico, para asignar el tipo de dato. 

* **String:** (Cadena propia). Una variable dinámica, que puede tener diferente cantidad de caracteres, será del tipo *String*, por ejemplo, un campo para ingresar nombres o descripciones. Al no tener un tamaño fijo definido en tiempo de ejecución, será alojado en el *Heap*, conteniendo los datos a los que hace referencia. Asimismo, utiliza una pequeña porción de memoria *Stack*, que almacena una pequeña estructura de tamaño conocido, que incluye un puntero, longitud y capacidad. 

* **&str:** (Slice de cadena). Una variable de texto estática, como un código, que tendrá siempre la misma longitud de caracteres, se definirá como *&str*, una referencia en el *Stack* que apunta a una ubicación de memoria compilada en el binario del programa. Es por ello que, por definición, es inmutable y no contienen los datos, sino un puntero y una longitud hacia el espacio de memoria compilado en el binario. También puede obtenerse un *&str* a partir de un *String*, es decir, que el puntero y longitud que están en el *Stack* apuntan a un espacio de memoria en el *Heap*. 

### Concatenación:
El operador + para cadenas en Rust funciona específicamente con *String* y *&str*, específicamente, el lado izquierdo de la operación debe ser un *String*, y el lado derecho *&str* 

## LIFETIMES
Cuándo se define un puntero en referencia a un espacio de memoria, utilizando la sintaxis de *& ampersand* es necesario que el compilador tenga la seguridad de que el dato exista siempre que el puntero intente acceder. Los *lifetimes*, *lifetime parameters* o *tiempos de vida*, garantizan la seguridad de la memoria del programa en tiempo de compilación, mediante una anotación que define el tiempo que será válido un pŕestamo de memoria, es decir, que el puntero y la referencia estén "vivos". la síntaxis de los lifetime es, por ejemplo, *&'a str*, donde se puede asignar cualquier letra minúscula luego del ampersand. Existe un lifetime especial *&'static* que indica que ese puntero debe ser válido durante toda la ejecución del programa.

## FUNCIONES:
Las funciones en Rust se definen mediante la sentencia *fn* un nombre y, entre parántesis, los parámetros especificando el tipo de dato que recibirán.

### Sin Retorno:
Luego de la definición se abren las llaves para ingresar el cuerpo de la función. Este tipo de funciones tienen un retorno por defécto que es una tupla vacía *()* por motivos de control de errores.

   fn arbitro(edad:u8) {
      if edad >= 18 {
         println!("Es mayor de edad.");
      } else {
         println!("Es menor.");
      }
   }

### Con Retorno:
Luego de la primera parte de la definición, se especifica el tipo de dato que retornará anteponiendo la sintaxis *->*. y posteriormente se define el cuerpo de la función, retornando los valores con la sentencia *return* 

      fn retorno(edad:u8) -> String{
         if edad >= 18 {
            return "Es mayor de edad.".to_string();
         } else {
            return "Es menor.".to_string();
         }
      }

* **Retorno implícito:** Si la última línea de una función (o un bloque de código, como un if o match) es una expresión (produce un valor) y no termina con un punto y coma *;*, se interpreta que el valor de esa expresión es el retorno, evitando el uso de la sentencia *return*.

      fn retorno(edad:u8) -> String{
         if edad >= 18 {
            "Es mayor de edad.".to_string()
         } else {
            "Es menor.".to_string()
         }
      }

## CARGO CLIPPY
Extensión del compilador que retorna posibles optimizaciones de código, para hacerlo más robusto y asegurar buenas prácticas. Cuenta con los parámetros adicionaes *Cargo clippy --fix --allow-dirty* que muestra las recomendaciones y las aplica automáticamente.

## CARGO FORMATER:
se encarga de formatear automáticamente tu código Rust según un estilo consistente y estandarizado. Su objetivo principal es garantizar que todo el código Rust tenga la misma apariencia, independientemente de quién lo escribió. Se ejecuta por línea de comandos mediante *cargo fmt*.

## TUPLAS:
Es un conjunto de datos no ordenados, de diferentes tipos y 12 elementos como máximo. Para crear una tupla se debe definir como tal, definir los tipos de datos que va a contener, y luego asignarle los valores:

      let persona: (&str, &str, i8) = ("Fede", "Boca", 38);

Luego, para acceder a esos datos, se utilizan los índices internos, mediante sintaxis de punto:

      println!("{} es de {} y tiene {}", persona.0, persona.1, persona.2);

Una función, aunque no tenga *return* siempre tendrá como retorno una tupla vacía *()* o *(Ok)* en caso de ejecución correcta, o una tupla *(Err(...))* con el código del error ocurrido.