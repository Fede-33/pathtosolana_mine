# OPCIONES DEL COMPILADOR 

## DOCUMENTACIÓN:
Mediante el comando **cargo doc** en la consola, es posible construir la documentación del proyecto, en formato *.html*. Mediante el comando *cargo doc --open* se abrirá en un navegador. La documentación se crea automáticamente, describiendo los componentes del código, con el mismo formato que la **Documentación de Rust:** https://doc.rust-lang.org/stable/

Pueden usarse los comentarios para describir las características de los bloques o líneas de código, y estos se agregarán automáticamente a la documentación compilada. 
* // comentarios que describen a la documentación general.
* /// comentarios que describen al próximo elemento del código.

## LIBRERÍAS:
Mediante el comando **cargo new (nombre) --lib** en vez de crear un ejecutable del código, se lo compila como una librería, también llamadas *crates* en Rust. Cómo toda librería, puede importarse desde otro código, y también crearse la documentación de cada *Crate* que se importe al proyecto.

## TESTS:
Mediante el comando **cargo test** pueden ejecutarse todos los test que se definan dentro del código. Los *tests* sirven para para probar unidades de código pequeñas y aisladas, como funciones individuales o módulos. Por ejemplo:

    pub fn sumar(a: i32, b: i32) -> i32 {
        a + b
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_sumar_positivos() {
            assert_eq!(sumar(2, 3), 5);
        }
    }
El código anterior define una función *sumar* y luego un test para comprobar su funcionamiento. La sintaxis *#[cfg(test)]* lo define como un test, y luego se llama al módulo *test* para describir el comportamiento que debería tener y su resultado esperado.

Es una buena práctica crear un directorio con un archivo *.rs* en el que se definan todos los tests. Cada una de las funciones que se vayan a testear, deberán ser importadas desde las librerías o ejecutables correspondientes de nuestro código, mediante la sintaxis *add*. 

# STRUCTS
La estructura es un tipos de dato compuesto, que permite agrupar varios valores relacionados en una sola unidad lógica, especificando qué datos contendrá y de qué tipo serán esos datos. Para definir y crear una instancia de un *struct* se utiliza la siguiente sintaxis:

       struct User {
           name: String,
           age: u8,
       }
       
       let user = User {
           name: String::from("Pepe"),
           age: 30,
       };
           let user2 = User {
           name: String::from("Ana"),
           age: 25,
       };

Se definió la estructura *User* con los atributos *name* y *age*. Luego, mediante la sentencia *let* se crearon dos instancias de *User* como si se definiera una variable. Para acceder a los elementos de un *struct* se utiliza la sintaxis de punto:

    println!("Nombre: {} - Edad: {}", user.name, user.age);
    println!("Nombre: {} - Edad: {}", user2.name, user2.age)

Los structs se almacenan en el *heap* y pueden contener otros *structs*. Por ejemplo:
        
    struct User {
        name: String,
        age: u8,
        manager: Manager
    }
    #[derive(Clone)]
    struct Manager {
        name: String,
        age: u8,
        group: u8
    }

    let manager1 = Manager {
        name: String::from("Ana"),
        age: 35,
        group: 1
    };
    let user1 = User {
        name: String::from("Pepe"),
        age: 30,
        manager: manager1.clone()
    };
    let user2 = User {
        name: String::from("Juan"),
        age: 25,
        manager: manager1.clone()
    };

    println!("Nombre: {} - Edad: {} - Coordinador: {}", user1.name, user1.age, user1.manager.name);
    println!("Nombre: {} - Edad: {} - Coordinador: {}", user2.name, user2.age, user2.manager.name);
    println!("Nombre: {} - Edad: {} - Grupo: {}", manager1.name, manager1.age, manager1.group)

El ejemplo anterior crea dos *structs* para organizar administradores (Manager) y usuarios (User). Los managers tienen un grupo a cargo, y los users tienen un manager asignado. Puesto que, cuando se instancien los usuarios, requerirán de que se les asigne un administrador en su atributo *manager*, es necesario que el struct *Manager* pueda usar el *Clone trait*, porque si al instanciar un usuario utilizamos la sintaxis *manager: manager1*, entonces se estaría cambiando el Ownsership de todos los datos de *manager1*, por lo que se debe asignar mediante *manager: manager1.clone()*. Para que la estructura *Manager* pueda usar el *Clone trait* se debe especificar mediante la sintaxis *#[derive(Clone)]*.   

## Not POO:
En Rust no existe la programación orientada a objetos, pero puede emularse utilizando estructuras *(structs)*, con la diferencia de que los métodos no forman parte de las  estructuras ni de sus interfaces, y que no existe la *herencia*, sino la *composición*.
