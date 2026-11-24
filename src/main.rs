mod casting;
mod memory;
mod util;

fn main() {
    if let Some(pid) = util::get_target_pid() {
        let money_location = 0xb7ce50;
        if let Some(money) = memory::get_value_at_location::<i32>(pid, money_location) {
            println!("Old Money: {}", money);
        } else {
            println!("Error reading money.");
        }

        let new_amount = 42069;

        if memory::set_value_at_location(pid, money_location, new_amount) {
            println!("New Money: {}", new_amount);
        } else {
            println!("Error setting money.");
        }
    } else {
        println!("GTA:SA not found.");
    }
}
