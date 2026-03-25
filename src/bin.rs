/*! mod | v26.3.20 */
#![allow
(
    dead_code,
    static_mut_refs,
    unused_attributes,
    unused_imports,
    unused_unsafe,
    unused_variables,
)]

#![feature
(

)]
/*
pub mod _
{
    pub use std::_::{*};
}

pub mod _
{
    /*!*/
    use crate::
    {
        *,
    };
}
*/
use std::io::BufRead;

pub static mut ARGUMENTS:Vec<String> = vec![];
pub static mut PREVIOUS_ARGUMENT:Option<String> = None;
pub static mut CURRENT_ARGUMENT:Option<String> = None;

pub static mut MODULES:Vec<String> = vec![];
pub static mut PREVIOUS_MODULES:Option<String> = None;
pub static mut CURRENT_MODULES:Option<String> = None;

pub static mut INPUT_FROM:Option<String> = None;
pub static mut OUTPUT_TO:Option<String> = None;

pub static mut PATHS:Vec<String> = vec![];
pub static mut PREVIOUS_PATH:Option<String> = None;
pub static mut CURRENT_PATH:Option<String> = None;

pub static mut LINES:Vec<String> = vec![];
pub static mut BUFFER:Vec<String> = vec![];

pub mod borrow
{
    pub use std::borrow::{*};
}

pub mod boxed
{
    pub use std::boxed::{*};
}

pub mod cell
{
    pub use std::cell::{*};
}

pub mod cmp
{
    pub use std::cmp::{*};
}

pub mod collections
{
    pub use std::collections::{*};
}

pub mod env
{
    pub use std::env::{*};
}

pub mod error
{
    pub use std::error::{*};
}

pub mod fmt
{
    pub use std::fmt::{*};
}

pub mod fs
{
    pub use std::fs::{*};
}

pub mod io
{
    pub use std::io::{*};
}

pub mod isize
{
    pub use std::isize::{*};
}

pub mod iter
{
    pub use std::iter::{*};
}

pub mod is
{
    /*!
    */
    use crate::
    {
        *,
    };
    /*
    */
    // fn is_meta_character(c: char) -> bool
    pub fn meta_character(c: char) -> bool
    {
        match c
        {
            '\\' | '.' | '+' | '*' | '?' | '(' | ')' | '|' | '[' | ']' | '{'
            | '}' | '^' | '$' | '#' | '&' | '-' | '~' => true,
            _ => false,
        }
    }
    // fn is_escapable_character(c: char) -> bool
    pub fn escapable_character(c: char) -> bool
    {
        if crate::is::meta_character(c) { return true; }
        
        if !c.is_ascii() { return false; }
        
        match c
        {
            '0'..='9' | 'A'..='Z' | 'a'..='z' => false,
            '<' | '>' => false,
            _ => true,
        }
    }
    // fn is_valid_cap_letter(b: u8) -> bool
    pub fn valid_cap_letter(b: u8) -> bool
    {
        match b
        {
            b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z' | b'_' => true,
            _ => false,
        }
    }
}

pub mod mem
{
    pub use std::mem::{*};
}

pub mod num
{
    pub use std::num::{*};
}

pub mod ops
{
    pub use std::ops::{*};
}

pub mod panic
{
    pub use std::panic::{*};
}

pub mod path
{
    pub use std::path::{*};
}

pub mod regex
{
    /*!
    Provides a **lightweight** regex engine for searching strings. */
    use crate::
    {
        *,
    };
    /*
    */
    pub mod error
    {
        /*!
        */
        use crate::
        {
            *,
        };
        /*
        */
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct Error
        {
            msg: &'static str,
        }

        impl Error {
            pub fn new(msg: &'static str) -> Error {
                Error { msg }
            }
        }
        
        impl crate::error::Error for Error {}

        impl crate::fmt::Display for Error 
        {
            fn fmt(&self, f: &mut crate::fmt::Formatter) -> crate::fmt::Result 
            {
                write!(f, "{}", self.msg)
            }
        }

    }
    pub use self::error::*;
    
    pub mod hir
    {
        /*!
        */
        use crate::
        {
            boxed::{ Box },
            regex::{ error::{ Error }, utf8 },
            string::{ String },
            vec::{ self, Vec },
            *,
        };
        /*
        */
        mod parse
        {
            /*!
            */
            use crate::
            {
                boxed::{ Box },
                cell::{Cell, RefCell},
                regex::
                {
                    error::Error,
                    hir::{self, Config, Flags, Hir, HirKind},
                },
                string::{String, ToString},
                vec::{ self, Vec },
                *,
            };
            /*
            */
            const ERR_TOO_MUCH_NESTING: &str = "pattern has too much nesting";
            const ERR_TOO_MANY_CAPTURES: &str = "too many capture groups";
            const ERR_DUPLICATE_CAPTURE_NAME: &str = "duplicate capture group name";
            const ERR_UNCLOSED_GROUP: &str = "found open group without closing ')'";
            const ERR_UNCLOSED_GROUP_QUESTION: &str = "expected closing ')', but got end of pattern";
            const ERR_UNOPENED_GROUP: &str = "found closing ')' without matching '('";
            const ERR_LOOK_UNSUPPORTED: &str = "look-around is not supported";
            const ERR_EMPTY_FLAGS: &str = "empty flag directive '(?)' is not allowed";
            const ERR_MISSING_GROUP_NAME: &str = "expected capture group name, but got end of pattern";
            const ERR_INVALID_GROUP_NAME: &str = "invalid group name";
            const ERR_UNCLOSED_GROUP_NAME: &str = "expected end of capture group name, but got end of pattern";
            const ERR_EMPTY_GROUP_NAME: &str = "empty capture group names are not allowed";
            const ERR_FLAG_UNRECOGNIZED: &str = "unrecognized inline flag";
            const ERR_FLAG_REPEATED_NEGATION: &str = "inline flag negation cannot be repeated";
            const ERR_FLAG_DUPLICATE: &str = "duplicate inline flag is not allowed";
            const ERR_FLAG_UNEXPECTED_EOF: &str = "expected ':' or ')' to end inline flags, but got end of pattern";
            const ERR_FLAG_DANGLING_NEGATION: &str = "inline flags cannot end with negation directive";
            const ERR_DECIMAL_NO_DIGITS: &str = "expected decimal number, but found no digits";
            const ERR_DECIMAL_INVALID: &str = "got invalid decimal number";
            const ERR_HEX_BRACE_INVALID_DIGIT: &str = "expected hexadecimal number in braces, but got non-hex digit";
            const ERR_HEX_BRACE_UNEXPECTED_EOF: &str = "expected hexadecimal number, but saw end of pattern before closing brace";
            const ERR_HEX_BRACE_EMPTY: &str = "expected hexadecimal number in braces, but got no digits";
            const ERR_HEX_BRACE_INVALID: &str = "got invalid hexadecimal number in braces";
            const ERR_HEX_FIXED_UNEXPECTED_EOF: &str = "expected fixed length hexadecimal number, but saw end of pattern first";
            const ERR_HEX_FIXED_INVALID_DIGIT: &str = "expected fixed length hexadecimal number, but got non-hex digit";
            const ERR_HEX_FIXED_INVALID: &str = "got invalid fixed length hexadecimal number";
            const ERR_HEX_UNEXPECTED_EOF: &str = "expected hexadecimal number, but saw end of pattern first";
            const ERR_ESCAPE_UNEXPECTED_EOF: &str = "saw start of escape sequence, but saw end of pattern before it finished";
            const ERR_BACKREF_UNSUPPORTED: &str = "backreferences are not supported";
            const ERR_UNICODE_CLASS_UNSUPPORTED: &str = "Unicode character classes are not supported";
            const ERR_ESCAPE_UNRECOGNIZED: &str = "unrecognized escape sequence";
            const ERR_POSIX_CLASS_UNRECOGNIZED: &str = "unrecognized POSIX character class";
            const ERR_UNCOUNTED_REP_SUB_MISSING: &str = "uncounted repetition operator must be applied to a sub-expression";
            const ERR_COUNTED_REP_SUB_MISSING: &str = "counted repetition operator must be applied to a sub-expression";
            const ERR_COUNTED_REP_UNCLOSED: &str = "found unclosed counted repetition operator";
            const ERR_COUNTED_REP_MIN_UNCLOSED: &str = "found incomplete and unclosed counted repetition operator";
            const ERR_COUNTED_REP_COMMA_UNCLOSED: &str = "found counted repetition operator with a comma that is unclosed";
            const ERR_COUNTED_REP_MIN_MAX_UNCLOSED: &str = "found counted repetition with min and max that is unclosed";
            const ERR_COUNTED_REP_INVALID: &str = "expected closing brace for counted repetition, but got something else";
            const ERR_COUNTED_REP_INVALID_RANGE: &str = "found counted repetition with a min bigger than its max";
            const ERR_CLASS_UNCLOSED_AFTER_ITEM: &str = "non-empty character class has no closing bracket";
            const ERR_CLASS_INVALID_RANGE_ITEM: &str = "character class ranges must start and end with a single character";
            const ERR_CLASS_INVALID_ITEM: &str = "invalid escape sequence in character class";
            const ERR_CLASS_UNCLOSED_AFTER_DASH: &str = "non-empty character class has no closing bracket after dash";
            const ERR_CLASS_UNCLOSED_AFTER_NEGATION: &str = "negated character class has no closing bracket";
            const ERR_CLASS_UNCLOSED_AFTER_CLOSING: &str = "character class begins with literal ']' but has no closing bracket";
            const ERR_CLASS_INVALID_RANGE: &str = "invalid range in character class";
            const ERR_CLASS_UNCLOSED: &str = "found unclosed character class";
            const ERR_CLASS_NEST_UNSUPPORTED: &str = "nested character classes are not supported";
            const ERR_CLASS_INTERSECTION_UNSUPPORTED: &str = "character class intersection is not supported";
            const ERR_CLASS_DIFFERENCE_UNSUPPORTED: &str = "character class difference is not supported";
            const ERR_CLASS_SYMDIFFERENCE_UNSUPPORTED: &str = "character class symmetric difference is not supported";
            const ERR_SPECIAL_WORD_BOUNDARY_UNCLOSED: &str = "special word boundary assertion is unclosed or has an invalid character";
            const ERR_SPECIAL_WORD_BOUNDARY_UNRECOGNIZED: &str = "special word boundary assertion is unrecognized";
            const ERR_SPECIAL_WORD_OR_REP_UNEXPECTED_EOF: &str = "found start of special word boundary or repetition without an end";
            
            #[derive(Clone, Debug)]
            pub struct Parser<'a>
            {
                config: Config,pattern: &'a str,                
                depth: Cell<u32>,pos: Cell<usize>,
                char: Cell<Option<char>>,capture_index: Cell<u32>,flags: RefCell<Flags>,
                capture_names: RefCell<Vec<String>>,
            }
            
            impl<'a> Parser<'a> 
            {
                pub fn new(config: Config, pattern: &'a str) -> Parser<'a> 
                {
                    Parser 
                    {
                        config,
                        pattern,
                        depth: Cell::new(0),
                        pos: Cell::new(0),
                        char: Cell::new(pattern.chars().next()),
                        capture_index: Cell::new(0),
                        flags: RefCell::new(config.flags),
                        capture_names: RefCell::new(vec![]),
                    }
                }
                
                fn pattern(&self) -> &str { self.pattern }               
                fn pos(&self) -> usize { self.pos.get() }
                
                fn increment_depth(&self) -> Result<u32, Error>
                {
                    let old = self.depth.get();
                    if old > self.config.nest_limit {
                        return Err(Error::new(ERR_TOO_MUCH_NESTING));
                    }
                    
                    let new = old.checked_add(1).unwrap();
                    self.depth.set(new);
                    Ok(old)
                }
                
                fn decrement_depth(&self)
                {
                    let old = self.depth.get();
                    
                    let new = old.checked_sub(1).unwrap();
                    self.depth.set(new);
                }
                
                fn char(&self) -> char { self.char.get().expect("codepoint, but parser is done") }                
                fn is_done(&self) -> bool { self.pos() == self.pattern.len() }                
                fn flags(&self) -> Flags { *self.flags.borrow() }
                fn bump(&self) -> bool
                {
                    if self.is_done() { return false; }
                    
                    self.pos.set(self.pos() + self.char().len_utf8());
                    self.char.set(self.pattern()[self.pos()..].chars().next());
                    self.char.get().is_some()
                }
                
                fn bump_if(&self, prefix: &str) -> bool
                {
                    if self.pattern()[self.pos()..].starts_with(prefix) {
                        for _ in 0..prefix.chars().count() {
                            self.bump();
                        }
                        true
                    } else {
                        false
                    }
                }
                
                fn bump_and_bump_space(&self) -> bool {
                    if !self.bump() {
                        return false;
                    }
                    self.bump_space();
                    !self.is_done()
                }

                fn bump_space(&self) {
                    if !self.flags().ignore_whitespace {
                        return;
                    }
                    
                    while !self.is_done() {
                        if self.char().is_whitespace() {
                            self.bump();
                        } else if self.char() == '#' {
                            self.bump();
                            while !self.is_done() {
                                let c = self.char();
                                self.bump();
                                if c == '\n' {
                                    break;
                                }
                            }
                        } else {
                            break;
                        }
                    }
                }
                
                fn peek(&self) -> Option<char> {
                    if self.is_done() {
                        return None;
                    }
                    self.pattern()[self.pos() + self.char().len_utf8()..].chars().next()
                }
                
                fn peek_space(&self) -> Option<char> {
                    if !self.flags().ignore_whitespace {
                        return self.peek();
                    }
                    
                    if self.is_done() {
                        return None;
                    }
                    
                    let mut start = self.pos() + self.char().len_utf8();
                    let mut in_comment = false;
                    for (i, ch) in self.pattern()[start..].char_indices() {
                        if ch.is_whitespace() {
                            continue;
                        } else if !in_comment && ch == '#' {
                            in_comment = true;
                        } else if in_comment && ch == '\n' {
                            in_comment = false;
                        } else {
                            start += i;
                            break;
                        }
                    }
                    self.pattern()[start..].chars().next()
                }

                fn next_capture_index(&self) -> Result<u32, Error> {
                    let current = self.capture_index.get();
                    let next = current
                        .checked_add(1)
                        .ok_or_else(|| Error::new(ERR_TOO_MANY_CAPTURES))?;
                    self.capture_index.set(next);
                    Ok(next)
                }
                
                fn add_capture_name(&self, name: &str) -> Result<(), Error> {
                    let mut names = self.capture_names.borrow_mut();
                    match names.binary_search_by(|n| name.cmp(n)) {
                        Ok(_) => Err(Error::new(ERR_DUPLICATE_CAPTURE_NAME)),
                        Err(i) => {
                            names.insert(i, name.to_string());
                            Ok(())
                        }
                    }
                }

                fn is_lookaround_prefix(&self) -> bool {
                    self.bump_if("?=")
                        || self.bump_if("?!")
                        || self.bump_if("?<=")
                        || self.bump_if("?<!")
                }
            }

            impl<'a> Parser<'a> {
                pub fn parse(&self) -> Result<Hir, Error> {
                    let hir = self.parse_inner()?;
                    check_hir_nesting(&hir, self.config.nest_limit)?;
                    Ok(hir)
                }

                fn parse_inner(&self) -> Result<Hir, Error> {
                    let depth = self.increment_depth()?;
                    let mut alternates = vec![];
                    let mut concat = vec![];
                    loop {
                        self.bump_space();
                        if self.is_done() {
                            break;
                        }
                        match self.char() {
                            '(' => {
                                let oldflags = *self.flags.borrow();
                                if let Some(sub) = self.parse_group()? {
                                    concat.push(sub);
                                    *self.flags.borrow_mut() = oldflags;
                                }
                                if self.char.get() != Some(')') {
                                    return Err(Error::new(ERR_UNCLOSED_GROUP));
                                }
                                self.bump();
                            }
                            ')' => {
                                if depth == 0 {
                                    return Err(Error::new(ERR_UNOPENED_GROUP));
                                }
                                break;
                            }
                            '|' => {
                                alternates.push(Hir::concat(core::mem::take(&mut concat)));
                                self.bump();
                            }
                            '[' => concat.push(self.parse_class()?),
                            '?' | '*' | '+' => {
                                concat = self.parse_uncounted_repetition(concat)?;
                            }
                            '{' => {
                                concat = self.parse_counted_repetition(concat)?;
                            }
                            _ => concat.push(self.parse_primitive()?),
                        }
                    }
                    self.decrement_depth();
                    alternates.push(Hir::concat(concat));
                    Ok(Hir::alternation(alternates))
                }
               
