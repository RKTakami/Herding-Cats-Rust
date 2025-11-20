/// Internationalization (i18n) Support System
/// Provides comprehensive multi-language support and localization features

use std::collections::{HashMap, BTreeMap};
use std::sync::{Arc, RwLock};
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, TimeZone, Local};
use unic_langid::{LanguageIdentifier, langid};

/// Supported languages and regions
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Language {
    pub code: String,        // e.g., "en", "es", "zh"
    pub region: Option<String>, // e.g., "US", "ES", "CN"
    pub name: String,        // Native name
    pub english_name: String,
    pub direction: TextDirection,
    pub plural_rules: PluralRules,
}

/// Text direction for languages
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextDirection {
    LeftToRight,
    RightToLeft,
    Vertical, // For languages like traditional Chinese
}

/// Plural rule types based on CLDR specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluralRules {
    pub rules: Vec<PluralRule>,
    pub default_rule: PluralRule,
}

/// Individual plural rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluralRule {
    pub category: PluralCategory,
    pub condition: PluralCondition,
    pub examples: Vec<String>,
}

/// Plural categories
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PluralCategory {
    Zero,
    One,
    Two,
    Few,
    Many,
    Other,
}

/// Plural condition for rule matching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluralCondition {
    pub operator: PluralOperator,
    pub operands: Vec<PluralOperand>,
}

/// Plural operators
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PluralOperator {
    Equals,
    NotEquals,
    InRange,
    NotInRange,
    Modulo,
}

/// Plural operands
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PluralOperand {
    Number(f64),
    Variable(String),
    Modulo { value: f64, divisor: f64 },
}

/// Translation bundle for a language
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationBundle {
    pub language: Language,
    pub translations: HashMap<String, TranslationEntry>,
    pub metadata: TranslationMetadata,
    pub version: String,
}

/// Translation entry with placeholders and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationEntry {
    pub key: String,
    pub value: String,
    pub context: Option<String>,
    pub description: Option<String>,
    pub placeholders: Vec<Placeholder>,
    pub examples: Vec<String>,
    pub fuzzy: bool,
    pub obsolete: bool,
    pub custom_translations: HashMap<String, CustomTranslation>,
}

/// Placeholder information for variable interpolation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Placeholder {
    pub name: String,
    pub placeholder_type: PlaceholderType,
    pub format_specifier: Option<String>,
    pub description: Option<String>,
    pub examples: Vec<String>,
}

/// Types of placeholders
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlaceholderType {
    Simple,       // {name}
    Number,       // {name, number}
    Date,         // {name, date}
    Time,         // {name, time}
    DateTime,     // {name, datetime}
    Currency,     // {name, currency}
    Percent,      // {name, percent}
    Plural,       // {name, plural}
    Select,       // {name, select}
    Custom(String),
}

/// Custom translation variations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomTranslation {
    pub context: String,
    pub value: String,
    pub conditions: Vec<TranslationCondition>,
}

/// Conditions for custom translations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationCondition {
    pub property: String,
    pub operator: ConditionOperator,
    pub value: serde_json::Value,
}

/// Condition operators
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConditionOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    Contains,
    StartsWith,
    EndsWith,
}

/// Translation metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationMetadata {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub translator: String,
    pub reviewer: Option<String>,
    pub quality_score: f32,
    pub completion_percentage: f32,
    pub notes: Vec<String>,
}

/// Locale-specific formatting rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocaleFormat {
    pub date_format: DateFormat,
    pub time_format: TimeFormat,
    pub number_format: NumberFormat,
    pub currency_format: CurrencyFormat,
    pub calendar_type: CalendarType,
}

/// Date formatting rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateFormat {
    pub short: String,     // e.g., "MM/dd/yyyy"
    pub medium: String,    // e.g., "MMM dd, yyyy"
    pub long: String,      // e.g., "MMMM dd, yyyy"
    pub full: String,      // e.g., "EEEE, MMMM dd, yyyy"
    pub custom_formats: HashMap<String, String>,
}

/// Time formatting rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeFormat {
    pub short: String,     // e.g., "HH:mm"
    pub medium: String,    // e.g., "HH:mm:ss"
    pub long: String,      // e.g., "HH:mm:ss z"
    pub full: String,      // e.g., "HH:mm:ss zzzz"
    pub custom_formats: HashMap<String, String>,
}

