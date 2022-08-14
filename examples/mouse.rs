extern crate uinput;

use std::{thread, time::Duration};
use uinput::event::{
    controller::{Controller::Mouse, Mouse::Left},
    relative::{
        Position::{X, Y},
        Relative::Position,
    },
    Event::{Controller, Relative},
};

fn main() {
    let mut device = uinput::default()
        .unwrap()
        .name("test")
        .unwrap()
        .event(Controller(Mouse(Left)))
        .unwrap() // It's necessary to enable any mouse button. Otherwise Relative events would not work.
        .event(Relative(Position(X)))
        .unwrap()
        .event(Relative(Position(Y)))
        .unwrap()
        .create()
        .unwrap();

    for _ in 1..10 {
        thread::sleep(Duration::from_secs(1));

        device.send(X, 50).unwrap();
        device.send(Y, 50).unwrap();
        device.synchronize().unwrap();
    }
}