                fn parse_primitive(&self) -> Result<Hir, Error> {
                    let ch = self.char();
                    self.bump();
                    match ch {
                        '\\' => self.parse_escape(),
                        '.' => Ok(self.hir_dot()),
                        '^' => Ok(self.hir_anchor_start()),
                        '$' => Ok(self.hir_anchor_end()),
                        ch => Ok(self.hir_char(ch)),
                    }
                }
                
                fn parse_escape(&self) -> Result<Hir, Error> {
                    if self.is_done() {
                        return Err(Error::new(ERR_ESCAPE_UNEXPECTED_EOF));
                    }
                    
                    let ch = self.char();
                    match ch {
                        '0'..='9' => return Err(Error::new(ERR_BACKREF_UNSUPPORTED)),
                        'p' | 'P' => {
                            return Err(Error::new(ERR_UNICODE_CLASS_UNSUPPORTED))
                        }
                        'x' | 'u' | 'U' => return self.parse_hex(),
                        'd' | 's' | 'w' | 'D' | 'S' | 'W' => {
                            return Ok(self.parse_perl_class());
                        }
                        _ => {}
                    }
                    
                    self.bump();
                    if is::meta_character(ch) || is::escapable_character(ch) {
                        return Ok(self.hir_char(ch));
                    }
                    
                    let special = |ch| Ok(self.hir_char(ch));
                    match ch {
                        'a' => special('\x07'),
                        'f' => special('\x0C'),
                        't' => special('\t'),
                        'n' => special('\n'),
                        'r' => special('\r'),
                        'v' => special('\x0B'),
                        'A' => Ok(Hir::look(hir::Look::Start)),
                        'z' => Ok(Hir::look(hir::Look::End)),
                        'b' => {
                            let mut hir = Hir::look(hir::Look::Word);
                            if !self.is_done() && self.char() == '{' {
                                if let Some(special) =
                                    self.maybe_parse_special_word_boundary()?
                                {
                                    hir = special;
                                }
                            }
                            Ok(hir)
                        }
                        'B' => Ok(Hir::look(hir::Look::WordNegate)),
                        '<' => Ok(Hir::look(hir::Look::WordStart)),
                        '>' => Ok(Hir::look(hir::Look::WordEnd)),
                        _ => Err(Error::new(ERR_ESCAPE_UNRECOGNIZED)),
                    }
                }
                
                fn maybe_parse_special_word_boundary(&self) -> Result<Option<Hir>, Error> {
                    assert_eq!(self.char(), '{');

                    let is_valid_char = |c| match c {
                        'A'..='Z' | 'a'..='z' | '-' => true,
                        _ => false,
                    };
                    let start = self.pos();
                    if !self.bump_and_bump_space() {
                        return Err(Error::new(ERR_SPECIAL_WORD_OR_REP_UNEXPECTED_EOF));
                    }
                    
                    if !is_valid_char(self.char()) {
                        self.pos.set(start);
                        self.char.set(Some('{'));
                        return Ok(None);
                    }
                    
                    let mut scratch = String::new();
                    while !self.is_done() && is_valid_char(self.char()) {
                        scratch.push(self.char());
                        self.bump_and_bump_space();
                    }
                    
                    if self.is_done() || self.char() != '}' {
                        return Err(Error::new(ERR_SPECIAL_WORD_BOUNDARY_UNCLOSED));
                    }
                    self.bump();
                    let kind = match scratch.as_str() {
                        "start" => hir::Look::WordStart,
                        "end" => hir::Look::WordEnd,
                        "start-half" => hir::Look::WordStartHalf,
                        "end-half" => hir::Look::WordEndHalf,
                        _ => {
                            return Err(Error::new(ERR_SPECIAL_WORD_BOUNDARY_UNRECOGNIZED))
                        }
                    };
                    Ok(Some(Hir::look(kind)))
                }
                
                fn parse_hex(&self) -> Result<Hir, Error> {
                    let digit_len = match self.char() {
                        'x' => 2,
                        'u' => 4,
                        'U' => 8,
                        unk => unreachable!(
                            "invalid start of fixed length hexadecimal number {unk}"
                        ),
                    };
                    
                    if  !self.bump_and_bump_space() {
                        return Err(Error::new(ERR_HEX_UNEXPECTED_EOF));
                    }
                    
                    if self.char() == '{' {
                        self.parse_hex_brace()
                    } else {
                        self.parse_hex_digits(digit_len)
                    }
                }
                
                fn parse_hex_digits(&self, digit_len: usize) -> Result<Hir, Error> {
                    let mut scratch = String::new();
                    for i in 0..digit_len {
                        if i > 0 && !self.bump_and_bump_space() {
                            return Err(Error::new(ERR_HEX_FIXED_UNEXPECTED_EOF));
                        }
                        if !is_hex(self.char()) {
                            return Err(Error::new(ERR_HEX_FIXED_INVALID_DIGIT));
                        }
                        scratch.push(self.char());
                    }
                    
                    self.bump_and_bump_space();
                    match u32::from_str_radix(&scratch, 16).ok().and_then(char::from_u32) {
                        None => Err(Error::new(ERR_HEX_FIXED_INVALID)),
                        Some(ch) => Ok(self.hir_char(ch)),
                    }
                }
                
                fn parse_hex_brace(&self) -> Result<Hir, Error> {
                    let mut scratch = String::new();
                    while self.bump_and_bump_space() && self.char() != '}' {
                        if !is_hex(self.char()) {
                            return Err(Error::new(ERR_HEX_BRACE_INVALID_DIGIT));
                        }
                        scratch.push(self.char());
                    }
                    
                    if self.is_done() {
                        return Err(Error::new(ERR_HEX_BRACE_UNEXPECTED_EOF));
                    }
                    assert_eq!(self.char(), '}');
                    self.bump_and_bump_space();

                    if scratch.is_empty() {
                        return Err(Error::new(ERR_HEX_BRACE_EMPTY));
                    }
                    match u32::from_str_radix(&scratch, 16).ok().and_then(char::from_u32) {
                        None => Err(Error::new(ERR_HEX_BRACE_INVALID)),
                        Some(ch) => Ok(self.hir_char(ch)),
                    }
                }
                
                fn parse_decimal(&self) -> Result<u32, Error> {
                    let mut scratch = String::new();
                    while !self.is_done() && self.char().is_whitespace() {
                        self.bump();
                    }
                    
                    while !self.is_done() && '0' <= self.char() && self.char() <= '9' {
                        scratch.push(self.char());
                        self.bump_and_bump_space();
                    }
                    
                    while !self.is_done() && self.char().is_whitespace() {
                        self.bump_and_bump_space();
                    }
                    
                    let digits = scratch.as_str();
                    if digits.is_empty() {
                        return Err(Error::new(ERR_DECIMAL_NO_DIGITS));
                    }
                    match u32::from_str_radix(digits, 10).ok() {
                        Some(n) => Ok(n),
                        None => Err(Error::new(ERR_DECIMAL_INVALID)),
                    }
                }
                
                fn parse_uncounted_repetition(
                    &self,
                    mut concat: Vec<Hir>,
                ) -> Result<Vec<Hir>, Error> {
                    let sub = match concat.pop() {
                        Some(hir) => Box::new(hir),
                        None => {
                            return Err(Error::new(ERR_UNCOUNTED_REP_SUB_MISSING));
                        }
                    };
                    let (min, max) = match self.char() {
                        '?' => (0, Some(1)),
                        '*' => (0, None),
                        '+' => (1, None),
                        unk => unreachable!("unrecognized repetition operator '{unk}'"),
                    };
                    let mut greedy = true;
                    if self.bump() && self.char() == '?' {
                        greedy = false;
                        self.bump();
                    }
                    
                    if self.flags().swap_greed {
                        greedy = !greedy;
                    }
                    concat.push(Hir::repetition(hir::Repetition {
                        min,
                        max,
                        greedy,
                        sub,
                    }));
                    Ok(concat)
                }
                
                fn parse_counted_repetition(
                    &self,
                    mut concat: Vec<Hir>,
                ) -> Result<Vec<Hir>, Error> {
                    assert_eq!(self.char(), '{', "expected opening brace");
                    let sub = match concat.pop() {
                        Some(hir) => Box::new(hir),
                        None => {
                            return Err(Error::new(ERR_COUNTED_REP_SUB_MISSING));
                        }
                    };
                    
                    if !self.bump_and_bump_space() {
                        return Err(Error::new(ERR_COUNTED_REP_UNCLOSED));
                    }
                    
                    let min = self.parse_decimal()?;
                    let mut max = Some(min);
                    if self.is_done() {
                        return Err(Error::new(ERR_COUNTED_REP_MIN_UNCLOSED));
                    }
                    
                    if self.char() == ',' {
                        if !self.bump_and_bump_space() {
                            return Err(Error::new(ERR_COUNTED_REP_COMMA_UNCLOSED));
                        }
                        if self.char() != '}' {
                            max = Some(self.parse_decimal()?);
                        } else {
                            max = None;
                        }
                        if self.is_done() {
                            return Err(Error::new(ERR_COUNTED_REP_MIN_MAX_UNCLOSED));
                        }
                    }
                    
                    if self.char() != '}' {
                        return Err(Error::new(ERR_COUNTED_REP_INVALID));
                    }

                    let mut greedy = true;
                    if self.bump_and_bump_space() && self.char() == '?' {
                        greedy = false;
                        self.bump();
                    }
                    
                    if self.flags().swap_greed {
                        greedy = !greedy;
                    }

                    if max.map_or(false, |max| min > max) {
                        return Err(Error::new(ERR_COUNTED_REP_INVALID_RANGE));
                    }
                    concat.push(Hir::repetition(hir::Repetition {
                        min,
                        max,
                        greedy,
                        sub,
                    }));
                    Ok(concat)
                }
                
                fn parse_group(&self) -> Result<Option<Hir>, Error> {
                    assert_eq!(self.char(), '(');
                    self.bump_and_bump_space();
                    if self.is_lookaround_prefix() {
                        return Err(Error::new(ERR_LOOK_UNSUPPORTED));
                    }
                    
                    if self.bump_if("?P<") || self.bump_if("?<") {
                        let index = self.next_capture_index()?;
                        let name = Some(Box::from(self.parse_capture_name()?));
                        let sub = Box::new(self.parse_inner()?);
                        let cap = hir::Capture { index, name, sub };
                        Ok(Some(Hir::capture(cap)))
                    } else if self.bump_if("?") {
                        if self.is_done() {
                            return Err(Error::new(ERR_UNCLOSED_GROUP_QUESTION));
                        }
                        let start = self.pos();
                        *self.flags.borrow_mut() = self.parse_flags()?;
                        let consumed = self.pos() - start;
                        if self.char() == ')' {
                            if consumed == 0 {
                                return Err(Error::new(ERR_EMPTY_FLAGS));
                            }
                            Ok(None)
                        } else {
                            assert_eq!(':', self.char());
                            self.bump();
                            self.parse_inner().map(Some)
                        }
                    } else {
                        let index = self.next_capture_index()?;
                        let sub = Box::new(self.parse_inner()?);
                        let cap = hir::Capture { index, name: None, sub };
                        Ok(Some(Hir::capture(cap)))
                    }
                }
                
                fn parse_capture_name(&self) -> Result<&str, Error> {
                    if self.is_done() {
                        return Err(Error::new(ERR_MISSING_GROUP_NAME));
                    }
                    
                    let start = self.pos();
                    loop {
                        if self.char() == '>' {
                            break;
                        }
                        if !is_capture_char(self.char(), self.pos() == start) {
                            return Err(Error::new(ERR_INVALID_GROUP_NAME));
                        }
                        if !self.bump() {
                            break;
                        }
                    }
                    
                    let end = self.pos();
                    if self.is_done() {
                        return Err(Error::new(ERR_UNCLOSED_GROUP_NAME));
                    }
                    assert_eq!(self.char(), '>');
                    self.bump();
                    let name = &self.pattern()[start..end];
                    if name.is_empty() {
                        return Err(Error::new(ERR_EMPTY_GROUP_NAME));
                    }
                    self.add_capture_name(name)?;
                    Ok(name)
                }
                
                fn parse_flags(&self) -> Result<Flags, Error> {
                    let mut flags = *self.flags.borrow();
                    let mut negate = false;
                    let mut last_was_negation = false;
                    let mut seen = [false; 128];
                    while self.char() != ':' && self.char() != ')' {
                        if self.char() == '-' {
                            last_was_negation = true;
                            if negate {
                                return Err(Error::new(ERR_FLAG_REPEATED_NEGATION));
                            }
                            negate = true;
                        } else {
                            last_was_negation = false;
                            self.parse_flag(&mut flags, negate)?;
                            let flag_byte = u8::try_from(self.char()).unwrap();
                            if seen[usize::from(flag_byte)] {
                                return Err(Error::new(ERR_FLAG_DUPLICATE));
                            }
                            seen[usize::from(flag_byte)] = true;
                        }
                        if !self.bump() {
                            return Err(Error::new(ERR_FLAG_UNEXPECTED_EOF));
                        }
                    }
                    
                    if last_was_negation {
                        return Err(Error::new(ERR_FLAG_DANGLING_NEGATION));
                    }
                    Ok(flags)
                }
                
                fn parse_flag(
                    &self,
                    flags: &mut Flags,
                    negate: bool,
                ) -> Result<(), Error> {
                    let enabled = !negate;
                    match self.char() {
                        'i' => flags.case_insensitive = enabled,
                        'm' => flags.multi_line = enabled,
                        's' => flags.dot_matches_new_line = enabled,
                        'U' => flags.swap_greed = enabled,
                        'R' => flags.crlf = enabled,
                        'x' => flags.ignore_whitespace = enabled,
                        'u' => {}
                        _ => return Err(Error::new(ERR_FLAG_UNRECOGNIZED)),
                    }
                    Ok(())
                }
                
                fn parse_class(&self) -> Result<Hir, Error> {
                    assert_eq!(self.char(), '[');

                    let mut union = vec![];
                    if !self.bump_and_bump_space() {
                        return Err(Error::new(ERR_CLASS_UNCLOSED));
                    }
                    
                    let negate = if self.char() != '^' {
                        false
                    } else {
                        if !self.bump_and_bump_space() {
                            return Err(Error::new(ERR_CLASS_UNCLOSED_AFTER_NEGATION));
                        }
                        true
                    };
                    
                    while self.char() == '-' {
                        union.push(hir::ClassRange { start: '-', end: '-' });
                        if !self.bump_and_bump_space() {
                            return Err(Error::new(ERR_CLASS_UNCLOSED_AFTER_DASH));
                        }
                    }
                    
                    if union.is_empty() && self.char() == ']' {
                        union.push(hir::ClassRange { start: ']', end: ']' });
                        if !self.bump_and_bump_space() {
                            return Err(Error::new(ERR_CLASS_UNCLOSED_AFTER_CLOSING));
                        }
                    }
                    loop {
                        self.bump_space();
                        if self.is_done() {
                            return Err(Error::new(ERR_CLASS_UNCLOSED));
                        }
                        match self.char() {
                            '[' => {
                                if let Some(class) = self.maybe_parse_posix_class() {
                                    union.extend_from_slice(&class.ranges);
                                    continue;
                                }

                                return Err(Error::new(ERR_CLASS_NEST_UNSUPPORTED));
                            }
                            ']' => {
                                self.bump();
                                let mut class = hir::Class::new(union);

                                if self.flags().case_insensitive {
                                    class.ascii_case_fold();
                                }
                                if negate {
                                    class.negate();
                                }
                                return Ok(Hir::class(class));
                            }
                            '&' if self.peek() == Some('&') => {
                                return Err(Error::new(
                                    ERR_CLASS_INTERSECTION_UNSUPPORTED,
                                ));
                            }
                            '-' if self.peek() == Some('-') => {
                                return Err(Error::new(ERR_CLASS_DIFFERENCE_UNSUPPORTED));
                            }
                            '~' if self.peek() == Some('~') => {
                                return Err(Error::new(
                                    ERR_CLASS_SYMDIFFERENCE_UNSUPPORTED,
                                ));
                            }
                            _ => self.parse_class_range(&mut union)?,
                        }
                    }
                }

                fn parse_class_range(
                    &self,
                    union: &mut Vec<hir::ClassRange>,
                ) -> Result<(), Error> {
                    let prim1 = self.parse_class_item()?;
                    self.bump_space();
                    if self.is_done() {
                        return Err(Error::new(ERR_CLASS_UNCLOSED_AFTER_ITEM));
                    }
                    
                    
                    if self.char() != '-'
                        || self.peek_space() == Some(']')
                        || self.peek_space() == Some('-')
                    {
                        union.extend_from_slice(&into_class_item_ranges(prim1)?);
                        return Ok(());
                    }
                    
                    if !self.bump_and_bump_space() {
                        return Err(Error::new(ERR_CLASS_UNCLOSED_AFTER_DASH));
                    }
                    
                    let prim2 = self.parse_class_item()?;
                    let range = hir::ClassRange {
                        start: into_class_item_range(prim1)?,
                        end: into_class_item_range(prim2)?,
                    };
                    
                    if  range.start > range.end {
                        return Err(Error::new(ERR_CLASS_INVALID_RANGE));
                    }
                    union.push(range);
                    Ok(())
                }
                
