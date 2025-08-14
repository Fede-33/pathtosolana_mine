# STRUCTS

## METODOS:
No se definen en el bloque de código del struct, sino que se administran como implementaciones o traits. Cuando se crea un *struct* se definen sus atributos (keys) especificando el tipo de dato que contendrá cada uno: 

    struct User{
        active: bool,
        username: String,
        email: String,
        last_login: u64
    }

Si es necesario que el *struct* tenga un comportamiento o responda a alguna funcionalidad, se puede lograr de dos maneras:

### BLOQUE DE COMPLEMENTACIÓN: 
Se agregan previos a la definición del *struct*, para otorgarle un determinado *trait*. Por ejemplo, si en tiempo de desarrollo fuera necesario imprimir por pantalla el contenido de cada instancia del struct anterior mediante la función *dbg!*, se debería agregar el trait *Debug* de la siguiente manera:

    #[derive(Debug)]
    struct User{
        active: bool,
        username: String,
        email: String,
        last_login: u64
    }

### BLOQUE DE IMPLEMENTACIÓN:
Si es necesario definir un comportamiento personalizado, se realiza posteriormente a la definición del *struct*, con la sintaxis de **impl** y **Self**, y codificando cada método como una función:

    impl User {
        fn new (name: String)-> Self{
            Self {
                active: true,
                username: name,
                email: String::from("mail@usuario.com"),
                last_login: 0
            }
        }
    }

En el ejemplo anterior se crea un método, como implementación del *struct* que recibe un nombre de tipo *String* y retorna el mismo *struct*, pero antes de retornarlo le asigna valors a los atributos. Sería un método que permitiría construir la instancia, como si fuera el método constructor en POO. 

## TUPLE STRUCT:
Son structs que tienen las mismas características de una tupla, pero permiten imlementaciones. Por ejemplo, para definir colores en RGB:

    struct Color (i32, i32, i32);

Para acceder a las posiciones, se utiliza la sintaxis de punto y el índice:

    Color.0
    Color.1
    Color.2




