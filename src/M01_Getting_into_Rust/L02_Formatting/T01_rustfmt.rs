fn main() {
    //lets make a ugly code
    //bad indentation and unnecessary whitespaces
    
    
    
    
                    println!("bad formatted code");
}

//to format this u can use rustfmt the file name(with extension obviuusly)
//that will format the file how rust programs should be written.
//try this in ur own ide and see how it works out.

//or using cargo tool.
//u can use cargo fmt( NOTE : u can run this in the root , and it can format the code for ur entire rust project)

//additional info:
//when u r an expert and working with large rust projects u will get other developers
//code in ur local machine .
//so best practice is that u should always run <cargo build> command to build ur entire project which
//creates a single executable file for u to execute.

//NOTE : cargo build runs in debug mode to see for bugs in ur RUST codebase.
//to run it u can do ./target/debug/<ur final executable file>

//NOTE for ur final released program u can run <cargo build --release> command which is 
//for final release , it takes some time to run  , but the final executable it gives to run ur program
//so that u can test , before pushing ur rust code to production.
//it will put the executable file in the targer/release directory

//so cargo debug is for debug build , and the final executable will be in target/debug folder in ur rust project/codebase.
//and cargo release is for release build.
//just keep that in mind for ur future RUST projects.

//to delete ur all compilation files , for a fresh start
//u can run : cargo clean (it will delete ur entire target folder and reset it to orignal state, and u can build for release again.)

//The cargo run command.
//u can do cargo run , to build and run ur executable in a single command.
//cargo run shows all the intermediate steps that rust compiler is going on.
//if u dont want ur terminal to be noise u can do cargo run --quiet. to build and run ur program in quiet mode.

//the cargo check command.
//so if u r a fan of a military/army movies . Army always plan for the worst outcomes.
//that is they check what can go wrong during their mission.
//so here also , ur mission is the rust program that u r writting.
//so u can run the command cargo check , to see what can go wrong in ur rust project.
//to see if u r missing out something or not.
//cargo check is generally used before building ur code . 
//u can always check if there are some syntax/any bad code u r writting in rust.
//so cargo check ensures u dont put bad code in ur build files.