                fn parse_class_item(&self) -> Result<Hir, Error> {
                    let ch = self.char();
                    self.bump();
                    if ch == '\\' {
                        self.parse_escape()
                    } else {
                        Ok(Hir::char(ch))
                    }
                }
                
                fn maybe_parse_posix_class(&self) -> Option<hir::Class> {
                    assert_eq!(self.char(), '[');
                    
                    let start_pos = self.pos();
                    let start_char = self.char.get();
                    let reset = || {
                        self.pos.set(start_pos);
                        self.char.set(start_char);
                    };

                    let mut negated = false;
                    if !self.bump() || self.char() != ':' {
                        reset();
                        return None;
                    }
                    
                    if !self.bump() {
                        reset();
                        return None;
                    }
                    
                    if self.char() == '^' {
                        negated = true;
                        if !self.bump() {
                            reset();
                            return None;
                        }
                    }
                    
                    let name_start = self.pos();
                    while self.char() != ':' && self.bump() {}
                    
                    if self.is_done() {
                        reset();
                        return None;
                    }
                    
                    let name = &self.pattern()[name_start..self.pos()];
                    if !self.bump_if(":]") {
                        reset();
                        return None;
                    }
                    
                    if let Ok(ranges) = posix_class(name) {
                        let mut class = hir::Class::new(ranges);
                        if negated {
                            class.negate();
                        }
                        return Some(class);
                    }
                    reset();
                    None
                }
                
                fn parse_perl_class(&self) -> Hir {
                    let ch = self.char();
                    self.bump();
                    let mut class = hir::Class::new(match ch {
                        'd' | 'D' => posix_class("digit").unwrap(),
                        's' | 'S' => posix_class("space").unwrap(),
                        'w' | 'W' => posix_class("word").unwrap(),
                        unk => unreachable!("invalid Perl class \\{unk}"),
                    });
                    if ch.is_ascii_uppercase() {
                        class.negate();
                    }
                    Hir::class(class)
                }

                fn hir_dot(&self) -> Hir {
                    if self.flags().dot_matches_new_line {
                        Hir::class(hir::Class::new([hir::ClassRange {
                            start: '\x00',
                            end: '\u{10FFFF}',
                        }]))
                    } else if self.flags().crlf {
                        Hir::class(hir::Class::new([
                            hir::ClassRange { start: '\x00', end: '\x09' },
                            hir::ClassRange { start: '\x0B', end: '\x0C' },
                            hir::ClassRange { start: '\x0E', end: '\u{10FFFF}' },
                        ]))
                    } else {
                        Hir::class(hir::Class::new([
                            hir::ClassRange { start: '\x00', end: '\x09' },
                            hir::ClassRange { start: '\x0B', end: '\u{10FFFF}' },
                        ]))
                    }
                }

                fn hir_anchor_start(&self) -> Hir {
                    let look = if self.flags().multi_line {
                        if self.flags().crlf {
                            hir::Look::StartCRLF
                        } else {
                            hir::Look::StartLF
                        }
                    } else {
                        hir::Look::Start
                    };
                    Hir::look(look)
                }

                fn hir_anchor_end(&self) -> Hir {
                    let look = if self.flags().multi_line {
                        if self.flags().crlf {
                            hir::Look::EndCRLF
                        } else {
                            hir::Look::EndLF
                        }
                    } else {
                        hir::Look::End
                    };
                    Hir::look(look)
                }

                fn hir_char(&self, ch: char) -> Hir {
                    if self.flags().case_insensitive {
                        let this = hir::ClassRange { start: ch, end: ch };
                        if let Some(folded) = this.ascii_case_fold() {
                            return Hir::class(hir::Class::new([this, folded]));
                        }
                    }
                    Hir::char(ch)
                }
            }
            
            fn check_hir_nesting(hir: &Hir, limit: u32) -> Result<(), Error> {
                fn recurse(hir: &Hir, limit: u32, depth: u32) -> Result<(), Error> {
                    if depth > limit {
                        return Err(Error::new(ERR_TOO_MUCH_NESTING));
                    }
                    
                    let Some(next_depth) = depth.checked_add(1) else {
                        return Err(Error::new(ERR_TOO_MUCH_NESTING));
                    };
                    match *hir.kind() {
                        HirKind::Empty
                        | HirKind::Char(_)
                        | HirKind::Class(_)
                        | HirKind::Look(_) => Ok(()),
                        HirKind::Repetition(hir::Repetition { ref sub, .. }) => {
                            recurse(sub, limit, next_depth)
                        }
                        HirKind::Capture(hir::Capture { ref sub, .. }) => {
                            recurse(sub, limit, next_depth)
                        }
                        HirKind::Concat(ref subs) | HirKind::Alternation(ref subs) => {
                            for sub in subs.iter() {
                                recurse(sub, limit, next_depth)?;
                            }
                            Ok(())
                        }
                    }
                }
                recurse(hir, limit, 0)
            }
            
            fn into_class_item_range(hir: Hir) -> Result<char, Error> {
                match hir.kind {
                    HirKind::Char(ch) => Ok(ch),
                    _ => Err(Error::new(ERR_CLASS_INVALID_RANGE_ITEM)),
                }
            }

            fn into_class_item_ranges(
                mut hir: Hir,
            ) -> Result<Vec<hir::ClassRange>, Error> {
                match core::mem::replace(&mut hir.kind, HirKind::Empty) {
                    HirKind::Char(ch) => Ok(vec![hir::ClassRange { start: ch, end: ch }]),
                    HirKind::Class(hir::Class { ranges }) => Ok(ranges),
                    _ => Err(Error::new(ERR_CLASS_INVALID_ITEM)),
                }
            }
            
            fn posix_class(
                kind: &str,
            ) -> Result<impl Iterator<Item = hir::ClassRange>, Error> {
                let slice: &'static [(u8, u8)] = match kind {
                    "alnum" => &[(b'0', b'9'), (b'A', b'Z'), (b'a', b'z')],
                    "alpha" => &[(b'A', b'Z'), (b'a', b'z')],
                    "ascii" => &[(b'\x00', b'\x7F')],
                    "blank" => &[(b'\t', b'\t'), (b' ', b' ')],
                    "cntrl" => &[(b'\x00', b'\x1F'), (b'\x7F', b'\x7F')],
                    "digit" => &[(b'0', b'9')],
                    "graph" => &[(b'!', b'~')],
                    "lower" => &[(b'a', b'z')],
                    "print" => &[(b' ', b'~')],
                    "punct" => &[(b'!', b'/'), (b':', b'@'), (b'[', b'`'), (b'{', b'~')],
                    "space" => &[
                        (b'\t', b'\t'),
                        (b'\n', b'\n'),
                        (b'\x0B', b'\x0B'),
                        (b'\x0C', b'\x0C'),
                        (b'\r', b'\r'),
                        (b' ', b' '),
                    ],
                    "upper" => &[(b'A', b'Z')],
                    "word" => &[(b'0', b'9'), (b'A', b'Z'), (b'_', b'_'), (b'a', b'z')],
                    "xdigit" => &[(b'0', b'9'), (b'A', b'F'), (b'a', b'f')],
                    _ => return Err(Error::new(ERR_POSIX_CLASS_UNRECOGNIZED)),
                };
                Ok(slice.iter().map(|&(start, end)| hir::ClassRange {
                    start: char::from(start),
                    end: char::from(end),
                }))
            }
            
            fn is_hex(c: char) -> bool {
                ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
            }

            
            fn is_capture_char(c: char, first: bool) -> bool {
                if first {
                    c == '_' || c.is_alphabetic()
                } else {
                    c == '_' || c == '.' || c == '[' || c == ']' || c.is_alphanumeric()
                }
            }

        }
        
        pub fn escape(pattern: &str) -> String
        {
            let mut buf = String::new();
            buf.reserve(pattern.len());
            
            for ch in pattern.chars()
            {
                if crate::is::meta_character(ch) { buf.push('\\'); }
                
                buf.push(ch);
            }
            
            buf
        }

        #[derive(Clone, Copy, Debug)]
        pub struct Config
        {
            pub nest_limit: u32,
            pub flags: Flags,
        }

        impl Default for Config 
        {
            fn default() -> Config
            {
                Config { nest_limit: 50, flags: Flags::default() }
            }
        }
        
        #[derive(Clone, Copy, Debug, Default)]
        pub struct Flags 
        {
            pub case_insensitive: bool,
            pub multi_line: bool,
            pub dot_matches_new_line: bool,
            pub swap_greed: bool,
            pub crlf: bool,
            pub ignore_whitespace: bool,
        }

        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct Hir 
        {
            kind: HirKind,
            is_start_anchored: bool,
            is_match_empty: bool,
            static_explicit_captures_len: Option<usize>,
        }

        #[derive(Clone, Debug, Eq, PartialEq)]
        pub enum HirKind 
        {
            Empty,
            Char(char),
            Class(Class),
            Look(Look),
            Repetition(Repetition),
            Capture(Capture),
            Concat(Vec<Hir>),
            Alternation(Vec<Hir>),
        }

        impl Hir
        {
            pub fn parse(config: Config, pattern: &str) -> Result<Hir, Error> { self::parse::Parser::new(config, pattern).parse() }
            
            pub fn kind(&self) -> &HirKind { &self.kind }
            
            pub fn is_start_anchored(&self) -> bool { self.is_start_anchored }
            
            pub fn is_match_empty(&self) -> bool { self.is_match_empty }
            
            pub fn static_explicit_captures_len(&self) -> Option<usize>
            { self.static_explicit_captures_len }

            fn fail() -> Hir
            {
                let kind = HirKind::Class(Class { ranges: vec![] });
                
                Hir 
                {
                    kind,
                    is_start_anchored: false,
                    is_match_empty: false,
                    static_explicit_captures_len: Some(0),
                }
            }

            fn empty() -> Hir 
            {
                let kind = HirKind::Empty;
                
                Hir
                {
                    kind,
                    is_start_anchored: false,
                    is_match_empty: true,
                    static_explicit_captures_len: Some(0),
                }
            }

            fn char(ch: char) -> Hir 
            {
                let kind = HirKind::Char(ch);
                
                Hir 
                {
                    kind,
                    is_start_anchored: false,
                    is_match_empty: false,
                    static_explicit_captures_len: Some(0),
                }
            }

            fn class(class: Class) -> Hir
            {
                let kind = HirKind::Class(class);
                
                Hir
                {
                    kind,
                    is_start_anchored: false,
                    is_match_empty: false,
                    static_explicit_captures_len: Some(0),
                }
            }

            fn look(look: Look) -> Hir
            {
                let kind = HirKind::Look(look);
                
                Hir
                {
                    kind,
                    is_start_anchored: matches!(look, Look::Start),
                    is_match_empty: true,
                    static_explicit_captures_len: Some(0),
                }
            }

            fn repetition(rep: Repetition) -> Hir
            {
                if rep.min == 0 && rep.max == Some(0) { return Hir::empty(); }                    
                else if rep.min == 1 && rep.max == Some(1) { return *rep.sub; }
                
                let is_start_anchored = rep.min > 0 && rep.sub.is_start_anchored;
                let is_match_empty = rep.min == 0 || rep.sub.is_match_empty;
                let mut static_explicit_captures_len = rep.sub.static_explicit_captures_len;
                
                if rep.min == 0 && static_explicit_captures_len.map_or(false, |len| len > 0)
                {
                    if rep.max == Some(0) { static_explicit_captures_len = Some(0); }                        
                    else { static_explicit_captures_len = None; }
                }
                
                Hir
                {
                    kind: HirKind::Repetition(rep),
                    is_start_anchored,
                    is_match_empty,
                    static_explicit_captures_len,
                }
            }

            fn capture(cap: Capture) -> Hir
            {
                let is_start_anchored = cap.sub.is_start_anchored;
                let is_match_empty = cap.sub.is_match_empty;
                let static_explicit_captures_len = cap.sub.static_explicit_captures_len.map(|len| len.saturating_add(1));
                let kind = HirKind::Capture(cap);
                
                Hir
                {
                    kind,
                    is_start_anchored,
                    is_match_empty,
                    static_explicit_captures_len,
                }
            }

            fn concat(mut subs: Vec<Hir>) -> Hir
            {
                if subs.is_empty() { Hir::empty() }
                
                else if subs.len() == 1 { subs.pop().unwrap() }
                
                else
                {
                    let is_start_anchored = subs[0].is_start_anchored;
                    let mut is_match_empty = true;
                    let mut static_explicit_captures_len = Some(0usize);
                    
                    for sub in subs.iter()
                    {
                        is_match_empty = is_match_empty && sub.is_match_empty;
                        static_explicit_captures_len = static_explicit_captures_len.and_then(|len1| { Some((len1, sub.static_explicit_captures_len?)) }).and_then(|(len1, len2)| Some(len1.saturating_add(len2)));
                    }
                    
                    Hir
                    {
                        kind: HirKind::Concat(subs),
                        is_start_anchored,
                        is_match_empty,
                        static_explicit_captures_len,
                    }
                }
            }

            fn alternation(mut subs: Vec<Hir>) -> Hir 
            {
                if subs.is_empty() { Hir::fail() }
                else if subs.len() == 1 { subs.pop().unwrap() }                
                else
                {
                    let mut it = subs.iter().peekable();
                    let mut is_start_anchored = it.peek().map_or(false, |sub| sub.is_start_anchored);
                    let mut is_match_empty = it.peek().map_or(false, |sub| sub.is_match_empty);
                    let mut static_explicit_captures_len = it.peek().and_then(|sub| sub.static_explicit_captures_len);
                    
                    for sub in it 
                    {
                        is_start_anchored = is_start_anchored && sub.is_start_anchored;
                        is_match_empty = is_match_empty || sub.is_match_empty;
                        if static_explicit_captures_len != sub.static_explicit_captures_len
                        {
                            static_explicit_captures_len = None;
                        }
                    }
                    
                    Hir 
                    {
                        kind: HirKind::Alternation(subs),
                        is_start_anchored,
                        is_match_empty,
                        static_explicit_captures_len,
                    }
                }
            }
        }

        impl HirKind 
        {
            fn subs(&self) -> &[Hir] 
            {
                use crate::slice::from_ref;

                match *self 
                {
                    HirKind::Empty | HirKind::Char(_) | HirKind::Class(_) | HirKind::Look(_) => &[],
                    HirKind::Repetition(Repetition { ref sub, .. }) => from_ref(sub),
                    HirKind::Capture(Capture { ref sub, .. }) => from_ref(sub),
                    HirKind::Concat(ref subs) => subs,
                    HirKind::Alternation(ref subs) => subs,
                }
            }
        }

        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct Class
        {
            pub ranges: Vec<ClassRange>,
        }

        impl Class
        {
            fn new<I: IntoIterator<Item = ClassRange>>(ranges: I) -> Class
            {
                let mut class = Class { ranges: ranges.into_iter().collect() };
                class.canonicalize();
                class
            }
            
            fn ascii_case_fold(&mut self) 
            {
                let len = self.ranges.len();
                
                for i in 0..len 
                {
                    if let Some(folded) = self.ranges[i].ascii_case_fold() { self.ranges.push(folded); }
                }
                self.canonicalize();
            }
            
            fn negate(&mut self) 
            {
                const MIN: char = '\x00';
                const MAX: char = char::MAX;

                if self.ranges.is_empty() 
                {
                    self.ranges.push(ClassRange { start: MIN, end: MAX });
                    return;
                }
                
                let drain_end = self.ranges.len();
                
                if self.ranges[0].start > MIN 
                {
                    self.ranges.push(ClassRange 
                    {
                        start: MIN,
                        end: prev_char(self.ranges[0].start).unwrap(),
                    });
                }
                
                for i in 1..drain_end 
                {
                    self.ranges.push(ClassRange 
                    {
                        start: next_char(self.ranges[i - 1].end).unwrap(),
                        end: prev_char(self.ranges[i].start).unwrap(),
                    });
                }
                
                if self.ranges[drain_end - 1].end < MAX 
                {
                    self.ranges.push(ClassRange 
                    {
                        start: next_char(self.ranges[drain_end - 1].end).unwrap(),
                        end: MAX,
                    });
                }
                
                self.ranges.drain(..drain_end);
            }
            
