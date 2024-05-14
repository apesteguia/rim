#[derive(Debug, PartialEq)]
pub enum Lenguaje {
    Rust,
    Elixir,
    C,
    Cpp,
    JavaScript,
    TypeScript,
    Java,
    Lua,
    Python,
    Txt,
    Markdown,
    Jsx,
    Assembly,
    Haskell,
    OCaml,
    Clojure,
    Go,
    Css,
    Html,
    Bash,
    Php,
    Ruby,
    Undefined,
}

pub fn obtener_nombre_lenguaje(codigo: &str) -> Option<Lenguaje> {
    match codigo.to_lowercase().as_str() {
        "rs" => Some(Lenguaje::Rust),
        "ex" | "exs" => Some(Lenguaje::Elixir),
        "c" | "h" => Some(Lenguaje::C),
        "cpp" | "c++" | "hpp" => Some(Lenguaje::Cpp),
        "js" => Some(Lenguaje::JavaScript),
        "ts" => Some(Lenguaje::TypeScript),
        "java" => Some(Lenguaje::Java),
        "lua" => Some(Lenguaje::Lua),
        "py" => Some(Lenguaje::Python),
        "txt" => Some(Lenguaje::Txt),
        "md" | "mdx" => Some(Lenguaje::Markdown),
        "jsx" | "tsx" => Some(Lenguaje::Jsx),
        "s" | "asm" | "nasm" => Some(Lenguaje::Assembly),
        "hs" => Some(Lenguaje::Haskell),
        "ml" | "mli" => Some(Lenguaje::OCaml),
        "cjl" => Some(Lenguaje::Clojure),
        "go" => Some(Lenguaje::Go),
        "css" => Some(Lenguaje::Css),
        "html" | "htmx" => Some(Lenguaje::Html),
        "sh" => Some(Lenguaje::Bash),
        "php" => Some(Lenguaje::Php),
        "rb" => Some(Lenguaje::Ruby),
        _ => Some(Lenguaje::Undefined),
    }
}

pub fn reserved_words(l: &Lenguaje) -> Vec<String> {
    match l {
        Lenguaje::Rust => vec![
            "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
            "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
            "return", "Self", "self", "static", "struct", "super", "trait", "true", "type",
            "unsafe", "use", "where", "while", "async", "await", "dyn", "None", "Ok", "Some",
            "Option", "Result", "Err",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect(),

        // Agrega más palabras reservadas para otros lenguajes aquí
        Lenguaje::Python => vec![
            "False", "def", "if", "raise", "None", "del", "import", "return", "True", "elif", "in",
            "try", "and", "else", "is", "while", "as", "except", "lambda", "with", "assert",
            "finally", "nonlocal", "yield", "break", "for", "not", "class", "from", "or",
            "continue", "global", "pass",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect(),

        Lenguaje::C => vec![
            "auto", "else", "if", "break", "case", "char", "const", "continue", "deafult", "do",
            "double", "enum", "extern", "float", "for", "goto", "int", "long", "register",
            "return", "short", "signed", "sizeof", "static", "struct", "switch", "typedef",
            "union", "unsigned", "void", "continue", "while", "volatile",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect(),

        Lenguaje::JavaScript => vec![
            "function",
            "else",
            "if",
            "break",
            "case",
            "const",
            "continue",
            "deafult",
            "do",
            "for",
            "let",
            "export",
            "return",
            "short",
            "signed",
            "sizeof",
            "static",
            "struct",
            "switch",
            "var",
            "class",
            "unsigned",
            "throw",
            "try",
            "while",
            "catch",
            "new",
            "protected",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect(),

        Lenguaje::Lua => vec![
            "and", "break", "do", "else", "elseif", "end", "false", "for", "function", "if", "in",
            "local", "nil", "not", "or", "repeat", "return", "then", "true", "until", "while",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect(),

        Lenguaje::OCaml => vec![
            "and",
            "as",
            "assert",
            "asr",
            "begin",
            "class",
            "constraint",
            "do",
            "done",
            "downto",
            "else",
            "end",
            "exception",
            "external",
            "false",
            "for",
            "fun",
            "function",
            "functor",
            "if",
            "in",
            "include",
            "inherit",
            "initializer",
            "land",
            "lazy",
            "let",
            "lor",
            "lsl",
            "lsr",
            "lxor",
            "match",
            "method",
            "mod",
            "module",
            "mutable",
            "new",
            "nonrec",
            "object",
            "of",
            "open",
            "or",
            "private",
            "rec",
            "sig",
            "struct",
            "then",
            "to",
            "true",
            "try",
            "type",
            "val",
            "virtual",
            "when",
            "while",
            "with",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect(),

        _ => Vec::new(),
    }
}
