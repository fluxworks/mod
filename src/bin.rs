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


mklink C:\Users\CKGuest\norman\\mod.exe C:\Users\CKGuest\norman\target\release\mod.exe
*/
pub static mut ARGUMENTS:Vec<String> = vec![];

pub mod env
{
    pub use std::env::{*};
}

pub mod error
{
    pub use std::error::{*};
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
(pub) mod module_name with the contents of the cooresponding module file.

usage | mod from <input-file> to <output-file>"#
                    );

                    return Ok(());
                }

                for argument in arguments
                {
                    ARGUMENTS.push(argument);
                }

                println!( r#"{:?}"#, ARGUMENTS );

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