            fn canonicalize(&mut self) 
            {
                if self.is_canonical() { return; }
                
                self.ranges.sort();
                assert!(!self.ranges.is_empty());
                
                let drain_end = self.ranges.len();
                
                for oldi in 0..drain_end 
                {
                    if self.ranges.len() > drain_end 
                    {
                        let (last, rest) = self.ranges.split_last_mut().unwrap();
                        
                        if let Some(union) = last.union(&rest[oldi]) 
                        {
                            *last = union;
                            continue;
                        }
                    }
                    
                    self.ranges.push(self.ranges[oldi]);
                }
                
                self.ranges.drain(..drain_end);
            }
            
            fn is_canonical(&self) -> bool
            {
                for pair in self.ranges.windows(2)
                {
                    if pair[0] >= pair[1] { return false; }                    
                    if pair[0].is_contiguous(&pair[1]) { return false; }
                }
                
                true
            }
        }

        #[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
        pub struct ClassRange 
        {
            pub start: char,
            pub end: char,
        }

        impl ClassRange 
        {
            fn ascii_case_fold(&self) -> Option<ClassRange> 
            {
                if !(ClassRange { start: 'a', end: 'z' }).is_intersection_empty(self)
                {
                    let start = crate::cmp::max(self.start, 'a');
                    let end = crate::cmp::min(self.end, 'z');
                    
                    return Some(ClassRange 
                    {
                        start: char::try_from(u32::from(start) - 32).unwrap(),
                        end: char::try_from(u32::from(end) - 32).unwrap(),
                    });
                }
                
                if !(ClassRange { start: 'A', end: 'Z' }).is_intersection_empty(self) 
                {
                    let start = crate::cmp::max(self.start, 'A');
                    let end = crate::cmp::min(self.end, 'Z');
                    
                    return Some(ClassRange 
                    {
                        start: char::try_from(u32::from(start) + 32).unwrap(),
                        end: char::try_from(u32::from(end) + 32).unwrap(),
                    });
                }
                
                None
            }
            
            fn union(&self, other: &ClassRange) -> Option<ClassRange> 
            {
                if !self.is_contiguous(other) { return None; }
                
                let start = crate::cmp::min(self.start, other.start);
                let end = crate::cmp::max(self.end, other.end);
                Some(ClassRange { start, end })
            }
            
            fn is_contiguous(&self, other: &ClassRange) -> bool 
            {
                let (s1, e1) = (u32::from(self.start), u32::from(self.end));
                let (s2, e2) = (u32::from(other.start), u32::from(other.end));
                crate::cmp::max(s1, s2) <=  crate::cmp::min(e1, e2).saturating_add(1)
            }
            
            fn is_intersection_empty(&self, other: &ClassRange) -> bool 
            {
                let (s1, e1) = (self.start, self.end);
                let (s2, e2) = (other.start, other.end);
                crate::cmp::max(s1, s2) >  crate::cmp::min(e1, e2)
            }
        }
        
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum Look 
        {
            Start = 1 << 0,
            End = 1 << 1,
            StartLF = 1 << 2,
            EndLF = 1 << 3,
            StartCRLF = 1 << 4,
            EndCRLF = 1 << 5,
            Word = 1 << 6,
            WordNegate = 1 << 7,
            WordStart = 1 << 8,
            WordEnd = 1 << 9,
            WordStartHalf = 1 << 10,
            WordEndHalf = 1 << 11,
        }

        impl Look 
        {
            pub fn is_match(&self, haystack: &[u8], at: usize) -> bool 
            {
                use self::Look::*;

                match *self 
                {
                    Start => at == 0,
                    End => at == haystack.len(),
                    StartLF => at == 0 || haystack[at - 1] == b'\n',
                    EndLF => at == haystack.len() || haystack[at] == b'\n',
                    
                    StartCRLF => 
                    {
                        at == 0
                            || haystack[at - 1] == b'\n'
                            || (haystack[at - 1] == b'\r'
                            && (at >= haystack.len() || haystack[at] != b'\n'))
                    }
                    
                    EndCRLF => 
                    {
                        at == haystack.len()
                            || haystack[at] == b'\r'
                            || (haystack[at] == b'\n'
                            && (at == 0 || haystack[at - 1] != b'\r'))
                    }
                    
                    Word => 
                    {
                        let word_before = at > 0 && utf8::is_word_byte(haystack[at - 1]);
                        let word_after = at < haystack.len() && utf8::is_word_byte(haystack[at]);
                        word_before != word_after
                    }
                    
                    WordNegate => 
                    {
                        let word_before = at > 0 && utf8::is_word_byte(haystack[at - 1]);
                        let word_after = at < haystack.len() && utf8::is_word_byte(haystack[at]);
                        word_before == word_after
                    }
                    
                    WordStart => 
                    {
                        let word_before = at > 0 && utf8::is_word_byte(haystack[at - 1]);
                        let word_after = at < haystack.len() && utf8::is_word_byte(haystack[at]);
                        !word_before && word_after
                    }
                    
                    WordEnd => 
                    {
                        let word_before = at > 0 && utf8::is_word_byte(haystack[at - 1]);
                        let word_after = at < haystack.len() && utf8::is_word_byte(haystack[at]);
                        word_before && !word_after
                    }
                    
                    WordStartHalf => 
                    {
                        let word_before = at > 0 && utf8::is_word_byte(haystack[at - 1]);
                        !word_before
                    }
                    
                    WordEndHalf => 
                    {
                        let word_after = at < haystack.len() && utf8::is_word_byte(haystack[at]);
                        !word_after
                    }
                }
            }
        }
        
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct Repetition
        {
            pub min: u32,
            pub max: Option<u32>,
            pub greedy: bool,
            pub sub: Box<Hir>,
        }
        
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct Capture
        {
            pub index: u32,
            pub name: Option<Box<str>>,
            pub sub: Box<Hir>,
        }

        fn next_char(ch: char) -> Option<char>
        {
            if ch == '\u{D7FF}' { return Some('\u{E000}'); }
            
            char::from_u32(u32::from(ch).checked_add(1).unwrap())
        }

        fn prev_char(ch: char) -> Option<char>
        {
            if ch == '\u{E000}' { return Some('\u{D7FF}'); }
            
            Some(char::from_u32(u32::from(ch).checked_sub(1)?).unwrap())
        }

        impl Drop for Hir
        {
            fn drop(&mut self)
            {
                use crate::mem;

                match *self.kind()
                {
                    HirKind::Empty | HirKind::Char(_) | HirKind::Class(_) | HirKind::Look(_) => return,
                    HirKind::Capture(ref x) if x.sub.kind.subs().is_empty() => return, 
                    HirKind::Repetition(ref x) if x.sub.kind.subs().is_empty() => { return }
                    HirKind::Concat(ref x) if x.is_empty() => return,
                    HirKind::Alternation(ref x) if x.is_empty() => return,
                    _ => {}
                }

                let mut stack = vec![mem::replace(self, Hir::empty())];
                
                while let Some(mut expr) = stack.pop()
                {
                    match expr.kind
                    {
                        HirKind::Empty | HirKind::Char(_) | HirKind::Class(_) | HirKind::Look(_) => {}
                        HirKind::Capture(ref mut x) => {  stack.push(mem::replace(&mut x.sub, Hir::empty()));  }
                        HirKind::Repetition(ref mut x) => {  stack.push(mem::replace(&mut x.sub, Hir::empty()));  }
                        HirKind::Concat(ref mut x) => {  stack.extend(x.drain(..));  }
                        HirKind::Alternation(ref mut x) => {  stack.extend(x.drain(..));  }
                    }
                }
            }
        }
    }
    pub use self::hir::*;
    
    pub mod int
    {
        /*!
        */
        use crate::
        {
            num::{ NonZeroUsize },
            *,
        };
        /*
        */

        pub trait U32
        {
            fn as_usize(self) -> usize;
        }

        impl U32 for u32 
        {
            fn as_usize(self) -> usize { self as usize }
        }
        
        #[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
        #[repr(transparent)]
        pub struct NonMaxUsize(NonZeroUsize);

        impl NonMaxUsize 
        {             
             pub fn new(value: usize) -> Option<NonMaxUsize>
            {
                NonZeroUsize::new(value.wrapping_add(1)).map(NonMaxUsize)
            }
             
             pub fn get(self) -> usize {
                self.0.get().wrapping_sub(1)
            }
        }
        
        impl crate::fmt::Debug for NonMaxUsize 
        {
            fn fmt(&self, f: &mut crate::fmt::Formatter) -> crate::fmt::Result {
                write!(f, "{:?}", self.get())
            }
        }
    }
    