/// Number formatting rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumberFormat {
    pub decimal_separator: char,
    pub thousands_separator: char,
    pub decimal_places: u8,
    pub grouping: GroupingStyle,
    pub negative_format: NegativeFormat,
}

/// Number grouping styles
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GroupingStyle {
    Western,      // 1,234,567.89
    Indian,       // 12,34,567.89
    Chinese,      // 123,4567.89
    None,
}

/// Negative number formatting
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NegativeFormat {
    Minus,        // -1234.56
    Parenthesis,  // (1234.56)
    Custom(String),
}

/// Currency formatting rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyFormat {
    pub symbol: String,
    pub symbol_position: SymbolPosition,
    pub decimal_places: u8,
    pub space_before: bool,
    pub custom_codes: HashMap<String, CurrencyInfo>,
}

/// Currency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyInfo {
    pub name: String,
    pub symbol: String,
    pub decimal_places: u8,
    pub symbol_position: SymbolPosition,
}

/// Currency symbol positioning
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SymbolPosition {
    Before,       // $123.45
    After,        // 123.45$
    SpaceBefore,  // $ 123.45
    SpaceAfter,   // 123.45 $
}

/// Calendar types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CalendarType {
    Gregorian,
    Islamic,
    Hebrew,
    Persian,
    Japanese,
    Chinese,
    Thai,
}

/// Regional preferences and cultural adaptations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalPreferences {
    pub region: String,
    pub cultural_adaptations: CulturalAdaptations,
    pub measurement_system: MeasurementSystem,
    pub paper_size: PaperSize,
    pub first_day_of_week: u8,
    pub weekend_days: Vec<u8>,
    pub holiday_calendar: HolidayCalendar,
}

/// Cultural adaptations for different regions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalAdaptations {
    pub greeting_style: GreetingStyle,
    pub date_preference: DatePreference,
    pub name_format: NameFormat,
    pub address_format: AddressFormat,
    pub phone_number_format: PhoneFormat,
}

/// Greeting styles
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GreetingStyle {
    Formal,
    Informal,
    Mixed,
    Cultural(String),
}

/// Date preferences
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DatePreference {
    DMY,  // Day/Month/Year
    MDY,  // Month/Day/Year
    YMD,  // Year/Month/Date,
    Cultural(String),
}

/// Name formatting preferences
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NameFormat {
    GivenFirst,    // John Smith
    FamilyFirst,   // Smith John
    FamilyFirstComma, // Smith, John
}

/// Address formatting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressFormat {
    pub order: Vec<AddressComponent>,
    pub format_string: String,
}

/// Address components
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AddressComponent {
    Name,
    Organization,
    Street,
    City,
    State,
    PostalCode,
    Country,
    Custom(String),
}

/// Phone number formatting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoneFormat {
    pub country_code: String,
    pub format_template: String,
    pub country_codes: HashMap<String, String>,
}

/// Measurement systems
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MeasurementSystem {
    Metric,
    Imperial,
    US,
}

/// Paper sizes
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PaperSize {
    A4,
    Letter,
    Legal,
    A3,
    Custom { width: f64, height: f64 },
}

/// Holiday calendar information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolidayCalendar {
    pub holidays: Vec<Holiday>,
    pub work_holidays: Vec<Holiday>,
    pub custom_holidays: Vec<Holiday>,
}

/// Holiday definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Holiday {
    pub name: String,
    pub date: HolidayDate,
    pub type_: HolidayType,
    pub recurring: bool,
}

/// Holiday date specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HolidayDate {
    Fixed { month: u8, day: u8 },
    Relative { base: String, offset: i32 },
    Computed { calculation: String },
}

/// Holiday types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HolidayType {
    National,
    Religious,
    Cultural,
    Regional,
    Custom,
}

/// Language manager for handling translations and locale
pub struct LanguageManager {
    current_language: Arc<RwLock<Language>>,
    available_languages: Arc<RwLock<Vec<Language>>>,
    translation_bundles: Arc<RwLock<HashMap<String, TranslationBundle>>>,
    locale_formats: Arc<RwLock<HashMap<String, LocaleFormat>>>,
    regional_preferences: Arc<RwLock<HashMap<String, RegionalPreferences>>>,
    fallback_language: Arc<RwLock<Language>>,
    translation_cache: Arc<RwLock<TranslationCache>>,
    events: Arc<RwLock<Vec<Box<dyn LanguageEventHandler + Send + Sync>>>>,
}

