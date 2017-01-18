use regex::Regex;

enum Token {
    CLASS,
    PUBLIC,
    STATIC,
    THIS,
    NEW,
    VOID,
    MAIN,
    STRING,
    EXTENDS,
    INT,
    BOOLEAN,
    IF,
    ELSE,
    WHILE,
    RETURN,
    PRINTLN,
    TRUE,
    FALSE,
    LENGTH,
    IDENTIFIER(String),
    NUMBER(i64),
}

pub fn tokenize(input: &str) {

    let comment_re = Regex::new(r"(/\*([^*]|[\r\n]|(\*+([^*/]|[\r\n])))*\*+/)|(//.*)").unwrap();

    let preprocessed = comment_re.replace_all(input, "");

}
