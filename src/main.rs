use std::io;
use std::cmp::Ordering;
use rand::Rng;

const VALUE_TO_COMPARE: u32 = 50;

fn hint(value: u32) -> u32 {

    if value > VALUE_TO_COMPARE {
        println!("value is bigger than 50");
    } else if value == VALUE_TO_COMPARE {
        println!("value is exactly 50");
    } else {
        println!("value is less than 50");
    }

    let computed = value * 2 + 1;
    computed
}

fn try_some_loops() {
    println!("");

    let mut counter = 0;
    let result = loop {
        counter+= 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Result of loop {}", result);
    
    let mut i: i32 = 1;
    println!("start while i={}", i);
    while i > 0 || i > -5 {
        i -= 1;
        println!("new value of i: {}", i);
    }
    
    println!("now iterate over list using for-loop");
    let mylist = [5,6,7,8];
    for element in mylist.iter() {
        println!("{}", element);
    }

    println!("");
}

/* STRINGS, OWNERSHIP, */

fn learn_string_on_the_heap() {
    let mut s = String::from("hello"); // s exists on the heap (instead of stack)
    s.push_str(" from learn string");
    println!("{}", s);
    
    // let s2 = s;
    // println!("{}", s); - not work s is no longer available
    
    let s2 = s.clone(); // clone data on the heap
    println!("s: {} s2: {}", s, s2);
}

fn takes_ownership(some_string: String) {
    println!("takes_ownership - {}", some_string);
}

fn makes_copy(some_int: i32) {
    println!("makes_copy - {}", some_int);
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

/* REFERENCES AND MUTABLE REFERENCES */
fn compute_length(my_string: &String) -> usize {
    my_string.len()
}

fn amend_string(my_string: &mut String) {
    my_string.push_str(" -amended");
}

fn main(){
    let test_string = String::from("abc");
    let test_string_2 = String::from("def");
    let test_int :i32 = 22;
    takes_ownership(test_string);
    makes_copy(test_int);
    let test_string_2 = takes_and_gives_back(test_string_2);
    // test_string is no longer available
    // but test_int is available
    // println!("test_string: {}", test_string);
    println!("test_int: {}", test_int);
    // we have "new" test_string_2 (ownership came back from function), we can print it
    println!("test_string_2: {}", test_string_2);
    
    // Now we can compute a length of test_string_2, we use here a reference
    let test_string_2_length = compute_length(&test_string_2); // <------------ HERE
    println!("length of {} is {}", test_string_2, test_string_2_length);

    // and now we can create and mutate the test_string_3 using mutable reference
    let mut test_string_3 = String::from("opa");
    amend_string(&mut test_string_3); // <------------ HERE
    println!("amended string: {}", test_string_3);
    

    try_some_loops();
    learn_string_on_the_heap();
    

    println!("\n\n");
    println!("Guess the number");
    
    let secret_number: u32 = rand::thread_rng().gen_range(1, 101); //inclusive bottom, exclusive top
    //println!("The secret_number is: {}", secret_number);
    let hint_value: u32 = hint(secret_number);

    println!("If you multiply the secret by 2 and add 1 then you will have {}", hint_value);
    
    
    loop {
        println!("Please input your guess");
        let mut guess = String::new();  //like static method
        
        io::stdin() // io::stdin returns inastance of std::io::Stdin
            .read_line(&mut guess)
            .expect("Failed to read the line");
        // the whole expression return io::Result
        
        // we change Str -> u32 (unsigned int, 32 bit)
        // we want to ingnore non-numbers, we don't want to panic
        //let guess: u32 = guess.trim().parse().expect("Please type a number"); - change expect to match expr
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("`{}` is not a number", guess.trim());
                continue;
            },
        };

        println!("You guessed: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
