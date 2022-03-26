const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    // let (mut missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    let (missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);

    // missiles = missiles - ready;
    // println!("{} missiles left", missiles);

    println!("{} missiles left", missiles - ready);
    // let something_deathstar: i32 = 2; // unused variable error
    // STARTING_MISSILES = 12; // cannot assign to this expression
}