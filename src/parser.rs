use lexer::*; 

enum BinaryOp {
    And,
    Less,
    Add,
    Sub,
    Mul
}

// struct Identifier {
    // name: String
// }

enum Type {
    Array_T,
    Bool_T,
    Integer_T,
    Class_T
}

enum Expression {
    BinaryExpression(BinaryOp, Box<Expression>, Box<Expression>),
    ArrayIndexExpression(Box<Expression>, Box<Expression>),
    ArrayLengthExpression(Box<Expression>),
    FuncCallExpression(Box<Expression>, String, Vec<Expression>),
    ConstantInteger(i64),
    ConstantBoolean(bool),
    Identifier(String),
    ThisExpression,
    NewArrayExpression(Box<Expression>),
    NewExpression(String),
    NotExpression(Box<Expression>)
}

enum Statement {
    StatementList(Vec<Statement>),
    IfStatement(Box<Expression>, Box<Statement>, Box<Statement>),
    WhileStatement(Box<Expression>, Box<Statement>),
    PrintlnStatement(Box<Expression>),
    AssignStatement(String, Box<Expression>),
    ArrayAssignStatement(String, Box<Expression>, Box<Expression>)
}

struct MainClass {
    name: String,
    state: Statement
}

use self::ParsingResult:: {
    Good,
    NotComplete,
    Bad 
};

pub enum ParsingResult<T> {
    Good(T, Vec<Token>),
    NotComplete,
    Bad(String)
}

enum ASTNode {
    MainClassNode(MainClass)
}

fn error<T>(message :&str) -> ParsingResult<T> {
    Bad(message.to_string())
}

pub fn parse(tokens :Vec<Token>) {

    let mut rest = tokens;

    rest.reverse();

    let first_token = match rest.last() {
        Some(token) => token.clone(),
        None => return
    };

    match first_token {
        Token::Class => {
            match parse_mainclass(&mut rest) {
                Good(ast, parsed_token) => println!("{}", "Good"),
                NotComplete => println!("{}", "NotComplete"),
                Bad(message) => println!("{}", message) 
            };
        },
        _ => panic!("The first token should be \"class\"")
    };

}

macro_rules! expect_token (
    ([ $($token:pat, $value:expr, $result:stmt);+ ] <= $tokens:ident, $parsed_tokens:ident, $error:expr) => (
        match $tokens.pop() {
            $(
                Some($token) => {
                    $parsed_tokens.push($value);
                    $result
                },
            )+
            None => {
                $parsed_tokens.reverse();
                $tokens.extend($parsed_tokens.into_iter());
                return NotComplete;
            },
            _ => return error($error)
        }                                                                                                      
    );    
                          
                          
);

fn parse_mainclass(tokens: &mut Vec<Token>) -> ParsingResult<ASTNode> {
    tokens.pop();

    let mut parsed_tokens = Vec::new();

    let class_name = expect_token!([Token::Identifier(class_name), Token::Identifier(class_name.clone()), class_name]
        <= tokens, parsed_tokens, "expected class name in main class declaration");

    expect_token!([Token::OpBrace, Token::OpBrace, ()]
        <= tokens, parsed_tokens, "expected '{' in main class declaration");

    expect_token!([Token::Public, Token::Public, ()]
        <= tokens, parsed_tokens, "expected 'public' in main function declaration");

    expect_token!([Token::Static, Token::Static, ()]
        <= tokens, parsed_tokens, "expected 'static' in main function declaration");

    expect_token!([Token::Void, Token::Void, ()]
        <= tokens, parsed_tokens, "expected 'void' in main function declaration");

    expect_token!([Token::Main, Token::Main, ()]
        <= tokens, parsed_tokens, "expected 'main' in main function declaration");
    
    expect_token!([Token::OpParethesis, Token::OpParethesis, ()]
        <= tokens, parsed_tokens, "expected '(' in main function declaration");
    
    expect_token!([Token::STRING, Token::STRING, ()]
        <= tokens, parsed_tokens, "expected 'String' in main function declaration");
    
    expect_token!([Token::OpBracket, Token::OpBracket, ()]
        <= tokens, parsed_tokens, "expected '[' in main function declaration");
    
    expect_token!([Token::ClBracket, Token::ClBracket, ()]
        <= tokens, parsed_tokens, "expected ']' in main function declaration");

    let arg_name = expect_token!([Token::Identifier(arg_name), Token::Identifier(arg_name.clone()), arg_name]
        <= tokens, parsed_tokens, "expected argument name in main function declaration");

    expect_token!([Token::ClParethesis, Token::ClParethesis, ()]
        <= tokens, parsed_tokens, "expected ')' in main function declaration");

    expect_token!([Token::OpBrace, Token::OpBrace, ()]
        <= tokens, parsed_tokens, "expected '{' in main function declaration");

    let state = match parse_statement(tokens) {
        Good(state, toks) => {
            parsed_tokens.extend(toks.into_iter());
            state
        },
        NotComplete => {
            parsed_tokens.reverse();
            tokens.extend(parsed_tokens.into_iter());
            return NotComplete
        },
        Bad(message) => return Bad(message)
    };

    Good(ASTNode::MainClassNode(MainClass{name: class_name, state: state}), parsed_tokens)
}

