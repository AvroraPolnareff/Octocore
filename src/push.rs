use std::time::Duration;
use rusb::{Context, UsbContext};

static ABLETON_VENDOR_ID: u16 = 0x2982;
static PUSH_2_PRODUCT_ID: u16 = 0x1967;

static FRAME_HEADER: [u8; 16] = [
    0xff, 0xcc, 0xaa, 0x88,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00
];

pub fn draw_image (image: &[u8]) {
    let mut context = Context::new().unwrap();
    let device = context.devices().unwrap().iter()
        .find(|device| {
            let descriptor = device.device_descriptor().unwrap();
            descriptor.vendor_id() == ABLETON_VENDOR_ID && descriptor.product_id() == PUSH_2_PRODUCT_ID
        }).expect("Ableton push 2 is not connected");
    let mut device_handle = device.open().expect("Error while opening a device connection");
    device_handle.claim_interface(0).unwrap();
    device_handle.write_bulk(1, &FRAME_HEADER, Duration::from_millis(17));
    device_handle.write_bulk(1, image, Duration::from_millis(17));
    std::thread::sleep(std::time::Duration::from_millis(5000));
    device_handle.release_interface(0);
}