/// Translation cache for performance optimization
#[derive(Debug, Clone)]
pub struct TranslationCache {
    cache: HashMap<String, String>,
    hit_count: u64,
    miss_count: u64,
    max_size: usize,
}

/// Language event handler trait
pub trait LanguageEventHandler: Send + Sync {
    fn on_language_changed(&self, old_language: &Language, new_language: &Language);
    fn on_translation_updated(&self, key: &str, language: &str);
    fn on_format_changed(&self, locale: &str);
}

/// Number formatting styles
#[derive(Debug, Clone)]
pub enum NumberStyle {
    Decimal(u8),
    Integer,
    Percent,
    Currency(String),
}

/// Date and time formatting helper
pub struct DateTimeFormatter {
    locale_format: LocaleFormat,
    current_timezone: String,
}

impl DateTimeFormatter {
    pub fn new(locale_code: String) -> Self {
        let locale_format = Self::get_locale_format(locale_code);
        
        Self {
            locale_format,
            current_timezone: chrono::Local::now().offset().to_string(),
        }
    }

    fn get_locale_format(_locale_code: String) -> LocaleFormat {
        // This would load the actual locale format from the language manager
        LocaleFormat {
            date_format: DateFormat {
                short: "MM/dd/yyyy".to_string(),
                medium: "MMM dd, yyyy".to_string(),
                long: "MMMM dd, yyyy".to_string(),
                full: "EEEE, MMMM dd, yyyy".to_string(),
                custom_formats: HashMap::new(),
            },
            time_format: TimeFormat {
                short: "HH:mm".to_string(),
                medium: "HH:mm:ss".to_string(),
                long: "HH:mm:ss z".to_string(),
                full: "HH:mm:ss zzzz".to_string(),
                custom_formats: HashMap::new(),
            },
            number_format: NumberFormat {
                decimal_separator: '.',
                thousands_separator: ',',
                decimal_places: 2,
                grouping: GroupingStyle::Western,
                negative_format: NegativeFormat::Minus,
            },
            currency_format: CurrencyFormat {
                symbol: "$".to_string(),
                symbol_position: SymbolPosition::Before,
                decimal_places: 2,
                space_before: false,
                custom_codes: HashMap::new(),
            },
            calendar_type: CalendarType::Gregorian,
        }
    }

    pub fn format_date(&self, date: &DateTime<Utc>, style: &str) -> String {
        let local_date = date.with_timezone(&chrono::Local);
        
        match style {
            "short" => local_date.format(&self.locale_format.date_format.short).to_string(),
            "medium" => local_date.format(&self.locale_format.date_format.medium).to_string(),
            "long" => local_date.format(&self.locale_format.date_format.long).to_string(),
            "full" => local_date.format(&self.locale_format.date_format.full).to_string(),
            custom => self.locale_format.date_format.custom_formats
                .get(custom)
                .map(|fmt| local_date.format(fmt).to_string())
                .unwrap_or_else(|| local_date.to_rfc3339()),
        }
    }

    pub fn format_time(&self, time: &DateTime<Utc>, style: &str) -> String {
        let local_time = time.with_timezone(&chrono::Local);
        
        match style {
            "short" => local_time.format(&self.locale_format.time_format.short).to_string(),
            "medium" => local_time.format(&self.locale_format.time_format.medium).to_string(),
            "long" => local_time.format(&self.locale_format.time_format.long).to_string(),
            "full" => local_time.format(&self.locale_format.time_format.full).to_string(),
            custom => self.locale_format.time_format.custom_formats
                .get(custom)
                .map(|fmt| local_time.format(fmt).to_string())
                .unwrap_or_else(|| local_time.to_rfc3339()),
        }
    }

    pub fn format_number(&self, number: f64, style: NumberStyle) -> String {
        match style {
            NumberStyle::Decimal(places) => {
                let formatted = format!("{:.prec$}", number, prec = places as usize);
                self.apply_number_formatting(&formatted)
            }
            NumberStyle::Integer => {
                let formatted = format!("{:.0}", number);
                self.apply_number_formatting(&formatted)
            }
            NumberStyle::Percent => {
                let formatted = format!("{:.1}%", number * 100.0);
                self.apply_number_formatting(&formatted)
            }
            NumberStyle::Currency(currency_code) => {
                let formatted = format!("{:.2}", number);
                let formatted = self.apply_number_formatting(&formatted);
                self.apply_currency_formatting(&formatted, &currency_code)
            }
        }
    }

