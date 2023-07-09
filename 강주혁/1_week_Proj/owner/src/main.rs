fn main() {
    let s1 = String::from("hello");
    let s2 = calculate_length(s1);

    println!("The length of '{}' is {}.", s2.0, s2.1);

    let s3 = String::from("hello");
    let s4 = takes_and_gives_back(&s3);

    println!("s4 is {}", s4);
    println!("Is s3 still valid? {}", s3);

    let mut s5 = String::from("hello");
    change(&mut s5);
    println!("s5 is {}", s5);

    let mut s6 = String::from("hello world");

    let word = first_word(&s6);

    println!("the first word is: {}", word);

    let hello = &s6[0..5];
    let world = &s6[6..11];

    // s6.clear();

    println!("{} {}", hello, world);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn takes_and_gives_back(a_string: &String) -> &String {
    a_string
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        // println!("{} {}", i, item);
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
