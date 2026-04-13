#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs;
    use std::path::Path;

    use tree_sitter::Parser;

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    /// Root of the extension repository (two levels up from tests/crate/).
    fn project_root() -> &'static Path {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
    }

    fn read_query(lang: &str, file: &str) -> String {
        let path = project_root().join("languages").join(lang).join(file);
        fs::read_to_string(&path).unwrap_or_else(|e| panic!("Failed to read {}: {e}", path.display()))
    }

    fn read_fixture(name: &str) -> String {
        let path = project_root().join("tests/fixtures").join(name);
        fs::read_to_string(&path).unwrap_or_else(|e| panic!("Failed to read {}: {e}", path.display()))
    }

    fn parse_apex(source: &str) -> tree_sitter::Tree {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_sfapex::apex::LANGUAGE.into())
            .expect("Failed to load Apex grammar");
        parser.parse(source, None).expect("Failed to parse Apex source")
    }

    fn parse_soql(source: &str) -> tree_sitter::Tree {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_sfapex::soql::LANGUAGE.into())
            .expect("Failed to load SOQL grammar");
        parser.parse(source, None).expect("Failed to parse SOQL source")
    }

    fn parse_sosl(source: &str) -> tree_sitter::Tree {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_sfapex::sosl::LANGUAGE.into())
            .expect("Failed to parse SOSL source");
        parser.parse(source, None).expect("Failed to parse SOSL source")
    }

    #[allow(dead_code)]
    /// Collect all node types that appear in a parse tree (useful for debugging).
    fn collect_node_types(tree: &tree_sitter::Tree) -> Vec<String> {
        let mut types = Vec::new();
        let mut cursor = tree.walk();
        collect_node_types_recursive(&mut cursor, &mut types);
        types.sort();
        types.dedup();
        types
    }

    #[allow(dead_code)]
    fn collect_node_types_recursive(
        cursor: &mut tree_sitter::TreeCursor,
        types: &mut Vec<String>,
    ) {
        types.push(cursor.node().kind().to_string());
        if cursor.goto_first_child() {
            loop {
                collect_node_types_recursive(cursor, types);
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            cursor.goto_parent();
        }
    }

    // -----------------------------------------------------------------------
    // Grammar loading tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_apex_grammar_loads() {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_sfapex::apex::LANGUAGE.into())
            .expect("Apex grammar should load successfully");
    }

    #[test]
    fn test_soql_grammar_loads() {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_sfapex::soql::LANGUAGE.into())
            .expect("SOQL grammar should load successfully");
    }

    #[test]
    fn test_sosl_grammar_loads() {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_sfapex::sosl::LANGUAGE.into())
            .expect("SOSL grammar should load successfully");
    }

    // -----------------------------------------------------------------------
    // Query validation tests — ensure .scm files are syntactically valid
    // against their respective grammars.
    // -----------------------------------------------------------------------

    fn validate_query(language: &tree_sitter::Language, query_source: &str, label: &str) {
        match tree_sitter::Query::new(language, query_source) {
            Ok(_) => {} // valid
            Err(e) => panic!("{label} query is invalid: {e}"),
        }
    }

    #[test]
    fn test_apex_highlights_query_valid() {
        let source = read_query("apex", "highlights.scm");
        let lang: tree_sitter::Language = tree_sitter_sfapex::apex::LANGUAGE.into();
        validate_query(&lang, &source, "apex/highlights.scm");
    }

    #[test]
    fn test_apex_brackets_query_valid() {
        let source = read_query("apex", "brackets.scm");
        let lang: tree_sitter::Language = tree_sitter_sfapex::apex::LANGUAGE.into();
        validate_query(&lang, &source, "apex/brackets.scm");
    }

    #[test]
    fn test_apex_indents_query_valid() {
        let source = read_query("apex", "indents.scm");
        let lang: tree_sitter::Language = tree_sitter_sfapex::apex::LANGUAGE.into();
        validate_query(&lang, &source, "apex/indents.scm");
    }

    #[test]
    fn test_apex_outline_query_valid() {
        let source = read_query("apex", "outline.scm");
        let lang: tree_sitter::Language = tree_sitter_sfapex::apex::LANGUAGE.into();
        validate_query(&lang, &source, "apex/outline.scm");
    }

    #[test]
    fn test_apex_injections_query_valid() {
        let source = read_query("apex", "injections.scm");
        let lang: tree_sitter::Language = tree_sitter_sfapex::apex::LANGUAGE.into();
        validate_query(&lang, &source, "apex/injections.scm");
    }

    #[test]
    fn test_soql_highlights_query_valid() {
        let source = read_query("soql", "highlights.scm");
        let lang: tree_sitter::Language = tree_sitter_sfapex::soql::LANGUAGE.into();
        validate_query(&lang, &source, "soql/highlights.scm");
    }

    #[test]
    fn test_soql_brackets_query_valid() {
        let source = read_query("soql", "brackets.scm");
        let lang: tree_sitter::Language = tree_sitter_sfapex::soql::LANGUAGE.into();
        validate_query(&lang, &source, "soql/brackets.scm");
    }

    #[test]
    fn test_soql_indents_query_valid() {
        let source = read_query("soql", "indents.scm");
        let lang: tree_sitter::Language = tree_sitter_sfapex::soql::LANGUAGE.into();
        validate_query(&lang, &source, "soql/indents.scm");
    }

    #[test]
    fn test_sosl_highlights_query_valid() {
        let source = read_query("sosl", "highlights.scm");
        let lang: tree_sitter::Language = tree_sitter_sfapex::sosl::LANGUAGE.into();
        validate_query(&lang, &source, "sosl/highlights.scm");
    }

    #[test]
    fn test_sosl_brackets_query_valid() {
        let source = read_query("sosl", "brackets.scm");
        let lang: tree_sitter::Language = tree_sitter_sfapex::sosl::LANGUAGE.into();
        validate_query(&lang, &source, "sosl/brackets.scm");
    }

    #[test]
    fn test_sosl_indents_query_valid() {
        let source = read_query("sosl", "indents.scm");
        let lang: tree_sitter::Language = tree_sitter_sfapex::sosl::LANGUAGE.into();
        validate_query(&lang, &source, "sosl/indents.scm");
    }

    // -----------------------------------------------------------------------
    // Parsing fixture tests — ensure sample files parse without errors.
    // -----------------------------------------------------------------------

    fn assert_no_errors(tree: &tree_sitter::Tree, label: &str) {
        let root = tree.root_node();
        assert!(
            !root.has_error(),
            "{label}: parse tree contains errors. Tree:\n{}",
            root.to_sexp()
        );
    }

    #[test]
    fn test_parse_sample_class() {
        let source = read_fixture("SampleClass.cls");
        let tree = parse_apex(&source);
        assert_no_errors(&tree, "SampleClass.cls");

        let root = tree.root_node();
        assert_eq!(root.kind(), "parser_output");
        assert!(
            root.child_count() > 0,
            "SampleClass.cls should produce a non-empty parse tree"
        );
    }

    #[test]
    fn test_parse_sample_trigger() {
        let source = read_fixture("SampleTrigger.trigger");
        let tree = parse_apex(&source);
        assert_no_errors(&tree, "SampleTrigger.trigger");

        let root = tree.root_node();
        let sexp = root.to_sexp();
        assert!(
            sexp.contains("trigger_declaration"),
            "Trigger file should contain a trigger_declaration node"
        );
    }

    #[test]
    fn test_parse_sample_soql() {
        let source = read_fixture("sample.soql");
        let tree = parse_soql(&source);
        assert_no_errors(&tree, "sample.soql");
    }

    #[test]
    fn test_parse_sample_sosl() {
        let source = read_fixture("sample.sosl");
        let tree = parse_sosl(&source);
        assert_no_errors(&tree, "sample.sosl");
    }

    // -----------------------------------------------------------------------
    // Highlights produce captures — ensure our queries actually match nodes
    // in realistic source files.
    // -----------------------------------------------------------------------

    fn count_captures(
        language: &tree_sitter::Language,
        query_source: &str,
        source_code: &[u8],
        tree: &tree_sitter::Tree,
    ) -> HashMap<String, usize> {
        use streaming_iterator::StreamingIterator;

        let query = tree_sitter::Query::new(language, query_source).unwrap();
        let mut cursor = tree_sitter::QueryCursor::new();
        let mut matches = cursor.matches(&query, tree.root_node(), source_code);

        let mut counts: HashMap<String, usize> = HashMap::new();
        while let Some(m) = matches.next() {
            for capture in m.captures {
                let name = query.capture_names()[capture.index as usize].to_string();
                *counts.entry(name).or_insert(0) += 1;
            }
        }
        counts
    }

    #[test]
    fn test_apex_highlights_produce_captures() {
        let source = read_fixture("SampleClass.cls");
        let tree = parse_apex(&source);
        let query_source = read_query("apex", "highlights.scm");
        let lang: tree_sitter::Language = tree_sitter_sfapex::apex::LANGUAGE.into();

        let captures = count_captures(&lang, &query_source, source.as_bytes(), &tree);

        assert!(
            !captures.is_empty(),
            "Apex highlights should produce at least some captures"
        );
        // Verify essential capture groups appear
        assert!(
            captures.contains_key("keyword"),
            "Should capture keywords, got: {captures:?}"
        );
        assert!(
            captures.contains_key("type"),
            "Should capture types, got: {captures:?}"
        );
        assert!(
            captures.contains_key("function.method"),
            "Should capture methods, got: {captures:?}"
        );
        assert!(
            captures.contains_key("comment"),
            "Should capture comments, got: {captures:?}"
        );
        assert!(
            captures.contains_key("string"),
            "Should capture strings, got: {captures:?}"
        );
        assert!(
            captures.contains_key("variable"),
            "Should capture variables, got: {captures:?}"
        );
    }

    #[test]
    fn test_apex_highlights_capture_annotations() {
        let source = read_fixture("SampleClass.cls");
        let tree = parse_apex(&source);
        let query_source = read_query("apex", "highlights.scm");
        let lang: tree_sitter::Language = tree_sitter_sfapex::apex::LANGUAGE.into();

        let captures = count_captures(&lang, &query_source, source.as_bytes(), &tree);

        assert!(
            captures.contains_key("attribute"),
            "Should capture @AuraEnabled as attribute, got: {captures:?}"
        );
    }

    #[test]
    fn test_apex_highlights_capture_enum() {
        let source = read_fixture("SampleClass.cls");
        let tree = parse_apex(&source);
        let query_source = read_query("apex", "highlights.scm");
        let lang: tree_sitter::Language = tree_sitter_sfapex::apex::LANGUAGE.into();

        let captures = count_captures(&lang, &query_source, source.as_bytes(), &tree);

        assert!(
            captures.contains_key("variant"),
            "Should capture enum constants as variant, got: {captures:?}"
        );
    }

    #[test]
    fn test_apex_outline_produces_captures() {
        let source = read_fixture("SampleClass.cls");
        let tree = parse_apex(&source);
        let query_source = read_query("apex", "outline.scm");
        let lang: tree_sitter::Language = tree_sitter_sfapex::apex::LANGUAGE.into();

        let captures = count_captures(&lang, &query_source, source.as_bytes(), &tree);

        assert!(
            captures.contains_key("name"),
            "Outline should capture @name nodes, got: {captures:?}"
        );
        assert!(
            captures.contains_key("item"),
            "Outline should capture @item nodes, got: {captures:?}"
        );
        // SampleClass has: 1 class + 1 constructor + 5 methods + 3 fields + 1 enum + 3 enum constants = 14 items
        let item_count = captures.get("item").copied().unwrap_or(0);
        assert!(
            item_count >= 5,
            "Outline should capture multiple items (class, methods, fields), got {item_count}"
        );
    }

    #[test]
    fn test_soql_highlights_produce_captures() {
        let source = read_fixture("sample.soql");
        let tree = parse_soql(&source);
        let query_source = read_query("soql", "highlights.scm");
        let lang: tree_sitter::Language = tree_sitter_sfapex::soql::LANGUAGE.into();

        let captures = count_captures(&lang, &query_source, source.as_bytes(), &tree);

        assert!(!captures.is_empty(), "SOQL highlights should produce captures");
        assert!(
            captures.contains_key("keyword"),
            "Should capture SOQL keywords, got: {captures:?}"
        );
        assert!(
            captures.contains_key("property"),
            "Should capture field names as properties, got: {captures:?}"
        );
    }

    #[test]
    fn test_sosl_highlights_produce_captures() {
        let source = read_fixture("sample.sosl");
        let tree = parse_sosl(&source);
        let query_source = read_query("sosl", "highlights.scm");
        let lang: tree_sitter::Language = tree_sitter_sfapex::sosl::LANGUAGE.into();

        let captures = count_captures(&lang, &query_source, source.as_bytes(), &tree);

        assert!(!captures.is_empty(), "SOSL highlights should produce captures");
        assert!(
            captures.contains_key("keyword"),
            "Should capture SOSL keywords, got: {captures:?}"
        );
    }

    #[test]
    fn test_trigger_highlights_produce_captures() {
        let source = read_fixture("SampleTrigger.trigger");
        let tree = parse_apex(&source);
        let query_source = read_query("apex", "highlights.scm");
        let lang: tree_sitter::Language = tree_sitter_sfapex::apex::LANGUAGE.into();

        let captures = count_captures(&lang, &query_source, source.as_bytes(), &tree);

        assert!(
            captures.contains_key("keyword"),
            "Trigger should have keywords highlighted"
        );
        assert!(
            captures.contains_key("type"),
            "Trigger should have types highlighted"
        );
    }

    // -----------------------------------------------------------------------
    // Extension manifest validation
    // -----------------------------------------------------------------------

    #[derive(serde::Deserialize)]
    #[allow(dead_code)]
    struct ExtensionManifest {
        id: String,
        name: String,
        version: String,
        schema_version: u32,
        authors: Vec<String>,
        description: String,
        repository: String,
        grammars: HashMap<String, GrammarEntry>,
        language_servers: HashMap<String, LanguageServerEntry>,
    }

    #[derive(serde::Deserialize)]
    struct GrammarEntry {
        repository: String,
        rev: String,
        path: Option<String>,
    }

    #[derive(serde::Deserialize)]
    struct LanguageServerEntry {
        name: String,
        languages: Vec<String>,
    }

    #[test]
    fn test_extension_toml_valid() {
        let content = fs::read_to_string(project_root().join("extension.toml"))
            .expect("extension.toml should exist");
        let manifest: ExtensionManifest =
            toml::from_str(&content).expect("extension.toml should be valid TOML");

        assert_eq!(manifest.id, "apex");
        assert_eq!(manifest.name, "Apex");
        assert_eq!(manifest.schema_version, 1);
        assert!(!manifest.authors.is_empty());
        assert!(!manifest.description.is_empty());
        assert!(!manifest.repository.is_empty());
    }

    #[test]
    fn test_extension_toml_grammars() {
        let content = fs::read_to_string(project_root().join("extension.toml")).unwrap();
        let manifest: ExtensionManifest = toml::from_str(&content).unwrap();

        // Should have all three grammars
        assert!(manifest.grammars.contains_key("apex"), "Missing apex grammar");
        assert!(manifest.grammars.contains_key("soql"), "Missing soql grammar");
        assert!(manifest.grammars.contains_key("sosl"), "Missing sosl grammar");

        // All grammars should point to the same repo
        for (name, grammar) in &manifest.grammars {
            assert!(
                grammar.repository.contains("tree-sitter-sfapex"),
                "{name} grammar should reference tree-sitter-sfapex"
            );
            assert!(
                !grammar.rev.is_empty(),
                "{name} grammar should have a rev"
            );
            assert!(
                grammar.path.is_some(),
                "{name} grammar should specify a path subdirectory"
            );
        }

        // Path should match grammar name
        assert_eq!(manifest.grammars["apex"].path.as_deref(), Some("apex"));
        assert_eq!(manifest.grammars["soql"].path.as_deref(), Some("soql"));
        assert_eq!(manifest.grammars["sosl"].path.as_deref(), Some("sosl"));
    }

    #[test]
    fn test_extension_toml_language_server() {
        let content = fs::read_to_string(project_root().join("extension.toml")).unwrap();
        let manifest: ExtensionManifest = toml::from_str(&content).unwrap();

        assert!(
            manifest.language_servers.contains_key("apex-jorje"),
            "Should define apex-jorje language server"
        );

        let server = &manifest.language_servers["apex-jorje"];
        assert!(!server.name.is_empty());
        assert!(
            server.languages.contains(&"Apex".to_string()),
            "Language server should serve Apex"
        );
    }

    // -----------------------------------------------------------------------
    // Language config validation
    // -----------------------------------------------------------------------

    #[derive(serde::Deserialize)]
    struct LanguageConfig {
        name: String,
        grammar: String,
        path_suffixes: Vec<String>,
        line_comments: Vec<String>,
    }

    #[test]
    fn test_apex_config_valid() {
        let content =
            fs::read_to_string(project_root().join("languages/apex/config.toml")).unwrap();
        let config: LanguageConfig = toml::from_str(&content).unwrap();

        assert_eq!(config.name, "Apex");
        assert_eq!(config.grammar, "apex");
        assert!(config.path_suffixes.contains(&"cls".to_string()));
        assert!(config.path_suffixes.contains(&"trigger".to_string()));
        assert!(config.path_suffixes.contains(&"apex".to_string()));
        assert!(!config.line_comments.is_empty());
    }

    #[test]
    fn test_soql_config_valid() {
        let content =
            fs::read_to_string(project_root().join("languages/soql/config.toml")).unwrap();
        let config: LanguageConfig = toml::from_str(&content).unwrap();

        assert_eq!(config.name, "SOQL");
        assert_eq!(config.grammar, "soql");
        assert!(config.path_suffixes.contains(&"soql".to_string()));
    }

    #[test]
    fn test_sosl_config_valid() {
        let content =
            fs::read_to_string(project_root().join("languages/sosl/config.toml")).unwrap();
        let config: LanguageConfig = toml::from_str(&content).unwrap();

        assert_eq!(config.name, "SOSL");
        assert_eq!(config.grammar, "sosl");
        assert!(config.path_suffixes.contains(&"sosl".to_string()));
    }

    // -----------------------------------------------------------------------
    // All query files exist
    // -----------------------------------------------------------------------

    #[test]
    fn test_all_expected_query_files_exist() {
        let root = project_root();
        let expected = [
            "languages/apex/highlights.scm",
            "languages/apex/brackets.scm",
            "languages/apex/indents.scm",
            "languages/apex/outline.scm",
            "languages/apex/injections.scm",
            "languages/soql/highlights.scm",
            "languages/soql/brackets.scm",
            "languages/soql/indents.scm",
            "languages/sosl/highlights.scm",
            "languages/sosl/brackets.scm",
            "languages/sosl/indents.scm",
        ];

        for file in &expected {
            let path = root.join(file);
            assert!(path.exists(), "Expected query file missing: {file}");
            let content = fs::read_to_string(&path).unwrap();
            assert!(!content.trim().is_empty(), "Query file should not be empty: {file}");
        }
    }
}
