use rusb::UsbContext;
use deno_core::plugin_api::Interface;
use deno_core::plugin_api::Op;
use deno_core::plugin_api::ZeroCopyBuf;

const SUCCESS: [u8; 1] = ['1' as u8];
const FAILURE: [u8; 1] = ['0' as u8];

fn op_init_context(
  _interface: &mut dyn Interface,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Op {
  let context = rusb::Context::new().unwrap();
  let vendor_id = 0x0403;
  let product_id = 0xcc4d;
  let device = context.devices()
      .unwrap()
      .iter()
      .find(|device| {
          let device_desc = device.device_descriptor().unwrap();
          return device_desc.vendor_id() == vendor_id && device_desc.product_id() == product_id;
      });
  if device.is_none() {
    Op::Sync(Box::new(FAILURE))
  } else {
    Op::Sync(Box::new(SUCCESS))
  }
}

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
  interface.register_op("initContext", op_init_context);
}
