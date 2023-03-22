fn multiply(a: f32, b: f32) -> f32 {
    return a * b;
}

struct MySpy {
    haircolor: String,
    ayecolor: String,
}

pub fn secret() {
    let mut special = MySpy {
        haircolor: String::from("blue"),
        ayecolor: String::from("red"),
    };
    println!("secret hair color {}", special.haircolor);
    special.haircolor = String::from("red");
    println!("secret hair color {}", special.haircolor);
}
