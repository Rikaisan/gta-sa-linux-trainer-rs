mod cheats;
mod casting;
mod error;
mod memory;
mod util;

use clap::{Arg, ArgAction, builder::Command, error::ErrorKind};

use crate::cheats::player;

fn money_parser(arg: &str) -> Result<i32, clap::Error> {
    Ok(arg.parse::<i32>().map_err(|_| clap::Error::new(ErrorKind::ValueValidation))?)
}

fn main() {
    if let Some(pid) = util::get_target_pid() {
        let m = Command::new("GTA:SA Trainer")
        .author("Rikai")
        .version("0.0.1")
        .about("A GTA:SA trainer for Linux")
        .subcommand(Command::new("money")
            .about("Modify the player's money")
            .arg(Arg::new("instant").short('i').required(false).action(ArgAction::SetTrue).help("Skip the rolling animation"))
            .subcommands([
                Command::new("get"),
                Command::new("add").arg(Arg::new("amount").required(true).value_parser(money_parser)),
                Command::new("set").arg(Arg::new("amount").required(true).value_parser(money_parser)),
                Command::new("sub").arg(Arg::new("amount").required(true).value_parser(money_parser)),
            ])
            .subcommand_required(true)
        ).subcommand_required(true)
        .get_matches();

        match m.subcommand() {
            Some(("money", mm)) => {
                let instant: bool = mm.get_flag("instant");
                match mm.subcommand() {
                    Some(("get", _)) => match player::get_money(pid) {
                            Ok(value) => println!("Money: {}", value),
                            Err(e) => eprintln!("Couldn't get money: {}", e),
                    },
                    Some(("add", msm)) => if let Some(amount) = msm.get_one::<i32>("amount") {
                        match player::add_money(pid, *amount, instant) {
                            Ok(new_value) => println!("Money added! New amount: {}", new_value),
                            Err(e) => eprintln!("Couldn't add money: {}", e),
                        }
                    },
                    Some(("set", msm)) => if let Some(amount) = msm.get_one::<i32>("amount") {
                        match player::set_money(pid, *amount, instant) {
                            Ok(new_value) => println!("Money set! New amount: {}", new_value),
                            Err(e) => eprintln!("Couldn't set money: {}", e),
                        }
                    },
                    Some(("sub", msm)) => if let Some(amount) = msm.get_one::<i32>("amount") {
                        match player::sub_money(pid, *amount, instant) {
                            Ok(new_value) => println!("Money subtracted! New amount: {}", new_value),
                            Err(e) => eprintln!("Couldn't subtract money: {}", e),
                        }
                    },
                    Some((_, _)) => unreachable!(),
                    None => unreachable!(),
                }
            },
            Some((_, _)) => unreachable!(),
            None => unreachable!(),
        }
    } else {
        println!("GTA:SA not found.");
    }
}
