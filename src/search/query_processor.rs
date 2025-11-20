//! Search Query Processing and Parsing
//! 
//! This module provides comprehensive search query processing including parsing,
//! operators, fuzzy matching, and advanced search functionality.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, BTreeMap};
use std::str::Chars;
use std::iter::Peekable;
use std::time::SystemTime;
use crate::search::{SearchOperator, SearchSort};

/// Search query parser and processor
#[derive(Debug)]
pub struct QueryProcessor {
    config: QueryProcessorConfig,
    operatorDefinitions: HashMap<String, SearchOperator>,
    synonyms: HashMap<String, HashSet<String>>,
    stopWords: HashSet<String>,
}

/// Configuration for query processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryProcessorConfig {
    pub enable_fuzzy_matching: bool,
    pub fuzzy_threshold: f32,
    pub enable_synonyms: bool,
    pub enable_stemming: bool,
    pub enable_phrase_search: bool,
    pub max_query_length: usize,
    pub max_terms_per_query: usize,
    pub default_operator: SearchOperator,
    pub enable_wildcards: bool,
    pub enable_regex: bool,
    pub case_sensitive: bool,
}

/// Parsed search query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedQuery {
    pub original_query: String,
    pub terms: Vec<SearchTerm>,
    pub operators: Vec<QueryOperator>,
    pub phrases: Vec<SearchPhrase>,
    pub filters: Vec<SearchFilter>,
    pub sort_specification: Option<SortSpecification>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

/// Individual search term
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchTerm {
    pub text: String,
    pub term_type: TermType,
    pub is_negated: bool,
    pub boost: f32,
    pub field_restriction: Option<String>,
}

/// Search term types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TermType {
    Word,
    Number,
    Date,
    Wildcard,
    Regex,
    Fuzzy(String),
}

/// Query operators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryOperator {
    pub operator: SearchOperator,
    pub position: usize,
    pub precedence: u8,
    pub is_unary: bool,
}

/// Search phrases (quoted strings)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchPhrase {
    pub text: String,
    pub is_negated: bool,
    pub boost: f32,
    pub field_restriction: Option<String>,
}

/// Search filters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilter {
    pub field: String,
    pub operator: FilterOperator,
    pub value: FilterValue,
    pub is_negated: bool,
}

/// Filter operators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Contains,
    StartsWith,
    EndsWith,
    InList,
    NotInList,
    Regex,
}

/// Filter values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Date(SystemTime),
    List(Vec<String>),
}

/// Sort specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortSpecification {
    pub fields: Vec<SortField>,
    pub direction: SortDirection,
}

/// Sort field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortField {
    pub field: String,
    pub direction: SortDirection,
}

/// Sort direction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortDirection {
    Ascending,
    Descending,
}

/// Query processing result
#[derive(Debug, Clone)]
pub struct ProcessedQuery {
    pub parsed_query: ParsedQuery,
    pub tokenized_terms: Vec<String>,
    pub expanded_terms: Vec<String>,
    pub execution_plan: QueryExecutionPlan,
    pub metadata: QueryMetadata,
}

/// Query execution plan
#[derive(Debug, Clone)]
pub struct QueryExecutionPlan {
    pub primary_terms: Vec<String>,
    pub secondary_terms: Vec<String>,
    pub required_terms: Vec<String>,
    pub prohibited_terms: Vec<String>,
    pub filter_clauses: Vec<SearchFilter>,
    pub sort_requirements: Vec<SortField>,
    pub estimated_complexity: QueryComplexity,
}

/// Query complexity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryComplexity {
    Simple,
    Moderate,
    Complex,
    VeryComplex,
}

/// Query processing metadata
#[derive(Debug, Clone)]
pub struct QueryMetadata {
    pub processing_time_ms: u64,
    pub terms_expanded: usize,
    pub synonyms_used: usize,
    pub fuzzy_matches: usize,
    pub parse_errors: Vec<ParseError>,
    pub warnings: Vec<String>,
}

