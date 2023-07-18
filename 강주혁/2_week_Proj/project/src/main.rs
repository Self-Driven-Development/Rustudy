#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }

    fn get_age(&self) -> String {
        self.age.to_string()
    }

    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn set_age(&mut self, age: u32) {
        self.age = age;
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

fn main() {
    let mut line = String::new();
    println!("이름을 입력해주세요 :");
    std::io::stdin().read_line(&mut line).unwrap();
    let name = line.trim().to_string();
    line.clear();

    println!("나이를 입력해주세요 :");
    std::io::stdin().read_line(&mut line).unwrap();
    let age = line.trim().parse::<u32>().unwrap();

    let mut person = Person::new(name, age);

    println!("이름 : {}", person.get_name());
    println!("나이 : {}", person.get_age());
    println!("{:?}", person);

    println!("{}", "=".repeat(20));

    person.set_name("홍길동".to_string());
    person.set_age(20);

    println!("이름 : {}", person.get_name());
}
