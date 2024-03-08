use std::time::{Duration};

use rgb565::Rgb565;
use rusb::{Context, DeviceHandle, UsbContext};

static ABLETON_VENDOR_ID: u16 = 0x2982;
static PUSH_2_PRODUCT_ID: u16 = 0x1967;

static FRAME_HEADER: [u8; 16] = [
    0xff, 0xcc, 0xaa, 0x88,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00
];

static XOR_ENCODE_VALUES: [u8; 4] = [0xE7, 0xF3, 0xE7, 0xFF];

pub struct Push2 {
  device_handle: Option<DeviceHandle<Context>>
}

impl Push2 {
  pub fn new () -> Self {
    Self { device_handle: None }
  }
  pub fn connect (&mut self) {
    let context = Context::new().unwrap();
    let device = context.devices().unwrap().iter()
      .find(|device| {
        let descriptor = device.device_descriptor().unwrap();
        descriptor.vendor_id() == ABLETON_VENDOR_ID && descriptor.product_id() == PUSH_2_PRODUCT_ID
      }).expect("Ableton push 2 is not connected");
    let mut device_handle = device.open().expect("Error while opening a device connection");
    device_handle.claim_interface(0).unwrap();
    self.device_handle = Some(device_handle)

  }
  pub fn draw_image (&self, image: &mut [u8]) {
    // todo too slow ;_;
    for line in image.chunks_mut(960) {
      for frame in line.chunks_exact_mut(2) {
        let pixel = Rgb565::from_rgb565_le([frame[0], frame[1]]).to_bgr565_le();
        frame[0] = pixel[0];
        frame[1] = pixel[1];
      }
      for frame in line.chunks_exact_mut(4) {
        frame[0] = XOR_ENCODE_VALUES[0] ^ frame[0];
        frame[1] = XOR_ENCODE_VALUES[1] ^ frame[1];
        frame[2] = XOR_ENCODE_VALUES[2] ^ frame[2];
        frame[3] = XOR_ENCODE_VALUES[3] ^ frame[3];
      }
    }
    // let elapsed = now.elapsed();
    // println!("Elapsed: {:.2?}", elapsed);
    let handle = self.device_handle.as_ref().expect("Device is not connected");
    handle.write_bulk(1, &FRAME_HEADER, Duration::from_millis(8)).unwrap();
    handle.write_bulk(1, image, Duration::from_millis(8)).unwrap();
  }
}

