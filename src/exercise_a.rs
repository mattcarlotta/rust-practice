const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;
// const MISSILES: (i32, i32) = (8, 2);

pub fn run() {
    let (missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);

    println!("Exercise A: Firing {} of my {} missiles...", ready, missiles);

    println!("Exercise A: {} missiles left", missiles - ready);
}
