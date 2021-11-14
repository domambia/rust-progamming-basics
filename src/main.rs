mod arrays;
mod cli;
mod conditionals;
mod enums;
mod functions;
mod loops;
mod pointer_ref;
mod print_formating;
mod strings;
mod structs;
mod tuples;
mod types;
mod variables;
mod vectors;

fn main() {
    print_formating::run();
    variables::run();
    types::run();
    strings::run();
    tuples::run();
    arrays::run();
    vectors::run();
    conditionals::run();
    loops::run();
    functions::run();
    pointer_ref::run();
    structs::run();
    enums::run();
    cli::run();
}
