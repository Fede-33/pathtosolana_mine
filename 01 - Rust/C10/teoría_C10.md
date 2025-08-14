# LIFETIMES
Para evitar la existencia de **dangling references**, punteros activos hacia espacios de memoria que ya han sido liberados. Los lifetimes definen una relación entre los tiempos de existencia de las referencias. Por ejemplo, en una función que recibe como parámetros referencias a dos valores y retorna uno de ellos, el compilador no puede inferir cuál de los dos será retornado, por lo que necesita la seguridad de que ambos valores serán válidos al momento de llamar a la función:

    fn longest<'a>(x:&'a str, y:&'a str) -> &'a str{
        if x.len() > y.len(){
            x
        } else{
            y
        }
    }

Mediante la sintaxis de **<'a>** en la definición y **'a** en los parámetros y retorno, de la firma de la función, se expresa que el valor de retorno, será válido un tiempo igual al menor de los tiempos de validez de los parámetros. Es decir, si el espacio de memoria al que apunta *x* se liberara antes que el espacio al que apunta *y*, la referencia del *return* dejaría de ser válida, porque de lo contrario, intentaría retornar un puntero hacia un espacio inexistente. Esto asegura que todos los valores apuntados, serán válidos al mismo tiempo de ejecución y dejarán de serlo cuando se termine ese scope, sin importar que alguno de los punteros continúe existiendo y siendo utilizado por otras partes del programa. La sintaxis recomendada es utilizando caracteres en minúscula y siguiendo el orden alfabético.

## STRUCTS:
    
    struct Foo<'a> {
        x: &'a str,
    }

En los casos en que se defina un *struct* en el que uno de sus campos sea del tipo referencia, será estríctamente necesario definir un **lifetime**, para prevenir que el *struct* completo no siga ocupando espacio en memoria, sobreviviendo al dato al que hace referencia.

## STATIC:
Es el único *lifetime* cuya sintaxis tiene un significado, que es que este tiempo de vida es equivalente a toda la duración del programa.

    let s1: &'static str = "hola";
    let s2: &'static str = "Mundo";

    println!("{}", longest(s1, s2));

Continuando con la función del primer ejemplo, si definimos dos variables en el código, que contienen un *&str* a las cuales les asignamos directamente su valor, entonces ese dato quedará almacenado en el binario luego de la compilación. Por consiguiente, ese dato será perenne y durará durante toda la ejecución del programa. 