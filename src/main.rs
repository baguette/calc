use std::io;
use std::io::prelude::*;


/**********************************************************************
 * LEXER
 */

#[derive(Clone,PartialEq,Eq,Debug)]
enum Token {
  Num(String),
  Plus,
  Minus,
  Times,
  Divide,
  LParen,
  RParen,
  Semi
}

struct Lexer {
  buffer: String,
  offset: usize
}

impl Lexer {
  fn new() -> Lexer {
    let mut buffer = String::new();
    let offset = 0;
    io::stdin().read_line(&mut buffer).unwrap_or_else(|_| {
      panic!("Could not initialize lexer:  unable to read line");
    });

    Lexer {
      buffer: buffer,
      offset: offset
    }
  }

  fn advance(&mut self) {
    if self.offset >= self.buffer.len() - 1 {
      self.buffer = String::new();
      io::stdin().read_line(&mut self.buffer).unwrap_or_else(|_| {
        panic!("Could not advance lexer:  unable to read line");
      });
      self.offset = 0;
    } else {
      self.offset += self.current().len_utf8();
    }
  }

  fn current(&self) -> char {
    self.buffer[self.offset..].chars().next()
      .expect("Tried to get a nonsensical character")
  }

  fn get_token(&mut self) -> Token {
    let mut t = String::new();
    let mut c = self.current();

    while c.is_whitespace() {
      self.advance();
      c = self.current();
    }

    while c.is_digit(10) {
      t.push(c);
      self.advance();
      c = self.current();
    }

    if t.len() > 0 {
      return Token::Num(t);
    }

    self.advance();

    match c {
      '+' => Token::Plus,
      '-' => Token::Minus,
      '*' => Token::Times,
      '/' => Token::Divide,
      '(' => Token::LParen,
      ')' => Token::RParen,
      ';' => Token::Semi,
      x => panic!("unrecognized character: {}", x)
    }
  }
}



/**********************************************************************
 * PARSER / INTERPRETER
 */

struct Parser<'a> {
  tok : Token,
  lex : &'a mut Lexer,
  stack : Vec<i32>
}

impl<'a> Parser<'a> {
  fn new(lex : &'a mut Lexer) -> Parser {
    Parser {
      tok : lex.get_token(),
      lex : lex,
      stack : Vec::new()
    }
  }

  fn get_token(&mut self) {
    self.tok = self.lex.get_token();
  }

  fn eat(&mut self, t : Token) {
    if self.tok == t {
      self.get_token();
    } else {
      panic!("Syntax error: expected {:?}, found {:?}", t, self.tok);
    }
  }
  
  /********************************************************************
   * GRAMMAR PRODUCTIONS
   * x_ productions are hacks to make the grammar right recursive
   * and therefore suitable for recursive descent parsing
   */
  // Starting production. Use this as entry into the parser.
  fn program(&mut self) {
    self.exp();
    self.semi();
  }

  fn exp(&mut self) {
    self.term();
    self.exp_();
  }

  fn exp_(&mut self) {
    match self.tok {
      Token::Plus  => { self.eat(Token::Plus);  self.term();
                        let x = self.stack.pop().unwrap();
                        let y = self.stack.pop().unwrap();
                        self.stack.push(y + x);
                        self.exp_(); },
      Token::Minus => { self.eat(Token::Minus); self.term();
                        let x = self.stack.pop().unwrap();
                        let y = self.stack.pop().unwrap();
                        self.stack.push(y - x);
                        self.exp_(); },
      _ => { return; }
    }
  }

  fn term(&mut self) {
    self.factor();
    self.term_();
  }

  fn term_(&mut self) {
    match self.tok {
      Token::Times  => { self.eat(Token::Times);  self.factor();
                         let x = self.stack.pop().unwrap();
                         let y = self.stack.pop().unwrap();
                         self.stack.push(y * x);
                         self.term_(); },
      Token::Divide => { self.eat(Token::Divide); self.factor();
                         let x = self.stack.pop().unwrap();
                         let y = self.stack.pop().unwrap();
                         self.stack.push(y / x);
                         self.term_(); },
      _ => { return; }
    }
  }

  fn factor(&mut self) {
    let tok = self.tok.clone();  // Make the borrow checker stop complaining
    match tok {
      Token::Num(ref x) => { self.stack.push(x.parse::<i32>().unwrap());
                             self.get_token() } ,
      Token::LParen => { self.eat(Token::LParen);
                         self.exp();
                         self.eat(Token::RParen) } ,
      _ => { panic!("Syntax error: expected number or parenthesis") }
    }
  }

  // Terminating production. Forces calculation and ends the program.
  fn semi(&mut self) {
    match self.tok {
      Token::Semi => { println!("{}", self.stack.pop().unwrap()) },
      _ => { panic!("Syntax error: expected semicolon") }
    }
  }
}



/**********************************************************************
 * MAIN
 */
 
fn main() {
  let mut lexer = Lexer::new();
  let mut parser = Parser::new(&mut lexer);

  parser.program();
}