    pub mod interpolate
    {
        /*!
        Routines for interpolating capture group references. */
        use crate::
        {
            string::{ String },
            *,
        };
        /*
        */
        pub fn string( mut replacement: &str, mut append: impl FnMut(usize, &mut String), mut name_to_index: impl FnMut(&str) -> Option<usize>, dst: &mut String )
        {
            while !replacement.is_empty()
            {
                match replacement.find('$')
                {
                    None => break,
                    Some(i) =>
                    {
                        dst.push_str(&replacement[..i]);
                        replacement = &replacement[i..];
                    }
                }
                
                if replacement.as_bytes().get(1).map_or(false, |&b| b == b'$')
                {
                    dst.push_str("$");
                    replacement = &replacement[2..];
                    continue;
                }
                
                debug_assert!(!replacement.is_empty());
                
                let cap_ref = match find_cap_ref(replacement.as_bytes())
                {
                    Some(cap_ref) => cap_ref,
                    None =>
            {
                        dst.push_str("$");
                        replacement = &replacement[1..];
                        continue;
                    }
                };
                
                replacement = &replacement[cap_ref.end..];
                
                match cap_ref.cap
                {
                    Ref::Number(i) => append(i, dst),
                    Ref::Named(name) =>
            {
                        if let Some(i) = name_to_index(name) {
                            append(i, dst);
                        }
                    }
                }
            }
            
            dst.push_str(replacement);
        }
        
        
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        struct CaptureRef<'a> 
        {
            cap: Ref<'a>,
            end: usize,
        }
        
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        enum Ref<'a>
        {
            Named(&'a str),
            Number(usize),
        }

        impl<'a> From<&'a str> for Ref<'a>
        {
            fn from(x: &'a str) -> Ref<'a>
            {
                Ref::Named(x)
            }
        }

        impl From<usize> for Ref<'static>
        {
            fn from(x: usize) -> Ref<'static>
            {
                Ref::Number(x)
            }
        }
        
        fn find_cap_ref(replacement: &[u8]) -> Option<CaptureRef<'_>>
        {
            let mut i = 0;
            let rep: &[u8] = replacement;            
            if rep.len() <= 1 || rep[0] != b'$' { return None; }
            
            i += 1;
            if rep[i] == b'{' { return find_cap_ref_braced(rep, i + 1); }
            
            let mut cap_end = i;
            
            while rep.get(cap_end).copied().map_or(false, crate::is::valid_cap_letter)
            {
                cap_end += 1;
            }
            
            if cap_end == i { return None; }
            
            let cap = crate::str::from_utf8(&rep[i..cap_end]).expect("valid UTF-8 capture name");
            
            Some(CaptureRef
            {
                cap: match cap.parse::<usize>()
                {
                    Ok(i) => Ref::Number(i),
                    Err(_) => Ref::Named(cap),
                },
                end: cap_end,
            })
        }
        
        fn find_cap_ref_braced(rep: &[u8], mut i: usize) -> Option<CaptureRef<'_>>
        {
            assert_eq!(b'{', rep[i.checked_sub(1).unwrap()]);
            let start = i;
            
            while rep.get(i).map_or(false, |&b| b != b'}') {
                i += 1;
            }
            
            if !rep.get(i).map_or(false, |&b| b == b'}') {
                return None;
            }
            
            let cap = match crate::str::from_utf8(&rep[start..i]) {
                Err(_) => return None,
                Ok(cap) => cap,
            };
            
            Some(CaptureRef 
            {
                cap: match cap.parse::<usize>() 
                {
                    Ok(i) => Ref::Number(i),
                    Err(_) => Ref::Named(cap),
                },
                end: i + 1,
            })
        }
    }
    
    pub mod nfa
    {
        /*!
        */
        use crate::
        {
            cell::{ RefCell },
            mem::{ size_of },
            regex::
            {
                error::Error,
                hir::{self, Hir, HirKind},
                int::U32,
            },
            string::{String},
            sync::{Arc},
            vec::{self, Vec},
            *,
        };
        /*
        use crate::{cell::RefCell, mem::size_of};

        use alloc::{string::String, sync::Arc, vec, vec::Vec};
        
        use crate::{
            error::Error,
            hir::{self, Hir, HirKind},
            int::U32,
        };
        */
        pub type StateID = u32;

        #[derive(Clone, Copy, Debug)]
        pub struct Config 
        {
            pub size_limit: Option<usize>,
        }

        impl Default for Config 
        {
            fn default() -> Config 
            {
                Config { size_limit: Some(10 * (1 << 20)) }
            }
        }

        #[derive(Clone)]
        pub struct NFA 
        {
            pattern: String,
            states: Vec<State>,
            start: StateID,
            is_start_anchored: bool,
            is_match_empty: bool,             
            static_explicit_captures_len: Option<usize>,
            cap_name_to_index: CaptureNameMap,             
            cap_index_to_name: Vec<Option<Arc<str>>>,
            memory_extra: usize,
        }

        impl NFA 
        {
            pub fn new( config: Config, pattern: String, hir: &Hir ) -> Result<NFA, Error> { Compiler::new(config, pattern).compile(hir) }
            
            pub fn pattern(&self) -> &str { &self.pattern }
            
            pub fn state(&self, id: StateID) -> &State { &self.states[id.as_usize()] }
            
            pub fn len(&self) -> usize { self.states.len() }
            
            pub fn start(&self) -> StateID { self.start }
            
            pub fn to_index(&self, name: &str) -> Option<usize> { self.cap_name_to_index.get(name).cloned().map(|i| i.as_usize()) }
            
            pub fn capture_names(&self) -> CaptureNames<'_> { CaptureNames { it: self.cap_index_to_name.iter() } }
            
            pub fn group_len(&self) -> usize { self.cap_index_to_name.len() }
            
            pub fn is_start_anchored(&self) -> bool { self.is_start_anchored }
            
            pub fn static_explicit_captures_len(&self) -> Option<usize> { self.static_explicit_captures_len }
            
            fn memory_usage(&self) -> usize 
            {
                (self.states.len() * size_of::<State>())
                + (self.cap_index_to_name.len() * size_of::<Option<Arc<str>>>())
                + self.memory_extra
            }
        }

        impl crate::fmt::Debug for NFA 
        {
            fn fmt(&self, f: &mut crate::fmt::Formatter<'_>) -> crate::fmt::Result 
            {
                writeln!(f, "NFA(")?;
                writeln!(f, "pattern: {}", self.pattern)?;
                
                for (sid, state) in self.states.iter().enumerate()
                {
                    writeln!(f, "{sid:07?}: {state:?}")?;
                }
                
                writeln!(f, ")")?;
                Ok(())
            }
        }
        
        #[derive(Clone, Debug)]
        pub struct CaptureNames<'a>
        {
            it: slice::Iter<'a, Option<Arc<str>>>,
        }

        impl<'a> Iterator for CaptureNames<'a>
        {
            type Item = Option<&'a str>;
            fn next(&mut self) -> Option<Option<&'a str>> { self.it.next().map(|n| n.as_deref()) }
        }

        #[derive(Clone, Eq, PartialEq)]
        pub enum State
        {
            Char { target: StateID, ch: char },
            Ranges { target: StateID, ranges: Vec<(char, char)> },
            Splits { targets: Vec<StateID>, reverse: bool },
            Goto { target: StateID, look: Option<hir::Look> },
            Capture { target: StateID, slot: u32 },
            Fail,
            Match,
        }

        impl State
        {
            fn memory_usage(&self) -> usize
            {
                match *self
                {
                    State::Char { .. } | State::Goto { .. } | State::Capture { .. } | State::Fail { .. } | State::Match => 0,
                    State::Splits { ref targets, .. } => { targets.len() * size_of::<StateID>() }
                    State::Ranges { ref ranges, .. } => { ranges.len() * size_of::<(char, char)>() }
                }
            }
            
            pub fn iter_splits<'a>( splits: &'a [StateID], reverse: bool ) -> impl Iterator<Item = StateID> + 'a
            {
                let mut it = splits.iter();
                iter::from_fn(move || { if reverse { it.next_back() } else { it.next() }.copied() })
            }
        }

        impl crate::fmt::Debug for State 
        {
            fn fmt(&self, f: &mut crate::fmt::Formatter<'_>) -> crate::fmt::Result 
            {
                match *self
                {
                    State::Char { target, ch } => { write!(f, "{ch:?} => {target:?}") }
                    State::Ranges { target, ref ranges } => 
                    {
                        for (i, &(start, end)) in ranges.iter().enumerate()
                        {
                            if i > 0 { write!(f, ", ")?; }
                            
                            write!(f, "{start:?}-{end:?} => {target:?}")?;
                        }
                        
                        Ok(())
                    }
                    
                    State::Splits { ref targets, reverse } =>
                    {
                        write!(f, "splits(")?;
                        
                        for (i, sid) in State::iter_splits(targets, reverse).enumerate()
                        {
                            if i > 0 { write!(f, ", ")?; }
                            
                            write!(f, "{sid:?}")?;
                        }
                        
                        write!(f, ")")
                    }
                    
                    State::Goto { target, look: None } => { write!(f, "goto({target:?})") }
                    State::Goto { target, look: Some(look) } => { write!(f, "{look:?} => {target:?}") }
                    State::Capture { target, slot } => { write!(f, "capture(slot={slot:?}) => {target:?}") }
                    State::Fail => write!(f, "FAIL"),
                    State::Match => { write!(f, "MATCH") }
                }
            }
        }
        
        type CaptureNameMap = crate::collections::HashMap<Arc<str>, u32>;

        #[derive(Debug)]
        struct Compiler
        {
            config: Config,
            nfa: RefCell<NFA>,
        }

        impl Compiler
        {
            fn new(config: Config, pattern: String) -> Compiler 
            {
                let nfa = RefCell::new(NFA 
                {
                    pattern,
                    states: vec![],
                    start: 0,
                    is_start_anchored: false,
                    is_match_empty: false,
                    static_explicit_captures_len: None,
                    cap_name_to_index: CaptureNameMap::default(),
                    cap_index_to_name: vec![],
                    memory_extra: 0,
                });
                
                Compiler { config, nfa }
            }

            fn compile(self, hir: &Hir) -> Result<NFA, Error> 
            {
                self.nfa.borrow_mut().is_start_anchored = hir.is_start_anchored();
                self.nfa.borrow_mut().is_match_empty = hir.is_match_empty();
                self.nfa.borrow_mut().static_explicit_captures_len = hir.static_explicit_captures_len();
                let compiled = self.c_capture(0, None, hir)?;
                let mat = self.add(State::Match)?;
                self.patch(compiled.end, mat)?;
                self.nfa.borrow_mut().start = compiled.start;
                
                Ok(self.nfa.into_inner())
            }

            fn c(&self, hir: &Hir) -> Result<ThompsonRef, Error> 
            {
                match *hir.kind() 
                {
                    HirKind::Empty => self.c_empty(),
                    HirKind::Char(ch) => self.c_char(ch),
                    HirKind::Class(ref class) => self.c_class(class),
                    HirKind::Look(ref look) => self.c_look(look),
                    HirKind::Repetition(ref rep) => self.c_repetition(rep),
                    HirKind::Capture(ref cap) => { self.c_capture(cap.index, cap.name.as_deref(), &cap.sub)  }
                    HirKind::Concat(ref subs) => { self.c_concat(subs.iter().map(|s| self.c(s)))  }
                    HirKind::Alternation(ref subs) => { self.c_alternation(subs.iter().map(|s| self.c(s)))  }
                }
            }
            
            fn c_fail(&self) -> Result<ThompsonRef, Error> 
            {
                let id = self.add(State::Fail)?;
                Ok(ThompsonRef { start: id, end: id })
            }

            fn c_empty(&self) -> Result<ThompsonRef, Error> 
            {
                let id = self.add_empty()?;
                Ok(ThompsonRef { start: id, end: id })
            }
            
            fn c_char(&self, ch: char) -> Result<ThompsonRef, Error> 
            {
                let id = self.add(State::Char { target: 0, ch })?;
                Ok(ThompsonRef { start: id, end: id })
            }
             
            fn c_class(&self, class: &hir::Class) -> Result<ThompsonRef, Error> 
            {
                let id = if class.ranges.is_empty() { self.add(State::Fail) } else
                {
                    let ranges = class.ranges.iter().map(|r| (r.start, r.end)).collect();
                    self.add(State::Ranges { target: 0, ranges })
                }?;
                
                Ok(ThompsonRef { start: id, end: id })
            }
             
            fn c_look(&self, look: &hir::Look) -> Result<ThompsonRef, Error> 
            {
                let id = self.add(State::Goto { target: 0, look: Some(*look) })?;
                Ok(ThompsonRef { start: id, end: id })
            }
             
            fn c_repetition( &self, rep: &hir::Repetition ) -> Result<ThompsonRef, Error>
            {
                match (rep.min, rep.max) {
                    (0, Some(1)) => self.c_zero_or_one(&rep.sub, rep.greedy),
                    (min, None) => self.c_at_least(&rep.sub, rep.greedy, min),
                    (min, Some(max)) if min == max => self.c_exactly(&rep.sub, min),
                    (min, Some(max)) => self.c_bounded(&rep.sub, rep.greedy, min, max),
                }
            }
            
            fn c_bounded( &self, hir: &Hir, greedy: bool, min: u32, max: u32 ) -> Result<ThompsonRef, Error>
            {
                let prefix = self.c_exactly(hir, min)?;
                if min == max {
                    return Ok(prefix);
                }
                
                let empty = self.add_empty()?;
                let mut prev_end = prefix.end;
                for _ in min..max {
                    let splits =
                        self.add(State::Splits { targets: vec![], reverse: !greedy })?;
                    let compiled = self.c(hir)?;
                    self.patch(prev_end, splits)?;
                    self.patch(splits, compiled.start)?;
                    self.patch(splits, empty)?;
                    prev_end = compiled.end;
                }
                self.patch(prev_end, empty)?;
                Ok(ThompsonRef { start: prefix.start, end: empty })
            }

            fn c_at_least( &self, hir: &Hir, greedy: bool, n: u32 ) -> Result<ThompsonRef, Error> 
            {
                if n == 0 
                {
                    if !hir.is_match_empty() 
                    {
                        let splits = self.add(State::Splits 
                        {
                            targets: vec![],
                            reverse: !greedy,
                        })?;
                        
                        let compiled = self.c(hir)?;
                        self.patch(splits, compiled.start)?;
                        self.patch(compiled.end, splits)?;
                        return Ok(ThompsonRef { start: splits, end: splits });
                    }
                    
                    let compiled = self.c(hir)?;
                    let plus = self.add(State::Splits { targets: vec![], reverse: !greedy })?;
                    self.patch(compiled.end, plus)?;
                    self.patch(plus, compiled.start)?;

                    let question = self.add(State::Splits { targets: vec![], reverse: !greedy })?;
                    let empty = self.add_empty()?;
                    self.patch(question, compiled.start)?;
                    self.patch(question, empty)?;
                    self.patch(plus, empty)?;
                    Ok(ThompsonRef { start: question, end: empty })
                }
                    
                else if n == 1 
                {
                    let compiled = self.c(hir)?;
                    let splits = self.add(State::Splits { targets: vec![], reverse: !greedy })?;
                    self.patch(compiled.end, splits)?;
                    self.patch(splits, compiled.start)?;
                    
                    Ok(ThompsonRef { start: compiled.start, end: splits })
                }
                    
                else 
                {
                    let prefix = self.c_exactly(hir, n - 1)?;
                    let last = self.c(hir)?;
                    let splits = self.add(State::Splits { targets: vec![], reverse: !greedy })?;
                    self.patch(prefix.end, last.start)?;
                    self.patch(last.end, splits)?;
                    self.patch(splits, last.start)?;
                    
                    Ok(ThompsonRef { start: prefix.start, end: splits })
                }
            }
            
            fn c_zero_or_one( &self, hir: &Hir, greedy: bool ) -> Result<ThompsonRef, Error>
            {
                let splits = self.add(State::Splits { targets: vec![], reverse: !greedy })?;
                let compiled = self.c(hir)?;
                let empty = self.add_empty()?;
                self.patch(splits, compiled.start)?;
                self.patch(splits, empty)?;
                self.patch(compiled.end, empty)?;
                Ok(ThompsonRef { start: splits, end: empty })
            }
            
            fn c_exactly(&self, hir: &Hir, n: u32) -> Result<ThompsonRef, Error> { self.c_concat((0..n).map(|_| self.c(hir))) }
            
            fn c_capture( &self, index: u32, name: Option<&str>, hir: &Hir ) -> Result<ThompsonRef, Error> 
            {
                let existing_groups_len = self.nfa.borrow().cap_index_to_name.len();
                
                for _ in 0..(index.as_usize().saturating_sub(existing_groups_len)) { self.nfa.borrow_mut().cap_index_to_name.push(None); }
                
                if index.as_usize() >= existing_groups_len
                {
                    if let Some(name) = name
                    {
                        let name = Arc::from(name);
                        let mut nfa = self.nfa.borrow_mut();
                        nfa.cap_name_to_index.insert(Arc::clone(&name), index);
                        nfa.cap_index_to_name.push(Some(Arc::clone(&name)));
                        nfa.memory_extra += name.len() + size_of::<u32>();
                    }
                    else { self.nfa.borrow_mut().cap_index_to_name.push(None); }
                }

                let Some(slot) = index.checked_mul(2) else { return Err(Error::new("capture group slots exhausted")); };                
                let start = self.add(State::Capture { target: 0, slot })?;
                let inner = self.c(hir)?;
                let Some(slot) = slot.checked_add(1) else { return Err(Error::new("capture group slots exhausted")); };
                let end = self.add(State::Capture { target: 0, slot })?;
                self.patch(start, inner.start)?;
                self.patch(inner.end, end)?;

                Ok(ThompsonRef { start, end })
            }
            
            fn c_concat<I>(&self, mut it: I) -> Result<ThompsonRef, Error> where
            I: Iterator<Item = Result<ThompsonRef, Error>>
            {
                let ThompsonRef { start, mut end } = match it.next() 
                {
                    Some(result) => result?,
                    None => return self.c_empty(),
                };
                
                for result in it 
                {
                    let compiled = result?;
                    self.patch(end, compiled.start)?;
                    end = compiled.end;
                }
                
                Ok(ThompsonRef { start, end })
            }

            fn c_alternation<I>(&self, mut it: I) -> Result<ThompsonRef, Error> where
            I: Iterator<Item = Result<ThompsonRef, Error>>
            {
                let first = match it.next() 
                {
                    None => return self.c_fail(),
                    Some(result) => result?,
                };
                
                let second = match it.next() 
                {
                    None => return Ok(first),
                    Some(result) => result?,
                };

                let splits = self.add(State::Splits { targets: vec![], reverse: false })?;
                let end = self.add_empty()?;
                self.patch(splits, first.start)?;
                self.patch(first.end, end)?;
                self.patch(splits, second.start)?;
                self.patch(second.end, end)?;
                
                for result in it 
                {
                    let compiled = result?;
                    self.patch(splits, compiled.start)?;
                    self.patch(compiled.end, end)?;
                }
                
                Ok(ThompsonRef { start: splits, end })
            }
 
            fn add_empty(&self) -> Result<StateID, Error> { self.add(State::Goto { target: 0, look: None }) }
            
            fn add(&self, state: State) -> Result<StateID, Error>
            {
                let id = u32::try_from(self.nfa.borrow().states.len()).map_err(|_| Error::new("exhausted state IDs, too many states"))?;
                self.nfa.borrow_mut().memory_extra += state.memory_usage();
                self.nfa.borrow_mut().states.push(state);
                self.check_size_limit()?;
                Ok(id)
            }

            fn patch(&self, from: StateID, to: StateID) -> Result<(), Error>
            {
                let mut new_memory_extra = self.nfa.borrow().memory_extra;
                
                match self.nfa.borrow_mut().states[from.as_usize()]
                {
                    State::Char { ref mut target, .. } => { *target = to; }                    
                    State::Ranges { ref mut target, .. } => { *target = to; }
                    State::Splits { ref mut targets, .. } =>
                    {
                        targets.push(to);
                        new_memory_extra += size_of::<StateID>();
                    }
                    
                    State::Goto { ref mut target, .. } => { *target = to; }
                    State::Capture { ref mut target, .. } => { *target = to; }                    
                    State::Fail | State::Match => {}
                }
                
                if new_memory_extra != self.nfa.borrow().memory_extra 
                {
                    self.nfa.borrow_mut().memory_extra = new_memory_extra;
                    self.check_size_limit()?;
                }
                
                Ok(())
            }
            
            fn check_size_limit(&self) -> Result<(), Error>
            {
                if let Some(limit) = self.config.size_limit 
                {
                    if self.nfa.borrow().memory_usage() > limit { return Err(Error::new("compiled regex exceeded size limit")); }
                }
                
                Ok(())
            }
        }
        
        #[derive(Clone, Copy, Debug)]
        struct ThompsonRef 
        {
            start: StateID,
            end: StateID,
        }
    }
    
    pub mod pikevm
    {
        /*!
        */
        use crate::
        {
            regex::
            {
                int::{NonMaxUsize, U32},
                nfa::{State, StateID, NFA},
                pool::CachePoolGuard,
                utf8,
            },
            vec::{ self, Vec },
            *,
        };
        /*
        */
        #[derive(Clone, Debug)]
        pub struct PikeVM 
        {
            nfa: NFA,
        }

        impl PikeVM 
        {
            pub fn new(nfa: NFA) -> PikeVM
            {
                PikeVM { nfa }
            }
            
            pub fn nfa(&self) -> &NFA { &self.nfa }
            
            pub fn find_iter<'r, 'h>( &'r self, cache: CachePoolGuard<'r>, haystack: &'h [u8] ) -> FindMatches<'r, 'h>
            {
                FindMatches
                {
                    pikevm: self,
                    cache,
                    haystack,
                    at: 0,
                    slots: vec![None, None],
                    last_match_end: None,
                }
            }
            
            pub fn captures_iter<'r, 'h>( &'r self, cache: CachePoolGuard<'r>, haystack: &'h [u8] ) -> CapturesMatches<'r, 'h>
            {
                let len = self.nfa().group_len().checked_mul(2).unwrap();
                CapturesMatches
                {
                    it: FindMatches
                    {
                        pikevm: self,
                        cache,
                        haystack,
                        at: 0,
                        slots: vec![None; len],
                        last_match_end: None,
                    },
                }
            }
            
            pub fn search( &self, cache: &mut Cache, haystack: &[u8], start: usize, end: usize, earliest: bool, slots: &mut [Option<NonMaxUsize>] ) -> bool
            {
                cache.setup_search(slots.len());
                if start > end { return false; }
                
                assert!
                (
                    haystack.len() < crate::usize::MAX,
                    "byte slice lengths must be less than usize MAX",
                );

                let Cache { stack, curr,  next } = cache;
                let start_id = self.nfa().start();
                let anchored = self.nfa().is_start_anchored();
                let mut matched = false;
                let mut at = start;
                
                while at <= end
                {
                    if curr.set.is_empty() 
                    {
                        if matched { break; }
                        
                        if anchored && at > start { break; }
                    }

                    if !matched {
                                                                                                                                                        //
                                                                                        let slots = next.slot_table.all_absent();
                        self.epsilon_closure(
                            stack, slots, curr, haystack, at, start_id,
                        );
                    }
                    
                    let (ch, len) = utf8::decode_lossy(&haystack[at..]);
                    if self.nexts(stack, curr, next, haystack, at, ch, len, slots) {
                        matched = true;
                    }
                                                                    if (earliest && matched) || len == 0 {
                        break;
                    }
                    crate::mem::swap(curr, next);
                    next.set.clear();
                    at += len;
                }
                matched
            }
            
            fn nexts( &self,
                stack: &mut Vec<FollowEpsilon>,
                curr: &mut ActiveStates,
                next: &mut ActiveStates,
                haystack: &[u8],
                at: usize,
                at_ch: char,
                at_len: usize,
                slots: &mut [Option<NonMaxUsize>] ) -> bool 
            {
                let ActiveStates { ref set, ref mut slot_table } = *curr;
                for sid in set.iter() {
                    if self.next(
                        stack, slot_table, next, haystack, at, at_ch, at_len, sid,
                    ) {
                        slots.copy_from_slice(slot_table.for_state(sid));
                        return true;
                    }
                }
                false
            }
            
            fn next( &self,
                stack: &mut Vec<FollowEpsilon>,
                curr_slot_table: &mut SlotTable,
                next: &mut ActiveStates,
                haystack: &[u8],
                at: usize,
                at_ch: char,
                at_len: usize,
                sid: StateID ) -> bool 
            {
                match *self.nfa.state(sid) {
                    State::Fail
                    | State::Goto { .. }
                    | State::Splits { .. }
                    | State::Capture { .. } => false,
                    State::Char { target, ch } =>
            {
                        if at_ch == ch && at_len > 0 {
                            let slots = curr_slot_table.for_state(sid);
                                                                                        let at = at.wrapping_add(at_len);
                            self.epsilon_closure(
                                stack, slots, next, haystack, at, target,
                            );
                        }
                        false
                    }
                    State::Ranges { target, ref ranges } =>
            {
                        for (start, end) in ranges.iter().copied() {
                            if start > at_ch {
                                break;
                            } else if start <= at_ch && at_ch <= end {
                                if at_len == 0 {
                                    return false;
                                }
                                let slots = curr_slot_table.for_state(sid);
                                                                                                        let at = at.wrapping_add(at_len);
                                self.epsilon_closure(
                                    stack, slots, next, haystack, at, target,
                                );
                            }
                        }
                        false
                    }
                    State::Match => true,
                }
            }
            
            fn epsilon_closure( &self,
                stack: &mut Vec<FollowEpsilon>,
                curr_slots: &mut [Option<NonMaxUsize>],
                next: &mut ActiveStates,
                haystack: &[u8],
                at: usize,
                sid: StateID,
            )
            {
                stack.push(FollowEpsilon::Explore(sid));
                while let Some(frame) = stack.pop() {
                    match frame {
                        FollowEpsilon::RestoreCapture { slot, offset } =>
            {
                            curr_slots[slot.as_usize()] = offset;
                        }
                        FollowEpsilon::Explore(sid) =>
            {
                            self.epsilon_closure_explore(
                                stack, curr_slots, next, haystack, at, sid,
                            );
                        }
                    }
                }
            }
            
            fn epsilon_closure_explore( &self,
                stack: &mut Vec<FollowEpsilon>,
                curr_slots: &mut [Option<NonMaxUsize>],
                next: &mut ActiveStates,
                haystack: &[u8],
                at: usize,
                mut sid: StateID,
            ) 
            {
                
                                loop {
                                            if !next.set.insert(sid) {
                        return;
                    }
                    match *self.nfa.state(sid) {
                        State::Fail
                        | State::Match { .. }
                        | State::Char { .. }
                        | State::Ranges { .. } =>
            {
                            next.slot_table.for_state(sid).copy_from_slice(curr_slots);
                            return;
                        }
                        State::Goto { target, look: None } =>
            {
                            sid = target;
                        }
                        State::Goto { target, look: Some(look) } =>
            {
                            if !look.is_match(haystack, at) {
                                return;
                            }
                            sid = target;
                        }
                        State::Splits { ref targets, reverse: false } =>
            {
                            sid = match targets.get(0) {
                                None => return,
                                Some(&sid) => sid,
                            };
                            stack.extend(
                                targets[1..]
                                    .iter()
                                    .copied()
                                    .rev()
                                    .map(FollowEpsilon::Explore),
                            );
                        }
                        State::Splits { ref targets, reverse: true } =>
            {
                            sid = match targets.last() {
                                None => return,
                                Some(&sid) => sid,
                            };
                            stack.extend(
                                targets[..targets.len() - 1]
                                    .iter()
                                    .copied()
                                    .map(FollowEpsilon::Explore),
                            );
                        }
                        State::Capture { target, slot } =>
            {
                                                                                                            if slot.as_usize() < curr_slots.len() {
                                stack.push(FollowEpsilon::RestoreCapture {
                                    slot,
                                    offset: curr_slots[slot.as_usize()],
                                });
                                                        curr_slots[slot.as_usize()] =
                                    Some(NonMaxUsize::new(at).unwrap());
                            }
                            sid = target;
                        }
                    }
                }
            }
        }
        
        #[derive(Debug)]
        pub struct FindMatches<'r, 'h>
        {
            pikevm: &'r PikeVM,
            cache: CachePoolGuard<'r>,
            haystack: &'h [u8],
            at: usize,
            slots: Vec<Option<NonMaxUsize>>,
            last_match_end: Option<usize>,
        }

        impl<'r, 'h> Iterator for FindMatches<'r, 'h>
        {
            type Item = (usize, usize);

            fn next(&mut self) -> Option<(usize, usize)>
            {
                if !self.pikevm.search(
                    &mut self.cache,
                    self.haystack,
                    self.at,
                    self.haystack.len(),
                    false,
                    &mut self.slots,
                ) {
                    return None;
                }
                let mut m =
                    (self.slots[0].unwrap().get(), self.slots[1].unwrap().get());
                if m.0 >= m.1 {
                    m = self.handle_overlapping_empty_match(m)?;
                }
                self.at = m.1;
                self.last_match_end = Some(m.1);
                Some(m)
            }
        }

        impl<'r, 'h> FindMatches<'r, 'h>
        {
            #[cold] #[inline(never)] fn handle_overlapping_empty_match(
                &mut self,
                mut m: (usize, usize) ) -> Option<(usize, usize)>
            {
                assert!(m.0 >= m.1);
                if Some(m.1) == self.last_match_end {
                    let len =
                         crate::cmp::max(1, utf8::decode(&self.haystack[self.at..]).1);
                    self.at = self.at.checked_add(len).unwrap();
                    if !self.pikevm.search(
                        &mut self.cache,
                        self.haystack,
                        self.at,
                        self.haystack.len(),
                        false,
                        &mut self.slots,
                    ) {
                        return None;
                    }
                    m = (self.slots[0].unwrap().get(), self.slots[1].unwrap().get());
                }
                Some(m)
            }
        }
        
        #[derive(Debug)]
        pub struct CapturesMatches<'r, 'h>
        {
            it: FindMatches<'r, 'h>,
        }

        impl<'r, 'h> Iterator for CapturesMatches<'r, 'h>
        {
            type Item = Vec<Option<NonMaxUsize>>;

            fn next(&mut self) -> Option<Vec<Option<NonMaxUsize>>>
            {
                self.it.next()?;
                Some(self.it.slots.clone())
            }
        }
        
        #[derive(Clone, Debug)]
        pub struct Cache 
        {            
            stack: Vec<FollowEpsilon>,             
            curr: ActiveStates,             
            next: ActiveStates,
        }

        impl Cache
        {
            pub fn new(re: &PikeVM) -> Cache 
            {
                Cache 
                {
                    stack: vec![],
                    curr: ActiveStates::new(re),
                    next: ActiveStates::new(re),
                }
            }
            
            fn setup_search(&mut self, captures_slot_len: usize) 
            {
                self.stack.clear();
                self.curr.setup_search(captures_slot_len);
                self.next.setup_search(captures_slot_len);
            }
        }
        
        #[derive(Clone, Debug)]
        struct ActiveStates 
        {
            set: SparseSet,            
            slot_table: SlotTable,
        }

        impl ActiveStates 
        {
            fn new(re: &PikeVM) -> ActiveStates 
            {
                let mut active = ActiveStates {
                    set: SparseSet::new(0),
                    slot_table: SlotTable::new(),
                };
                active.reset(re);
                active
            }
            
            fn reset(&mut self, re: &PikeVM) 
            {
                self.set.resize(re.nfa().len());
                self.slot_table.reset(re);
            }
            
            fn setup_search(&mut self, captures_slot_len: usize) 
            {
                self.set.clear();
                self.slot_table.setup_search(captures_slot_len);
            }
        }
        
        #[derive(Clone, Debug)]
        struct SlotTable 
        {
            table: Vec<Option<NonMaxUsize>>,             
            slots_per_state: usize,            
            slots_for_captures: usize,
        }

        impl SlotTable 
        {             
            fn new() -> SlotTable {
                SlotTable { table: vec![], slots_for_captures: 0, slots_per_state: 0 }
            }
             
            fn reset(&mut self, re: &PikeVM) 
            {
                let nfa = re.nfa();
                self.slots_per_state = nfa.group_len().checked_mul(2).unwrap();                
                self.slots_for_captures = self.slots_per_state;
                let len = nfa.len().checked_add(1).and_then(|x| x.checked_mul(self.slots_per_state)).expect("slot table length doesn't overflow");
                self.table.resize(len, None);
            }
            
            fn setup_search(&mut self, captures_slot_len: usize) 
            {
                self.slots_for_captures = captures_slot_len;
            }

            fn for_state(&mut self, sid: StateID) -> &mut [Option<NonMaxUsize>] 
            {
                let i = sid.as_usize() * self.slots_per_state;
                &mut self.table[i..i + self.slots_for_captures]
            }
            
            fn all_absent(&mut self) -> &mut [Option<NonMaxUsize>] 
            {
                let i = self.table.len() - self.slots_per_state;
                &mut self.table[i..i + self.slots_for_captures]
            }
        }
        
        #[derive(Clone, Debug)]
        enum FollowEpsilon 
        {
                Explore(StateID),
                RestoreCapture { slot: u32, offset: Option<NonMaxUsize> },
        }
        
        #[derive(Clone)]
        struct SparseSet 
        {
            len: usize,
            dense: Vec<StateID>,
            sparse: Vec<StateID>,
        }

        impl SparseSet 
        {
            fn new(capacity: usize) -> SparseSet {
                let mut set = SparseSet { len: 0, dense: vec![], sparse: vec![] };
                set.resize(capacity);
                set
            }
            
            fn resize(&mut self, new_capacity: usize)
            {
                assert!(
                    new_capacity <= u32::MAX.as_usize(),
                    "sparse set capacity cannot exceed {:?}",
                    u32::MAX,
                );
                self.clear();
                self.dense.resize(new_capacity, 0);
                self.sparse.resize(new_capacity, 0);
            }
            
            fn capacity(&self) -> usize {
                self.dense.len()
            }
            
            fn len(&self) -> usize {
                self.len
            }
            
            fn is_empty(&self) -> bool {
                self.len() == 0
            }
            
            fn insert(&mut self, id: StateID) -> bool {
                if self.contains(id) {
                    return false;
                }

                let index = self.len();
                assert!(
                    index < self.capacity(),
                    "{:?} exceeds capacity of {:?} when inserting {:?}",
                    index,
                    self.capacity(),
                    id,
                );
                self.dense[index] = id;
                                self.sparse[id.as_usize()] = u32::try_from(index).unwrap();
                self.len += 1;
                true
            }
            
            fn contains(&self, id: StateID) -> bool {
                let index = self.sparse[id.as_usize()];
                index.as_usize() < self.len() && self.dense[index.as_usize()] == id
            }
            
            fn clear(&mut self) {
                self.len = 0;
            }
             
            fn iter(&self) -> SparseSetIter<'_>
            {
                SparseSetIter(self.dense[..self.len()].iter())
            }
        }

        impl crate::fmt::Debug for SparseSet 
        {
            fn fmt(&self, f: &mut crate::fmt::Formatter) -> crate::fmt::Result {
                let elements: Vec<StateID> = self.iter().collect();
                f.debug_tuple("SparseSet").field(&elements).finish()
            }
        }
        
        #[derive(Debug)]
        struct SparseSetIter<'a>(core::slice::Iter<'a, StateID>);

        impl<'a> Iterator for SparseSetIter<'a>
        {
            type Item = StateID;

            fn next(&mut self) -> Option<StateID>
            {
                self.0.next().map(|&id| id)
            }
        }
    }
    
    pub mod pool
    {
        /*!
        */
        use crate::
        {
            boxed::{ Box },
            panic::{RefUnwindSafe, UnwindSafe},
            regex::{ pikevm },
            sync::{Mutex},
            vec::{ self, Vec },
            *,
        };
        /*
        */
        pub type CachePool = Pool<pikevm::Cache, CachePoolFn>;

        pub type CachePoolGuard<'a> = PoolGuard<'a, pikevm::Cache, CachePoolFn>;

        pub type CachePoolFn = Box<dyn Fn() -> pikevm::Cache + Send + Sync + UnwindSafe + RefUnwindSafe>;
        
        pub struct Pool<T, F>
        {
            stack: Mutex<Vec<Box<T>>>,             
            create: F,
        }

        impl<T: UnwindSafe, F: UnwindSafe> RefUnwindSafe for Pool<T, F> {}

        impl<T, F> Pool<T, F>
        {
            pub const fn new(create: F) -> Pool<T, F>
            {
                Pool { stack: Mutex::new(vec![]), create }
            }
        }

        impl<T: Send, F: Fn() -> T> Pool<T, F>
        {
            pub fn get(&self) -> PoolGuard<'_, T, F>
            {
                let mut stack = self.stack.lock().unwrap();
                let value = match stack.pop() {
                    None => Box::new((self.create)()),
                    Some(value) => value,
                };
                PoolGuard { pool: self, value: Some(value) }
            }
            
            fn put_value(&self, value: Box<T>) 
            {
                let mut stack = self.stack.lock().unwrap();
                stack.push(value);
            }
        }

        impl<T: crate::fmt::Debug, F> crate::fmt::Debug for Pool<T, F>
        {
            fn fmt(&self, f: &mut crate::fmt::Formatter<'_>) -> crate::fmt::Result {
                f.debug_struct("Pool").field("stack", &self.stack).finish()
            }
        }

        pub struct PoolGuard<'a, T: Send, F: Fn() -> T>
        {
                pool: &'a Pool<T, F>,
                value: Option<Box<T>>,
        }

        impl<'a, T: Send, F: Fn() -> T> Drop for PoolGuard<'a, T, F>
        {
            fn drop(&mut self) {
                if let Some(value) = self.value.take() {
                    self.pool.put_value(value);
                }
            }
        }

        impl<'a, T: Send, F: Fn() -> T> crate::ops::Deref for PoolGuard<'a, T, F>
        {
            type Target = T;

            fn deref(&self) -> &T {
                self.value.as_deref().unwrap()
            }
        }

        impl<'a, T: Send, F: Fn() -> T> crate::ops::DerefMut for PoolGuard<'a, T, F>
        {
            fn deref_mut(&mut self) -> &mut T {
                self.value.as_deref_mut().unwrap()
            }
        }

        impl<'a, T: Send + crate::fmt::Debug, F: Fn() -> T> crate::fmt::Debug
        for PoolGuard<'a, T, F>
        {
            fn fmt(&self, f: &mut crate::fmt::Formatter) -> crate::fmt::Result {
                f.debug_struct("PoolGuard")
                    .field("pool", &self.pool)
                    .field("value", &self.value)
                    .finish()
            }
        }
    }
    
    pub mod string
    {
        /*!
        */
        use crate::
        {
            borrow::{ Cow },
            boxed::{ Box },
            regex::
            {
                error::Error,
                hir::{self, Hir},
                int::NonMaxUsize,
                interpolate,
                nfa::{self, NFA},
                pikevm::{self, Cache, PikeVM},
                pool::CachePool,
            },
            string::{ String, ToString },
            sync::{ Arc },
            vec::{self, Vec},
            *,
        };
        /*
        */
        pub struct Regex 
        {
            pikevm: Arc<PikeVM>,
            pool: CachePool,
        }

        impl Clone for Regex 
        {
            fn clone(&self) -> Regex 
            {
                let pikevm = Arc::clone(&self.pikevm);
                let pool = {
                    let pikevm = Arc::clone(&self.pikevm);
                    let create = Box::new(move || Cache::new(&pikevm));
                    CachePool::new(create)
                };
                Regex { pikevm, pool }
            }
        }

        impl crate::fmt::Display for Regex 
        {
                fn fmt(&self, f: &mut crate::fmt::Formatter<'_>) -> crate::fmt::Result {
                write!(f, "{}", self.as_str())
            }
        }

        impl crate::fmt::Debug for Regex 
        {
                fn fmt(&self, f: &mut crate::fmt::Formatter<'_>) -> crate::fmt::Result {
                f.debug_tuple("Regex").field(&self.as_str()).finish()
            }
        }

        impl crate::str::FromStr for Regex 
        {
            type Err = Error;
            
            fn from_str(s: &str) -> Result<Regex, Error>
            {
                Regex::new(s)
            }
        }

        impl TryFrom<&str> for Regex 
        {
            type Error = Error;
            
            fn try_from(s: &str) -> Result<Regex, Error>
            {
                Regex::new(s)
            }
        }

        impl TryFrom<String> for Regex 
        {
            type Error = Error;
            
            fn try_from(s: String) -> Result<Regex, Error>
            {
                Regex::new(&s)
            }
        }

        impl Regex 
        {
            pub fn new(pattern: &str) -> Result<Regex, Error>
            {
                RegexBuilder::new(pattern).build()
            }
            
            #[inline] pub fn is_match(&self, haystack: &str) -> bool  { self.is_match_at( haystack, 0 ) }
            
            #[inline] pub fn find<'h>(&self, haystack: &'h str) -> Option<Match<'h>> { self.find_at( haystack, 0 ) }
            
            #[inline] pub fn find_iter<'r, 'h>(&'r self, haystack: &'h str) -> Matches<'r, 'h>
            {
                Matches {
                    haystack,
                    it: self.pikevm.find_iter(self.pool.get(), haystack.as_bytes()),
                }
            }
            
            #[inline] pub fn captures<'h>(&self, haystack: &'h str) -> Option<Captures<'h>> { self.captures_at( haystack, 0 ) }
            
            #[inline] pub fn captures_iter<'r, 'h>(
                &'r self,
                haystack: &'h str ) -> CaptureMatches<'r, 'h>
            {
                CaptureMatches {
                    haystack,
                    re: self,
                    it: self
                        .pikevm
                        .captures_iter(self.pool.get(), haystack.as_bytes()),
                }
            }
            
            #[inline] pub fn split<'r, 'h>(&'r self, haystack: &'h str) -> Split<'r, 'h>
            {
                Split { haystack, finder: self.find_iter(haystack), last: 0 }
            }
            
            #[inline] pub fn splitn<'r, 'h>(
                &'r self,
                haystack: &'h str,
                limit: usize ) -> SplitN<'r, 'h>
            {
                SplitN { splits: self.split(haystack), limit }
            }
            
            #[inline] pub fn replace<'h, R: Replacer>( &self, haystack: &'h str, rep: R ) -> Cow<'h, str> { self.replacen( haystack, 1, rep ) }
            
            #[inline] pub fn replace_all<'h, R: Replacer>( &self, haystack: &'h str, rep: R ) -> Cow<'h, str> { self.replacen( haystack, 0, rep ) }
            
            #[inline] pub fn replacen<'h, R: Replacer>( &self,
                haystack: &'h str,
                limit: usize,
                mut rep: R ) -> Cow<'h, str>
            {
                if let Some(rep) = rep.no_expansion() 
                {
                    let mut it = self.find_iter(haystack).enumerate().peekable();
                    if it.peek().is_none() {
                        return Cow::Borrowed(haystack);
                    }
                    
                    let mut new = String::with_capacity(haystack.len());
                    let mut last_match = 0;
                    for (i, m) in it {
                        new.push_str(&haystack[last_match..m.start()]);
                        new.push_str(&rep);
                        last_match = m.end();
                        if limit > 0 && i >= limit - 1 { break; }
                    }
                    new.push_str(&haystack[last_match..]);
                    return Cow::Owned(new);
                }
                
                let mut it = self.captures_iter(haystack).enumerate().peekable();
                if it.peek().is_none() {
                    return Cow::Borrowed(haystack);
                }
                let mut new = String::with_capacity(haystack.len());
                let mut last_match = 0;
                for (i, cap) in it {
                                let m = cap.get(0).unwrap();
                    new.push_str(&haystack[last_match..m.start()]);
                    rep.replace_append(&cap, &mut new);
                    last_match = m.end();
                    if limit > 0 && i >= limit - 1 {
                        break;
                    }
                }
                new.push_str(&haystack[last_match..]);
                Cow::Owned(new)
            }
        }
        
        impl Regex 
        {
            #[inline] pub fn shortest_match(&self, haystack: &str) -> Option<usize> { self.shortest_match_at( haystack, 0 ) }
            
            #[inline] pub fn shortest_match_at( &self,
                haystack: &str,
                start: usize ) -> Option<usize>
            {
                let mut cache = self.pool.get();
                let mut slots = [None, None];
                let matched = self.pikevm.search(
                    &mut cache,
                    haystack.as_bytes(),
                    start,
                    haystack.len(),
                    true,
                    &mut slots,
                );
                if !matched {
                    return None;
                }
                Some(slots[1].unwrap().get())
            }
            
            #[inline] pub fn is_match_at(&self, haystack: &str, start: usize) -> bool 
            {
                let mut cache = self.pool.get();
                self.pikevm.search(
                    &mut cache,
                    haystack.as_bytes(),
                    start,
                    haystack.len(),
                    true,
                    &mut [],
                )
            }
            
            #[inline] pub fn find_at<'h>( &self,
                haystack: &'h str,
                start: usize ) -> Option<Match<'h>>
            {
                let mut cache = self.pool.get();
                let mut slots = [None, None];
                let matched = self.pikevm.search(
                    &mut cache,
                    haystack.as_bytes(),
                    start,
                    haystack.len(),
                    false,
                    &mut slots,
                );
                if !matched {
                    return None;
                }
                let (start, end) = (slots[0].unwrap().get(), slots[1].unwrap().get());
                Some(Match::new(haystack, start, end))
            }
            
