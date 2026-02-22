//! Types forts pour les langues
//!
//! Ce module fournit un typage sécurisé pour les codes de langue
//! utilisés dans toute l'application.

use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Langues supportées par GSP
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum Language {
    /// Français (France)
    French,
    /// Anglais (États-Unis)
    EnglishUS,
    /// Anglais (Royaume-Uni)
    EnglishGB,
    /// Allemand
    German,
    /// Espagnol
    Spanish,
    /// Italien
    Italian,
    /// Portugais
    Portuguese,
    /// Néerlandais
    Dutch,
    /// Polonais
    Polish,
    /// Russe
    Russian,
    /// Japonais
    Japanese,
    /// Chinois (Mandarin)
    Chinese,
    /// Coréen
    Korean,
    /// Arabe
    Arabic,
    /// Hébreu
    Hebrew,
    /// Hindi
    Hindi,
    /// Turc
    Turkish,
    /// Suédois
    Swedish,
    /// Norvégien
    Norwegian,
    /// Danois
    Danish,
    /// Finnois
    Finnish,
    /// Détection automatique
    Auto,
}

impl Language {
    /// Code de langue au format BCP 47
    pub fn code(&self) -> &'static str {
        match self {
            Language::French => "fr-FR",
            Language::EnglishUS => "en-US",
            Language::EnglishGB => "en-GB",
            Language::German => "de-DE",
            Language::Spanish => "es-ES",
            Language::Italian => "it-IT",
            Language::Portuguese => "pt-PT",
            Language::Dutch => "nl-NL",
            Language::Polish => "pl-PL",
            Language::Russian => "ru-RU",
            Language::Japanese => "ja-JP",
            Language::Chinese => "zh-CN",
            Language::Korean => "ko-KR",
            Language::Arabic => "ar-SA",
            Language::Hebrew => "he-IL",
            Language::Hindi => "hi-IN",
            Language::Turkish => "tr-TR",
            Language::Swedish => "sv-SE",
            Language::Norwegian => "no-NO",
            Language::Danish => "da-DK",
            Language::Finnish => "fi-FI",
            Language::Auto => "auto",
        }
    }

    /// Nom lisible de la langue
    pub fn name(&self) -> &'static str {
        match self {
            Language::French => "Français",
            Language::EnglishUS => "English (US)",
            Language::EnglishGB => "English (UK)",
            Language::German => "Deutsch",
            Language::Spanish => "Español",
            Language::Italian => "Italiano",
            Language::Portuguese => "Português",
            Language::Dutch => "Nederlands",
            Language::Polish => "Polski",
            Language::Russian => "Русский",
            Language::Japanese => "日本語",
            Language::Chinese => "中文",
            Language::Korean => "한국어",
            Language::Arabic => "العربية",
            Language::Hebrew => "עברית",
            Language::Hindi => "हिन्दी",
            Language::Turkish => "Türkçe",
            Language::Swedish => "Svenska",
            Language::Norwegian => "Norsk",
            Language::Danish => "Dansk",
            Language::Finnish => "Suomi",
            Language::Auto => "Détection automatique",
        }
    }

    /// Liste de toutes les langues supportées (sauf Auto)
    pub fn all() -> &'static [Language] {
        &[
            Language::French,
            Language::EnglishUS,
            Language::EnglishGB,
            Language::German,
            Language::Spanish,
            Language::Italian,
            Language::Portuguese,
            Language::Dutch,
            Language::Polish,
            Language::Russian,
            Language::Japanese,
            Language::Chinese,
            Language::Korean,
            Language::Arabic,
            Language::Hebrew,
            Language::Hindi,
            Language::Turkish,
            Language::Swedish,
            Language::Norwegian,
            Language::Danish,
            Language::Finnish,
        ]
    }

    /// Tente de détecter la langue depuis un code BCP 47
    pub fn from_code(code: &str) -> Option<Self> {
        code.parse().ok()
    }
}

impl FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "fr-fr" | "fr" => Ok(Language::French),
            "en-us" | "en" => Ok(Language::EnglishUS),
            "en-gb" => Ok(Language::EnglishGB),
            "de-de" | "de" => Ok(Language::German),
            "es-es" | "es" => Ok(Language::Spanish),
            "it-it" | "it" => Ok(Language::Italian),
            "pt-pt" | "pt-br" | "pt" => Ok(Language::Portuguese),
            "nl-nl" | "nl" => Ok(Language::Dutch),
            "pl-pl" | "pl" => Ok(Language::Polish),
            "ru-ru" | "ru" => Ok(Language::Russian),
            "ja-jp" | "ja" => Ok(Language::Japanese),
            "zh-cn" | "zh-tw" | "zh" => Ok(Language::Chinese),
            "ko-kr" | "ko" => Ok(Language::Korean),
            "ar-sa" | "ar" => Ok(Language::Arabic),
            "he-il" | "he" => Ok(Language::Hebrew),
            "hi-in" | "hi" => Ok(Language::Hindi),
            "tr-tr" | "tr" => Ok(Language::Turkish),
            "sv-se" | "sv" => Ok(Language::Swedish),
            "no-no" | "no" => Ok(Language::Norwegian),
            "da-dk" | "da" => Ok(Language::Danish),
            "fi-fi" | "fi" => Ok(Language::Finnish),
            "auto" => Ok(Language::Auto),
            _ => Err(format!("Langue non supportée: {}", s)),
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name(), self.code())
    }
}