/// Parse error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseError {
    pub error_type: ParseErrorType,
    pub message: String,
    pub position: usize,
    pub suggestion: Option<String>,
}

/// Parse error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParseErrorType {
    SyntaxError,
    UnknownOperator,
    UnclosedQuote,
    InvalidFilter,
    TooManyTerms,
    InvalidRegex,
    UnsupportedFeature,
}

/// Query suggestion for auto-complete
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuerySuggestion {
    pub suggestion: String,
    pub score: f32,
    pub suggestion_type: SuggestionType,
    pub description: Option<String>,
}

/// Suggestion types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuggestionType {
    Term,
    Phrase,
    Operator,
    Filter,
    Field,
}

/// Query processor error types
#[derive(Debug, thiserror::Error)]
pub enum QueryProcessorError {
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Query too complex: {0}")]
    QueryTooComplex(String),
    
    #[error("Invalid operator: {0}")]
    InvalidOperator(String),
    
    #[error("Query timeout")]
    Timeout,
    
    #[error("Memory limit exceeded")]
    MemoryLimitExceeded,
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
}

/// Result type for query processing operations
pub type QueryResult<T> = Result<T, QueryProcessorError>;

impl Default for QueryProcessorConfig {
    fn default() -> Self {
        Self {
            enable_fuzzy_matching: true,
            fuzzy_threshold: 0.8,
            enable_synonyms: true,
            enable_stemming: true,
            enable_phrase_search: true,
            max_query_length: 1000,
            max_terms_per_query: 100,
            default_operator: SearchOperator::And,
            enable_wildcards: true,
            enable_regex: false,
            case_sensitive: false,
        }
    }
}

impl QueryProcessor {
    /// Create new query processor
    pub fn new(config: QueryProcessorConfig) -> Self {
        let mut operatorDefinitions = HashMap::new();
        operatorDefinitions.insert("AND".to_string(), SearchOperator::And);
        operatorDefinitions.insert("OR".to_string(), SearchOperator::Or);
        operatorDefinitions.insert("NOT".to_string(), SearchOperator::Not);
        
        let mut synonyms = HashMap::new();
        synonyms.insert("character".to_string(), hashset!["person", "figure", "protagonist"]);
        synonyms.insert("plot".to_string(), hashset!["story", "narrative", "arc"]);
        synonyms.insert("setting".to_string(), hashset!["location", "environment", "scene"]);
        
        let mut stopWords = HashSet::new();
        stopWords.insert("the".to_string());
        stopWords.insert("a".to_string());
        stopWords.insert("an".to_string());
        stopWords.insert("and".to_string());
        stopWords.insert("or".to_string());
        stopWords.insert("but".to_string());
        stopWords.insert("in".to_string());
        stopWords.insert("on".to_string());
        stopWords.insert("at".to_string());
        stopWords.insert("to".to_string());
        stopWords.insert("for".to_string());
        stopWords.insert("of".to_string());
        stopWords.insert("with".to_string());
        stopWords.insert("by".to_string());
        
        Self {
            config,
            operatorDefinitions,
            synonyms,
            stopWords,
        }
    }
    
    /// Parse and process a search query
    pub fn parse_query(&self, query: &str) -> QueryResult<ParsedQuery> {
        let start_time = SystemTime::now();
        
        // Validate query length
        if query.len() > self.config.max_query_length {
            return Err(QueryProcessorError::QueryTooComplex(
                format!("Query exceeds maximum length of {} characters", self.config.max_query_length)
            ));
        }
        
        // Tokenize and parse
        let tokens = self.tokenize(query)?;
        let mut parser = QueryParser::new(tokens, self.config.clone());
        let parsed_query = parser.parse()?;
        
        // Process and validate
        let processed_query = self.process_parsed_query(parsed_query)?;
        
        // Generate metadata
        let processing_time = start_time.elapsed().unwrap_or_default().as_millis() as u64;
        let metadata = QueryMetadata {
            processing_time_ms: processing_time,
            terms_expanded: 0, // Would be calculated during processing
            synonyms_used: 0,
            fuzzy_matches: 0,
            parse_errors: Vec::new(),
            warnings: Vec::new(),
        };
        
        Ok(processed_query)
    }
    
