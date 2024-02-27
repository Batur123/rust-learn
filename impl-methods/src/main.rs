#[derive(Debug)]
struct Calculator {
    x: i32,
    y: i32,
}

impl Calculator {
    fn product(&self) -> i32 {
        self.x * self.y
    }

    fn sum(&self) -> i32 {
        self.x + self.y
    }

    fn minus(&self) -> i32 {
        self.x - self.y
    }

    fn divide(&self) -> f64 {
        self.x as f64 / self.y as f64
    }

    fn pow(&self) -> i32 {
        let mut result = 1;
        let mut exponent = self.y;

        while exponent > 0 {
            result *= self.x;
            exponent -= 1;
        }

        result
    }
}

fn main() {
    let x = 5;
    let y = 10;
    let rect1 = Calculator {
        x,
        y,
    };

    println!("Sum of {} and {} = {:?}", x, y, rect1.sum());
    println!("Product of {} and {} = {:?}", x, y, rect1.product());
    println!("Minus of {} and {} = {:?}", x, y, rect1.minus());
    println!("Divide of {} and {} = {:?}", x, y, rect1.divide());
    println!("Exp of {}^{} = {:?}", x, y, rect1.pow());

}