/// Stack of Boxes: You have a stack of n boxes, with widths w_i , heights h-i , and depths d_i.
/// The boxes cannot be rotated and can only be stacked on top of one another if each box in the
/// stack is strictly larger than the box above it in width, height, and depth. Implement a method
/// to compute the height of the tallest possible stack. The height of a stack is the sum of the
/// heights of each box.
use std::u32;

fn main() {
    let mut boxes = Vec::new();
    boxes.push(StorageBox::new(120, 20, 20));
    boxes.push(StorageBox::new(120, 60, 60));
    boxes.push(StorageBox::new(60, 40, 40));
    boxes.push(StorageBox::new(60, 40, 40));
    boxes.push(StorageBox::new(40, 20, 20));
    boxes.push(StorageBox::new(40, 10, 20));
    boxes.push(StorageBox::new(30, 10, 20));
    boxes.push(StorageBox::new(30, 5, 5));

    let res = find_max_stacking(&mut boxes);
    for i in res.iter() {
        println!("h: {}, w: {}, d: {}", boxes[*i].height, boxes[*i].width, boxes[*i].depth);
    }
}

fn find_max_stacking(boxes: &mut Vec<StorageBox>) -> Vec<usize> {
    boxes.sort_unstable_by(|a, b| a.height.partial_cmp(&b.height).unwrap());
    stacking(boxes, &StorageBox::new(u32::MAX, u32::MAX, u32::MAX)).0
}

fn stacking(boxes: &[StorageBox], last_box: &StorageBox) -> (Vec<usize>, u32) {
    let mut ret = Vec::new();
    let mut current_height = 0;

    for i in 0..boxes.len() {
        let index = boxes.len()-1-i;
        if boxes[index].is_smaller_than(last_box) {
            match stacking(&boxes[..index], &boxes[index]) {
                (mut v, h) if h + boxes[index].height > current_height => {
                    v.push(index);
                    ret = v;
                    current_height = h + boxes[index].height;
                },
                (_, _) => {},
            }
        }
    }
    (ret, current_height)
}

struct StorageBox {
    width: u32,
    height: u32,
    depth: u32,
}

impl StorageBox {
    fn new(w: u32, h: u32, d: u32) -> StorageBox {
        StorageBox {
            width: w,
            height: h,
            depth: d
        }
    }

    fn is_smaller_than(&self, other: &StorageBox) -> bool {
        self.width < other.width && self.height < other.height && self.depth < other.depth
    }
}