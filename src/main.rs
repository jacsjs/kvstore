// use std::collections::HashMap;
fn main() {

    //Args is an iterator. We want to skip the first element because that is the program name, so we can use the methord "skip()".
    let mut args = std::env::args().skip(1); //In rust language: binding to a variable
    //  ^^
    //In rust, everything is immutable by default in contrast to most other languages. Thus, mutibility must be declared explicitly.

    let key = args.next().unwrap(); //<< Unwrapping the string from Option<String>, may cause the program to panic.
            //^^ Option is a model to tell if a thing is there or not.

    let value = args.next().expect("Value was not there"); // Write a panic message with expect(). thread 'main' panicked at 'Value was not there';

    println!("The key is '{}' and the value is '{}'", key, value); //Macro

    let contents = format!("{}\t{}\n", key, value); // << Similar to println! macro, but outputs to string instead.
    let write_result = std::fs::write("kv.db", contents);
                    //  ^^^ Either unit, or error.
    //Macros will generate code, while function calls works as in other languages.
    

    //Works almost like a try/ catch block.
    match write_result {
        Ok(()) => {
        }
        Err(e) => {
            
        }
    }

    //Stack allocated
    let database = Database::new().expect("Database::new() crashed");
}
//Rust is not object oriented, so classic c/ c++ structs are used instead.
//Private fields are still visible to the entire file.
//Structs are stack allocated in rust
struct Database {
    map: std::collections::HashMap<String, String>,
}

//Implementation the database struct.
impl Database {
    //"Constructor" like function; the name "new" is just naming convention.
    fn new() -> Result<Database, std::io::Error>{ //retirn value denoted by ->

        //read the lv.db file
        /*
        let contents = match std::fs::read_to_string("kv.db") {

            //If ok, then bind the content to c and return c into the binding name contents.
            Ok(c) => c,
                //contens = c if treaded as a statement, and not an expression
            Err(error) => {


                //Explicit returns, while returns from functions are implicit.
                //Return a result in the error case, with the given error.
                return Result::Err(error);
            }
        };*/
        //However, this setup is so common that, in fact, we can write this instead:
        let contents = std::fs::read_to_string("kv.db")?;
        Ok(Database{
            map: std::collections::HashMap::new(),
        })
    }
}