            #[inline] pub fn captures_at<'h>( &self,
                haystack: &'h str,
                start: usize ) -> Option<Captures<'h>>
            {
                let mut caps = Captures {
                    haystack,
                    slots: self.capture_locations(),
                    pikevm: Arc::clone(&self.pikevm),
                };
                let mut cache = self.pool.get();
                let matched = self.pikevm.search(
                    &mut cache,
                    haystack.as_bytes(),
                    start,
                    haystack.len(),
                    false,
                    &mut caps.slots.0,
                );
                if !matched {
                    return None;
                }
                Some(caps)
            }
            
            #[inline] pub fn captures_read<'h>( &self,
            locs: &mut CaptureLocations,
            haystack: &'h str ) -> Option<Match<'h>> { self.captures_read_at( locs, haystack, 0 ) }
            
            #[inline] pub fn captures_read_at<'h>( &self,
            locs: &mut CaptureLocations,
            haystack: &'h str,
            start: usize ) -> Option<Match<'h>>
            {
                let mut cache = self.pool.get();
                let matched = self.pikevm.search(
                    &mut cache,
                    haystack.as_bytes(),
                    start,
                    haystack.len(),
                    false,
                    &mut locs.0,
                );
                if !matched {
                    return None;
                }
                let (start, end) = locs.get(0).unwrap();
                Some(Match::new(haystack, start, end))
            }
        }

        impl Regex 
        {
            #[inline] pub fn as_str(&self) -> &str { &self.pikevm.nfa().pattern() }
            
            #[inline] pub fn capture_names(&self) -> CaptureNames<'_>
            {
                CaptureNames(self.pikevm.nfa().capture_names())
            }
            
            #[inline] pub fn captures_len(&self) -> usize { self.pikevm.nfa().group_len() }
            
            #[inline] pub fn static_captures_len(&self) -> Option<usize>
            {
                self.pikevm
                .nfa()
                .static_explicit_captures_len()
                .map(|len| len.saturating_add(1))
            }
            
            #[inline] pub fn capture_locations(&self) -> CaptureLocations 
            {
                let len = self.pikevm.nfa().group_len().checked_mul(2).unwrap();
                CaptureLocations(vec![None; len])
            }
        }
        
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Match<'h>
        {
            haystack: &'h str,
            start: usize,
            end: usize,
        }

        impl<'h> Match<'h>
        {
            #[inline] fn new(haystack: &'h str, start: usize, end: usize) -> Match<'h>
            {
                Match { haystack, start, end }
            }
            
            #[inline] pub fn start(&self) -> usize { self.start }
            #[inline] pub fn end(&self) -> usize { self.end }
            #[inline] pub fn is_empty(&self) -> bool { self.start == self.end }
            #[inline] pub fn len(&self) -> usize { self.end - self.start }
            #[inline] pub fn range(&self) -> crate::ops::Range<usize> { self.start..self.end }
            #[inline] pub fn as_str(&self) -> &'h str { &self.haystack[self.range()] }
        }

        impl<'h> crate::fmt::Debug for Match<'h>
        {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
            {
                f.debug_struct("Match")
                .field("start", &self.start)
                .field("end", &self.end)
                .field("string", &self.as_str())
                .finish()
            }
        }

        impl<'h> From<Match<'h>> for &'h str 
        {
            fn from(m: Match<'h>) -> &'h str { m.as_str() }
        }

        impl<'h> From<Match<'h>> for crate::ops::Range<usize>
        {
            fn from(m: Match<'h>) -> crate::ops::Range<usize> { m.range() }
        }

        pub struct Captures<'h>
        {
            haystack: &'h str,
            slots: CaptureLocations,            
            pikevm: Arc<PikeVM>,
        }

        impl<'h> Captures<'h> 
        {
            #[inline] pub fn get(&self, i: usize) -> Option<Match<'h>> { self.slots.get(i).map(|(s, e)| Match::new(self.haystack, s, e)) }

            #[inline] pub fn name(&self, name: &str) -> Option<Match<'h>>
            {
                let i = self.pikevm.nfa().to_index(name)?;
                self.get(i)
            }

            pub fn extract<const N: usize>(&self) -> (&'h str, [&'h str; N]) 
            {
                let len = self
                .pikevm
                .nfa()
                .static_explicit_captures_len()
                .expect("number of capture groups can vary in a match");
                
                assert_eq!(N, len, "asked for {N} groups, but must ask for {len}");
                
                let mut matched = self.iter().flatten();
                let whole_match = matched.next().expect("a match").as_str();
                let group_matches = [0; N].map(|_| 
                {
                    matched.next().expect("too few matching groups").as_str()
                });
                
                (whole_match, group_matches)
            }
            
            #[inline] pub fn expand(&self, replacement: &str, dst: &mut String) 
            {
                interpolate::string
                (
                    replacement,
                    |index, dst| 
                    {
                        let m = match self.get(index) 
                        {
                            None => return,
                            Some(m) => m,
                        };
                        
                        dst.push_str(&self.haystack[m.range()]);
                    },
                    |name| self.pikevm.nfa().to_index(name),
                    dst,
                );
            }
            
            #[inline] pub fn iter<'c>(&'c self) -> SubCaptureMatches<'c, 'h>
            {
                SubCaptureMatches 
                {
                    caps: self,
                    it: self.pikevm.nfa().capture_names().enumerate(),
                }
            }
            
            #[inline] pub fn len(&self) -> usize { self.pikevm.nfa().group_len() }
        }

        impl<'h> crate::fmt::Debug for Captures<'h>
        {
            fn fmt(&self, f: &mut crate::fmt::Formatter<'_>) -> crate::fmt::Result 
            {
                struct CapturesDebugMap<'a>
                {
                    caps: &'a Captures<'a>,
                }

                impl<'a> crate::fmt::Debug for CapturesDebugMap<'a>
                {
                    fn fmt(&self, f: &mut crate::fmt::Formatter) -> crate::fmt::Result
                    {
                        let mut map = f.debug_map();
                        let names = self.caps.pikevm.nfa().capture_names();
                        for (group_index, maybe_name) in names.enumerate() {
                            let key = Key(group_index, maybe_name);
                            match self.caps.get(group_index) {
                                None => map.entry(&key, &None::<()>),
                                Some(mat) => map.entry(&key, &Value(mat)),
                            };
                        }
                        map.finish()
                    }
                }

                struct Key<'a>(usize, Option<&'a str>);

                impl<'a> crate::fmt::Debug for Key<'a>
                {
                    fn fmt(&self, f: &mut crate::fmt::Formatter) -> crate::fmt::Result {
                        write!(f, "{}", self.0)?;
                        if let Some(name) = self.1 {
                            write!(f, "/{name:?}")?;
                        }
                        Ok(())
                    }
                }

                struct Value<'a>(Match<'a>);

                impl<'a> crate::fmt::Debug for Value<'a>
                {
                    fn fmt(&self, f: &mut crate::fmt::Formatter) -> crate::fmt::Result {
                        write!(
                            f,
                            "{}..{}/{:?}",
                            self.0.start(),
                            self.0.end(),
                            self.0.as_str()
                        )
                    }
                }

                f.debug_tuple("Captures")
                .field(&CapturesDebugMap { caps: self })
                .finish()
            }
        }
        
        impl<'h> crate::ops::Index<usize> for Captures<'h>
        {
            type Output = str;
            fn index(&self, i: usize) -> &str 
            {
                self.get(i)
                    .map(|m| m.as_str())
                    .unwrap_or_else(|| panic!("no group at index '{i}'"))
            }
        }
        
        impl<'h, 'n> crate::ops::Index<&'n str> for Captures<'h>
        {
            type Output = str;

            fn index<'a>(&'a self, name: &'n str) -> &'a str 
            {
                self.name(name)
                    .map(|m| m.as_str())
                    .unwrap_or_else(|| panic!("no group named '{name}'"))
            }
        }
        
        #[derive(Clone, Debug)]
        pub struct CaptureLocations(Vec<Option<NonMaxUsize>>);

        impl CaptureLocations 
        {
            #[inline] pub fn get(&self, i: usize) -> Option<(usize, usize)>
            {
                let slot = i.checked_mul(2)?;
                let start = self.0.get(slot).copied()??.get();
                let slot = slot.checked_add(1)?;
                let end = self.0.get(slot).copied()??.get();
                Some((start, end))
            }
            
            #[inline] pub fn len(&self) -> usize { self.0.len().checked_shr(1).unwrap() }
        }
        
        #[derive(Debug)]
        pub struct Matches<'r, 'h>
        {
            haystack: &'h str,
            it: pikevm::FindMatches<'r, 'h>,
        }

        impl<'r, 'h> Iterator for Matches<'r, 'h>
        {
            type Item = Match<'h>;

            #[inline] fn next(&mut self) -> Option<Match<'h>> { self.it.next().map(|(s, e)| Match::new(self.haystack, s, e)) }

            #[inline] fn count(self) -> usize { self.it.count() }
        }

        impl<'r, 'h> crate::iter::FusedIterator for Matches<'r, 'h> {}
        
        #[derive(Debug)]
        pub struct CaptureMatches<'r, 'h>
        {
            haystack: &'h str,
            re: &'r Regex,
            it: pikevm::CapturesMatches<'r, 'h>,
        }

        impl<'r, 'h> Iterator for CaptureMatches<'r, 'h>
        {
            type Item = Captures<'h>;

            #[inline] fn next(&mut self) -> Option<Captures<'h>>
            {
                self.it.next().map(|slots| Captures 
                {
                    haystack: self.haystack,
                    slots: CaptureLocations(slots),
                    pikevm: Arc::clone(&self.re.pikevm),
                })
            }

            #[inline] fn count(self) -> usize { self.it.count() }
        }

        impl<'r, 'h> crate::iter::FusedIterator for CaptureMatches<'r, 'h> {}
        
        #[derive(Debug)]
        pub struct Split<'r, 'h>
        {
            haystack: &'h str,
            finder: Matches<'r, 'h>,
            last: usize,
        }

        impl<'r, 'h> Iterator for Split<'r, 'h>
        {
            type Item = &'h str;

            #[inline] fn next(&mut self) -> Option<&'h str>
            {
                match self.finder.next() 
                {
                    None =>
                    {
                        let len = self.haystack.len();
                        if self.last > len { None }
                        else 
                        {
                            let range = self.last..len;
                            self.last = len + 1;                     
                            Some(&self.haystack[range])
                        }
                    }
                    
                    Some(m) =>
                    {
                        let range = self.last..m.start();
                        self.last = m.end();
                        Some(&self.haystack[range])
                    }
                }
            }
        }

        impl<'r, 't> crate::iter::FusedIterator for Split<'r, 't> {}
        
        #[derive(Debug)]
        pub struct SplitN<'r, 'h>
        {
            splits: Split<'r, 'h>,
            limit: usize,
        }

        impl<'r, 'h> Iterator for SplitN<'r, 'h>
        {
            type Item = &'h str;

            #[inline] fn next(&mut self) -> Option<&'h str>
            {
                if self.limit == 0 {
                    return None;
                }

                self.limit -= 1;
                if self.limit > 0 {
                    return self.splits.next();
                }

                let len = self.splits.haystack.len();
                if self.splits.last > len {
                                None
                } else {
                                Some(&self.splits.haystack[self.splits.last..len])
                }
            }

            #[inline] fn size_hint(&self) -> (usize, Option<usize>) {
                self.splits.size_hint()
            }
        }

        impl<'r, 't> crate::iter::FusedIterator for SplitN<'r, 't> {}
        
        #[derive(Clone, Debug)]
        pub struct CaptureNames<'r>(nfa::CaptureNames<'r>);

        impl<'r> Iterator for CaptureNames<'r>
        {
            type Item = Option<&'r str>;

            #[inline] fn next(&mut self) -> Option<Option<&'r str>>
            {
                self.0.next()
            }

            #[inline] fn size_hint(&self) -> (usize, Option<usize>) {
                self.0.size_hint()
            }

            #[inline] fn count(self) -> usize {
                self.0.count()
            }
        }

        impl<'r> ExactSizeIterator for CaptureNames<'r> {}

        impl<'r> crate::iter::FusedIterator for CaptureNames<'r> {}
        
        #[derive(Clone, Debug)]
        pub struct SubCaptureMatches<'c, 'h>
        {
            caps: &'c Captures<'h>,
            it: crate::iter::Enumerate<nfa::CaptureNames<'c>>,
        }

        impl<'c, 'h> Iterator for SubCaptureMatches<'c, 'h>
        {
            type Item = Option<Match<'h>>;

            #[inline] fn next(&mut self) -> Option<Option<Match<'h>>>
            {
                let (group_index, _) = self.it.next()?;
                Some(self.caps.get(group_index))
            }

            #[inline] fn size_hint(&self) -> (usize, Option<usize>) {
                self.it.size_hint()
            }

            #[inline] fn count(self) -> usize {
                self.it.count()
            }
        }

        impl<'c, 'h> ExactSizeIterator for SubCaptureMatches<'c, 'h> {}

        impl<'c, 'h> crate::iter::FusedIterator for SubCaptureMatches<'c, 'h> {}
        
        pub trait Replacer 
        {
            fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String);
             
            fn no_expansion<'r>(&'r mut self) -> Option<Cow<'r, str>>
            {
                None
            }
            
            fn by_ref<'r>(&'r mut self) -> ReplacerRef<'r, Self>
            {
                ReplacerRef(self)
            }
        }

        impl<'a> Replacer for &'a str 
        {
            fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) { caps.expand(*self, dst); }
            fn no_expansion(&mut self) -> Option<Cow<'_, str>> { no_expansion(self) }
        }

        impl<'a> Replacer for &'a String
        {
            fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) { self.as_str().replace_append(caps, dst) }
            fn no_expansion(&mut self) -> Option<Cow<'_, str>> { no_expansion(self) }
        }

        impl Replacer for String
        {
            fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) { self.as_str().replace_append(caps, dst) }
            fn no_expansion(&mut self) -> Option<Cow<'_, str>> { no_expansion(self) }
        }

        impl<'a> Replacer for Cow<'a, str>
        {
            fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) { self.as_ref().replace_append(caps, dst) }
            fn no_expansion(&mut self) -> Option<Cow<'_, str>> { no_expansion(self) }
        }

        impl<'a> Replacer for &'a Cow<'a, str>
        {
            fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) { self.as_ref().replace_append(caps, dst) }
            fn no_expansion(&mut self) -> Option<Cow<'_, str>> { no_expansion(self) }
        }

        impl<F, T> Replacer for F where
        F: FnMut(&Captures<'_>) -> T,
        T: AsRef<str>
        {
            fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) { dst.push_str((*self)(caps).as_ref()); }
        }

        #[derive(Debug)]
        pub struct ReplacerRef<'a, R: ?Sized>(&'a mut R);

        impl<'a, R: Replacer + ?Sized + 'a> Replacer for ReplacerRef<'a, R>
        {
            fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) { self.0.replace_append(caps, dst) }
            fn no_expansion(&mut self) -> Option<Cow<'_, str>> { self.0.no_expansion() }
        }

        #[derive(Clone, Debug)]
        pub struct NoExpand<'t>(pub &'t str);

        impl<'t> Replacer for NoExpand<'t>
        {
            fn replace_append(&mut self, _: &Captures<'_>, dst: &mut String) { dst.push_str(self.0); }
            fn no_expansion(&mut self) -> Option<Cow<'_, str>> { Some(Cow::Borrowed(self.0)) }
        }

        fn no_expansion<T: AsRef<str>>(t: &T) -> Option<Cow<'_, str>>
        {
            let s = t.as_ref();
            match s.find('$')
            {
                Some(_) => None,
                None => Some(Cow::Borrowed(s)),
            }
        }

        #[derive(Debug)]
        pub struct RegexBuilder
        {
            pattern: String,
            hir_config: hir::Config,
            nfa_config: nfa::Config,
        }

        impl RegexBuilder
        {
            pub fn new(pattern: &str) -> RegexBuilder
            {
                RegexBuilder
                {
                    pattern: pattern.to_string(),
                    hir_config: hir::Config::default(),
                    nfa_config: nfa::Config::default(),
                }
            }

            pub fn build(&self) -> Result<Regex, Error>
            {
                let hir = Hir::parse(self.hir_config, &self.pattern)?;
                let nfa = NFA::new(self.nfa_config, self.pattern.clone(), &hir)?;
                let pikevm = Arc::new(PikeVM::new(nfa));
                let pool =
                {
                    let pikevm = Arc::clone(&pikevm);
                    let create = Box::new(move || Cache::new(&pikevm));
                    CachePool::new(create)
                };

                Ok(Regex { pikevm, pool })
            }

            pub fn case_insensitive(&mut self, yes: bool) -> &mut RegexBuilder
            {
                self.hir_config.flags.case_insensitive = yes;
                self
            }
            
            pub fn multi_line(&mut self, yes: bool) -> &mut RegexBuilder
            {
                self.hir_config.flags.multi_line = yes;
                self
            }

            pub fn dot_matches_new_line(&mut self, yes: bool) -> &mut RegexBuilder
            {
                self.hir_config.flags.dot_matches_new_line = yes;
                self
            }

            pub fn crlf(&mut self, yes: bool) -> &mut RegexBuilder
            {
                self.hir_config.flags.crlf = yes;
                self
            }

            pub fn swap_greed(&mut self, yes: bool) -> &mut RegexBuilder
            {
                self.hir_config.flags.swap_greed = yes;
                self
            }

            pub fn ignore_whitespace(&mut self, yes: bool) -> &mut RegexBuilder
            {
                self.hir_config.flags.ignore_whitespace = yes;
                self
            }

            pub fn size_limit(&mut self, limit: usize) -> &mut RegexBuilder
            {
                self.nfa_config.size_limit = Some(limit);
                self
            }

            pub fn nest_limit(&mut self, limit: u32) -> &mut RegexBuilder
            {
                self.hir_config.nest_limit = limit;
                self
            }
        }
    }
    pub use self::string::*;

    pub mod utf8
    {
        /*!
        */
        use crate::
        {
            *,
        };
        /*
        */
        pub fn is_word_byte(b: u8) -> bool
        {
            const fn mkwordset() -> [bool; 256]
            {
                let mut set = [false; 256];
                set[b'_' as usize] = true;
                let mut byte = b'0';

                while byte <= b'9'
                {
                    set[byte as usize] = true;
                    byte += 1;
                }

                byte = b'A';

                while byte <= b'Z'
                {
                    set[byte as usize] = true;
                    byte += 1;
                }

                byte = b'a';

                while byte <= b'z'
                {
                    set[byte as usize] = true;
                    byte += 1;
                }

                set
            }

            const WORD: [bool; 256] = mkwordset();
            WORD[b as usize]
         }

        const ACCEPT: usize = 12;
        const REJECT: usize = 0;

        pub fn decode_lossy<B: AsRef<[u8]>>(slice: B) -> (char, usize)
        {
            match decode(slice)
            {
                (Some(ch), size) => (ch, size),
                (None, size) => ('\u{FFFD}', size),
            }
        }

        pub fn decode<B: AsRef<[u8]>>(slice: B) -> (Option<char>, usize)
        {
            let slice = slice.as_ref();
            match slice.get(0)
            {
                None => return (None, 0),
                Some(&b) if b <= 0x7F => return (Some(b as char), 1),
                _ => {}
            }

            let (mut state, mut cp, mut i) = (ACCEPT, 0, 0);

            while i < slice.len()
            {
                decode_step(&mut state, &mut cp, slice[i]);
                i += 1;

                if state == ACCEPT {
                                            //
                                            let ch = char::from_u32(cp).unwrap();
                    return (Some(ch), i);
                } else if state == REJECT {
                                return (None,  crate::cmp::max(1, i.saturating_sub(1)));
                }
            }

            (None, i)
        }

        fn decode_step(state: &mut usize, cp: &mut u32, b: u8)
        {
            #[rustfmt::skip] const CLASSES: [u8; 256] = [ 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,8,8,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,10,3,3,3,3,3,3,3,3,3,3,3,3,4,3,3, 11,6,6,6,5,8,8,8,8,8,8,8,8,8,8,8 ];
            #[rustfmt::skip] const STATES_FORWARD: &'static [u8] = &[ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 24, 36, 60, 96, 84, 0, 0, 0, 48, 72, 0, 12, 0, 0, 0, 0, 0, 12, 0, 12, 0, 0, 0, 24, 0, 0, 0, 0, 0, 24, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 36, 0, 0, 0, 36, 0, 0, 0, 0, 0, 36, 0, 36, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ];
            let class = CLASSES[usize::from(b)];

            if *state == ACCEPT { *cp = (0xFF >> class) & (b as u32); }
            else { *cp = (b as u32 & 0b111111) | (*cp << 6); }

            *state = usize::from(STATES_FORWARD[*state + usize::from(class)]);
        }
    }
}

