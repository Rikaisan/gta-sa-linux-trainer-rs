use crate::memory;
use crate::error::Errno;

const LOC_MONEY: u32 = 0xb7ce50;
const LOC_MONEY_ANIM_1: u32= 0xb7ce54;
const LOC_MONEY_ANIM_2: u32= 0xbaa430;

pub fn get_money(pid: u32) -> Result<i32, Errno> {
    memory::get_value_at_location(pid, LOC_MONEY)
}

pub fn add_money(pid: u32, amount: i32, instant: bool) -> Result<i32, Errno> {
    let current: i32 = get_money(pid)?;
    let new_amount = current + amount;
    memory::set_value_at_location(pid, LOC_MONEY, new_amount)?;
    if instant {
        memory::set_value_at_location(pid, LOC_MONEY_ANIM_1, new_amount)?;
        memory::set_value_at_location(pid, LOC_MONEY_ANIM_2, new_amount)?;
    }
    get_money(pid)
}

pub fn set_money(pid: u32, amount: i32, instant: bool) -> Result<i32, Errno> {
    memory::set_value_at_location(pid, LOC_MONEY, amount)?;
    if instant {
        memory::set_value_at_location(pid, LOC_MONEY_ANIM_1, amount)?;
        memory::set_value_at_location(pid, LOC_MONEY_ANIM_2, amount)?;
    }
    get_money(pid)
}

pub fn sub_money(pid: u32, amount: i32, instant: bool) -> Result<i32, Errno> {
    let current: i32 = get_money(pid)?;
    let new_amount = current - amount;
    memory::set_value_at_location(pid, LOC_MONEY, new_amount)?;
    if instant {
        memory::set_value_at_location(pid, LOC_MONEY_ANIM_1, new_amount)?;
        memory::set_value_at_location(pid, LOC_MONEY_ANIM_2, new_amount)?;
    }
    get_money(pid)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::util::get_target_pid;

    #[test]
    fn test_set_money() {
        let target_money = 696969;
        let gta_pid = get_target_pid().unwrap();

        set_money(gta_pid, target_money, true).unwrap();
        assert_eq!(target_money, get_money(gta_pid).unwrap());
    }
}