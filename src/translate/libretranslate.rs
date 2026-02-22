use std::collections::HashMap;

use super::TranslateEngine;
use tracing::{debug, error, info, warn};
// use serde_json::json;

pub struct Libretranslate {}

impl TranslateEngine for Libretranslate {
    fn translate(
        &self,
        text: &str,
        lang_from: &str,
        lang_to: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = "https://libretranslate.com/translate";

        debug!("Traduction depuis {} vers {}", lang_from, lang_to);
        debug!("Texte à traduire: {} caractères", text.len());

        // request user agent
        let client = reqwest::blocking::Client::new();

        let lang_from = lang_from[..2].to_string();
        let lang_to = lang_to[..2].to_string();

        let mut params = HashMap::new();
        params.insert("q", text);
        params.insert("source", &lang_from);
        params.insert("target", &lang_to);
        params.insert("format", "text");
        params.insert("api_key", "");

        info!("Envoi de la requête de traduction à {}", url);
        let res = client
            .post(url)
            .header("Accept", "*/*")
            .header("Accept-Language", "fr,fr-FR;q=0.8,en-US;q=0.5,en;q=0.3")
            .header("Cache-Control", "no-cache")
            .header("Connection", "keep-alive")
            .header("Origin", "https://libretranslate.com")
            .header("Pragma", "no-cache")
            .header("Referer", "https://libretranslate.com/")
            .header("Sec-Fetch-Dest", "empty")
            .header("Sec-Fetch-Mode", "no-cors")
            .header("Sec-Fetch-Site", "same-origin")
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; rv:105.0) Gecko/20100101 Firefox/105.0",
            )
            .form(&params)
            .send()
            .map_err(|e| {
                error!("Échec de l'envoi de la requête: {}", e);
                e
            })?;

        // let res = client
        //     .post(url)
        //     .json(&json!({
        //         "q": text,
        //         "source": lang_from,
        //         "target": lang_to,
        //     }))
        //     .header(
        //         "User-Agent",
        //         "Mozilla/5.0 (Windows NT 10.0; rv:105.0) Gecko/20100101 Firefox/105.0",
        //     )
        //     .send()
        //     .unwrap();

        let status = res.status();
        debug!("Statut de la réponse: {}", status);

        let body = res.text()?;
        debug!("Réponse brute: {} caractères", body.len());

        let body: serde_json::Value = serde_json::from_str(&body).map_err(|e| {
            error!("Échec du parsing JSON: {}", e);
            e
        })?;

        if body["translatedText"].is_null() {
            warn!("Aucun texte traduit dans la réponse");
            Ok(String::from(""))
        } else {
            let translated = body["translatedText"].to_string();
            info!("Traduction réussie: {} caractères", translated.len());
            Ok(translated)
        }
    }

    fn is_available(&self) -> bool {
        true
    }
}
