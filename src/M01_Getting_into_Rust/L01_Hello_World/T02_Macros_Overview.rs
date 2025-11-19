

//you can also create your own macro like:
//your code defines a macro.
//macro is not a function
//macros runs at compile-time
//it expands into actual rust code.
//it can generate code
//it can take patterns not values
//it can change syntax
//it can allow repeating code , variable arguments, pattern matching
//this is why it has a "!" at the end
macro_rules! say_hello {
    () => {
        println!("Hello!!")
    };
}

//main difference to keep in mind.
//functions runs code.
//macros generate code.

//so in short.
fn hello() {
    println!("Hello from function!");
}

macro_rules! helloMacro {
    () => {
        println!("Hello!");
    };
}

fn  main() {
    say_hello!();

    //what happens when u call ur macro ?
    //ur compiler expands it before running
    //therefore println("Hello!");
    //so the macro literally injects code into ur file.

    hello(); //u r calling the function hello here this executes at runtime
    helloMacro!(); //expands into println! first , then compiled.

}