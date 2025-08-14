# MÓDULOS
Se denominan módulos a los diferentes archivos en los que se definen las funcionalidades de un programa, para mejor organización y diferenciación de sus partes. También existen módulos predeterminados incluidos en las librerías del lenguaje de programación. Por ejemplo, en un archivo llamado **network.rs** podemos definir las siguientes funciones:

    pub fn connect () {
        println!("Conectando...");
    }
    pub fn disconnect () {
        println!("Desconectando...");
    }
    pub fn process () {
        println!("Procesando...");
    }

Para invocar a los módulos desde otro archivo, por ejemplo **main.rs** se pueden utilizar las siguientes sintaxis:

    mod network;
    network::connect();

importa todo el módulo, se debe utilizar la sintaxis de :: al invocar la función
    
    use network::{connect, disconnect}; 
    disconnect();
    connect();    

importa esas dos funciones y se pueden invocar por su nombre individual.
    
    use network::*; 
    process();

importa todas las funciones y pueden invocarse por sus nombres individuales.

    use network::{connect as cxn};
    cxn();

En este ejemplo, se importa la función *connect* con un alias, esto es útil cuando se importan dos funciones que tengan el mismo nombre, o cuando una función importada tiene un nombre muy extenso.


# EXTENSIONES VSCODE

* **Dependi:** Para verificar que las dependencias que se importan al cargo.toml son válidas y no tienen issues. También muestra las versiones recomendadas.
* **Rust-analyzer:** Un servidor de lenguaje (LSP) que entiende la semántica de tu código Rust y te ofrece una gran variedad de funcionalidades como Autocompletado, Inlay Hints, Análisis en tiempo real, etc.
* **GitHub Copilot:** un asistente de programación impulsado por inteligencia artificial. Su función principal es ayudarte a escribir código más rápido y con mayor eficiencia al proporcionar sugerencias, autocompletado y, en general, un "copiloto" que trabaja contigo mientras programas. 
* **Copilot chat:** Versión conversacional de GitHub Copilot. Permite interactuar con la IA a través de una ventana de chat.
* **Gitlens:** mejora la capacidad para entender y navegar por el historial de un repositorio Git directamente desde la interfaz de VS Code. Evita el uso de la línea de comandos o una aplicación externa para ver los cambios en el código. 

# CRATE SERDE
Librería destinada a Serializar y Deserializar estructuras de datos. Su nombre es una abreviación de **SERialization & DEserialization** y facilita la conversión de estructuras de datos de Rust a diferentes formatos (como JSON, YAML, TOML) y viceversa. Para su utilización se debe añadir la dependencia en el archivo **cargo.toml**:

    [dependencies]
    serde = { version = "1.0", features = ["derive"] }
    serde_json = "1.0"

Y en el código se añade el *trait* a la estructura que se necesitará serializar y deserializar, mediante la sintaxis de *derive*:

    #[derive(Serialize, Deserialize, Debug)]
    struct Persona {
        nombre: String,
        edad: u8,
    }

