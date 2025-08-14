use eframe::egui;

#[derive(Debug, Default)]
struct Task {
    description: String,
    done: bool
}

struct TaskManagerApp {
    tasks: Vec<Task>,
    new_task: String,
}

impl Default for TaskManagerApp{
    fn default() -> Self {
        Self{
            tasks: Vec::new(),
            new_task: String::new(),
        }
    }
}

impl eframe::App for TaskManagerApp{
    fn update (&mut self, ctx:&egui::Context, _frame:&mut eframe::Frame){
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("GESTOR DE TAREAS");
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.label("Nueva Tarea:");
                ui.text_edit_singleline(&mut self.new_task);
                if ui.button("Agregar Tarea").clicked(){
                    let new_description = self.new_task.trim().to_string();
                    if !new_description.is_empty(){
                        self.tasks.push(Task{
                            description: new_description,
                            done: false,
                        });
                        self.new_task.clear();
                    }
                }
            });

            ui.add_space(20.0);

            ui.heading("TAREAS PENDIENTES:");

            for (i,task) in self.tasks.iter().enumerate() {
                if !task.done {
                    ui.label(format!("{}. {}", i+1,task.description));
                }
            }
        });
    }
}


fn main() -> eframe::Result<()> {
    
    let options = eframe::NativeOptions {
        viewport:egui::ViewportBuilder::default().with_inner_size([600.0, 300.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Gestor de tareas",
        options,
        Box::new(|_cc| Ok(Box::new(TaskManagerApp::default()))),
    )
}
