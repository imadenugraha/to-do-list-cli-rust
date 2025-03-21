#[path ="utils.rs"]
mod utils;

#[derive(Debug)]
pub struct Tasks {
    id: u32,
    title: String,
    completed: bool
}

impl Tasks {
    pub fn new(id: u32, title: String) -> Self {
        Self { id, title, completed: false }
    }

    pub fn mark_completed(&mut self) {
        self.completed = true;
    }
}

pub fn add_task(tasks: &mut Vec<Tasks>) {
    let mut id_input: String = String::new();

    println!("Masukkan ID: ");
    if let Err(e) = utils::input_handler(&mut id_input) {
        println!("Error: {:?}", e);
        return;
    }

    println!("Masukkan Title Task: ");
    let mut title: String = String::new();
    if let Err(e) = utils::input_handler(&mut title) {
        println!("Error: {:?}", e);
        return;
    }

    let id: u32 = match id_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("ID harus berupa angka!\n");
            return;
        }
    };

    tasks.push(Tasks::new(id, title));

    println!("Task berhasil ditambahkan!");
}

pub fn list(tasks: &[Tasks]) {
    for task in tasks.iter() {
        println!("{}. Title: {}, Completed: {}", 
        task.id, 
        task.title, 
        if task.completed {"Selesai!"} else {"Belum Selesai!"});
    }
}

pub fn remove(tasks: &mut Vec<Tasks>) {
    let mut id_input: String = String::new();

    println!("Masukkan ID: ");
    if let Err(e) = utils::input_handler(&mut id_input) {
        println!("Error: {:?}", e);
        return;
    }

    let id: u32 = match id_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("ID harus berupa angka");
            return;
        }
    };

    match tasks.iter().position(|task: &Tasks| task.id == id) {
        Some(index) => {
            tasks.remove(index);
            println!("Task berhasil dihapus!");
        }
        None => println!("Task tidak ditemukan!"),
    }
}

pub fn check_task(tasks: &mut Vec<Tasks>) {
    let mut id_input: String = String::new();

    println!("Masukan ID: ");
    if let Err(e) = utils::input_handler(&mut id_input) {
        println!("Error: {:?}", e);
        return;
    }

    let id: u32 = match id_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("ID harus berupa angka");
            return;
        }
    };

    if let Some(task) = tasks.iter_mut().find(|x: &&mut Tasks| x.id == id) {
        task.mark_completed();
        println!("Task selesai!");
    } else {
        println!("Task tidak ditemukan!");
    }
}
