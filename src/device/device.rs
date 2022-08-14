use crate::{
    event::{Code, Kind, Position, Press, Release},
    Event,
};
use ffi::*;
use libc::{c_int, gettimeofday, timeval};
use nix::unistd;
use std::{mem, ptr, slice};

/// The virtual device.
pub struct Device {
    fd: c_int,
}

impl Device {
    /// Wrap a file descriptor in a `Device`.
    pub fn new(fd: c_int) -> Self {
        Device { fd }
    }

    #[doc(hidden)]
    pub fn write(
        &mut self,
        kind: c_int,
        code: c_int,
        value: c_int,
    ) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            let mut event = input_event {
                time: timeval {
                    tv_sec: 0,
                    tv_usec: 0,
                },
                kind: kind as u16,
                code: code as u16,
                value: value as i32,
            };

            gettimeofday(&mut event.time, ptr::null_mut());

            let ptr = std::ptr::addr_of!(event).cast::<u8>();
            let size = mem::size_of_val(&event);

            unistd::write(self.fd, slice::from_raw_parts(ptr, size))?;
        }

        Ok(())
    }

    /// Synchronize the device.
    pub fn synchronize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.write(EV_SYN, SYN_REPORT, 0)
    }

    /// Send an event.
    pub fn send<T: Into<Event>>(
        &mut self,
        event: T,
        value: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let event = event.into();
        self.write(event.kind(), event.code(), value)
    }

    /// Send a press event.
    pub fn press<T: Press>(&mut self, event: &T) -> Result<(), Box<dyn std::error::Error>> {
        self.write(event.kind(), event.code(), 1)
    }

    /// Send a release event.
    pub fn release<T: Release>(&mut self, event: &T) -> Result<(), Box<dyn std::error::Error>> {
        self.write(event.kind(), event.code(), 0)
    }

    /// Send a press and release event.
    pub fn click<T: Press + Release>(
        &mut self,
        event: &T,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.press(event)?;
        self.release(event)?;

        Ok(())
    }

    /// Send a relative or absolute positioning event.
    pub fn position<T: Position>(
        &mut self,
        event: &T,
        value: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.write(event.kind(), event.code(), value)
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            ui_dev_destroy(self.fd);
        }
    }
}
