#![no_std]
#![no_main]

safe_vex::entry! {
    // declare the opcontrol entrypoint
    opcontrol => safe_vex_template::opcontrol::opcontrol();
    autonomous => safe_vex_template::autonomous::autonomous();
    initialize => safe_vex_template::initialize::initialize();
    disabled => safe_vex_template::disabled::disabled();
}
