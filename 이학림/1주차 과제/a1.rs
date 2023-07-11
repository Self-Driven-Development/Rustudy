
use std::io;
use std::io::{stdout, Write};

fn main() {
    user_interactive();
}

struct Bill{
    name: String,
    owed: f64,
    num: i32,
}
impl Bill{
    fn new(name: &str, owed: f64, num: i32) -> Self{
        Self{
            name: name.to_owned(),
            owed: owed,
            num: num
        }
    }
}

//interactive = 최종
fn user_interactive() -> Result<(), io::Error> {
    let mut bills: Vec<Bill> = vec![];
    let mut BILL_NUMS = 0;

    loop{
        println!("1:영수증 추가, 2:영수증 조회, 3:영수증 삭제, 4:영수증 수정, 5:이전 작업 취소");
        print!("어떤 작업을 하시겠어요? => ");
        io::stdout().flush(); 

        let user_input = get_input()?;
        match user_input.as_str() {
            "1" => add_bill(&mut bills, &mut BILL_NUMS),
            "2" => read_bill(&mut bills),
            "3" => delete_bill(&mut bills),
            "4" => update_bill(&mut bills, &mut BILL_NUMS),
            _ => println!("not impl yet"),
        }

        println!("");
    }

}

//get input
fn get_input() -> Result<String, io::Error> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    
    Ok(buffer.trim().to_owned())
}

//add bills
fn add_bill(bills: &mut Vec<Bill>, BILL_NUMS: &mut i32) {
    let mut result: Result<String, io::Error>;
    let mut name = String::new();
    let mut owed: f64 = 0.0;

    print!("이름: ");
    io::stdout().flush(); 
    result = get_input();
    match result{
        Ok(buffer) => {
            name = buffer;
        }
        Err(_) => {
            println!("Add failed - IO issue");
        }
    }
    print!("가격: ");
    io::stdout().flush(); 
    result = get_input();
    match result{
        Ok(buffer) => {
            owed = buffer.parse().unwrap();
        }
        Err(_) => {
            println!("Add failed - IO issue");
        }
    }

    let bill = Bill::new(&name, owed, *BILL_NUMS);
    *BILL_NUMS += 1;
    bills.push(bill);
}

// Read bills
fn read_bill(bills: &Vec<Bill>){
    let mut result: Result<String, io::Error>;
    let mut i: i32 = 0;

    print!("이름: ");
    io::stdout().flush(); 
    result = get_input();

    match result{
        Ok(name) => {

            for bill in bills{
                if bill.name == name{
                    println!("[{:?}] #{:?} name: {:?}, owed: {:?}", i, bill.num, bill.name, bill.owed);
                    i+=1;
                }
            }
            if i == 0 {
                println!("name: {:?} bill NOT FOUND!", name);
            }
        }
        Err(_) => {
            println!("Read failed - IO issue");
        }
    }
}

// Delete bills
fn delete_bill(bills: &mut Vec<Bill>){
    let mut result: Result<String, io::Error>;
    let mut bill_num = -1;
    let mut index = 0;

    print!("삭제할 영수증 번호: ");
    io::stdout().flush(); 
    result = get_input();

    match result{
        Ok(num) => {
            bill_num = num.parse().unwrap();

            for bill in &mut *bills{
                if bill.num == bill_num{
                    bills.remove(index);
                    println!("Delete #{:?}", bill_num);
                    break;
                }
                index+=1;
            }
            if index == bills.len(){
                println!("#{:?} does not exist", bill_num);
            }
        }
        Err(_) => {
            println!("Add failed - IO issue");
        }
    }


}

// Update bills
fn update_bill(bills: &mut Vec<Bill>, BILL_NUMS: &mut i32){
    let mut result: Result<String, io::Error>;
    let mut bill_num = -1;
    let mut price:f64 = 0.0;
    let mut index = 0;
    let mut exist = false;

    print!("바꿀 영수증 번호: ");
    io::stdout().flush(); 
    result = get_input();

    match result{
        Ok(num) => {
            bill_num = num.parse().unwrap();

            for bill in &mut *bills{
                if bill.num == bill_num{
                    println!("#{:?} name: {:?}, owed: {:?}", bill.num, bill.name, bill.owed);
                    exist = true;
                    break;
                }
                index+=1;
            }
            if !exist{
                println!("#{:?} does not exist", bill_num);
            }
        }
        Err(_) => {
            println!("Add failed - IO issue");
        }
    }

    if exist{
        print!("수정할 금액: ");
        io::stdout().flush(); 
        result = get_input();

        match result{
            Ok(prices) => {
                price = prices.parse().unwrap();
                let bill: Bill = Bill::new(
                    bills[index].name.as_str(),
                    price,
                    bills[index].num,
                );

                bills.remove(index);
                bills.push(bill);

                println!("=> owed: {:?}", price);
            }
            Err(_) => {
                println!("Add failed - IO issue");
            }
        }
    }
}

// Roll back
