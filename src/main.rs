fn main() {
    //Args is an iterator. We want to skip the first element because that is the program name, so we can use the methord "skip()".
    let mut args = std::env::args().skip(1); //In rust language: binding to a variable

    //In rust, everything is immutable by default in contrast to most other languages. Thus, mutibility must be declared explicitly.
    let key = args.next().unwrap();

    //<< Unwrapping the string from Option<String>, may cause the program to panic.
    let value = args.next().expect("Value was not there"); // Write add panic message with expect(). thread 'main' panicked at 'Value was not there';

    println!("The key is '{}' and the value is '{}'", key, value); //Macro

    // let contents = format!("{}\t{}\n", key, value); // << Similar to println! macro, but outputs to string instead.
    // let write_result = std::fs::write("kv.db", contents);
    //Macros will generate code, while function calls works as in other languages.

    //Works almost like a try/ catch block.
    // match write_result {
    //     Ok(()) => {}

    //     Err(e) => {}
    // }
    //Stack allocated

    //Ownership transfer processes 2/2: The database gets moved and is now owned by
    //the binding "database". When the binding goes our of scope, the database it owns is dropped,
    //and all of its items is owned is dropped.
    let mut database = Database::new().expect("Database::new() crashed");

    //Wouldn't need to clone if this was the only time key and value would be used, because the ownership
    //is transferred, that is, moved, to the key and value variables in insert()
    //Wouldn't need to clone if this was the only time key and value would be used, because the ownership
    //is transferred, that is, moved, to the key and value variables in database.insert().
    database.insert(key.clone(), value.clone());

    //Tells rust that we acknowledge the return value, but don't care about it.
    let _ = database.flush();
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
    fn new() -> Result<Database, std::io::Error> {
        //The map binding owns a hashmap consisting of String key and value pairs.
        //The map binding goes out of scope when new() ends. However, hashmap is not
        //dropped since the ownership is transferred between bindings. That is, the hashmap
        //is moved into the database struct at the end of this function.
        let mut map = std::collections::HashMap::new();

        // The "?" replaces a match expression
        //The variable contents owns the string, meaning that string allocates memory and once
        // contents (The "owner") goes out of scope, then the String is dropped (or free in c/c++).
        //An owned string.
        let contents = std::fs::read_to_string("kv.db")?;

        //Line is a reference (borrowing in rust terms), so when line goes out of scope, nothing
        //happens.
        //As far as I understand rust right now, owned data by a variable acts like a smart pointer
        //in c++; That is, when the "smart pointer" - here: the owner - goes out of scope, the data it
        //points to gets feeed. In rust terms: the data gets dropped.
        for line in contents.lines() {
            // Single quotes denotes a char type.
            let (key, value) = line.split_once('\t').expect("Corrupt database");

            // Two ways of transfering ownership of borrowed Strings.
            map.insert(key.to_owned(), String::from(value));
        }

        //Ownership transfer processes 1/2: The hashmap gets moved into the database
        //from this new() function.
        Ok(Database { map: map })
    }

    //This is a method because the first argument is "self", in contrast to a function that wouldn't have self.
    //Comparatable to the difference between functions and methods in OOP.
    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }
    //std::io::Result works like normal Result, but Error is always bound to io::Err
    fn flush(&self) -> std::io::Result<()> {
        let mut contents = String::new();

        //The hashmap returns the key value pain in form of String references
        //when traversing through it. The given variable is a tuple and can be assigned to two variables directly.
        for (key, value) in &self.map {
            contents.push_str(key);
            contents.push('\t');
            contents.push_str(value);
            contents.push('\n');
        }
        return std::fs::write("kv.db", contents);
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        //Here again, acknowledge the returned result, but won't be used and thus ignored with _.
        let _ = self.flush();
        println!("Database is dropped!");
    }
}
