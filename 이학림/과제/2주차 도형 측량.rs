
fn main(){
    let triangle1 = Triangle{
        base_line: 12.0,
        height: 6.0
    };
    let square1 = Square{
        width: 4.0,
        height: 8.0
    };
    let circle1 = Circle{
        radius: 7.0
    };

    println!("{:?}", triangle1.area());
    println!("{:?}", square1.length());
    println!("{:?}", circle1.area());


}

trait Shape{
    fn area(&self) -> f64;
    fn length(&self) -> f64;
}

struct Triangle{
    base_line: f64,
    height: f64
}
impl Shape for Triangle{
    fn area(&self)->f64{
        self.base_line * self.height / 2.0
    }
    fn length(&self) -> f64{
        println!("unknown!");
        0.0
    }
}

struct Square{
    width: f64,
    height: f64
}
impl Shape for Square{
    fn area(&self)->f64{
        self.width * self.height
    }
    fn length(&self) -> f64{
        (self.width + self.height) *2.0
    }
}

struct Circle{
    radius: f64
}
impl Shape for Circle{
    fn area(&self)->f64{
        self.radius * self.radius * 3.14
    }
    fn length(&self) -> f64{
        2.0 *self.radius*3.14
    }
}
