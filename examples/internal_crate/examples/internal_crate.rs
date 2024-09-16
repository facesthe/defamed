/// This function is private to the crate, but is accessible to all child modules.
///
/// In this case, this function formats a tz offset string.
#[defamed::defamed(crate)]
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

/// This function is also accessible to all child modules, but it can only
/// resolve calls correctly in its locally defined module.
#[defamed::defamed]
fn top_level_local_function(#[def] greet: Option<&str>) -> String {
    format!(
        "Hello{}!",
        match greet {
            Some(g) => format!(", {}", g),
            None => String::new(),
        }
    )
}

mod inner_module {
    mod nested_inner {
        // in order to use the top-level crate function, outside of its local module,
        // the attribute macro needs to be annotated with the path from the root module
        /// This function can also call the top-level crate function.
        #[allow(unused)]
        fn nested_inner_fn() {
            let _output: String = crate::top_level_crate_function!();
        }

        #[allow(unused)]
        fn nested_inner_fn_2() {
            // to use mod-local functions, they need to be explicitly imported.
            // this brings in the function and macro definitions into scope.
            use crate::top_level_local_function;

            crate::top_level_local_function!();
        }
    }

    pub fn greet() -> String {
        use crate::top_level_local_function;

        top_level_local_function!()
    }

    pub fn greet_world() -> String {
        use crate::top_level_local_function;

        top_level_local_function!(Some("world"))
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

    println!();
    println!("{}", inner_module::greet());
    println!("{}", inner_module::greet_world())
}
