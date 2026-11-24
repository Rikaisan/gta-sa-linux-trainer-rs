mod casting;
mod cheats;
mod error;
mod memory;
mod util;

use crate::{cheats::player::{self, PlayerAction, Stat}, error::Errno};
use clap::{Arg, ArgAction, ArgMatches, builder::Command, error::ErrorKind};
use std::{fmt::Display, str::FromStr};

fn main() {
    if let Some(pid) = util::get_target_pid() {
        let m = Command::new("GTA:SA Trainer")
            .author("Rikai")
            .version("0.0.1")
            .about("A GTA:SA trainer for Linux")
            .subcommands([
                create_gass_cmd::<i32>("money").arg(
                    Arg::new("instant")
                        .short('i')
                        .required(false)
                        .action(ArgAction::SetTrue)
                        .help("Skip the rolling animation"),
                ),
                create_gass_cmd::<f32>("armor"),
                create_gass_cmd::<f32>("health"),
            ])
            .subcommand_required(true)
            .get_matches();

        match m.subcommand() {
            Some((name @ "money", mm)) => {
                let instant: bool = mm.get_flag("instant");
                handle_gass_cmd(
                    name,
                    mm,
                    || player::execute_action(pid, PlayerAction::GetValue(Stat::Money)),
                    |amount: i32| if instant { player::execute_action(pid, PlayerAction::AddInstantMoney(amount)) } else { player::execute_action(pid, PlayerAction::AddValue(Stat::Money, amount)) },
                    |amount: i32| if instant { player::execute_action(pid, PlayerAction::SetInstantMoney(amount)) } else { player::execute_action(pid, PlayerAction::SetValue(Stat::Money, amount)) },
                    |amount: i32| if instant { player::execute_action(pid, PlayerAction::SubInstantMoney(amount)) } else { player::execute_action(pid, PlayerAction::SubValue(Stat::Money, amount)) },
                );
            }
            Some((name @ "armor", mm)) => handle_gass_cmd(
                name,
                mm,
                || player::execute_action(pid, PlayerAction::GetValue(Stat::Armor)),
                |amount: f32| player::execute_action(pid, PlayerAction::AddValue(Stat::Armor, amount)),
                |amount: f32| player::execute_action(pid, PlayerAction::SetValue(Stat::Armor, amount)),
                |amount: f32| player::execute_action(pid, PlayerAction::SubValue(Stat::Armor, amount)),
            ),
            Some((name @ "health", mm)) => handle_gass_cmd(
                name,
                mm,
                || player::execute_action(pid, PlayerAction::GetValue(Stat::Health)),
                |amount: f32| player::execute_action(pid, PlayerAction::AddValue(Stat::Health, amount)),
                |amount: f32| player::execute_action(pid, PlayerAction::SetValue(Stat::Health, amount)),
                |amount: f32| player::execute_action(pid, PlayerAction::SubValue(Stat::Health, amount)),
            ),
            Some((_, _)) | None => unreachable!(),
        }
    } else {
        println!("GTA:SA not found.");
    }
}

fn num_parser<T>(arg: &str) -> Result<T, clap::Error>
where
    T: FromStr,
{
    arg.parse::<T>()
        .map_err(|_| clap::Error::new(ErrorKind::ValueValidation))
}

fn create_gass_cmd<T>(name: &'static str) -> Command
where
    T: FromStr + Copy + Send + Sync + 'static,
{
    Command::new(name)
        .about(format!("Modify the player's {name}"))
        .subcommands([
            Command::new("get").about(format!("Gets the player's {name}")),
            Command::new("add")
                .about(format!("Adds <amount> to the player's {name}"))
                .arg(
                    Arg::new("amount")
                        .required(true)
                        .value_parser(num_parser::<T>),
                ),
            Command::new("set")
                .about(format!("Sets the player's {name} to <amount>"))
                .arg(
                    Arg::new("amount")
                        .required(true)
                        .value_parser(num_parser::<T>),
                ),
            Command::new("sub")
                .about(format!("Subtracts <amount> to the player's {name}"))
                .arg(
                    Arg::new("amount")
                        .required(true)
                        .value_parser(num_parser::<T>),
                ),
        ])
        .subcommand_required(true)
}

fn handle_gass_cmd<T, GetFn, AddFn, SetFn, SubFn>(
    name: &str,
    matches: &ArgMatches,
    get_fn: GetFn,
    add_fn: AddFn,
    set_fn: SetFn,
    sub_fn: SubFn,
) where
    T: Clone + Copy + Display + Send + Sync + 'static,
    GetFn: FnOnce() -> Result<T, Errno>,
    AddFn: FnOnce(T) -> Result<T, Errno>,
    SetFn: FnOnce(T) -> Result<T, Errno>,
    SubFn: FnOnce(T) -> Result<T, Errno>,
{
    match matches.subcommand() {
        Some(("get", _)) => match get_fn() {
            Ok(value) => println!("{name}: {value}"),
            Err(e) => eprintln!("Couldn't get {name}: {e}"),
        },
        Some(("add", msm)) => {
            if let Some(amount) = msm.get_one::<T>("amount") {
                match add_fn(*amount) {
                    Ok(new_value) => println!("{name} added! New amount: {new_value}"),
                    Err(e) => eprintln!("Couldn't add {name}: {e}"),
                }
            }
        }
        Some(("set", msm)) => {
            if let Some(amount) = msm.get_one::<T>("amount") {
                match set_fn(*amount) {
                    Ok(new_value) => println!("{name} set! New amount: {new_value}"),
                    Err(e) => eprintln!("Couldn't set {name}: {e}"),
                }
            }
        }
        Some(("sub", msm)) => {
            if let Some(amount) = msm.get_one::<T>("amount") {
                match sub_fn(*amount) {
                    Ok(new_value) => println!("{name} subtracted! New amount: {new_value}"),
                    Err(e) => eprintln!("Couldn't subtract {name}: {e}"),
                }
            }
        }
        Some((_, _)) | None => unreachable!(),
    }
}
