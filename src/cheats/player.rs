#![allow(clippy::unreadable_literal)]
use std::ops::{Add, Sub};

use crate::casting::CVoidPtr;
use crate::error::Errno;
use crate::memory;

const LOC_MONEY_ANIM_1: u32 = 0xb7ce54;
const LOC_MONEY_ANIM_2: u32 = 0xbaa430;

// TODO: Find a way to bind the type of the stat to this module instead of relying on other modules to use the correct generic

pub enum Stat {
    Armor,
    Health,
    Money,
}

impl Stat {
    pub const fn get_location(&self) -> u32 {
        match self {
            Self::Armor => 0x502af90,
            Self::Health => 0x502af88,
            Self::Money => 0xb7ce50,
        }
    }
}

pub enum PlayerAction<T> {
    GetValue(Stat),
    AddValue(Stat, T),
    SetValue(Stat, T),
    SubValue(Stat, T),
    AddInstantMoney(T),
    SetInstantMoney(T),
    SubInstantMoney(T),
}

pub fn execute_action<T>(pid: u32, action: PlayerAction<T>) -> Result<T, Errno>
where
    T: Default + Copy + Add<T, Output = T> + Sub<T, Output = T> + CVoidPtr,
{
    match action {
        PlayerAction::GetValue(stat) => memory::get_value_at_location(pid, stat.get_location()),
        PlayerAction::AddValue(stat, value) => memory::add_value_at_location(pid, stat.get_location(), value),
        PlayerAction::SetValue(stat, value) => set_value(pid, stat.get_location(), value),
        PlayerAction::SubValue(stat, value) =>  memory::sub_value_at_location(pid, stat.get_location(), value),
        PlayerAction::AddInstantMoney(_)
        | PlayerAction::SetInstantMoney(_)
        | PlayerAction::SubInstantMoney(_) => {
            let location = Stat::Money.get_location();
            let new_amount = match action {
                PlayerAction::AddInstantMoney(value) => memory::add_value_at_location(pid, location, value)?,
                PlayerAction::SetInstantMoney(value) => set_value(pid, location, value)?,
                PlayerAction::SubInstantMoney(value) =>  memory::sub_value_at_location(pid, location, value)?,
                _ => unreachable!(),
            };
            memory::set_value_at_location(pid, LOC_MONEY_ANIM_1, new_amount)?;
            memory::set_value_at_location(pid, LOC_MONEY_ANIM_2, new_amount)?;
            Ok(new_amount)
        }
    }
}

/// Wrapper around `memory::set_value_at_location` that returns the new value
fn set_value<T>(pid: u32, location: u32, amount: T) -> Result<T, Errno>
where
    T: Default + Copy + CVoidPtr,
{
    memory::set_value_at_location(pid, location, amount)?;
    Ok(amount)
}
