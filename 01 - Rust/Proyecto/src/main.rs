use std::io;

struct Task {
    description: String,
    done: bool
}

impl Task {
    fn format_task(&self, id:usize){
        let status = if self.done{"[X]"} else {"[ ]"};
        println!("{} {}: {}", status, id, self.description);
    }
}

fn list_tasks(list: &Vec<Task> ) {
    println!("\n LISTA DE TAREAS:");
    for (i, item) in list.iter().enumerate() {
        item.format_task (i+1);
    }
}

fn main() {
    println!("---GESTOR DE TAREAS---");

    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("\nIngrese un comando: ");
        println!("agregar <descripción> / completar <id>  / listar / salir");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Inténtelo nuevamente.");
        let input = input.trim();

        if input == "salir"{
            println!("\n---FINALIZANDO GESTOR---");
            break;
        }else if input.starts_with("agregar") {
            let description = input[7..].trim().to_string();
            if !description.is_empty() {
                tasks.push(Task{
                    description: description.to_string(),
                    done: false
                });
                println!("\nTarea agregada: {}", description);
            }else {
                println!("La descripción no puede estar vacía.")
            }
        } else if input == "listar"{
            list_tasks(&tasks);
        }else if input.starts_with("completar") {
            let id: usize = match input[9..].trim().parse(){
                Ok(num)=> num,
                Err(_) =>{
                    println!("\nID no válido.");
                    continue;
                }
            };
            if id > 0 && id <= tasks.len() {
                tasks[id-1].done = true;
                println!("\nTarea {} completada:\n{}.", id, tasks[id-1].description);
            } else{
                println!("\nID no válido.");
            }
        }else {
            println!("Comando no reconocido. Inténtelo otra vez.\n")
        }
    }
}
