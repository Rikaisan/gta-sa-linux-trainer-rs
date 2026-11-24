use crate::memory;
use crate::error::Errno;

const LOC_MONEY: u32 = 0xb7ce50;
const LOC_MONEY_ANIM_1: u32= 0xb7ce54;
const LOC_MONEY_ANIM_2: u32= 0xbaa430;

pub fn get_money(pid: u32) -> Result<i32, Errno> {
    memory::get_value_at_location(pid, LOC_MONEY)
}

pub fn add_money(pid: u32, amount: i32, instant: bool) -> Result<i32, Errno> {
    let new_amount = memory::add_value_at_location(pid, LOC_MONEY, amount)?;
    if instant {
        memory::set_value_at_location(pid, LOC_MONEY_ANIM_1, new_amount)?;
        memory::set_value_at_location(pid, LOC_MONEY_ANIM_2, new_amount)?;
    }
    Ok(new_amount)
}

pub fn set_money(pid: u32, amount: i32, instant: bool) -> Result<i32, Errno> {
    memory::set_value_at_location(pid, LOC_MONEY, amount)?;
    if instant {
        memory::set_value_at_location(pid, LOC_MONEY_ANIM_1, amount)?;
        memory::set_value_at_location(pid, LOC_MONEY_ANIM_2, amount)?;
    }
    Ok(amount)
}

pub fn sub_money(pid: u32, amount: i32, instant: bool) -> Result<i32, Errno> {
    let new_amount = memory::sub_value_at_location(pid, LOC_MONEY, amount)?;
    if instant {
        memory::set_value_at_location(pid, LOC_MONEY_ANIM_1, new_amount)?;
        memory::set_value_at_location(pid, LOC_MONEY_ANIM_2, new_amount)?;
    }
    Ok(new_amount)
}