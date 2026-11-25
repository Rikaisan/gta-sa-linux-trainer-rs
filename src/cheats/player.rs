#![allow(clippy::unreadable_literal)]

macro_rules! create_stat_module {
    ($name: ident, $loc: expr, $t: ty) => {
        pub mod $name {
            #![allow(dead_code)]
            use crate::error::Errno;
            use crate::memory;

            pub fn get(pid: u32) -> Result<$t, Errno> {
                memory::get_value_at_location(pid, $loc)
            }

            pub fn add(pid: u32, amount: $t) -> Result<$t, Errno> {
                memory::add_value_at_location(pid, $loc, amount)
            }

            pub fn set(pid: u32, amount: $t) -> Result<$t, Errno> {
                memory::set_value_at_location(pid, $loc, amount)?;
                Ok(amount)
            }

            pub fn sub(pid: u32, amount: $t) -> Result<$t, Errno> {
                memory::sub_value_at_location(pid, $loc, amount)
            }
        }
    };
}

create_stat_module!(health, 0x502af88, f32);
create_stat_module!(armor, 0x502af90, f32);
create_stat_module!(money, 0xb7ce50, i32);
create_stat_module!(money_anim, 0xb7ce54, i32);