mod tasks_manager;
mod utils;

use tasks_manager::Tasks;

fn main() {
    println!("Tasks Manager!");

    let mut tasks: Vec<Tasks> = Vec::new();

    loop {
        println!("Pilih Aktivitas!");
        println!("1. Tambah Task");
        println!("2. List semua Task");
        println!("3. Hapus Task");
        println!("4. Tandai task");
        println!("0. Keluar");

        let mut choice: String = String::new();
        utils::input_handler(&mut choice);

        match choice.trim() {
            "1" => Tasks::new(&mut tasks),
            "2" => Tasks::list(&tasks),
            "3" => Tasks::remove(&mut tasks),
            "4" => Tasks::check_task(&mut tasks),
            "0" => {
                println!("Keluar program...");
                break;
            }
            _ => println!("Pilihan tidak valid, coba lagi")
        }
    }
}
