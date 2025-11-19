
fn main() {
    //as we understood how mutable variables works this will be straight forward.
    //we just not mention mut keyword on our variables.
    //the rust compiler will not let us mutate the variable.

    let children = 1;
    //children = 2; //error message
    println!("children are : {children}")
}