    /// Generate suggestions for auto-complete
    pub fn generate_suggestions(&self, partial_query: &str, limit: usize) -> QueryResult<Vec<QuerySuggestion>> {
        let mut suggestions = Vec::new();
        let partial_lower = partial_query.to_lowercase();
        
        // Generate term suggestions
        for (term, synonym_set) in &self.synonyms {
            if term.starts_with(&partial_lower) {
                suggestions.push(QuerySuggestion {
                    suggestion: term.clone(),
                    score: 1.0,
                    suggestion_type: SuggestionType::Term,
                    description: Some(format!("Synonyms: {}", synonym_set.iter().take(3).collect::<Vec<_>>().join(", "))),
                });
            }
        }
        
        // Generate operator suggestions
        for (op_name, _) in &self.operatorDefinitions {
            if op_name.starts_with(&partial_lower) {
                suggestions.push(QuerySuggestion {
                    suggestion: op_name.clone(),
                    score: 0.8,
                    suggestion_type: SuggestionType::Operator,
                    description: Some(format!("Logical operator {}", op_name)),
                });
            }
        }
        
        // Sort by score and limit
        suggestions.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        suggestions.truncate(limit);
        
        Ok(suggestions)
    }
    
    /// Expand query with synonyms and related terms
    pub fn expand_query(&self, query: &ParsedQuery) -> QueryResult<Vec<String>> {
        let mut expanded_terms = Vec::new();
        
        for term in &query.terms {
            if !term.is_negated && !self.stopWords.contains(&term.text.to_lowercase()) {
                expanded_terms.push(term.text.clone());
                
                // Add synonyms if enabled
                if self.config.enable_synonyms {
                    if let Some(synonym_set) = self.synonyms.get(&term.text.to_lowercase()) {
                        expanded_terms.extend(synonym_set.iter().cloned());
                    }
                }
            }
        }
        
        // Add phrase terms
        for phrase in &query.phrases {
            if !phrase.is_negated {
                expanded_terms.push(phrase.text.clone());
            }
        }
        
        // Remove duplicates and return
        let unique_terms: Vec<String> = expanded_terms.into_iter().collect::<HashSet<_>>().into_iter().collect();
        Ok(unique_terms)
    }
    
    /// Apply fuzzy matching to terms
    pub fn apply_fuzzy_matching(&self, terms: &[String]) -> QueryResult<HashMap<String, Vec<String>>> {
        if !self.config.enable_fuzzy_matching {
            return Ok(HashMap::new());
        }
        
        let mut fuzzy_matches = HashMap::new();
        
        for term in terms {
            let mut matches = Vec::new();
            
            // Simple fuzzy matching based on string similarity
            for synonym_set in self.synonyms.values() {
                for synonym in synonym_set {
                    if self.calculate_similarity(term, synonym) >= self.config.fuzzy_threshold {
                        matches.push(synonym.clone());
                    }
                }
            }
            
            if !matches.is_empty() {
                fuzzy_matches.insert(term.clone(), matches);
            }
        }
        
        Ok(fuzzy_matches)
    }
    
