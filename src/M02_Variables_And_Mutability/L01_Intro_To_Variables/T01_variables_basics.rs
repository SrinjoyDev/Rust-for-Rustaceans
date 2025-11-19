

// a variable is a  name assigned to a program
fn main() {
    let computers : i32 = 10;
    //we assign a value to a variable.
    //for reusablity , and also maintainability .
    //so that other poeple understand what is that exact value used for?

    //u notice there is a i32.
    //what is i32 : it is a integer of 32 bits.
    //rust is a type safe language each thing each variable u define there is a data type there.
    //so for "computers" variable we are storing a number (integer) so its type becomes i32.
    //integer of 32 bits. (integer is a category of data)
    //so we define it as let variableName : typeOFvariable = "the value we are assigning";

    //if u dont use that variable u declared if u r using a rust compiler in ur local ide.
    //it will show a yellow squiggy underline.
    //that tells u declared but u have not used for anything.
    //it is just a warning , nothing to be scared about.

    //another deep level knowledge is:
    //rust will see the right hand side of ur declaration first and then the left side.

    //for example
    let oranges = 14 + 6;
    //means rust will add the value then assign it to the variable name oranges.
    //that means rust sees the right hand side of the declaration before assigning a value to a variable.
    //just for ur info to keep in mind , this will be later when u do extreme low level programming, building low latency trading servers for example

    let apples = 20;
    let fruits = apples + oranges;

    println!("Total fruits in my garden: {}" , fruits);
    //u might be worndering what is this"{} with the print statement.
    //lets see that in the next topic.

}