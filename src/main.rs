struct Task {
    description: String,
    done: bool,
}

impl Task {
    fn new(description: &str) -> Task {
        Task {
            description: description.to_string(),
            done: false,
        }
    }
}

struct TodoApp {
    tasks: Vec<Task>,
}

impl TodoApp {
    fn new() -> TodoApp {
        TodoApp {
            tasks: Vec::new(),
        }
    }

    // Adding tasks
    fn add_new_task(&mut self, description: &str) {
        let task = Task::new(description);
        self.tasks.push(task);
    }

    fn mark_task_as_done(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.done = true;
        }
    }

    // Displaying tasks
    fn show_tasks(&self) {
        for (index, task) in self.tasks.iter().enumerate() {
            let status = if task.done { "[X]" } else { "[ ]" };
            println!("{} {} : {}", index + 1, task.description, status);
        }
    }
}



fn main() {
    let mut app = TodoApp::new();
    
    app.add_new_task("Learn Rust");
    app.add_new_task("Build a project");
    
    app.show_tasks();
}


