/// This function is private to the crate, but is accessible to all child modules.
///
/// In this case, this function formats a tz offset string.
#[defamed::defamed]
fn top_level_crate_function(
    #[def(true)] sign: bool,
    #[def] hours: u8,
    #[def] minutes: u8,
) -> String {
    format!(
        "{}{:02}:{:02}",
        if sign { '+' } else { '-' },
        hours,
        minutes
    )
}

mod inner_module {
    mod nested_inner {
        // in order to use the top-level crate function, we need to explicitly import it.
        // this also imports the macro as it has the same name.
        use crate::top_level_crate_function;

        /// This function can also call the top-level crate function.
        #[allow(unused)]
        fn nested_inner_fn() {
            let _output: String = top_level_crate_function!();
        }
    }
}

fn main() {
    println!("Hello, world!");

    // all default parameters are substituted
    println!("No offset: {}", top_level_crate_function!());

    // override specific parameters
    println!("Pos 10   : {}", top_level_crate_function!(hours = 10));
    println!(
        "Neg 10   : {}",
        top_level_crate_function!(sign = false, hours = 10)
    );
    println!(
        "Pos 4.5  : {}",
        top_level_crate_function!(hours = 4, minutes = 30)
    );
}
