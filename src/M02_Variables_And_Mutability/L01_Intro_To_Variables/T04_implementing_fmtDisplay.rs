
//in this topic we will see how to implement fmt:Display on ur custom data types.

//so as we have learned:
//{} -> fmt:Display is human facing output , it is meant for like user messages , logs , cli outputs , return type ...etc
//and also fmt:Duplay would not print strcut , we need to implement that manually thats what this topic is about.
//lets see how to do that in rust.

//{:?} -> fmt:Debug , for developer facing output , we can also do pretty print like {:?#}
//this is meant for debugging only , it prints raw format , sometimes ugly but complete for debuggin puposes.
//real world use case : u r examining what one of ur services is actually producing raw data , and u have to see very deep . so u can use debug printing.
//when we are debigging -> debug shows field names , struct layout , nested structure , option, result , enums with variatns , referencecs and pointers ,raw internal details

//import via use the 'fmt' module to make it available:
use std::fmt::{self,write};

struct Structure(i32);

//to use the '{}' marker the trait fmt:Display must be implemented manually for the type.
//here we are telling rust when u print structure print the innter number not the entire raw format of the structure.
//othewise if we use fmt:Display simply rust will tell us : "how should i print the struct u didnt tell me!!"

impl fmt::Display for Structure { //impleemt fmt:Display for the struct Structure we just defined
    //the above line means attach this trait's behavior to this (this is the struct here , Structure in this case)
    //this trait requires fmt with this exact signature , that is basically Structure supports being implemented with Display.
    //fmt:Display requires one function that is the below function right here.
    //this function decides how {} prints your type(struct)
    fn fmt(&self , f: &mut fmt::Formatter) -> fmt::Result {
        //what is the weird function signature ?
        //&self -> is same as method in other languages it is a this structure, like
        //in java it is like having  a class and u have varialbles of that class , instead of naming that varialble.
        //u would wanna the same variable so u use this.{variableName}
        //here also &self means the THIS structure.

        //f: &mut Formatter.
        //this is where you write your output.
        //think of it like : print into this output buffer.
        //fmt:Result : return value , success or failure (almost always success)


        //write strictly the first element into supploed output
        //stream 'f'. returrns fmt::Result which indicates whetethr the operation 
        //succeeded or failed . NOTE that 'write!' use syntax which is very similar to println!
        write!(f,"{}", self.0)

        //write!(f,"{}",self.0)
        //it writes to the Formatter buffer.
        //f -> output buffer.
        //"{}" -> formatting string
        //the first field of the tuple struct(Structure(i32))
        //it is equivalent to println!("{}", self.0);


        //so all this drama, we do because rust does not know how to pretty print your struct.
        //so you just manually teach it using impl Display.
    }
}

//another friendly  example

struct User {
    name : String,
}

//now we need to do pretty printing but do print a struct we cant use {}.
//we need to use debug printing , but that is a lil ugly.
//so what we can do is .
//tell rust print the struct like this
impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "user : {}" , self.name)
    }
}


//now 
fn main() {
    let U = User { name: "Peter".into()};
    println!("{}",U)
}
