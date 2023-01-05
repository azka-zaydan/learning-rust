mod option;
mod ticket;

fn main() {
    let a = 21;
    if a != 21 {
        println!("a is not 21");
        return;
    }
    println!("a is 21");
    let first_name = "Azka";
    let last_name = "Rafif";
    println!("{} {}", first_name, last_name);

    let boolbruh = false;
    match boolbruh {
        true => println!("good"),
        false => println!("bad"),
    }

    let person: Person = Person::create(&first_name, &last_name);
    person.print_full_name();
    person.print_full_name();

    let mut nums = vec![1, 2, 3];
    nums.push(4);
    println!("{:?}", nums);
    for num in nums {
        println!("{}", num);
    }
    ticket::show_tickets();
    option::main();
}
// enum Direction {
//     Up,
//     Down,
//     Left,
//     Right,
// }

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn create(first_name: &str, last_name: &str) -> Person {
        Person {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        }
    }
    fn print_full_name(&self) {
        println!("{} {}", self.first_name, self.last_name);
    }
}