pub mod slice
{
    pub use std::slice::{*};
}

pub mod str
{
    pub use std::str::{*};
}

pub mod string
{
    pub use std::string::{*};
}

pub mod sync
{
    pub use std::sync::{ atomic as _, *};

    pub mod atomic
    {
        pub use std::sync::atomic::{*};
    }
}

pub mod usize
{
    pub use std::usize::{*};
}

pub mod vec
{
    pub use std::vec::{*};
}

/*
create
read
edit
append
test
emit
signal
*/
unsafe fn read_module_path( from:&crate::path::Path ) -> Result<(), Box<dyn crate::error::Error>>
{
    unsafe
    {
        let file = fs::File::open( from ).expect("File not found");
        let buffer = crate::io::BufReader::new(file);

        for line in buffer.lines()
        {
            BUFFER.push( line? )
        }

        MODULES.push( from.to_string_lossy().into_owned() );

        return Ok(());
    }
}

unsafe fn read_module_line( from:&crate::path::Path ) -> Result<(), Box<dyn crate::error::Error>>
{
    unsafe
    {
        let file = fs::File::open( from ).expect("File not found");
        let buffer = crate::io::BufReader::new(file);
        let mut lines:Vec<String> = vec![];

        for line in buffer.lines()
        {
            lines.push( line? )
        }

        lines.append(  &mut BUFFER );
        BUFFER = lines.clone();
        MODULES.push( from.to_string_lossy().into_owned() );

        return Ok(());
    }
}