fn parse_statement(tokens: &mut Vec<Token>) -> ParsingResult<Statement> {
    match tokens.last() {
        Some(&Token::OpBrace) => parse_statement_list(tokens),
        Some(&Token::If) => parse_if_statement(tokens),
        Some(&Token::While) => parse_while_statement(tokens),
        Some(&Token::Println) => parse_println_statement(tokens),
        Some(&Token::Identifier(_)) => parse_assign_statement(tokens),
        None => return ParsingResult::NotComplete,
        _ => error("unknow token when expecting a statement")
    }
}

fn parse_statement_list(tokens: &mut Vec<Token>) -> ParsingResult<Statement> {
    NotComplete
}

fn parse_if_statement(tokens: &mut Vec<Token>) -> ParsingResult<Statement> { 
    NotComplete
}

fn parse_while_statement(tokens: &mut Vec<Token>) -> ParsingResult<Statement> { 
    NotComplete
}

fn parse_println_statement(tokens: &mut Vec<Token>) -> ParsingResult<Statement> { 
    // pop "System.out.println"
    tokens.pop();

    let mut parsed_tokens = vec![Token::Println];

    expect_token!([Token::OpParethesis, Token::OpParethesis, ()]
        <= tokens, parsed_tokens, "expected '(' in System.out.println");
    
    let expr = match parse_expression(tokens) {
        Good(expr, toks) => {
            parsed_tokens.extend(toks.into_iter());
            expr
        },
        NotComplete => {
            parsed_tokens.reverse();
            tokens.extend(parsed_tokens.into_iter());
            return NotComplete
        },
        Bad(message) => return Bad(message)
    };

    expect_token!([Token::ClParethesis, Token::ClParethesis, ()]
        <= tokens, parsed_tokens, "expected ')' in System.out.println");
    
    Good(Statement::PrintlnStatement(Box::new(expr)), parsed_tokens)
}

fn parse_assign_statement(tokens: &mut Vec<Token>) -> ParsingResult<Statement> { 
    NotComplete
}

fn parse_expression(tokens: &mut Vec<Token>) -> ParsingResult<Expression> {
    
    let expr = match tokens.last() {
        Some(&Token::Number(_)) => parse_literal_expression(tokens),
        Some(&Token::True) => parse_true_expression(tokens),
        Some(&Token::False) => parse_false_expression(tokens),
        Some(&Token::Identifier(_)) => parse_identifier_expression(tokens),
        Some(&Token::This) => parse_this_expression(tokens),
        Some(&Token::New) => parse_new_newarray_expression(tokens),
        // Some(&Token::Not) => parse_not_expression(tokens),
        None => return NotComplete,
        _ => error("unknown token when expecting a expression")
    };   
 
    match expr {
        Good(expr, toks) => {
            Good(expr, toks)
        },
        NotComplete => {
            return NotComplete
        },
        Bad(message) => return Bad(message)
    }
}

fn parse_literal_expression(tokens: &mut Vec<Token>) -> ParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();

    let value = expect_token!([Token::Number(value), Token::Number(value), value]
        <= tokens, parsed_tokens, "literal expected");
    
    Good(Expression::ConstantInteger(value), parsed_tokens)
}

fn parse_true_expression(tokens: &mut Vec<Token>) -> ParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();

    expect_token!([Token::True, Token::True, ()]
        <= tokens, parsed_tokens, "'true' expected");
    
    Good(Expression::ConstantBoolean(true), parsed_tokens)
}

