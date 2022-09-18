// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Ticket {
    Backstage(String, f32),
    Vip(String, f32),
    Standard(f32),
}

fn main() {
    let standard_ticket = Ticket::Standard(100.0);
    let vip_ticket = Ticket::Vip(String::from("John"), 200.0);
    let backstage_ticket = Ticket::Backstage(String::from("Jane"), 300.0);

    let tickets = vec![standard_ticket, vip_ticket, backstage_ticket];
    for ticket in tickets {
        match ticket {
            Ticket::Standard(price) => println!("Standard ticket: ${}", price),
            Ticket::Vip(name, price) => println!("Vip ticket: ${} for {}", price, name),
            Ticket::Backstage(name, price) => println!("Backstage ticket: ${} for {}", price, name),
        }
    }
}