    /// Create execution plan for query
    pub fn create_execution_plan(&self, query: &ParsedQuery) -> QueryResult<QueryExecutionPlan> {
        let mut primary_terms = Vec::new();
        let mut secondary_terms = Vec::new();
        let mut required_terms = Vec::new();
        let mut prohibited_terms = Vec::new();
        
        // Classify terms based on their properties
        for term in &query.terms {
            if term.is_negated {
                prohibited_terms.push(term.text.clone());
            } else {
                match term.boost {
                    boost if boost > 1.0 => primary_terms.push(term.text.clone()),
                    boost if boost < 1.0 => secondary_terms.push(term.text.clone()),
                    _ => required_terms.push(term.text.clone()),
                }
            }
        }
        
        // Extract filter clauses
        let filter_clauses = query.filters.clone();
        
        // Extract sort requirements
        let sort_requirements = match &query.sort_specification {
            Some(spec) => spec.fields.clone(),
            None => Vec::new(),
        };
        
        // Estimate query complexity
        let complexity = self.estimate_complexity(query);
        
        Ok(QueryExecutionPlan {
            primary_terms,
            secondary_terms,
            required_terms,
            prohibited_terms,
            filter_clauses,
            sort_requirements,
            estimated_complexity: complexity,
        })
    }
    
    /// Validate query against search index capabilities
    pub fn validate_query(&self, query: &ParsedQuery) -> QueryResult<Vec<String>> {
        let mut warnings = Vec::new();
        
        // Check for too many terms
        if query.terms.len() > self.config.max_terms_per_query {
            warnings.push(format!(
                "Query has {} terms, exceeding recommended maximum of {}",
                query.terms.len(),
                self.config.max_terms_per_query
            ));
        }
        
        // Check for unsupported operators
        for term in &query.terms {
            match &term.term_type {
                TermType::Regex if !self.config.enable_regex => {
                    warnings.push("Regular expression search is disabled".to_string());
                }
                TermType::Wildcard if !self.config.enable_wildcards => {
                    warnings.push("Wildcard search is disabled".to_string());
                }
                _ => {}
            }
        }
        
        // Check for complex filter expressions
        if query.filters.len() > 10 {
            warnings.push("Query has many filter expressions, may impact performance".to_string());
        }
        
        Ok(warnings)
    }
    
    // Private helper methods
    
    fn tokenize(&self, query: &str) -> QueryResult<Vec<QueryToken>> {
        let mut tokens = Vec::new();
        let mut chars = query.chars().peekable();
        let mut position = 0;
        
        while let Some(&ch) = chars.peek() {
            match ch {
                ' ' => {
                    chars.next(); // Skip whitespace
                    position += 1;
                }
                '"' => {
                    // Parse quoted phrase
                    let (phrase, new_position) = self.parse_quoted_phrase(&mut chars, position)?;
                    tokens.push(QueryToken::Phrase(phrase));
                    position = new_position;
                }
                '(' => {
                    tokens.push(QueryToken::LeftParen);
                    chars.next();
                    position += 1;
                }
                ')' => {
                    tokens.push(QueryToken::RightParen);
                    chars.next();
                    position += 1;
                }
                _ if ch.is_ascii_alphanumeric() || ch == '_' => {
                    // Parse word/number
                    let (word, new_position) = self.parse_word(&mut chars, position)?;
                    tokens.push(QueryToken::Word(word));
                    position = new_position;
                }
                _ => {
                    // Parse operator or special character
                    let (operator, new_position) = self.parse_operator(&mut chars, position)?;
                    if let Some(op_token) = operator {
                        tokens.push(op_token);
                    }
                    position = new_position;
                }
            }
        }
        
        Ok(tokens)
    }
    
    fn parse_quoted_phrase(&self, chars: &mut Peekable<Chars>, start_position: usize) -> QueryResult<(String, usize)> {
        chars.next(); // Skip opening quote
        
        let mut phrase = String::new();
        let mut position = start_position + 1;
        
        while let Some(&ch) = chars.peek() {
            if ch == '"' {
                chars.next(); // Skip closing quote
                return Ok((phrase, position + 1));
            } else if ch == '\\' {
                chars.next(); // Skip escape character
                if let Some(&next_ch) = chars.peek() {
                    phrase.push(next_ch);
                    chars.next();
                    position += 2;
                }
            } else {
                phrase.push(ch);
                chars.next();
                position += 1;
            }
        }
        
        Err(QueryProcessorError::ParseError("Unclosed quote".to_string()))
    }
    