fn parse_false_expression(tokens: &mut Vec<Token>) -> ParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();

    expect_token!([Token::False, Token::False, ()]
        <= tokens, parsed_tokens, "'false' expected");
    
    Good(Expression::ConstantBoolean(false), parsed_tokens)
}

fn parse_identifier_expression(tokens: &mut Vec<Token>) -> ParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();

    let name = expect_token!([Token::Identifier(name), Token::Identifier(name.clone()), name]
        <= tokens, parsed_tokens, "identifier expected");
    
    Good(Expression::Identifier(name), parsed_tokens)
}

fn parse_this_expression(tokens: &mut Vec<Token>) -> ParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();

    expect_token!([Token::This, Token::This, ()]
        <= tokens, parsed_tokens, "'this' expected");
    
    Good(Expression::ThisExpression, parsed_tokens)
}

fn parse_new_newarray_expression(tokens: &mut Vec<Token>) -> ParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();

    expect_token!([Token::New, Token::New, ()]
        <= tokens, parsed_tokens, "'new' expected");

    let result = match tokens.last() {
        Some(&Token::Int) => parse_newarray_expression(tokens), 
        Some(&Token::Identifier(_)) => parse_new_expression(tokens), 
        None => return NotComplete,
        _ => error("unknown token when expecting a new expression")
    };

    match result {
        Good(expr, toks) => {
            parsed_tokens.extend(toks.into_iter());
            Good(expr, parsed_tokens)
        },
        NotComplete => {
            parsed_tokens.reverse();
            tokens.extend(parsed_tokens.into_iter());
            return NotComplete
        },
        Bad(message) => return Bad(message)
    }
}

fn parse_newarray_expression(tokens: &mut Vec<Token>) -> ParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();

    expect_token!([Token::Int, Token::Int, ()]
        <= tokens, parsed_tokens, "'int' expected");
    
    expect_token!([Token::OpBracket, Token::OpBracket, ()]
        <= tokens, parsed_tokens, "'[' expected in constructing a integer array");

    let expr = match parse_expression(tokens) {
        Good(expr, toks) => {
            parsed_tokens.extend(toks.into_iter());
            expr
        },
        NotComplete => {
            parsed_tokens.reverse();
            tokens.extend(parsed_tokens.into_iter());
            return NotComplete
        },
        Bad(message) => return Bad(message)
    };

    expect_token!([Token::ClBracket, Token::ClBracket, ()]
        <= tokens, parsed_tokens, "']' expected in constructing a integer array");

    Good(Expression::NewArrayExpression(Box::new(expr)), parsed_tokens)
}

fn parse_new_expression(tokens: &mut Vec<Token>) -> ParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();

    let name = expect_token!([Token::Identifier(name), Token::Identifier(name.clone()), name]
        <= tokens, parsed_tokens, "identifier expected");
    
    expect_token!([Token::OpParethesis, Token::OpParethesis, ()]
        <= tokens, parsed_tokens, "'(' expected in constructing object");

    expect_token!([Token::ClParethesis, Token::ClParethesis, ()]
        <= tokens, parsed_tokens, "')' expected in constructing object");
    
    Good(Expression::NewExpression(name), parsed_tokens)
}

// fn parse_identifier(tokens: &mut Vec<Token>) -> ParsingResult<Identifier> { 
    // let mut parsed_tokens = Vec::new();

    // let name = expect_token!([Token::Identifier(name), Token::Identifier(name.clone()), name]
        // <= tokens, parsed_tokens, "identifier expected");
    
    // Good(Identifier{name: name}, parsed_tokens)
// }

// fn parse_not_expression(tokens: &mut Vec<Token>) -> ParsingResult<Expression> {
    // let mut parsed_tokens = Vec::new();

    // expect_token!([Token::Not, Token::Not, ()]
        // <= tokens, parsed_tokens, "'!' expected");

    // let expr = match parse_expression(tokens) {
        // Good(expr, toks) => {
            // parsed_tokens.extend(toks.into_iter());
            // expr
        // },
        // NotComplete => {
            // parsed_tokens.reverse();
            // tokens.extend(parsed_tokens.into_iter());
            // return NotComplete
        // },
        // Bad(message) => return Bad(message)
    // };

    // ParsingResult::Good(Expression::NotExpression(Box::new(expr)), parsed_tokens)
// }
