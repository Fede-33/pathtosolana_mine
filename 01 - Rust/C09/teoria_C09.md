# TRAITS
son una forma de definir comportamientos o funcionalidades, que luego pueden ser implementados a distintos elementos. En Rust son una de las formas mas simples de lograr el polimorfismo cuando se trabaja con *structs*. Para definirlos, se utiliza la sentencia *trait* y se define una función, que será la firma que deberán incluir los conjuntos de datos que luego necesiten implementar el *trait*. Por ejemplo:

    pub trait Summary {
        fn summarize (&self) -> String;
    }
Aquí se define un *trait* llamado *Summary* que incluye una funcionalidad *summarize* que toma referencias de los datos de sí mismo, y retorna un *String*. A continuación se definen dos structs:

    pub struct News{
        pub headline: String,
        pub author: String,
    }
    pub struct Tweet{
        pub username: String,
        pub content: String,
    }
Para implementar el trait *Summary* en esos *structs* se debe hacer explícitamente, y definir que comportamiento tendrá el *trait* en cada *struct*:

    impl Summary for News {
        fn summarize (&self) -> String {
            format!("{} by {}", self.headline, self.author)
        }
    }
    impl Summary for Tweet {
        fn summarize (&self) -> String {
            format!("@{}: {}", self.username, self.content)
        }
    }
En este ejemplo, se define que el trait *Summary* realizará formatos dentro de los datos de cada *struct*, pero para cada uno de ellos, formateará de manera diferente. A continuación se puede definir una función que integre ambas aplicaciones del *trait*:

    pub fn notify (item: &impl Summary){
        println! ("Breaking news! {}", item.summarize())
    }
Esta función *notify()* recibe un parámetro *item*, que se especifica que es una referencia a un tipo de dato que tiene implementado el trait *Summary*. Es decir, que esta función no podrá aceptar como parámetro, algo que no tenga implementado ese *trait*. El comportamiento de la función es imprimir lo que retorna la función *summarize()*, que es la propia del trait *Summary*

    fn main() {

        let tweet_pepe = Tweet{
            username: "Pepe_eaea_pepe".to_string(),
            content: "Andate Riquelme!!!".to_string()
        };
        let news_pepe = News{
            headline: "Riquelme se tiene que ir".to_string(),
            author: "Pepe_eaea_pepe".to_string()
        };

        notify(&tweet_pepe);
        notify(&news_pepe);
    }
En el código principal, se definen dos structs del tipo *Tweet* y *News*, y puede llamarse a la función *notify()* usando una referencia a cada uno de ellos como parámetro, porque tienen implementado el trait *Summary*. Lo que retorna la impresión del formato que retorna *summarize()*

# GENERIC TYPE
Cuando se tiene que aplicar una función a diferentes tipos de dato, deberíamos definirla para cada uno, de esta manera:

    fn largest_i32 (lista: &[i32]) -> i32 {
        let mut larger = lista[0];
        for &item in lista.iter() {
            if item > larger {
                larger = item;
            }
        }
        larger
    }
    fn largest_i64 (lista: &[i64]) -> i64 {
        let mut larger = lista[0];
        for &item in lista.iter() {
            if item > larger {
                larger = item;
            }
        }
        larger
    }

Dos funciones exactamente iguales para ordenar conjuntos de datos, definidas para cada tipo de dato. Esto podría solucionarse mediante el uso de un tipo genérico en su definición:

    fn largest<T> (lista: &[T]) -> T {
        let mut larger = lista[0];
        for &item in lista.iter() {
            if item > larger {
                larger = item;
            }
        }
        larger
    }

En este caso, se define que **T** será un tipo genérico y que la función ahora puede aceptar como parámetro cualquier tipo de dato. Esto presenta un problema, y es que cuando el compilador llegue a la comparación entre menor y mayor **>** notará que no todos los tipos de datos son comparables. Entonces retornará un error de compilación, diciendo que es una operación inaplicable a un tipo genérico, porque en caso de ingresar, por ejemplo, un *struct* o un *enum* como parámetro, no podría compararlo directamente. Para solucionar este problema, se utilizan los **traits**:

use std::cmp::PartialOrd;

fn largest<T: PartialOrd + Copy> (lista: &[T]) -> T {
    let mut larger = lista[0];
    for &item in lista.iter() {
        if item > larger {
            larger = item;
        }
    }
    larger
}

En este ejemplo, se define que el tipo genérico **T** deberá ser, sí o sí, un tipo de dato que incluya los traits *PartialOrd* y *Copy*. Ambos son de la librería standard. **PartialOrd** implementa la funcionalidad de poder ordenar cada uno de los elementos del conjunto, según un criterio de mayor a menor, puede ser numérico o alfanumérico. **Copy** es necesario porque en la definición de *let mut larger = lista[0];* se asigna el valor del primer elemento de la lista a la variable, sin el trait *copy* Rust intentaría mover el *Ownership* del valor, lo cual derivaría en un error, y lo mismo sudecería cuando el iterador asignara cada valor de la lista al *item*.