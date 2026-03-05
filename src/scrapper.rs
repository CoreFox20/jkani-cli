use reqwest::blocking::Client;
use scraper::{Html, Selector};
use regex::Regex;
use base64::{Engine as _, engine::general_purpose};

pub struct Anime{
    pub titulo: String,
    pub slug: String
}

pub fn buscar_anime(query: &str) -> Vec<Anime> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36")
        .build()
        .unwrap();

    let url = format!("https://jkanime.net/buscar/{}",query);
    let res = client.get(url).send().unwrap().text().unwrap();

    let doc = Html::parse_document(&res);

    let selector = Selector::parse("div.anime__item__text").unwrap();
    let link_sel = Selector::parse("a").unwrap();
    let titulo_sel = Selector::parse("h5").unwrap();

    let mut resultados = Vec::new();

    for elemento in doc.select(&selector){
        let h5 = elemento.select(&titulo_sel).next();

        let slug = h5
            .and_then(|e| e.select(&link_sel).next())
            .and_then(|e| e.value().attr("href"))
            .unwrap_or("")
            .trim_matches('/')
            .split('/')
            .last()
            .unwrap_or("")
            .to_string();

        // obtener título del texto
        let titulo = h5
            .map(|e| e.text().collect::<String>())
            .unwrap_or_default()
            .trim()
            .to_string();

        if !titulo.is_empty(){
            resultados.push(Anime {titulo,slug});
        }
    }
        resultados
}

pub fn obtener_episodios(slug: &str) -> u32 {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36")
        .build()
        .unwrap();

    let url = format!("https://jkanime.net/{}/", slug);
    let res = client.get(&url).send().unwrap().text().unwrap();

    let doc = Html::parse_document(&res);

    let span_sel = Selector::parse("div.anime__details__content ul li span").unwrap();

    for span in doc.select(&span_sel) {
        let texto = span.text().collect::<String>();
        
        if texto.trim() == "Episodios:" {
            if let Some(padre) = span.parent() {
                let padre_el = scraper::ElementRef::wrap(padre).unwrap();
                let texto_padre = padre_el.text().collect::<String>();
                let numero = texto_padre
                    .split("Episodios:")
                    .last()
                    .unwrap_or("0")
                    .trim()
                    .parse::<u32>()
                    .unwrap_or(0);

                if numero > 0 {
                    return numero;
                }
            }
        }
    }

     // Método 2: buscar div.proxep y extraer número del link
    let proxep_sel = Selector::parse("#proxep a").unwrap();

    for elemento in doc.select(&proxep_sel) {
        let href = elemento.value().attr("href").unwrap_or("");
        let numero = href
            .trim_matches('/')
            .split('/')
            .last()
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);

        if numero > 0 {
            return numero;
        }
    }

    0
}

pub fn obtener_video_url(slug: &str, episodio: u32) -> Option<String> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36")
        .build()
        .unwrap();

    // Paso 1: obtener la página del episodio
    let url = format!("https://jkanime.net/{}/{}/", slug, episodio);
    let res = client.get(&url)
        .header("Referer", "https://jkanime.net/")
        .send().ok()?.text().ok()?;

    // Paso 2: extraer la URL del jkplayer del array video[0]
    let re_player = Regex::new(r#"video\[0\]\s*=\s*'<iframe[^>]+src="([^"]+)""#).unwrap();
    let player_url = re_player.captures(&res)?.get(1)?.as_str().to_string();

    // Paso 3: hacer GET al jkplayer
    let player_html = client.get(&player_url)
        .header("Referer", "https://jkanime.net/")
        .send().ok()?.text().ok()?;

    // Paso 4: extraer el base64 del atob('...')
    let re_atob = Regex::new(r#"atob\('([A-Za-z0-9+/=]+)'\)"#).unwrap();
    let b64 = re_atob.captures(&player_html)?.get(1)?.as_str();

    // Paso 5: decodificar base64
    let decoded = general_purpose::STANDARD.decode(b64).ok()?;
    String::from_utf8(decoded).ok()
}