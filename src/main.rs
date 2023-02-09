mod create_default_context;
mod helpers;

use helpers::{create_engine_state, create_stack, create_stdin_input, eval_source};

fn main() {
    let mut engine_state = create_engine_state();
    let mut stack = create_stack();
    let input = create_stdin_input();

    //For fancier source you may want to use heavy duty quoting like this:
    //let source = br#"""ls | length"""#;

    let source = b"ls | length";

    eval_source(
        &mut engine_state,
        &mut stack,
        source,
        "application",
        input,
        true,
    );
}
