#[derive(Debug)]
struct User{
        active: bool,
        username: String,
        email: String,
        last_login: i64
    }

impl User {
    fn new (name: String)-> Self{
        Self {
            active: true,
            username: name,
            email: String::from("mail@usuario.com"),
            last_login: 0
        }
    }

    fn mostrar (&self) {
        println!("-----USUARIO-----");
        println!("Nombre: {}", self.username);
        println!("{}", if self.active{"Usuario activo"}else{"Usuario inactivo"});
        println!("Email: {}", self.email);
        println!("ültima conexión: {}", self.last_login);
    }
}

fn main() {
    
    let mut user1 = User{
        active: true,        
        username: String::from("Juan Perez"),
        email: String::from("juan@usuario.com"),
        last_login: 1,
    };    

    println!("{}", user1.username);
    user1.username = "Pepe".to_string();
    println!("{}", user1.username);
    dbg!("{}", &user1);
    
    let mut user2= User::new("Jorge".to_string()); 
    dbg!("{}", &user2);

    user1.mostrar();
    user2.mostrar();
}
