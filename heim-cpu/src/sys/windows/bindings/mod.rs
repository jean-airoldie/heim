use std::mem;

use winapi::um::sysinfoapi;
use winapi::shared::{ntdef, minwindef};

use heim_common::units::{Time, time};

pub mod power;
pub mod winternl;

const HI_T: f64 = 429.496_729_6;
const LO_T: f64 = 1e-7;

// TODO: This one can be cached in the `lazy_static`
pub unsafe fn get_system_info() -> sysinfoapi::SYSTEM_INFO {
    let mut info = mem::MaybeUninit::<sysinfoapi::SYSTEM_INFO>::uninit();
    sysinfoapi::GetSystemInfo(info.as_mut_ptr());

    info.assume_init()
}

pub trait IntoTime {
    fn into_time(self) -> Time;
}

impl IntoTime for minwindef::FILETIME {
    #[inline]
    fn into_time(self) -> Time {
        let value = (HI_T * f64::from(self.dwHighDateTime))
            + (LO_T * f64::from(self.dwLowDateTime));

        Time::new::<time::second>(value)
    }
}

impl IntoTime for ntdef::LARGE_INTEGER {
    #[inline]
    fn into_time(self) -> Time {
        let s = unsafe { self.s() };
        let value = (HI_T * f64::from(s.HighPart))
            + (LO_T * f64::from(s.LowPart));

        Time::new::<time::second>(value)
    }
}