unsafe fn create_module() -> Result<(), Box<dyn crate::error::Error>>
{
    unsafe
    {
        for line in BUFFER.iter()
        {
            //let parts:Vec<&str> = line.split(' ').collect();
            //println!( r#"parts( {:?} )"#, parts );
            match true
            {
                true if line.starts_with( r#"#[path"# ) =>
                {
                    match true
                    {
                        true if line.ends_with( r#".rs"]"# ) =>
                        {
                            println!("Found Module File Attribute( {} )", line);

                            let re = crate::regex::Regex::new
                            (
                                r#"#\[path\s?=\s?"(.+){1}"\]"#
                            )?;

                            let caps = re.captures( r#"#[path = "de/mod.rs"]"# ).unwrap();
                            let from = caps.get(1).unwrap().as_str();
                            //println!( "{:?}", caps.get(1).unwrap().as_str() );
                            println!( "{:?}", CURRENT_PATH );
                            let buffer = read_module_line( crate::path::Path::new( from ) );
                        }

                        _  =>
                        {
                            println!("Found Module Path Attribute( {} )", line);
                        }
                    }
                }

                true if line.starts_with( r#"mod "# ) | line.starts_with( r#"pub mod "# ) =>
                {
                    match true
                    {
                        true if line.ends_with( r#";"# ) =>
                        {
                            println!("Found External Module File Attribute( {} )", line);
                        }

                        _  =>
                        {
                            println!("Found Inline Module( {} )", line);
                        }
                    }
                }

                _  =>
                {

                }
            }
        }

        return Ok(());
    }
}

unsafe fn domain() -> Result<(), Box<dyn crate::error::Error>>
{
    unsafe
    {
        /*
        use crate::regex::Regex;

        let re = Regex::new(r"(?x)
(?P<year>\d{4})  # the year
-
(?P<month>\d{2}) # the month
-
(?P<day>\d{2})   # the day
").unwrap();

        let caps = re.captures("2010-03-14").unwrap();
        assert_eq!("2010", &caps["year"]);
        assert_eq!("03", &caps["month"]);
        assert_eq!("14", &caps["day"]); */

        let arguments = crate::env::args().skip(1).collect::<Vec<String>>();

        match arguments.len()
        {
            0 =>
            {
                println!
                (
                    r#"
mod |    Replaces in the provided rust file,  every instance of
(pub) mod module_name with the contents of the cooresponding module file.

usage | mod from <input-file> to <output-file>"#
                );

                return Ok(());
            }

            _ =>
            {
                let len = arguments.len();

                if len % 2 !=0
                {
                    println!
                    (
                        r#"
error | mismatched number of arguments.
Please check the arrangement of the provided arguments.

mod | Replaces in the provided rust file,  every instance of
(pub) mod module_name with the contents of the corresponding module file.

usage | mod from <input-file> to <output-file>"#
                    );

                    return Ok(());
                }

                let current_path:String = env::current_dir().unwrap().to_str().unwrap().to_string();
                PATHS.push(current_path.clone());
                CURRENT_PATH = Some(current_path.clone());
                /*
                println!( r#"paths( {:?} )"#, PATHS );
                println!( r#"current_path( {:?} )"#, &current_path ); */

                for ( index, argument ) in arguments.iter().enumerate()
                {
                    PREVIOUS_ARGUMENT = CURRENT_ARGUMENT.clone();
                                        
                    let current_argument = argument.clone();
                    
                    match current_argument.as_str()
                    {
                        "from" | "to" =>
                        {
                            CURRENT_ARGUMENT = Some(current_argument.clone());
                        }
                        
                        current =>
                        {
                            match &PREVIOUS_ARGUMENT
                            {
                                Some( previous ) =>
                                {
                                    match previous.as_str()
                                    {
                                        "from" =>
                                        {
                                            if INPUT_FROM.is_none()
                                            {
                                                let mut arg:Vec<_> = argument.split('\\').collect();
                                                match arg.len()
                                                {
                                                    0 | 1 => {}
                                                    _ =>
                                                    {
                                                        CURRENT_PATH = Some( format!( r#"{}\{}"#, CURRENT_PATH.clone().unwrap(), arg[0] ) );
                                                    }
                                                }

                                                INPUT_FROM = Some(argument.clone());
                                                println!( r#"INPUT_FROM( {:?} )"#, INPUT_FROM )
                                            }

                                            else
                                            {
                                                println!
                                                (
                                                    r#"
error | input file has already been set.
Please check the arrangement of the provided arguments.

mod | Replaces in the provided rust file,  every instance of
(pub) mod module_name with the contents of the corresponding module file.

usage | mod from <input-file> to <output-file>"#
                                                );

                                                return Ok(());
                                            }
                                        }

                                        "to" =>
                                        {
                                            if OUTPUT_TO.is_none()
                                            {
                                                OUTPUT_TO = Some(argument.clone());
                                                println!( r#"OUTPUT_TO( {:?} )"#, OUTPUT_TO )
                                            }

                                            else
                                            {
                                                println!
                                                (
                                                    r#"
error | output file has already been set
Please check the arrangement of the provided arguments.

mod | Replaces in the provided rust file,  every instance of
(pub) mod module_name with the contents of the corresponding module file.

usage | mod from <input-file> to <output-file>"#
                                                );

                                                return Ok(());
                                            }
                                        }
                                        
                                        previously =>
                                        {
                                            println!( r#"previously( {:?} )"#, previously )
                                        }
                                    }
                                }
                                
                                None =>
                                {
                                    match ARGUMENTS.len()
                                    {
                                        0 =>
                                        {
                                            INPUT_FROM = Some(argument.clone());
                                            ARGUMENTS.push( format!( r#"from"# ) );
                                            println!( r#"INPUT_FROM( {:?} )"#, INPUT_FROM )
                                        }

                                        _=>
                                        {
                                            println!
                                            (
                                                r#"
    error | unexpected arguments
    Please check the arrangement of the provided arguments.

    mod | Replaces in the provided rust file,  every instance of
    (pub) mod module_name with the contents of the corresponding module file.

    usage | mod from <input-file> to <output-file>"#
                                            );

                                            return Ok(());
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                    ARGUMENTS.push(argument.clone());
                }

                println!( r#"ARGUMENTS( {:?} )"#, ARGUMENTS );
                /*
                let input_from = INPUT_FROM.clone().unwrap();
                let cwd = input_from.split( '\\' ).collect::<Vec<&str>>();
                println!( r#"cwd( {:?} )"#, cwd );

                match CURRENT_PATH == INPUT_FROM.unwrap().s
                {
                    true =>
                    {

                    }

                    _=>
                    {

                    }
                } */

                read_module_path(  crate::path::Path::new( &INPUT_FROM.clone().unwrap() ) )?;

                // println!( r#"BUFFER( {:?} )"#, BUFFER );

                create_module()?;

                return Ok(());
            }
        }
    }
}

fn main() -> Result<(), Box<dyn crate::error::Error>>
{
    unsafe
    {
        domain()
    }
}
// 3493
