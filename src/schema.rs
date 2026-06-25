//! Schema module for claude-code-config-lsp.
//!
//! Provides `StaticSchema` — a compile-time tree of config file field definitions
//! derived from the domain ontology (`claude-code-config.ttl`).
//!
//! # Types
//!
//! - `SchemaNode` — a single field in the schema tree, with optional children for nested objects
//! - `StaticSchema` — root container holding the field array
//!
//! # Usage
//!
//! Generate schema modules via `schema_module.rs.tera` for each config surface:
//!
//! ```ignore
//! pub mod settings;  // generated: SettingsJson surface
//! pub mod mcp;       // generated: McpJson surface
//! // etc.
//!
//! // Use for hover, completion, validation:
//! if let Some(field) = settings::SCHEMA.find_field("effortLevel") {
//!     println!("{}:  {}", field.name, field.description);
//! }
//! ```

/// A single field in a config schema, potentially with nested children.
///
/// Encodes type information, cardinality, enum values, and documentation.
/// Compile-time static constants; no heap allocation.
#[derive(Debug, Clone, Copy)]
pub struct SchemaNode {
    /// Field name as it appears in the config file (e.g., "effortLevel").
    pub name: &'static str,

    /// Human-readable description from ontology, used in hover + diagnostics.
    pub description: &'static str,

    /// LSP field type: "string" | "boolean" | "number" | "integer" | "object" | "array" | "enum"
    pub field_type: &'static str,

    /// True if the field is required by the schema law.
    pub required: bool,

    /// Enum value list; empty slice if not an enum field.
    /// For enum fields, space-separated or newline-separated literals.
    /// Example: `&["low", "medium", "high", "max"]`
    pub enum_values: &'static [&'static str],

    /// Nested field definitions for object children.
    /// Empty slice if this is a leaf node (string, boolean, enum, etc.).
    pub children: &'static [SchemaNode],
}

impl SchemaNode {
    /// Find a descendant field by name (depth-first search).
    ///
    /// Returns the first field matching `target_name`, searching breadth-first:
    /// direct children before grandchildren.
    pub fn find_field(&self, target_name: &str) -> Option<&Self> {
        // Check direct children first
        for child in self.children {
            if child.name == target_name {
                return Some(child);
            }
        }
        // Then search recursively in each child's subtree
        for child in self.children {
            if let Some(found) = child.find_field(target_name) {
                return Some(found);
            }
        }
        None
    }

    /// Check if this field's type is compatible with a given LSP value type.
    ///
    /// Used in hover + completion logic to determine which completions to offer.
    pub fn matches_type(&self, value_type: &str) -> bool {
        match (self.field_type, value_type) {
            (ft, vt) if ft == vt => true,
            // "enum" matches any of the enum values
            ("enum", _) => self.enum_values.contains(&value_type),
            // object/array accept compound structures
            ("object", "object") | ("array", "array") => true,
            // Fallback for schema evolution
            _ => false,
        }
    }

    /// Render this field's type and constraints as a human-readable string.
    ///
    /// Format: `string` or `boolean` or `enum (low | medium | high)` etc.
    pub fn type_signature(&self) -> String {
        if !self.enum_values.is_empty() {
            format!(
                "enum ({})",
                self.enum_values
                    .iter()
                    .map(|s| format!("\"{}\"", s))
                    .collect::<Vec<_>>()
                    .join(" | ")
            )
        } else {
            self.field_type.to_string()
        }
    }
}

/// Root container for a config surface's schema.
///
/// Generated for each surface (SettingsJson, McpJson, etc.) by `schema_module.rs.tera`.
/// Use `load()` to instantiate at compile time.
#[derive(Debug, Clone, Copy)]
pub struct StaticSchema {
    /// Top-level fields of this config surface.
    pub fields: &'static [SchemaNode],
}

impl StaticSchema {
    /// Create a schema from a static field array.
    ///
    /// Called by generated `schema_module.rs` code:
    ///
    /// ```ignore
    /// pub static SCHEMA: StaticSchema = StaticSchema::load(&[
    ///     SchemaNode {
    ///         name: "effortLevel",
    ///         description: "Reasoning effort...",
    ///         field_type: "enum",
    ///         required: false,
    ///         enum_values: &["low", "medium", "high", "max"],
    ///         children: &[],
    ///     },
    ///     // ...
    /// ]);
    /// ```
    pub const fn load(fields: &'static [SchemaNode]) -> Self {
        StaticSchema { fields }
    }

    /// Find a top-level field by name.
    pub fn find_field(&self, name: &str) -> Option<&SchemaNode> {
        self.fields.iter().find(|f| f.name == name)
    }

    /// Find a nested field by dotted path (e.g., "hooks.command").
    ///
    /// Splits on `.` and descends the tree. Returns the deepest matching node.
    pub fn find_field_by_path(&self, path: &str) -> Option<&SchemaNode> {
        let mut parts = path.split('.');
        let first = parts.next()?;
        let mut current = self.find_field(first)?;

        for part in parts {
            current = current.find_field(part)?;
        }

        Some(current)
    }

