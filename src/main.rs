fn main() {
    println!("Hello, world!");
    println!("My name is {}", print_name());
    println!("The number passed is {}", return_int(69));

    let var: i32 = 69;
    let mut var2: i32 = 96;
    var2 = var2 + var;
    println!("{}", var2);
    const CONST_VAR: u32 = 10_000;
    println!("{}", CONST_VAR);
    static STATIC_VAR: u64 = 10_000_000;
    println!("{}", STATIC_VAR);

    let kimi_no_na_wa: &str = "Kurome-san"; // immutable, borrowed string
    let coin_name: Box<str> = "SquirtCoin".into(); // mutable
    println!("{}", kimi_no_na_wa);
    println!("{}", coin_name);
    
    let owned_string: String = String::from("This is an Owned String"); //Stored on heap
    println!("{}", owned_string);

    print_borrowed_owned_string("borrowed", String::from("owned"));

    // str -> Heap
    // &str -> Stack
    // Heap -> size not known on compile time, done by user
    // Stack -> size know at compile time, done by compiler

    // panic_fn();
    // panic_fn2();
}

fn return_int(var: i32) -> i32 {
    var
}

fn print_name() -> &'static str {
    "Ritvik"
}

fn print_borrowed_owned_string(_str_1: &str, _str_2: String) {
    print!("{}, ", _str_1);
    println!("{}", _str_2);
}

// diverging function
// fn panic_fn() {
//     panic!("Maa chod diye bhaiyaji!"); never returns
// }
// //OR
// fn panic_fn2() -> ! {
//     panic!("Fuck Yeah!");
// }
