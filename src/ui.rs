use inquire::{Select, Text};
use crate::scrapper;
use crate::player;

pub fn iniciar() {
    // Paso 1: pedir búsqueda
    let query = Text::new("Buscar anime:")
        .prompt()
        .unwrap();

    // Paso 2: buscar y mostrar resultados
    let resultados = scrapper::buscar_anime(&query);

    if resultados.is_empty() {
        println!("No se encontraron resultados para '{}'", query);
        return;
    }

    // Paso 3: elegir anime
    let opciones: Vec<String> = resultados
        .iter()
        .map(|a| a.titulo.clone())
        .collect();

    let seleccion = Select::new("Elige un anime:", opciones.clone())
        .prompt()
        .unwrap();

    let anime = resultados
        .iter()
        .find(|a| a.titulo == seleccion)
        .unwrap();

    println!("Obteniendo episodios de {}...", anime.titulo);

    // Paso 4: obtener total de episodios
    let total = scrapper::obtener_episodios(&anime.slug);

    if total == 0 {
        println!("No se pudieron obtener los episodios");
        return;
    }

    println!("Total de episodios: {}", total);

    // Paso 5: elegir episodio
    let episodios: Vec<String> = (1..=total)
        .map(|i| format!("Episodio {}", i))
        .collect();

    let ep_seleccion = Select::new("Elige un episodio:", episodios)
        .prompt()
        .unwrap();

    let ep_numero: u32 = ep_seleccion
        .replace("Episodio ", "")
        .parse()
        .unwrap();

    // Paso 6: obtener URL y reproducir
    println!("Obteniendo URL del video...");

    match scrapper::obtener_video_url(&anime.slug, ep_numero) {
        Some(url) => player::reproducir(&url),
        None      => println!("No se pudo obtener la URL del video"),
    }
}