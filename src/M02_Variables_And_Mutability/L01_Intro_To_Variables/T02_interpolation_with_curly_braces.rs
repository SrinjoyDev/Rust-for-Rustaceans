

fn main(){
    let apples = 10;
    let oranges = 10 + 10;
    let fruits = apples + oranges;

    println!("My garden has {} fruits" , fruits);
    //the curly brackets is nothig but an opening in ur output string
    //when we are printing something it is a string that is a programmer's term for somthing in " " double quotes.
    //so with a curly braces we are telling rust to create an opening in that string for dynamic variables in it.
    //so we want fruits to make space there.
    //thats why if we run now the output would be "My garden has 30 fruits"
    //the value of fruits is interpolated in the output string.

    //in other rust terms it is using fmt:Display method in rust.
    //we will know about this in more debth in time. :)

    //we can also do argument indexing in display methods.
    //for example
    println!("{0} , this is {1}. {1} , this is {0}" , "Srinjoy" , "Vijay");
    //so Srinjoy is the argument here at index 0 , so whatever that is there in 0 index will be replaced by Srinjoy
    //and similarly vijay is at index 1 so it will be replaced in the 1 places.

    //similarly we can do as named arguments as well.
    println!("{name} {age} {hobby}" , 
        name="Peter",
        age = 18,
        hobby=  "play football"
    );

    //just for ur info {} this means , that we are telling rust to print that reference in a human friendly way.
    //we will learn more about this , why we are telling it human friendly way.

    //different formatting can be invoked by specifying the format charecter>
    println!("Base 10:               {}",   69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c

    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>5}", number=1);

    // You can pad numbers with extra zeroes,
    println!("{number:0>5}", number=1); // 00001
    // and left-adjust by flipping the sign. This will output "10000".
    println!("{number:0<5}", number=1); // 10000

    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0>width$}", number=1, width=5);

    let number : f64 = 5.0; //64 bit float number 5
    let width: usize = 5; //a number type used for sizes , to tell rust , make output 5 charecters wide.
    println!("{number:>width$}");
    //{} -> display number
    //:> width 5 charecters wide.
    //shift to right
    //    1 (4 spaces and 1)

    //we define struct (that is an interface) , if u r coming from a differenet language , like this in rust.
    #[allow(dead_code)] //this is telling rust it is a dead code , it is used no where , so dont throw a warning here.
    struct Structure(i32); //this is a custom struct we defined.
    //it means make a structure that contains a 32 bit integer.

    //rust dosent let us print this using the diplay method that is the "{}" or fmt:Display in rust terms.
    //display ({}) works with numbers , strings,floats...etc
    //but not with custom data types.
    //we can print a struct using fmt:Debug that is the debug method of printing,  that is the programmer way of printing raw data.

    //another thing ,if someone asks u what is fmt:Display and fmt:Debug.
    //they are traits in rust.
    //traits are a collection of methods that a type must implement.
    //think of traits like a contract or set of rules.
    // If a struct “implements” a trait, it agrees to provide the methods defined by that trait.
    //if u r coming from java , traits = interfaces in java.

    // trait Animal {
    //     fn speak(&self);
    // }

    //we will learn about traits more later.

    //lets learn more about debug printing in the next topic.
    
    
}