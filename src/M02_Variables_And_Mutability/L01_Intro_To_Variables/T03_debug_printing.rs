
//debug printing is also called fmt:Display method in rust.
//can be used using {:?}

//as we saw in the prev topic that for fmt:Display we must manually set the traits.
//custom data types cant have fmt:Display.

//whereas in fmt:Debug that can be automatically done using fmt:Display.

//if we want to print a custom struct.
//we can define as

#[derive(Debug)] //this is telling rust we are debugging, so let us print this.
#[allow(dead_code)] //we are not using the field i32 in the struct , so it is better to specify allow dead code here otherwise rust complains about warnings.
struct PrintableStruct(i32);

fn main() {
    //struct UnprintableStruct(i32);

    //fmt:Debug makes the printable but sacrifices some elegance , Rust also provides pretty debug printing using {:#?}

    //this can be printable with derive debug that is the debug method of printing.
    println!("now {:?} will print!" , PrintableStruct(3));
    //uncomment this below line and line no 18. to see why it throws error.
    //becuase custom data types are not printable we must specify fmt:Debug to print custom data type
    //println!("print unprintablle {:?}" , UnprintableStruct(7));

    //now ur question might be how can i make the struct printable , this is just debug printing
    //lets see that in the next topic.
}