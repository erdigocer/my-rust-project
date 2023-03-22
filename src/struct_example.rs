fn sum_two_numbers(n1: i32, n2: i32) -> i32 {
    return n1 + n2;
}

fn main() {
    let asdas: Vec<i32> = vec![1, 2, 3, 4, 5];
    let names: Vec<&str> = vec!["John", "Jane", "Jack"];
    let asd = 10;
    let n2 = 20;
    let sum = sum_two_numbers(asd, n2);
    println!("The sum of {asd} and {n2} is {sum}");
}

struct Person {
    name: String,
    age: u8, // The 8-bit unsigned integer type.
    height: f32,
    weight: f32,
}

impl Person {
    fn calculate_bmi(&self) -> f32 {
        return self.weight / (self.height * self.height);
    }
}

struct Home {
    address: String,
    owner: Person,
}

fn my_function() {
    let erdi = Person {
        name: String::from("Erdi"),
        age: 34,
        height: 180.0,
        weight: 80.0,
    };

    print!("adi: {}", erdi.name);

    let bmi = erdi.calculate_bmi();
    println!("BMI: {bmi}");
}
