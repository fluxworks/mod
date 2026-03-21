/*! mod | v26.3.20 */
#![allow
(
    static_mut_refs,
    unused_attributes,
    unused_unsafe,
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
    use ::
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

pub mod env
{
    pub use std::env::{*};
}

pub mod error
{
    pub use std::error::{*};
}

pub mod fs
{
    pub use std::fs::{*};
}

pub mod io
{
    pub use std::io::{*};
}

pub mod path
{
    pub use std::path::{*};
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
        //let lined:Vec<String> = BUFFER.clone();

        for line in buffer.lines()
        {
            lines.push( line? )
        }

        lines.append(  &mut BUFFER );
        BUFFER = lines.clone();

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
