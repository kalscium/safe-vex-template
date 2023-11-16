//! Operator Control

use safe_vex::{controller::{self, Controller, ControllerAnalog}, motor, port::SmartPort, rtos};

/// How long to wait inbetween opcontrol cycles
const TICK_DELAY: u32 = 50;

/// The smartport of the left drive motor
const LEFT_DRIVE_PORT: SmartPort = SmartPort::One;

/// The smartport of the right drive motor
const RIGHT_DRIVE_PORT: SmartPort = SmartPort::Two;

/// Defines the robot opcontrol entrypoint
pub fn opcontrol() {
    let mut now = rtos::millis(); // the time of the last cycle

    // opcontrol loop
    loop {
        // get y values of the two joysticks
        let ly = controller::get_analog(Controller::Master, ControllerAnalog::LeftY).unwrap();
        let ry = controller::get_analog(Controller::Master, ControllerAnalog::RightY).unwrap();

        // move the motors based upon the joystick values
        motor::move_i8(LEFT_DRIVE_PORT, false, ly).unwrap();
        motor::move_i8(RIGHT_DRIVE_PORT, true, ry).unwrap();

        // wait for the tick
        rtos::task_delay_until(&mut now, TICK_DELAY);
    }
}
