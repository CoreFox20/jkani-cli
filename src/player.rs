use std::process::Command;

pub fn reproducir(url: &str) {
    println!("Reproduciendo...");

    let status = Command::new("mpv")
        .arg(url)
        .arg("--really-quiet")  // menos output en terminal
        .status();

    match status {
        Ok(s) => {
            if !s.success() {
                println!("mpv terminó con error");
            }
        }
        Err(e) => {
            println!("Error al lanzar mpv: {}", e);
            println!("¿Está mpv instalado? Prueba: sudo apt install mpv");
        }
    }
}