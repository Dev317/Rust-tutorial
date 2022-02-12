// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue : u8
}

// Tuple Struct
struct Color(u8, u8, u8);

struct Person {
    first_name : String,
    last_name: String
}

impl Person {
    // Construct person
    fn new(first: &str, last : &str) -> Person {
        Person {
            first_name : first.to_string(),
            last_name : last.to_string()
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tupple
    fn to_tupple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    // For traditional struct
    let mut c = Color {
        red : 255,
        green: 0,
        blue: 0
    };

    c.red = 200;

    println!("Color: {} {} {}", c.red, c.green, c.blue);


    // For tuple struct
    let mut d = Color(255,0,0);
    println!("Color: {} {} {}", d.0, d.1, d.2);


    let mut p = Person::new("John", "Doe");
    println!("Person {} {}", p.first_name, p.last_name);
    println!("Person {}", p.full_name());

    p.set_last_name("Wil");
    println!("Person {}", p.full_name());

    println!("Person Tupple {:?}", p.to_tupple());
}