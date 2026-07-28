//! V5 Controller

use std::sync::LazyLock;

use parking_lot::Mutex;
use roboscope_ipc::{Subscriber, controller::ControllerInput};
pub use vex_sdk::{V5_ControllerId, V5_ControllerIndex, V5_ControllerStatus};

#[derive(Debug)]
pub struct SimController {
    pub connected: V5_ControllerStatus,
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

impl Default for SimController {
    fn default() -> Self {
        Self {
            connected: V5_ControllerStatus::kV5ControllerOffline,
            left_x: 0, left_y: 0,
            right_x: 0, right_y: 0, 
            button_l1: false, button_l2: false,
            button_r1: false, button_r2: false,
            button_up: false, button_down: false,
            button_left: false, button_right: false,
            button_x: false, button_b: false,
            button_y: false, button_a: false,
            button_power: false
        }
    }
}

pub static PRIMARY_CONTROLLER: LazyLock<Mutex<SimController>> =
    LazyLock::new(|| Mutex::new(SimController::default()));

pub static SECONDARY_CONTROLLER: LazyLock<Mutex<SimController>> =
    LazyLock::new(|| Mutex::new(SimController::default()));

pub(crate) static PRIMARY_CONTROLLER_SUB: LazyLock<Mutex<Option<Subscriber<ControllerInput>>>> =
    LazyLock::new(|| Mutex::new(None));

pub(crate) static SECONDARY_CONTROLLER_SUB: LazyLock<Mutex<Option<Subscriber<ControllerInput>>>> =
    LazyLock::new(|| Mutex::new(None));

pub(crate) fn update_controller_status() {
    let mut prim_controller = PRIMARY_CONTROLLER.lock();
    if let Some(subscriber) = &*PRIMARY_CONTROLLER_SUB.lock() {
        while let Some(snapshot) = subscriber.receive().expect("Couldn't Receive Sample") {
            prim_controller.connected = match snapshot.connected {
                roboscope_ipc::controller::ControllerStatus::Offline => V5_ControllerStatus::kV5ControllerOffline,
                roboscope_ipc::controller::ControllerStatus::Wired => V5_ControllerStatus::kV5ControllerTethered,
                roboscope_ipc::controller::ControllerStatus::Wireless => V5_ControllerStatus::kV5ControllerVexnet,
            };
            prim_controller.left_x = snapshot.left_x; prim_controller.left_y = snapshot.left_y;
            prim_controller.right_x = snapshot.right_x; prim_controller.right_y = snapshot.right_y;
            prim_controller.button_l1 = snapshot.button_l1; prim_controller.button_l2 = snapshot.button_l2;
            prim_controller.button_r1 = snapshot.button_r1; prim_controller.button_r2 = snapshot.button_r2;
            prim_controller.button_up = snapshot.button_up; prim_controller.button_down = snapshot.button_down;
            prim_controller.button_left = snapshot.button_left; prim_controller.button_right = snapshot.button_right;
            prim_controller.button_x = snapshot.button_x; prim_controller.button_b = snapshot.button_b;
            prim_controller.button_y = snapshot.button_y; prim_controller.button_a = snapshot.button_a;
            prim_controller.button_power = snapshot.button_power;
        }
    }

    let mut secondary_controller = SECONDARY_CONTROLLER.lock();
    if let Some(subscriber) = &*SECONDARY_CONTROLLER_SUB.lock() {
        while let Some(snapshot) = subscriber.receive().expect("Couldn't Receive Sample") {
            secondary_controller.connected = match snapshot.connected {
                roboscope_ipc::controller::ControllerStatus::Offline => V5_ControllerStatus::kV5ControllerOffline,
                roboscope_ipc::controller::ControllerStatus::Wired => V5_ControllerStatus::kV5ControllerTethered,
                roboscope_ipc::controller::ControllerStatus::Wireless => V5_ControllerStatus::kV5ControllerVexnet,
            };
            secondary_controller.left_x = snapshot.left_x; secondary_controller.left_y = snapshot.left_y;
            secondary_controller.right_x = snapshot.right_x; secondary_controller.right_y = snapshot.right_y;
            secondary_controller.button_l1 = snapshot.button_l1; secondary_controller.button_l2 = snapshot.button_l2;
            secondary_controller.button_r1 = snapshot.button_r1; secondary_controller.button_r2 = snapshot.button_r2;
            secondary_controller.button_up = snapshot.button_up; secondary_controller.button_down = snapshot.button_down;
            secondary_controller.button_left = snapshot.button_left; secondary_controller.button_right = snapshot.button_right;
            secondary_controller.button_x = snapshot.button_x; secondary_controller.button_b = snapshot.button_b;
            secondary_controller.button_y = snapshot.button_y; secondary_controller.button_a = snapshot.button_a;
            secondary_controller.button_power = snapshot.button_power;
        }
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn vexControllerGet(id: V5_ControllerId, index: V5_ControllerIndex) -> i32 {
    update_controller_status();
    if id == V5_ControllerId::kControllerMaster {
        let cont = PRIMARY_CONTROLLER.lock();
        match index {
            V5_ControllerIndex::AnaLeftX => cont.left_x,
            V5_ControllerIndex::AnaLeftY => cont.left_y,
            V5_ControllerIndex::AnaRightX => cont.right_x,
            V5_ControllerIndex::AnaRightY => cont.right_y,
            V5_ControllerIndex::AnaSpare1 => 0,
            V5_ControllerIndex::AnaSpare2 => 0,
            V5_ControllerIndex::ButtonL1 => if cont.button_l1 { 1 } else { 0 }
            V5_ControllerIndex::ButtonL2 => if cont.button_l2 { 1 } else { 0 }
            V5_ControllerIndex::ButtonR1 => if cont.button_r1 { 1 } else { 0 }
            V5_ControllerIndex::ButtonR2 => if cont.button_r2 { 1 } else { 0 }
            V5_ControllerIndex::ButtonUp => if cont.button_up { 1 } else { 0 }
            V5_ControllerIndex::ButtonDown => if cont.button_down { 1 } else { 0 }
            V5_ControllerIndex::ButtonLeft => if cont.button_left { 1 } else { 0 }
            V5_ControllerIndex::ButtonRight => if cont.button_right { 1 } else { 0 }
            V5_ControllerIndex::ButtonX => if cont.button_x { 1 } else { 0 }
            V5_ControllerIndex::ButtonB => if cont.button_b { 1 } else { 0 }
            V5_ControllerIndex::ButtonY => if cont.button_y { 1 } else { 0 }
            V5_ControllerIndex::ButtonA => if cont.button_a { 1 } else { 0 }
            V5_ControllerIndex::ButtonSEL => if cont.button_power { 1 } else { 0 }
            V5_ControllerIndex::ButtonAll => { super::sdk_unimplemented!("vexControllerGet(V5ControllerIndex::ButtonAll)"); 0 },
            V5_ControllerIndex::BatteryCapacity => { super::sdk_unimplemented!("vexControllerGet(V5ControllerIndex::BatteryCapacity)"); 0 },
            V5_ControllerIndex::BatteryLevel => { super::sdk_unimplemented!("vexControllerGet(V5ControllerIndex::BatteryLevel)"); 0 },
            V5_ControllerIndex::Flags => { super::sdk_unimplemented!("vexControllerGet(V5ControllerIndex::Flags)"); 0 },
            V5_ControllerIndex(_) => 0
        }
    } else  {
        let cont = SECONDARY_CONTROLLER.lock();
        match index {
            V5_ControllerIndex::AnaLeftX => cont.left_x,
            V5_ControllerIndex::AnaLeftY => cont.left_y,
            V5_ControllerIndex::AnaRightX => cont.right_x,
            V5_ControllerIndex::AnaRightY => cont.right_y,
            V5_ControllerIndex::AnaSpare1 => 0,
            V5_ControllerIndex::AnaSpare2 => 0,
            V5_ControllerIndex::ButtonL1 => if cont.button_l1 { 1 } else { 0 }
            V5_ControllerIndex::ButtonL2 => if cont.button_l2 { 1 } else { 0 }
            V5_ControllerIndex::ButtonR1 => if cont.button_r1 { 1 } else { 0 }
            V5_ControllerIndex::ButtonR2 => if cont.button_r2 { 1 } else { 0 }
            V5_ControllerIndex::ButtonUp => if cont.button_up { 1 } else { 0 }
            V5_ControllerIndex::ButtonDown => if cont.button_down { 1 } else { 0 }
            V5_ControllerIndex::ButtonLeft => if cont.button_left { 1 } else { 0 }
            V5_ControllerIndex::ButtonRight => if cont.button_right { 1 } else { 0 }
            V5_ControllerIndex::ButtonX => if cont.button_x { 1 } else { 0 }
            V5_ControllerIndex::ButtonB => if cont.button_b { 1 } else { 0 }
            V5_ControllerIndex::ButtonY => if cont.button_y { 1 } else { 0 }
            V5_ControllerIndex::ButtonA => if cont.button_a { 1 } else { 0 }
            V5_ControllerIndex::ButtonSEL => if cont.button_power { 1 } else { 0 }
            V5_ControllerIndex::ButtonAll => { super::sdk_unimplemented!("vexControllerGet(V5ControllerIndex::ButtonAll)"); 0 },
            V5_ControllerIndex::BatteryCapacity => { super::sdk_unimplemented!("vexControllerGet(V5ControllerIndex::BatteryCapacity)"); 0 },
            V5_ControllerIndex::BatteryLevel => { super::sdk_unimplemented!("vexControllerGet(V5ControllerIndex::BatteryLevel)"); 0 },
            V5_ControllerIndex::Flags => { super::sdk_unimplemented!("vexControllerGet(V5ControllerIndex::Flags)"); 0 },
            V5_ControllerIndex(_) => 0
        }
    }
}
#[unsafe(no_mangle)]
pub extern "system" fn vexControllerConnectionStatusGet(
    id: V5_ControllerId,
) -> V5_ControllerStatus {
    update_controller_status();
    if id == V5_ControllerId::kControllerMaster {
        PRIMARY_CONTROLLER.lock().connected
    } else {
        SECONDARY_CONTROLLER.lock().connected
    }
}
#[unsafe(no_mangle)]
pub extern "system" fn vexControllerTextSet(id: u32, line: u32, col: u32, buf: *const u8) -> u32 {
    super::sdk_unimplemented!("vexControllerTextSet");
    Default::default()
}
