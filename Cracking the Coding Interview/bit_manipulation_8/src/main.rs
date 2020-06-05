/// Draw Line: A monochrome screen is stored as a single array of bytes, allowing eight consecutive
/// pixels to be stored in one byte. The screen has width w, where w is divisible by 8 (that is, no
/// byte will be split across rows). The height of the screen, of course, can be derived from the
/// length of the array and the width. Implement a function that draws a horizontal line from
/// ( xl, y) to ( x2, y).
const W: usize = 16;

fn main() {
    let mut data: [i8; 9*W/8] = [0; 9*W/8];
    draw_line(&mut data, 5, 11, 3);
    for i in &data {
        println!("{:0b}", i);
    }
}

/// This performs the actual task.
fn draw_line(data: &mut [i8; 9*W/8], x1: usize, x2: usize, y: usize) {
    let mut array_index = W/8 * y + x1 / 8;
    let byte_index = x1 % 8;

    for i in 0..(x2-x1) {
        data[array_index] = set_bit(data[array_index], (byte_index + i)%8);
        if (byte_index + i) % 8 == 7 {
            array_index += 1;
        }
    }
}

fn set_bit(num: i8, i: usize) -> i8 {
    num | (1 << i as i8)
}