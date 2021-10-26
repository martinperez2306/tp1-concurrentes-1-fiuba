use std::fs::OpenOptions;
use std::io::Write;

/** It logs the message through console and to a file located in ./resources/reserves.log.
* The file needs to be previously created. */
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