    /// Iterate all fields at this level (not recursive).
    pub fn iter(&self) -> impl Iterator<Item = &SchemaNode> {
        self.fields.iter()
    }

    /// Count of top-level fields.
    pub fn field_count(&self) -> usize {
        self.fields.len()
    }

    /// Returns true if all required fields are present in the given key set.
    ///
    /// `keys` is a set of field names present in the config file (e.g., parsed JSON keys).
    /// Checks only the top level, not nested required fields.
    pub fn required_fields_present(&self, keys: &std::collections::HashSet<&str>) -> bool {
        self.fields
            .iter()
            .filter(|f| f.required)
            .all(|f| keys.contains(f.name))
    }

    /// Collect all required field names at the top level.
    pub fn required_field_names(&self) -> Vec<&str> {
        self.fields
            .iter()
            .filter(|f| f.required)
            .map(|f| f.name)
            .collect()
    }

    /// Collect all top-level field names.
    pub fn field_names(&self) -> Vec<&str> {
        self.fields.iter().map(|f| f.name).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_node_find_field_direct() {
        let child1 = SchemaNode {
            name: "level",
            description: "Effort level",
            field_type: "enum",
            required: false,
            enum_values: &["low", "high"],
            children: &[],
        };
        let parent = SchemaNode {
            name: "config",
            description: "Root",
            field_type: "object",
            required: true,
            enum_values: &[],
            children: &[child1],
        };

        assert!(parent.find_field("level").is_some());
        assert_eq!(parent.find_field("level").unwrap().name, "level");
        assert!(parent.find_field("missing").is_none());
    }

    #[test]
    fn schema_node_type_signature() {
        let enum_node = SchemaNode {
            name: "effort",
            description: "Effort level",
            field_type: "enum",
            required: false,
            enum_values: &["low", "medium", "high"],
            children: &[],
        };
        assert_eq!(
            enum_node.type_signature(),
            "enum (\"low\" | \"medium\" | \"high\")"
        );

        let string_node = SchemaNode {
            name: "name",
            description: "Name",
            field_type: "string",
            required: false,
            enum_values: &[],
            children: &[],
        };
        assert_eq!(string_node.type_signature(), "string");
    }

    #[test]
    fn static_schema_load() {
        let fields = [
            SchemaNode {
                name: "field1",
                description: "First field",
                field_type: "string",
                required: true,
                enum_values: &[],
                children: &[],
            },
            SchemaNode {
                name: "field2",
                description: "Second field",
                field_type: "boolean",
                required: false,
                enum_values: &[],
                children: &[],
            },
        ];

        let schema = StaticSchema::load(&fields);
        assert_eq!(schema.field_count(), 2);
        assert!(schema.find_field("field1").is_some());
        assert!(schema.find_field("missing").is_none());
    }

    #[test]
    fn static_schema_required_fields() {
        let fields = [
            SchemaNode {
                name: "required_field",
                description: "Required",
                field_type: "string",
                required: true,
                enum_values: &[],
                children: &[],
            },
            SchemaNode {
                name: "optional_field",
                description: "Optional",
                field_type: "string",
                required: false,
                enum_values: &[],
                children: &[],
            },
        ];

        let schema = StaticSchema::load(&fields);
        let required = schema.required_field_names();
        assert_eq!(required, vec!["required_field"]);
    }

    #[test]
    fn static_schema_find_by_path() {
        let grandchild = SchemaNode {
            name: "command",
            description: "Command to run",
            field_type: "string",
            required: true,
            enum_values: &[],
            children: &[],
        };
        let child = SchemaNode {
            name: "hooks",
            description: "Hook definitions",
            field_type: "object",
            required: false,
            enum_values: &[],
            children: &[grandchild],
        };
        let root = [child];
        let schema = StaticSchema::load(&root);

        // Direct path
        assert!(schema.find_field_by_path("hooks").is_some());
        // Nested path
        assert!(schema.find_field_by_path("hooks.command").is_some());
        assert_eq!(
            schema.find_field_by_path("hooks.command").unwrap().name,
            "command"
        );
        // Missing path
        assert!(schema.find_field_by_path("hooks.missing").is_none());
        assert!(schema.find_field_by_path("missing.command").is_none());
    }

    #[test]
    fn static_schema_present_check() {
        use std::collections::HashSet;

        let fields = [
            SchemaNode {
                name: "required_field",
                description: "Required",
                field_type: "string",
                required: true,
                enum_values: &[],
                children: &[],
            },
            SchemaNode {
                name: "optional_field",
                description: "Optional",
                field_type: "string",
                required: false,
                enum_values: &[],
                children: &[],
            },
        ];

        let schema = StaticSchema::load(&fields);

        let mut keys = HashSet::new();
        keys.insert("required_field");
        assert!(schema.required_fields_present(&keys));

        let mut keys = HashSet::new();
        assert!(!schema.required_fields_present(&keys));
    }
}
