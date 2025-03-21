#[path ="utils.rs"]
mod utils;

pub struct Tasks {
    id: u32,
    title: String,
    completed: bool
}

impl Tasks {
    pub fn new(tasks: &mut Vec<Self>) {
        let mut id_input: String = String::new();

        println!("Masukkan ID: ");
        utils::input_handler(&mut id_input);

        println!("Masukkan Title Task: ");
        let mut title: String = String::new();
        utils::input_handler(&mut title);

        let id: u32 = match id_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("ID harus berupa angka!\n");
                return;
            }
        };

        tasks.push(Self {id, title, completed: false});

        println!("Task berhasil ditambahkan!");
    }

    pub fn list(tasks: &Vec<Self>) {
        for (_, task) in tasks.iter().enumerate() {
            println!("{}, Title: {}, Completed: {}", task.id, task.title, task.completed);
        }
    }

    pub fn remove(tasks: &mut Vec<Self>) {
        let mut id_input: String = String::new();

        println!("Masukkan ID: ");
        utils::input_handler(&mut id_input);

        let id: u32 = match id_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("ID harus berupa angka");
                return;
            }
        };

        if let Some(task) = tasks.iter_mut().position(|x| x.id == id) {
            tasks.remove(task);
            println!("Task berhasil dihapus!");
        } else {
            println!("Task tidak ditemukan!");
        }
    }

    pub fn check_task(tasks: &mut Vec<Self>) {
        let mut id_input: String = String::new();

        println!("Masukan ID: ");
        utils::input_handler(&mut id_input);

        let id: u32 = match id_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("ID harus berupa angka");
                return;
            }
        };

        if let Some(task) = tasks.iter_mut().find(|x| x.id == id) {
            task.completed = true;
            println!("Task selesai!");
        } else {
            println!("Task tidak ditemukan!");
        }
    }
}