    fn parse_word(&self, chars: &mut Peekable<Chars>, start_position: usize) -> QueryResult<(String, usize)> {
        let mut word = String::new();
        let mut position = start_position;
        
        while let Some(&ch) = chars.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                word.push(ch);
                chars.next();
                position += 1;
            } else {
                break;
            }
        }
        
        Ok((word, position))
    }
    
    fn parse_operator(&self, chars: &mut Peekable<Chars>, start_position: usize) -> QueryResult<(Option<QueryToken>, usize)> {
        let mut position = start_position;
        let mut operator_chars = String::new();
        
        // Collect operator characters
        while let Some(&ch) = chars.peek() {
            if !ch.is_ascii_alphanumeric() && ch != '_' && ch != '-' && ch != ':' {
                break;
            }
            operator_chars.push(ch);
            chars.next();
            position += 1;
        }
        
        // Check if it's a known operator
        let operator_upper = operator_chars.to_uppercase();
        if let Some(search_op) = self.operatorDefinitions.get(&operator_upper) {
            Ok((Some(QueryToken::Operator(search_op.clone())), position))
        } else if !operator_chars.is_empty() {
            // Treat as a field specification
            let parts: Vec<&str> = operator_chars.split(':').collect();
            if parts.len() == 2 {
                Ok((Some(QueryToken::Field(parts[0].to_string(), parts[1].to_string())), position))
            } else {
                Ok((None, position))
            }
        } else {
            Ok((None, position))
        }
    }
    
    fn process_parsed_query(&self, query: ParsedQuery) -> QueryResult<ParsedQuery> {
        // Apply stemming if enabled
        let mut processed_query = query;
        
        if self.config.enable_stemming {
            processed_query = self.apply_stemming(processed_query)?;
        }
        
        // Validate processed query
        self.validate_query_structure(&processed_query)?;
        
        Ok(processed_query)
    }
    
    fn apply_stemming(&self, query: ParsedQuery) -> QueryResult<ParsedQuery> {
        // Simple stemming implementation
        let mut processed_query = query;
        
        for term in &mut processed_query.terms {
            if !self.stopWords.contains(&term.text.to_lowercase()) {
                // Apply basic stemming rules
                term.text = self.stem_word(&term.text);
            }
        }
        
        Ok(processed_query)
    }
    
    fn stem_word(&self, word: &str) -> String {
        // Very basic stemming implementation
        let mut stemmed = word.to_lowercase();
        
        // Remove common suffixes
        let suffixes = ["ing", "ly", "ed", "ies", "ied", "ies", "ied", "s"];
        for suffix in &suffixes {
            if stemmed.ends_with(suffix) && stemmed.len() > suffix.len() + 2 {
                stemmed.truncate(stemmed.len() - suffix.len());
                break;
            }
        }
        
        stemmed
    }
    
    fn validate_query_structure(&self, query: &ParsedQuery) -> QueryResult<()> {
        // Check parentheses matching
        let mut paren_count = 0;
        for token in &query.original_query.chars() {
            match token {
                '(' => paren_count += 1,
                ')' => {
                    if paren_count == 0 {
                        return Err(QueryProcessorError::ParseError("Unmatched closing parenthesis".to_string()));
                    }
                    paren_count -= 1;
                }
                _ => {}
            }
        }
        
        if paren_count > 0 {
            return Err(QueryProcessorError::ParseError("Unmatched opening parenthesis".to_string()));
        }
        
        Ok(())
    }
    
    fn calculate_similarity(&self, s1: &str, s2: &str) -> f32 {
        // Simple string similarity calculation
        let s1_lower = s1.to_lowercase();
        let s2_lower = s2.to_lowercase();
        
        if s1_lower == s2_lower {
            return 1.0;
        }
        
        let max_len = s1_lower.len().max(s2_lower.len());
        if max_len == 0 {
            return 1.0;
        }
        
        // Calculate Levenshtein distance
        let distance = self.levenshtein_distance(&s1_lower, &s2_lower);
        1.0 - (distance as f32 / max_len as f32)
    }
    
    fn levenshtein_distance(&self, s1: &str, s2: &str) -> usize {
        let len1 = s1.len();
        let len2 = s2.len();
        
        if len1 == 0 {
            return len2;
        }
        if len2 == 0 {
            return len1;
        }
        
        let mut previous_row = (0..=len2).collect::<Vec<_>>();
        
        for (i, c1) in s1.chars().enumerate() {
            let mut current_row = vec![i + 1];
            
            for (j, c2) in s2.chars().enumerate() {
                let insertions = previous_row[j + 1] + 1;
                let deletions = current_row[j] + 1;
                let substitutions = previous_row[j] + if c1 == c2 { 0 } else { 1 };
                
                current_row.push(insertions.min(deletions).min(substitutions));
            }
            
            previous_row = current_row;
        }
        
        previous_row[len2]
    }
    
    fn estimate_complexity(&self, query: &ParsedQuery) -> QueryComplexity {
        let term_count = query.terms.len() + query.phrases.len();
        let filter_count = query.filters.len();
        let operator_count = query.operators.len();
        
        let complexity_score = term_count + (filter_count * 2) + (operator_count * 3);
        
        match complexity_score {
            score if score <= 10 => QueryComplexity::Simple,
            score if score <= 25 => QueryComplexity::Moderate,
            score if score <= 50 => QueryComplexity::Complex,
            _ => QueryComplexity::VeryComplex,
        }
    }
}

