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
}