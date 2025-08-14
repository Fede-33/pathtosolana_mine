fn main() {
    
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
}
