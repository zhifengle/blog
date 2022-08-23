use std::fmt;

struct City {
    name: String,
}

struct Country {
    name: String,
}

// -----------------------------------------
trait Stringable {
    fn to_string_test(self) -> String;
}

fn print_str(p: impl Stringable) {
    println!("{}", p.to_string_test());
}

impl Stringable for City {
    fn to_string_test(self) -> String {
        self.name
    }
}
impl Stringable for Country {
    fn to_string_test(self) -> String {
        self.name
    }
}

// -----------------------------------------

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "city = {}", self.name)
    }
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "country = {}", self.name)
    }
}

fn print_str2(p: impl fmt::Display) {
    println!("{}", p);
}

#[test]
fn t_trait_display() {
    let c1 = City {
        name: "Beijing".to_string(),
    };
    let c2 = Country {
        name: "China".to_string(),
    };
    print_str2(&c2);
    print_str2(&c1);

    println!("{}", c1.to_string());
    println!("{}", c1);
    println!("{}", c2.to_string());
    println!("{}", c2);
}

#[test]
fn t_trait_stringable() {
    let c1 = City {
        name: "Beijing".to_string(),
    };
    let c2 = Country {
        name: "China".to_string(),
    };

    print_str(c1);
    print_str(c2);
    // println!("{}", c1.name);
}
