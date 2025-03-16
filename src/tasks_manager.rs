#[path ="utils.rs"]
mod utils;

pub struct Tasks {
    pub id: u32,
    pub title: String,
    pub completed: bool
}

pub fn add_task(tasks: &mut Vec<Tasks>) {
    let mut id_input: String = String::new();

    println!("Masukan ID: ");
    utils::input_handler(&mut id_input);

    println!("Masukan Title Task: ");
    let mut title: String = String::new();
    utils::input_handler(&mut title);

    let id: u32 = match id_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("ID harus berupa angka!\n");
            return;
        }
    };

    tasks.push(Tasks{ id, title: String::from(title), completed: false});

    println!("Task berhasil ditambahkan!\n");
}

pub fn list_tasks(tasks: &Vec<Tasks>) {
    for (_, task) in tasks.iter().enumerate() {
        println!("{}. Title: {}, Completed: {}\n", task.id, task.title, task.completed);
    }
}

pub fn delete_task(tasks: &mut Vec<Tasks>) {
    let mut id_input: String = String::new();

    println!("Masukkan ID: ");
    utils::input_handler(&mut id_input);

    let id: u32 = match id_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("ID harus berupa angka!\n");
            return;
        }
    };

    if let Some(task) = tasks.iter_mut().position(|x: &mut Tasks | x.id == id) {
        tasks.remove(task);
        println!("Tasks with ID: {} deleted!\n", id);
    } else {
        println!("Tasks with ID: {} not found!\n", id);
    }
}

pub fn check_task(tasks: &mut Vec<Tasks>) {
    let mut id_input: String = String::new();

    println!("Masukkan ID: ");
    utils::input_handler(&mut id_input);

    let id: u32 = match id_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("ID harus berupa angka!\n");
            return;
        }
    };

    if let Some(task) = tasks.iter_mut().find(|x: &&mut Tasks| x.id == id) {
        task.completed = true;
        println!("Task with ID: {} done!\n", id);
    } else {
        println!("Task with ID: {} not found!\n", id);
    }
}
