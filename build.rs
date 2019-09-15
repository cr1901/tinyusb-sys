extern crate cc;

use std::env;
use std::path::{PathBuf};

fn main() {
    let tusb_dir = env::var("TINYUSB_DIR");
    let use_git = tusb_dir.is_err();

    let mut tusb_root = PathBuf::from(&tusb_dir.unwrap_or(env::var("OUT_DIR").unwrap()));

    if use_git {
        tusb_root = tusb_root.join("tinyusb")
    }

    let cc = cc::Build::new()
        .files([
            tusb_root.join("src/tusb.c"),
            tusb_root.join("src/common/tusb_fifo.c"),
            tusb_root.join("src/device/usbd.c"),
            tusb_root.join("src/device/usbd_control.c"),
            tusb_root.join("src/class/msc/msc_device.c"),
            tusb_root.join("src/class/cdc/cdc_device.c"),
            tusb_root.join("src/class/hid/hid_device.c"),
            tusb_root.join("src/class/midi/midi_device.c"),
            tusb_root.join("src/class/vendor/vendor_device.c")
        ].iter())
        .include(tusb_root.join("src"))
        .compile("tinyusb");

//src/portable/$(VENDOR)/$(CHIP_FAMILY)/dcd_$(CHIP_FAMILY).c
}
