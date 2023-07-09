fn main() {
    let y = {
        let x = 3;
        x
    };

    println!("The value of y is: {y}");
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(measurement: i32, unit: char) {
    println!("Measurement: {}{}", measurement, unit);
}
