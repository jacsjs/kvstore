fn main() {

    //Args is an iterator. We want to skip the first element because that is the program name, so we can use the methord "skip()".
    let mut args = std::env::args().skip(1); //In rust language: binding to a variable
    //  ^^
    //In rust, everything is immutable by default in contrast to most other languages. Thus, mutibility must be declared explicitly.

    let key = args.next().unwrap(); //<< Unwrapping the string from Option<String>, may cause the program to panic.
            //^^ Option is a model to tell if a thing is there or not.

    let value = args.next().unwrap(); // No such thing as null in normal rust
    
    println!("The key is '{}' and the value is '{}'", key, value);
}

