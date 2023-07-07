use std::{thread, time};

fn main() {
    let image = [
        "  __",
        "<(o )___",
        " ( ._> /",
        "  '---'",
    ];

    let delay = 300;
    let margins = [(1, 1), (0, -1), (0, 1), (1, -1)];
    let width = 20;

    for (i, (x, y)) in margins.iter().cycle().enumerate() {
        draw(&image, *x + (width - i % width) as u8, *y);
        thread::sleep(time::Duration::from_millis(delay));
    }
}

fn draw(image: &[&str], margin_x: u8, margin_y: i8) {
    (0..margin_y).for_each(|_| println!());
    for line in image {
        println!("{}{}", " ".repeat(margin_x as usize), line);
    }
    (0..-margin_y).for_each(|_| println!());
}
