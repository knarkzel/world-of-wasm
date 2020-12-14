#![no_std]

use core::sync::atomic::{AtomicU32, Ordering};
use core::f32::consts::PI;

#[panic_handler]
fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern {
    fn js_sin(x: f32) -> f32;
}

fn sin(x: f32) -> f32 {
    unsafe { js_sin(x) }
}

fn cos(x: f32) -> f32 {
    sin(x - PI / 2.)
}

fn tan(x: f32) -> f32 {
    sin(x) / (cos(x))
}

const WIDTH: usize = 600;
const HEIGHT: usize = 600;
const ZOOM: f32 = 1.;

#[no_mangle]
static mut BUFFER: [u32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];

static FRAME: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub unsafe extern fn go() {
    render_frame_safe(&mut BUFFER)
}

fn render_frame_safe(buffer: &mut [u32; WIDTH * HEIGHT]) {
    let f = FRAME.fetch_add(1, Ordering::Relaxed);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let value = (x ^ y) as f32 * ZOOM;
            buffer[y * WIDTH + x] = f.wrapping_add(value as u32) | 0xFF_00_00_00;
            buffer[y * WIDTH + x] ^= buffer[y * WIDTH + x] << (tan((x | y) as f32) * ZOOM) as usize;
        }
    }
}
