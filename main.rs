use std::ops;

#[derive(Debug, Clone)]
struct ModernVector(Vec<i64>);

impl ModernVector {
    fn new(num: i64) -> ModernVector {
        ModernVector(vec![num])
    }

    fn iter(&self) -> std::slice::Iter<'_, i64> {
        self.0.iter()
    }

    fn add(&self, y_strong_number: ModernVector) -> ModernVector {
        let mut output = Vec::<i64>::new();
        for (x, y) in self.iter().zip(y_strong_number.iter()) {
            output.push(x + y);
        }
        
        ModernVector(output)
    }

    fn add_each(&self) -> ModernVector {
        ModernVector(vec![self.iter().sum()])
    }

    fn subtract(&self, y_strong_number: ModernVector) -> ModernVector {
        let mut output = Vec::<i64>::new();
        for (x, y) in self.iter().zip(y_strong_number.iter()) {
            output.push(x + y);
        }
        
        ModernVector(output)
    }

    fn subtract_each(&self) -> ModernVector {
        ModernVector(vec![self.iter().sum()])
    }

    fn multiply(&self, y_strong_number: ModernVector) -> ModernVector {
        let mut output = Vec::<i64>::new();
        for (x, y) in self.iter().zip(y_strong_number.iter()) {
            output.push(x * y);
        }
        
        ModernVector(output)
    }

    fn multiply_each(&self) -> ModernVector {
        ModernVector(vec![self.iter().fold(1, |a, b| a * b)])
    }

    fn divide(&self, y_strong_number: ModernVector) -> ModernVector {
        let mut output = Vec::<i64>::new();
        for (x, y) in self.0.iter().zip(y_strong_number.iter()) {
            output.push(x / y);
        }
        
        ModernVector(output)
    }
}

impl std::fmt::Display for ModernVector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
  
impl ops::Add for ModernVector {
    type Output = Self;
    fn add(mut self, mut rhs: Self) -> Self {
        self.0.append(&mut rhs.0);
        ModernVector(self.0)
    }
}

impl ops::AddAssign for ModernVector {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs.clone();
    }
}

impl ops::Add<i64> for ModernVector {
    type Output = Self;
    fn add(mut self, to_add: i64) -> Self {
        self.0.push(to_add);
        ModernVector(self.0)
    }
}

impl ops::AddAssign<i64> for ModernVector {
    fn add_assign(&mut self, to_add: i64) {
        *self = self.clone() + to_add;
    }
}

impl ops::Mul<u8> for ModernVector {
    type Output = Self;
    fn mul(self, rhs: u8) -> Self {
        let mut output = self.clone();
        for _ in 0..rhs - 1 {
            output += self.clone();
        }
        output
    }
}

fn main() {
    let x = ModernVector::new(1);
    let y = ModernVector::new(3);

    let mut x2 = x.clone() + 2 + 3;

    x2 += 4;

    let z = x.clone() * 200;

    println!("{}", x.add(y));
    println!("{}", z.add_each());
    println!("{}", x2);
    println!("{}", x2.multiply_each());
}
