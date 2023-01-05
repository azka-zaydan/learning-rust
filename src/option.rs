/// a Customer Class
#[derive(Debug, Clone)]
pub struct Customer {
    age: Option<i32>,
    name: String,
}

pub fn main() {
    let customer = Customer {
        age: Some(21),
        name: "Azka".to_owned(),
    };
    let customer2 = Customer {
        age: Some(22),
        name: "Rafif".to_owned(),
    };
    let customer3 = Customer {
        age: Some(23),
        name: "Azka Rafif".to_owned(),
    };
    let customers = vec![customer, customer2, customer3];
    find_customer(&customers, Some(22));
    println!("{:?}", &customers);
    println!("{:?}", customers.is_empty());
}

fn find_customer(customers: &Vec<Customer>, age: Option<i32>) -> Option<Customer> {
    for customer in customers {
        if customer.age == age {
            println!("Found customer: {}", customer.name);
            return Some(customer.clone());
        }
    }
    println!("Customer not found");
    None
}
