/// Represents a individual programming language. Can be used to provide
/// information about the language, such as multi line comments, single line
/// comments, string literal syntax, whether a given language allows nesting
/// comments.
pub enum LanguageType {
    {% for key, _ in languages -%}
        #[allow(missing_docs)] {{key}},
    {% endfor %}
}