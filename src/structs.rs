//traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
//tuple strcut
struct Color1(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}
impl Person {
    //Construct Person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }
    //get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    //set last name
    fn set_last_name(&mut self,last:&str){
        self.last_name=last.to_string();
    }

    //name to tuple
    fn to_tuple(self)->(String,String){
        (self.first_name,self.last_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    println!("color:{},{},{}", c.red, c.green, c.blue);
    c.red = 200;
    println!("color:{},{},{}", c.red, c.green, c.blue);

    let mut c1 = Color1(135, 20, 0);
    println!("color:{},{},{}", c1.0, c1.1, c1.2);
    c1.0 = 245;
    println!("color:{},{},{}", c1.0, c1.1, c1.2);

    let mut p = Person::new("willy", "Doe");
    println!("Person {}",p.full_name());
    println!("Person {} {}", p.first_name, p.last_name);
    p.set_last_name("locotron");
    println!("Person last name: {}", p.last_name);
    
    println!("Person tuple: {:?}", p.to_tuple());
    

}
