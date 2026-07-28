use std::{sync::LazyLock, time::Duration};

use iceoryx2::prelude::ZeroCopySend;

pub static CONTROLLER_UPDATE_PERIOD: LazyLock<Duration> =
    LazyLock::new(|| Duration::from_millis(25));

#[derive(Debug, ZeroCopySend, PartialEq, Clone, Copy)]
#[repr(C)]
pub struct ControllerInput {
    pub connected: ControllerStatus,
    pub left_x: i32, pub left_y: i32,
    pub right_x: i32, pub right_y: i32,
    pub button_l1: bool, pub button_l2: bool,
    pub button_r1: bool, pub button_r2: bool,
    pub button_up: bool, pub button_down: bool,
    pub button_left: bool, pub button_right: bool,
    pub button_x: bool, pub button_b: bool,
    pub button_y: bool, pub button_a: bool,
    pub button_power: bool,
}

#[derive(Debug, ZeroCopySend, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub enum ControllerStatus {
    Offline,
    Wired,
    Wireless
}