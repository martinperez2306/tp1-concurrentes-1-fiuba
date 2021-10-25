use std::fs::OpenOptions;
use std::io::Write;

pub fn log(message: String) {
    let result = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("resources/reserves.log");
    match result {
        Ok(mut file) => {
            println!("{}", message);
            if let Err(e) = writeln!(file, "{}", message) {
                println!("No se pudo escribir el archivo: {}", e);
            }
        }
        Err(e) => { println!("No se pudo abrir el archivo: {}", e); }
    }
}