    fn apply_number_formatting(&self, number: &str) -> String {
        let format = &self.locale_format.number_format;
        
        // This is a simplified implementation
        // Real implementation would handle complex number formatting
        number.to_string()
    }

    fn apply_currency_formatting(&self, amount: &str, currency_code: &str) -> String {
        let format = &self.locale_format.currency_format;
        
        match format.symbol_position {
            SymbolPosition::Before => format!("{}{}", format.symbol, amount),
            SymbolPosition::After => format!("{}{}", amount, format.symbol),
            SymbolPosition::SpaceBefore => format!("{} {}", format.symbol, amount),
            SymbolPosition::SpaceAfter => format!("{} {}", amount, format.symbol),
        }
    }
}

impl LanguageManager {
    /// Create new language manager
    pub fn new() -> Self {
        let default_language = Language {
            code: "en".to_string(),
            region: Some("US".to_string()),
            name: "English".to_string(),
            english_name: "English".to_string(),
            direction: TextDirection::LeftToRight,
            plural_rules: PluralRules {
                rules: vec![PluralRule {
                    category: PluralCategory::Other,
                    condition: PluralCondition {
                        operator: PluralOperator::Equals,
                        operands: vec![PluralOperand::Variable("n".to_string())],
                    },
                    examples: vec!["1".to_string()],
                }],
                default_rule: PluralRule {
                    category: PluralCategory::Other,
                    condition: PluralCondition {
                        operator: PluralOperator::Equals,
                        operands: vec![PluralOperand::Variable("n".to_string())],
                    },
                    examples: vec!["1".to_string()],
                },
            },
        };

        Self {
            current_language: Arc::new(RwLock::new(default_language.clone())),
            available_languages: Arc::new(RwLock::new(vec![default_language])),
            translation_bundles: Arc::new(RwLock::new(HashMap::new())),
            locale_formats: Arc::new(RwLock::new(HashMap::new())),
            regional_preferences: Arc::new(RwLock::new(HashMap::new())),
            fallback_language: Arc::new(RwLock::new(default_language)),
            translation_cache: Arc::new(RwLock::new(TranslationCache {
                cache: HashMap::new(),
                hit_count: 0,
                miss_count: 0,
                max_size: 1000,
            })),
            events: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Set current language
    pub fn set_language(&self, code: &str, region: Option<&str>) {
        let new_language = self.find_language(code, region);
        if let Some(language) = new_language {
            let old_language = self.get_current_language();
            
            {
                let mut current = self.current_language.write().unwrap();
                *current = language.clone();
            }

            // Load translation bundle for the language
            self.load_translation_bundle(&language);

            // Notify event handlers
            let handlers = self.events.read().unwrap();
            for handler in &*handlers {
                handler.on_language_changed(&old_language, &language);
            }
        }
    }

    /// Get current language
    pub fn get_current_language(&self) -> Language {
        self.current_language.read().unwrap().clone()
    }

    /// Get available languages
    pub fn get_available_languages(&self) -> Vec<Language> {
        self.available_languages.read().unwrap().clone()
    }

    /// Translate a key to the current language
    pub fn t(&self, key: &str, args: Option<HashMap<String, serde_json::Value>>) -> String {
        let current_language = self.get_current_language();
        let language_key = self.get_language_key(&current_language);
        
        // Check cache first
        {
            let cache = self.translation_cache.read().unwrap();
            if let Some(cached) = cache.cache.get(key) {
                return cached.clone();
            }
        }

        // Get translation
        let translation = self.get_translation(key, &language_key, args);
        
        // Cache the result
        {
            let mut cache = self.translation_cache.write().unwrap();
            if cache.cache.len() < cache.max_size {
                cache.cache.insert(key.to_string(), translation.clone());
            }
        }

        translation
    }

    /// Get translation with fallback
    fn get_translation(&self, key: &str, language_key: &str, args: Option<HashMap<String, serde_json::Value>>) -> String {
        let bundles = self.translation_bundles.read().unwrap();
        
        // Try current language
        if let Some(bundle) = bundles.get(language_key) {
            if let Some(entry) = bundle.translations.get(key) {
                return self.process_translation(entry, args);
            }
        }

        // Try fallback language
        let fallback_language = self.fallback_language.read().unwrap();
        let fallback_key = self.get_language_key(&fallback_language);
        
        if let Some(bundle) = bundles.get(&fallback_key) {
            if let Some(entry) = bundle.translations.get(key) {
                return self.process_translation(entry, args);
            }
        }

        // Return key as fallback
        key.to_string()
    }

    /// Process translation with placeholder substitution
    fn process_translation(&self, entry: &TranslationEntry, args: Option<HashMap<String, serde_json::Value>>) -> String {
        let mut result = entry.value.clone();
        
        if let Some(ref args) = args {
            for placeholder in &entry.placeholders {
                if let Some(value) = args.get(&placeholder.name) {
                    let formatted_value = self.format_placeholder_value(value, &placeholder);
                    result = result.replace(&format!("{{{}}}", placeholder.name), &formatted_value);
                }
            }
        }

        result
    }

    /// Format placeholder value based on type
    fn format_placeholder_value(&self, value: &serde_json::Value, placeholder: &Placeholder) -> String {
        match placeholder.placeholder_type {
            PlaceholderType::Simple => value.to_string(),
            PlaceholderType::Number => {
                if let Some(num) = value.as_f64() {
                    let formatter = DateTimeFormatter::new("en-US".to_string());
                    formatter.format_number(num, NumberStyle::Integer)
                } else {
                    value.to_string()
                }
            }
            PlaceholderType::Date => {
                if let Some(timestamp) = value.as_u64() {
                    let date = Utc.timestamp_opt(timestamp as i64, 0).single();
                    if let Some(date) = date {
                        let formatter = DateTimeFormatter::new("en-US".to_string());
                        formatter.format_date(&date, "medium")
                    } else {
                        value.to_string()
                    }
                } else {
                    value.to_string()
                }
            }
            PlaceholderType::Currency => {
                if let Some(amount) = value.as_f64() {
                    let formatter = DateTimeFormatter::new("en-US".to_string());
                    formatter.format_number(amount, NumberStyle::Currency("USD".to_string()))
                } else {
                    value.to_string()
                }
            }
            _ => value.to_string(),
        }
    }

    /// Get translations for a specific language
    pub fn get_translations_for_language(&self, language: &Language) -> HashMap<String, TranslationEntry> {
        let language_key = self.get_language_key(language);
        let bundles = self.translation_bundles.read().unwrap();
        
        if let Some(bundle) = bundles.get(&language_key) {
            bundle.translations.clone()
        } else {
            HashMap::new()
        }
    }

    /// Get locale format for language
    pub fn get_locale_format(&self, locale_code: &str) -> Option<LocaleFormat> {
        let formats = self.locale_formats.read().unwrap();
        formats.get(locale_code).cloned()
    }

    /// Load translation bundle for language
    fn load_translation_bundle(&self, language: &Language) {
        let language_key = self.get_language_key(language);
        
        // This would load from actual translation files
        // For now, create a basic bundle
        let bundle = TranslationBundle {
            language: language.clone(),
            translations: HashMap::new(),
            metadata: TranslationMetadata {
                created_at: Utc::now(),
                updated_at: Utc::now(),
                translator: "System".to_string(),
                reviewer: None,
                quality_score: 100.0,
                completion_percentage: 100.0,
                notes: vec![],
            },
            version: "1.0".to_string(),
        };
        
        let mut bundles = self.translation_bundles.write().unwrap();
        bundles.insert(language_key, bundle);
    }

    /// Find language by code and region
    fn find_language(&self, code: &str, region: Option<&str>) -> Option<Language> {
        let languages = self.available_languages.read().unwrap();
        
        for language in languages.iter() {
            if language.code == code {
                if region.is_none() || language.region == region.map(|r| r.to_string()) {
                    return Some(language.clone());
                }
            }
        }
        
        None
    }

    /// Get language key for lookup
    fn get_language_key(&self, language: &Language) -> String {
        if let Some(ref region) = language.region {
            format!("{}-{}", language.code, region)
        } else {
            language.code.clone()
        }
    }

    /// Register event handler
    pub fn register_event_handler(&self, handler: Box<dyn LanguageEventHandler + Send + Sync>) {
        let mut handlers = self.events.write().unwrap();
        handlers.push(handler);
    }

    /// Add supported language
    pub fn add_language(&self, language: Language) {
        let mut languages = self.available_languages.write().unwrap();
        if !languages.iter().any(|l| l.code == language.code && l.region == language.region) {
            languages.push(language);
        }
    }
}

/// Export formats for translations
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExportFormat {
    JSON,
    PO,
    XLIFF,
    Properties,
}

/// Import formats for translations
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImportFormat {
    JSON,
    PO,
    XLIFF,
    Properties,
}

/// Global language manager singleton
use once_cell::sync::Lazy;
pub static LANGUAGE_MANAGER: Lazy<LanguageManager> = Lazy::new(LanguageManager::new);

/// Translation helper macro
#[macro_export]
macro_rules! t {
    ($key:expr) => {
        $crate::i18n::LANGUAGE_MANAGER.t($key, None)
    };
    ($key:expr, $($arg_name:expr => $arg_value:expr),*) => {{
        let mut args = std::collections::HashMap::new();
        $(
            args.insert($arg_name.to_string(), serde_json::Value::from($arg_value));
        )*
        $crate::i18n::LANGUAGE_MANAGER.t($key, Some(args))
    }};
}

/// Plural formatting helper
pub fn pluralize(key: &str, count: usize, locale: &str) -> String {
    let language_manager = &LANGUAGE_MANAGER;
    let current_language = language_manager.get_current_language();
    
    // Apply plural rules
    let category = get_plural_category(&current_language.plural_rules, count);
    
    // Try to get plural variant
    let plural_key = format!("{}_{}", key, category.as_str());
    language_manager.t(&plural_key, Some(serde_json::json!({ "count": count })))
}

fn get_plural_category(rules: &PluralRules, count: usize) -> PluralCategory {
    for rule in &rules.rules {
        if evaluate_plural_condition(&rule.condition, count) {
            return rule.category.clone();
        }
    }
    rules.default_rule.category.clone()
}

fn evaluate_plural_condition(condition: &PluralCondition, count: usize) -> bool {
    match condition.operator {
        PluralOperator::Equals => {
            if let PluralOperand::Number(number) = &condition.operands[0] {
                count as f64 == *number
            } else {
                false
            }
        }
        _ => false, // Simplified implementation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_creation() {
        let language = Language {
            code: "en".to_string(),
            region: Some("US".to_string()),
            name: "English".to_string(),
            english_name: "English".to_string(),
            direction: TextDirection::LeftToRight,
            plural_rules: PluralRules {
                rules: vec![],
                default_rule: PluralRule {
                    category: PluralCategory::Other,
                    condition: PluralCondition {
                        operator: PluralOperator::Equals,
                        operands: vec![PluralOperand::Variable("n".to_string())],
                    },
                    examples: vec![],
                },
            },
        };
        
        assert_eq!(language.code, "en");
        assert_eq!(language.direction, TextDirection::LeftToRight);
    }

    #[test]
    fn test_translation_placeholder() {
        let entry = TranslationEntry {
            key: "greeting".to_string(),
            value: "Hello, {name}!".to_string(),
            context: None,
            description: Some("Greeting message".to_string()),
            placeholders: vec![Placeholder {
                name: "name".to_string(),
                placeholder_type: PlaceholderType::Simple,
                format_specifier: None,
                description: None,
                examples: vec![],
            }],
            examples: vec![],
            fuzzy: false,
            obsolete: false,
            custom_translations: HashMap::new(),
        };
        
        assert!(entry.placeholders[0].name == "name");
        assert!(entry.value.contains("{name}"));
    }

    #[test]
    fn test_language_manager_translation() {
        let manager = LanguageManager::new();
        
        // Test translation (will return key as fallback)
        let result = manager.t("test_key", None);
        assert_eq!(result, "test_key");
    }

    #[test]
    fn test_date_time_formatter() {
        let formatter = DateTimeFormatter::new("en-US".to_string());
        let date = Utc::now();
        
        let formatted = formatter.format_date(&date, "medium");
        assert!(!formatted.is_empty());
    }

    #[test]
    fn test_number_formatter() {
        let formatter = DateTimeFormatter::new("en-US".to_string());
        
        let formatted = formatter.format_number(1234.56, NumberStyle::Integer);
        assert!(formatted.contains("1234"));
        
        let currency = formatter.format_number(1234.56, NumberStyle::Currency("USD".to_string()));
        assert!(currency.contains('$'));
    }
}