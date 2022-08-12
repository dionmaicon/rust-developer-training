struct Person {
    name: String,
    age: u8,
}

struct Dog {
    name: String,
    age: u8,
}

trait Voice {
    fn voice(&self);
    fn has_voice(&self) -> bool;
}

impl Voice for Person {
    fn voice(&self) {
         println!("Hello my name is: {}", self.name);
    }

    fn has_voice(&self) -> bool {
        return self.age >= 18;
    }
}

impl Voice for Dog {
    fn voice(&self) {
         println!("{} ruf ruf ruf", self.name);
    }

    fn has_voice(&self) -> bool {
        return self.age >= 2;
    }
}

pub trait MyTrait {
    fn sum(&self, _value: u32) -> u32 {
        0
    }
}

struct MyStruct {}

impl MyTrait for MyStruct {}

#[derive(Debug)]
struct MyStruct2 {
    size: u32,
}

impl MyTrait for MyStruct2 {
    fn sum(&self, value: u32) -> u32 {
        return self.size + value;
    }
}

fn print_sum(m: &impl MyTrait){
    println!("sum: {}", m.sum(3));
}

fn est_value<T, S: Into<String>>(list: &[T], est_option: S) -> T  where T: std::cmp::PartialOrd + Copy, S: Ord + Copy {
    let mut est = list[0];
    for &item in list {
        if est_option.into() == "largest" {
            if item > est {
                est = item;
            }
        }

        if est_option.into() == "smallest" {
            if item < est {
                est = item;
            }
        }
    }
    est
}

impl std::fmt::Display for MyStruct2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Size ({})", self.size)
    }
}
pub trait PrintValue {
    fn print_value(&self);
}

impl<T: std::fmt::Display> PrintValue for T {
    fn print_value(&self) {
        println!("{}", self);
    }
}


macro_rules! sumplusone{
 // macth like arm for macro
    ($a:expr,$b:expr)=>{
 // macro expand to this code
        {
// $a and $b will be templated using the value/variable provided to macro
            format!("{} + {} + 1 = {}", $a, $b, $a+$b+1)
        }
    }
}


fn main() {
    let person: Person = Person{ name: String::from("Magnata"), age: 30 };
    let dog: Dog = Dog{name: String::from("Dooug"), age: 4 };
    person.voice();
    dog.voice();
    
    println!("Has voice: {}", person.has_voice());
    println!("Has voice: {}", dog.has_voice());

    let my_struct: MyStruct = MyStruct {};
    let my_struct2: MyStruct2 = MyStruct2 { size: 90 };

    print_sum(&my_struct);
    print_sum(&my_struct2);

    // List Sum with Generics

    let list_integers = vec![102, 4,8, 4,3,8,23,436,2 ];
    println!("Largest Value {}", est_value(&list_integers, "largest"));
    println!("Smallest Value {}", est_value(&list_integers, "smallest"));
    let list_chars = vec!["asa", "casa", "casada", "casado", "casado" ];
    println!("Largest Value {}", est_value(&list_chars, "largest"));
    println!("Smallest Value {}", est_value(&list_chars, "smallest"));

    // Implementing Generics Formatter

    let my_struct2b = MyStruct2{ size: 100 };

    my_struct2b.print_value();

    println!("Sum Plus One Macro {}", sumplusone!(5, 3));
    let base: usize = 2;
    let size = std::mem::size_of::<MyStruct2>() * base.pow(30);

    println!("Space do allocate {}", size);

    let mut allocated: Vec<MyStruct2> = Vec::with_capacity(size);
    
    // 4294967295
    // 67108864
    // 4294967296

    allocated.push(MyStruct2 { size: 4 });
    allocated.push(MyStruct2 { size: 5 });
    allocated.push(MyStruct2 { size: 6 });

    println!("Size: {:#?}", std::mem::size_of_val(&allocated));
    println!("Capacity {}", allocated.capacity());
    println!("Len {}", allocated.len());
}