/// Query token types for parsing
#[derive(Debug, Clone)]
enum QueryToken {
    Word(String),
    Phrase(String),
    Operator(SearchOperator),
    Field(String, String),
    LeftParen,
    RightParen,
}

/// Query parser
struct QueryParser {
    tokens: Vec<QueryToken>,
    config: QueryProcessorConfig,
    position: usize,
}

impl QueryParser {
    fn new(tokens: Vec<QueryToken>, config: QueryProcessorConfig) -> Self {
        Self {
            tokens,
            config,
            position: 0,
        }
    }
    
    fn parse(&mut self) -> QueryResult<ParsedQuery> {
        let mut terms = Vec::new();
        let mut operators = Vec::new();
        let mut phrases = Vec::new();
        let mut filters = Vec::new();
        let mut sort_specification = None;
        let mut limit = None;
        let mut offset = None;
        
        while !self.is_at_end() {
            match self.current_token()? {
                QueryToken::Word(word) => {
                    terms.push(self.parse_term(&word)?);
                }
                QueryToken::Phrase(phrase) => {
                    phrases.push(self.parse_phrase(&phrase)?);
                }
                QueryToken::Operator(op) => {
                    operators.push(self.parse_operator(op)?);
                }
                QueryToken::Field(field, value) => {
                    filters.push(self.parse_filter(&field, &value)?);
                }
                _ => {
                    self.advance();
                }
            }
        }
        
        Ok(ParsedQuery {
            original_query: self.tokens_to_string(),
            terms,
            operators,
            phrases,
            filters,
            sort_specification,
            limit,
            offset,
        })
    }
    
    fn parse_term(&mut self, word: &str) -> QueryResult<SearchTerm> {
        let mut term_text = word.to_string();
        let mut is_negated = false;
        let mut boost = 1.0;
        let mut field_restriction = None;
        let mut term_type = TermType::Word;
        
        // Check for negation
        if term_text.starts_with('-') || term_text.starts_with('!') {
            is_negated = true;
            term_text = term_text[1..].to_string();
        }
        
        // Check for boost
        if term_text.ends_with('^') {
            if let Some((base, boost_str)) = term_text.rsplit_once('^') {
                if let Ok(boost_value) = boost_str.parse::<f32>() {
                    boost = boost_value;
                    term_text = base.to_string();
                }
            }
        }
        
        // Check for field restriction
        if term_text.contains(':') {
            let parts: Vec<&str> = term_text.split(':').collect();
            if parts.len() == 2 {
                field_restriction = Some(parts[0].to_string());
                term_text = parts[1].to_string();
            }
        }
        
        // Determine term type
        if term_text.contains('*') || term_text.contains('?') {
            term_type = TermType::Wildcard;
        } else if term_text.starts_with('/') && term_text.ends_with('/') {
            term_type = TermType::Regex;
        } else if term_text.contains('~') {
            let parts: Vec<&str> = term_text.split('~').collect();
            term_text = parts[0].to_string();
            if parts.len() > 1 {
                term_type = TermType::Fuzzy(parts[1].to_string());
            }
        }
        
        Ok(SearchTerm {
            text: term_text,
            term_type,
            is_negated,
            boost,
            field_restriction,
        })
    }
    
