//! Template System Module
//! 
//! Manages document templates for different types of content creation.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Template category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TemplateCategory {
    Academic,
    Business,
    Creative,
    Technical,
    Publishing,
    Screenwriting,
    General,
}

impl TemplateCategory {
    /// Get the display name for the category
    pub fn display_name(&self) -> &'static str {
        match self {
            TemplateCategory::Academic => "Academic",
            TemplateCategory::Business => "Business",
            TemplateCategory::Creative => "Creative",
            TemplateCategory::Technical => "Technical",
            TemplateCategory::Publishing => "Publishing",
            TemplateCategory::Screenwriting => "Screenwriting",
            TemplateCategory::General => "General",
        }
    }
}

/// Document template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: TemplateCategory,
    pub content: String,
    pub variables: Vec<String>,
    pub tags: Vec<String>,
    pub created_date: String,
    pub last_modified: String,
}

impl Template {
    /// Create a new template
    pub fn new(
        id: String,
        name: String,
        description: String,
        category: TemplateCategory,
        content: String,
    ) -> Self {
        let now = chrono::Utc::now().format("%Y-%m-%d").to_string();
        
        Self {
            id,
            name,
            description,
            category,
            content,
            variables: Vec::new(),
            tags: Vec::new(),
            created_date: now.clone(),
            last_modified: now,
        }
    }

    /// Add a variable to the template
    pub fn add_variable(&mut self, variable: String) {
        if !self.variables.contains(&variable) {
            self.variables.push(variable);
        }
    }

    /// Add a tag to the template
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    /// Process the template with variables
    pub fn process(&self, variables: &HashMap<String, String>) -> String {
        let mut result = self.content.clone();
        
        for (key, value) in variables {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, value);
        }
        
        result
    }
}

/// Template registry for managing all templates
pub struct TemplateRegistry {
    templates: HashMap<String, Template>,
}

impl Default for TemplateRegistry {
    fn default() -> Self {
        Self {
            templates: HashMap::new(),
        }
    }
}

impl TemplateRegistry {
    /// Create a new template registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a template to the registry
    pub fn add_template(&mut self, template: Template) {
        self.templates.insert(template.id.clone(), template);
    }

    /// Get a template by ID
    pub fn get_template(&self, template_id: &str) -> Option<&Template> {
        self.templates.get(template_id)
    }

    /// Get templates by category
    pub fn get_templates_by_category(&self, category: &TemplateCategory) -> Vec<&Template> {
        self.templates.values()
            .filter(|template| template.category == *category)
            .collect()
    }

