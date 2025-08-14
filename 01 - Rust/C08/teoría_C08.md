# CLOSURES
Son sentencias similares a las funciones, pero de una sintaxis más directa. Pueden ser almacenadas en variables o ser pasadas como argumentos de otras funciones. Las *Closures* son funciones anónimas, porque no tienen que ser declaradas con un nombre y pueden acceder a variables que están fuera de su propio bloque de código. La sintaxis para definirlas incluye pipes para los parámetros, y los tipos de datos suelen ser inferibles, salvo que se vaya a realizar operaciones con ellos. Es decir, si la *closure* solo va a formatear, o retornar un valor, no hace falta especificar el tipo de dato que recibe y retorna:

    let duplicar = |x:f32| -> f32 { x * 2.0 }; 
    let saludar = |x| { format!("Hola {}", x) };

    println!("Resultado {}", duplicar(12.5));
    println!("{:?}", saludar("Pepe"))

son similares a las funciones *lambda* de Python. A su vez, un *closure* puede tener cuántas tareas sean necesarias y escribirse en cuántas líneas sea necesario.

# ITERATOR
Es un *trait* que puede implementarse a ciertos tipos de datos complejos, que sean colecciones y que tengan implementado el método *next()*. Este método se define de la siguente manera:

    fn next(&mut self) -> Option<Self::Item>

Una vez que el *trait* está aplicado a un elemento, este admite toda la lista de métodos proporcionados por el **iterator**, como por ejemplo:

* **all()**: Que verifica que alguna condición sea cumplida por todos los elementos de la colección.
* **any()**: Verifica que al menos un elemento cumpla con una condición.
* **count()**: Cuenta la cantidad de elementos.
* **enumerate()**: Crea una lista numerada.
* **filter()**: aplica filtros.
* **for_each()**: recorre los elementos
* **map()**: Retorna una nueva colección con los resultados de aplicar una operación a cada elemento.
* **reduce()**: acumula todos los valores en uno solo.

### Ejemplo map():

    let numeros = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let dobles = numeros.iter().map(|&x| duplicar(x)).collect::<Vec<f32>>();

En este caso se declara un vector con elementos de tipo *f32* y luego se intenta crear un vector con sus dobles. Primeramente se toma el vector inicial y se le implementa el **iter()** *trait*. Entonces puede aplicársele el método *.map()* para que recorra cada elemento, lo referencie en el parámetro x, especificado en **|&x|** y le aplique la *closure* **duplicar(x)**. Se observa que el comportamiento que lleva a cabo el método **.map()** es una *closure* que llama a otra *closure*. Este método es un *lazy method*, es decir, que define un comportamiento pero no lo realiza hasta que sus elementos son consumidos, ya sea mediante extraerlos o convertirlos en una nueva colección. Finalmente, para consumir esos valores, se utiliza el método **.collect::<Vec<f32>>()** y así formar una nueva colección con los resultados. Nótese que, en este punto, el método **collect()** requiere que se especifique el tipo de colección requerido **Vec** y el tipo de dato que contendrá **f32**. Esto funcionaría de la misma forma si se hubiera especificado el tipo de colección y dato al comienzo de la declaración:
    
    let dobles:Vec<f32> = numeros.iter().map(|&x| duplicar(x)).collect();


### Ejemplo filter():

    let pares = numeros.iter().filter(|&x| x % 2.0 == 0.0).collect::<Vec<&f32>>();

El método se aplica de forma similar, pero en vez de un comportamiento se define una condición para filtrar los datos. Se debe tener en cuenta que al momento de consumir los valores en una nueva colección, se especifica que será un vector de referencias **Vec<&i32>** porque la función que se aplicaba en el caso anterior, retorna explícitamente un **i32**, por lo que la referencia se transformaba en dato automáticamente, mientras que el **.filter()** tan solo trabaja con las referencias que se declaran como parámetro. Para evitar incovenientes, puede definirse un tipo general para el vector resultante, con un **_**:

    let pares = numeros.iter().filter(|&x| x % 2.0 == 0.0).collect::<Vec<_>>();

### Ejemplo encadenado:

    let suma_impares = numeros.iter().filter(|&x| x % 2.0 != 0.0).sum::<f32>();

De la misma manera, pueden encadenarse métodos para obtener resultados más complejos. En este caso primero se filtran los valores impares, y luego se les aplica el método **.sum()** para obtener un solo valor que los sume enter ellos. Se debe especificar el tipo que se intenta operar.

# HASHMAPS
Son un tipo de dato complejo que almacena elementos del tipo Key-Value (Clave-Valor). Tanto la clave como el valor son de los tipos especificados en la definición y no pueden cambiar. Para poder utilizarlos se deben importar de la biblioteca standard:

    use std::collections::HashMap;

    let mut puntaje = HashMap::new();
    puntaje.insert("Ana", 10.0);
    puntaje.insert("Pepe", 9.0);
    puntaje.insert("Juan", 9.5);

En el ejemplo anterior se importa la biblioteca, se define un **Hashmap** nuevo, que estará vacío, por lo que se hace mutable y sin especificar tipos de dato. Posteriormente se insertan datos, y es cuando se inferirá el tipo de la clave y el valor. Para obtener los datos de cada elemento del **Hashmap** se accede con la clave y el método **.get()**:

    let punt_ana = puntaje.get("Ana");
    let punt_jose = puntaje.get("jose");

Este método retorna un *Option*, ya que en caso de existir la clave, devuelve un *Some()* con el valor dentro, y si no existe la clave retorna un *None*.

## ITERACIÓN:
Los **Hashmaps** son iterables, pero hay que tener en cuenta que la mayoría de los métodos considerarán sus pares *key-value* como una tupla en cada iteración:

    let puntaje_doble:HashMap<_,f32> = puntaje.iter().map(|(&k, &v)| (k, duplicar(v))).collect();

En este caso se define el **Hashmap** *puntaje_doble* es pedificando cualquier dato para la Key y *f32* para el value. Luego se lo hace iterable y se le aplica el método **.map()** pero entre los parámetros se define una tupla, con la clave y el valor referenciados. Luego, en el comportamiento del *map* se especifica la forma de la tupla, y que la *closure* solo se aplicará a uno de los valores. Finalmente se recompilan los conjuntos de datos en un nuevo **Hashmap**. 
