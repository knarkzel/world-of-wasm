#![no_std]

use core::sync::atomic::{AtomicU32, Ordering};

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

#[panic_handler]
fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern {
    fn js_tan(x: f32) -> f32;
}

fn tan(x: f32) -> f32 {
    unsafe { js_tan(x) }
}

#[no_mangle]
static mut BUFFER: [u32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];

static FRAME: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub unsafe extern fn go(x: usize, y: usize) {
    render_frame_safe(&mut BUFFER, x, y)
}

fn render_frame_safe(buffer: &mut [u32; WIDTH * HEIGHT], x1: usize, y1: usize) {
    let f = FRAME.fetch_add(1, Ordering::Relaxed);

    for y2 in 0..HEIGHT {
        for x2 in 0..WIDTH {
            let x = x1 + x2;
            let y = y1 + y2;
            buffer[y2 * WIDTH + x2] = f.wrapping_add((x ^ y) as u32) | 0xFF_00_00_00;
            buffer[y2 * WIDTH + x2] ^= buffer[y2 * WIDTH + x2] << tan((x | y) as f32) as usize;
        }
    }
}