    /// Search templates by name or tags
    pub fn search_templates(&self, query: &str) -> Vec<&Template> {
        let query_lower = query.to_lowercase();
        self.templates.values()
            .filter(|template| {
                template.name.to_lowercase().contains(&query_lower) ||
                template.description.to_lowercase().contains(&query_lower) ||
                template.tags.iter().any(|tag| tag.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    /// Get all template categories
    pub fn get_categories(&self) -> Vec<TemplateCategory> {
        use std::collections::BTreeSet;
        
        let mut categories = BTreeSet::new();
        for template in self.templates.values() {
            categories.insert(template.category);
        }
        
        categories.into_iter().collect()
    }

    /// Get all template IDs
    pub fn get_template_ids(&self) -> Vec<String> {
        self.templates.keys().cloned().collect()
    }

    /// Process a template with variables
    pub fn process_template(
        &self,
        template_id: &str,
        variables: &HashMap<String, String>
    ) -> Result<String, String> {
        if let Some(template) = self.get_template(template_id) {
            Ok(template.process(variables))
        } else {
            Err(format!("Template '{}' not found", template_id))
        }
    }

    /// Remove a template
    pub fn remove_template(&mut self, template_id: &str) -> bool {
        self.templates.remove(template_id).is_some()
    }

    /// Get template statistics
    pub fn get_statistics(&self) -> TemplateStatistics {
        let total_templates = self.templates.len();
        let mut category_counts = HashMap::new();
        
        for template in self.templates.values() {
            *category_counts.entry(template.category).or_insert(0) += 1;
        }
        
        let total_variables = self.templates.values()
            .map(|t| t.variables.len())
            .sum();
        let total_tags = self.templates.values()
            .map(|t| t.tags.len())
            .sum();

        TemplateStatistics {
            total_templates,
            category_counts,
            total_variables,
            total_tags,
        }
    }

    /// Create default templates for all categories
    pub fn create_default_templates(&mut self) {
        // Academic templates
        self.add_template(Template::new(
            "academic_research_paper".to_string(),
            "Research Paper".to_string(),
            "Template for academic research papers".to_string(),
            TemplateCategory::Academic,
            include_str!("../../templates/academic_research_paper.md").to_string(),
        ));
        
        self.add_template(Template::new(
            "academic_thesis".to_string(),
            "Thesis".to_string(),
            "Template for academic theses".to_string(),
            TemplateCategory::Academic,
            include_str!("../../templates/academic_thesis.md").to_string(),
        ));

        // Business templates
        self.add_template(Template::new(
            "business_proposal".to_string(),
            "Business Proposal".to_string(),
            "Template for business proposals".to_string(),
            TemplateCategory::Business,
            include_str!("../../templates/business_proposal.md").to_string(),
        ));
        
        self.add_template(Template::new(
            "business_report".to_string(),
            "Business Report".to_string(),
            "Template for business reports".to_string(),
            TemplateCategory::Business,
            include_str!("../../templates/business_report.md").to_string(),
        ));

        // Creative templates
        self.add_template(Template::new(
            "creative_story".to_string(),
            "Short Story".to_string(),
            "Template for creative writing".to_string(),
            TemplateCategory::Creative,
            include_str!("../../templates/creative_story.md").to_string(),
        ));

        // Technical templates
        self.add_template(Template::new(
            "technical_documentation".to_string(),
            "Technical Documentation".to_string(),
            "Template for technical documentation".to_string(),
            TemplateCategory::Technical,
            include_str!("../../templates/technical_documentation.md").to_string(),
        ));

        // Publishing templates
        self.add_template(Template::new(
            "publishing_manuscript_submission".to_string(),
            "Manuscript Submission".to_string(),
            "Template for publisher manuscript submissions".to_string(),
            TemplateCategory::Publishing,
            include_str!("../../templates/publishing_manuscript_submission.md").to_string(),
        ));

        // Screenwriting templates
        self.add_template(Template::new(
            "screenwriting_movie".to_string(),
            "Movie Script".to_string(),
            "Template for feature film screenplays".to_string(),
            TemplateCategory::Screenwriting,
            include_str!("../../templates/screenwriting_movie.md").to_string(),
        ));
        
        self.add_template(Template::new(
            "screenwriting_television".to_string(),
            "TV Script".to_string(),
            "Template for television scripts".to_string(),
            TemplateCategory::Screenwriting,
            include_str!("../../templates/screenwriting_television.md").to_string(),
        ));
    }
}

/// Template statistics
#[derive(Debug)]
pub struct TemplateStatistics {
    pub total_templates: usize,
    pub category_counts: HashMap<TemplateCategory, usize>,
    pub total_variables: usize,
    pub total_tags: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_creation() {
        let template = Template::new(
            "test".to_string(),
            "Test Template".to_string(),
            "A test template".to_string(),
            TemplateCategory::General,
            "Content with {{variable}}".to_string(),
        );
        
        assert_eq!(template.id, "test");
        assert_eq!(template.name, "Test Template");
        assert_eq!(template.category, TemplateCategory::General);
    }

    #[test]
    fn test_template_registry() {
        let mut registry = TemplateRegistry::new();
        
        let template = Template::new(
            "test".to_string(),
            "Test Template".to_string(),
            "A test template".to_string(),
            TemplateCategory::General,
            "Content".to_string(),
        );
        
        registry.add_template(template);
        
        assert!(registry.get_template("test").is_some());
        assert_eq!(registry.get_templates_by_category(&TemplateCategory::General).len(), 1);
    }

    #[test]
    fn test_template_processing() {
        let mut registry = TemplateRegistry::new();
        
        let mut template = Template::new(
            "test".to_string(),
            "Test Template".to_string(),
            "A test template".to_string(),
            TemplateCategory::General,
            "Hello {{name}}!".to_string(),
        );
        template.add_variable("name".to_string());
        
        registry.add_template(template);
        
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "World".to_string());
        
        let result = registry.process_template("test", &variables).unwrap();
        assert_eq!(result, "Hello World!");
    }
}