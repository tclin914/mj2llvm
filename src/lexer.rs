use regex::Regex;

pub enum Token {
    Class,
    Public,
    Static,
    This,
    New,
    Void,
    Main,
    STRING,
    Extends,
    If,
    Else,
    While,
    Return,
    Println,
    True,
    False,
    Length,
    Int,
    Boolean,
    Delimiter,
    OpBrace,
    ClBrace,
    OpBracket,
    ClBracket,
    OpParethesis,
    ClParethesis,
    Comma,
    Dot,
    And,
    Less,
    Add,
    Sub,
    Mul,
    Not,
    Assign,
    Identifier(String),
    Number(i64),
    Text(String)
}

pub fn tokenize(input: &str) -> Vec<Token> {

    let comment_re = Regex::new(r"(/\*([^*]|[\r\n]|(\*+([^*/]|[\r\n])))*\*+/)|(//.*)").unwrap();

    let preprocessed = comment_re.replace_all(input, "");

    let mut result = Vec::new();

    let token_re = Regex::new(concat!(
            r"(?P<println>System\.out\.println)|",
            r"(?P<identifier>\p{Alphabetic}\w*)|",
            "(?P<text>\".*?\")|",
            r"(?P<number>\d+)|", 
            r"(?P<delimiter>;)|",
            r"(?P<opbrace>\{)|",
            r"(?P<clbrace>\})|",
            r"(?P<opbracket>\[)|",
            r"(?P<clbracket>\])|",
            r"(?P<opparethesis>\()|",
            r"(?P<clparethesis>\))|",
            r"(?P<comma>,)|",
            r"(?P<dot>\.)|",
            r"(?P<operator>&&|<|\+|-|\*|!|=)")).unwrap();
    
    for cap in token_re.captures_iter(preprocessed.into_owned().as_str()) {
        let token = if cap.name("identifier").is_some() {
            match cap.name("identifier").unwrap().as_str() {
                "class" => Token::Class,
                "public" => Token::Public,
                "static" => Token::Static,
                "this" => Token::This,
                "new" => Token::New,
                "void" => Token::Void,
                "main" => Token::Main,
                "String" => Token::STRING,
                "extends" => Token::Extends,
                "if" => Token::If,
                "else" => Token::Else,
                "while" => Token::While,
                "return" => Token::Return,
                "System.out.println" => Token::Println,
                "true" => Token::True,
                "false" => Token::False,
                "length" => Token::Length,
                "int" => Token::Int,
                "boolean" => Token::Boolean,
                identifier => Token::Identifier(identifier.to_string()),
            }
        } else if cap.name("println").is_some() {
            Token::Println
        } else if cap.name("text").is_some() {
            Token::Text(cap.name("text").unwrap().as_str().to_string())
        } else if cap.name("number").is_some() {
            match cap.name("number").unwrap().as_str().parse() {
                Ok(number) => Token::Number(number),
                Err(_) => panic!("Lexer failed trying to parse number")
            }
        } else if cap.name("delimiter").is_some() {
            Token::Delimiter
        } else if cap.name("opbrace").is_some() {
            Token::OpBrace
        } else if cap.name("clbrace").is_some() {
            Token::ClBrace
        } else if cap.name("opbracket").is_some() {
            Token::OpBracket
        } else if cap.name("clbracket").is_some() {
            Token::ClBracket
        } else if cap.name("opparethesis").is_some() {
            Token::OpParethesis
        } else if cap.name("clparethesis").is_some() {
            Token::ClParethesis
        } else if cap.name("comma").is_some() {
            Token::Comma
        } else if cap.name("dot").is_some() {
            Token::Dot
        } else if cap.name("operator").is_some() {
            match cap.name("operator").unwrap().as_str() {
                "&&" => Token::And,
                "<" => Token::Less,
                "+" => Token::Add,
                "-" => Token::Sub,
                "*" => Token::Mul,
                "!" => Token::Not,
                "=" => Token::Assign,
                x => panic!("Unexpected invalid token {:?}", x)
            }
        } else {
            panic!("Unexpected invalid text");
        };      
        result.push(token);
    }
    result
}