    fn parse_phrase(&mut self, phrase: &str) -> QueryResult<SearchPhrase> {
        Ok(SearchPhrase {
            text: phrase.to_string(),
            is_negated: false,
            boost: 1.0,
            field_restriction: None,
        })
    }
    
    fn parse_operator(&mut self, op: SearchOperator) -> QueryResult<QueryOperator> {
        Ok(QueryOperator {
            operator: op,
            position: self.position,
            precedence: self.get_operator_precedence(&op),
            is_unary: matches!(op, SearchOperator::Not),
        })
    }
    
    fn parse_filter(&mut self, field: &str, value: &str) -> QueryResult<SearchFilter> {
        Ok(SearchFilter {
            field: field.to_string(),
            operator: FilterOperator::Equals,
            value: FilterValue::String(value.to_string()),
            is_negated: false,
        })
    }
    
    fn get_operator_precedence(&self, op: &SearchOperator) -> u8 {
        match op {
            SearchOperator::Not => 4,
            SearchOperator::And => 3,
            SearchOperator::Or => 2,
            SearchOperator::Phrase(_) => 1,
            _ => 0,
        }
    }
    
    fn current_token(&self) -> QueryResult<&QueryToken> {
        self.tokens.get(self.position)
            .ok_or_else(|| QueryProcessorError::ParseError("Unexpected end of input".to_string()))
    }
    
    fn advance(&mut self) {
        self.position += 1;
    }
    
    fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len()
    }
    
    fn tokens_to_string(&self) -> String {
        self.tokens.iter()
            .map(|token| match token {
                QueryToken::Word(w) => w.clone(),
                QueryToken::Phrase(p) => format!("\"{}\"", p),
                QueryToken::Operator(op) => format!("{:?}", op),
                QueryToken::Field(f, v) => format!("{}:{}", f, v),
                QueryToken::LeftParen => "(".to_string(),
                QueryToken::RightParen => ")".to_string(),
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_processor_creation() {
        let config = QueryProcessorConfig::default();
        let processor = QueryProcessor::new(config);
        
        assert!(processor.config.enable_fuzzy_matching);
        assert_eq!(processor.config.max_terms_per_query, 100);
    }

    #[test]
    fn test_simple_query_parsing() {
        let config = QueryProcessorConfig::default();
        let processor = QueryProcessor::new(config);
        
        let query = "hello world";
        let parsed = processor.parse_query(query).unwrap();
        
        assert_eq!(parsed.terms.len(), 2);
        assert_eq!(parsed.terms[0].text, "hello");
        assert_eq!(parsed.terms[1].text, "world");
    }

    #[test]
    fn test_operator_parsing() {
        let config = QueryProcessorConfig::default();
        let processor = QueryProcessor::new(config);
        
        let query = "apple AND orange";
        let parsed = processor.parse_query(query).unwrap();
        
        assert_eq!(parsed.terms.len(), 2);
        assert_eq!(parsed.operators.len(), 1);
    }

    #[test]
    fn test_fuzzy_similarity() {
        let config = QueryProcessorConfig::default();
        let processor = QueryProcessor::new(config);
        
        let similarity = processor.calculate_similarity("character", "charactr");
        assert!(similarity > 0.8);
    }
}