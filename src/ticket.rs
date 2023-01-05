#[derive(Debug)]
pub enum Ticket {
    Backstage(f64, String),
    Vip(f64, String),
    Regular(f64),
}

pub fn show_tickets() {
    let tickets = vec![
        Ticket::Backstage(300.0, "Azka".to_owned()),
        Ticket::Vip(200.0, "Rafif".to_owned()),
        Ticket::Regular(50.0),
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(price, name) => {
                println!("Backstage ticket for {} costs {}", name, price);
            }
            Ticket::Vip(price, name) => {
                println!("VIP ticket for {} costs {}", name, price);
            }
            Ticket::Regular(price) => {
                println!("Regular ticket costs {}", price);
            }
        }
    }
}
