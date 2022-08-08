use std::collections::HashMap;

macro_rules! hey {
    ($name:expr) => {
        println!("Hey {}", $name)
    };
}

macro_rules! map {
    ($( $key:expr => $value:expr),*) => {
        {
            let mut hm = HashMap::new();
            $( hm.insert($key, $value); )*
            hm
        }
    };
}

fn main() {
    hey!("Finn");
}

#[test]
fn test() {
    main();
    let mut m = HashMap::new();
    m.insert("name", "Foo");
    let m2 = map!(
        "name" => "Foo"
    );
    assert_eq!(m, m2);
}
