use leptess::{leptonica, tesseract};

use enigo::{Enigo, Key, KeyboardControllable};
use image::GenericImageView;
use screenshots::{Compression, Screen};
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let mut api = tesseract::TessApi::new(Some("tes"), "eng").unwrap();

    loop {
        let can_click = capture_screenshot();
        let pix = leptonica::pix_read(Path::new("screenshot.png")).unwrap();
        api.set_image(&pix);
        let mut key_pressed = "";
        println!("{key_pressed}");
        let text = api.get_utf8_text();
        if (text.as_ref().unwrap().to_lowercase().contains("a")).to_owned() {
            press_key('a');
            key_pressed = "a";
        } else if (text.as_ref().unwrap().to_lowercase().contains("w")).to_owned() {
            press_key('w');
            key_pressed = "w";
        } else if (text.as_ref().unwrap().to_lowercase().contains("s")).to_owned() {
            press_key('s');
            key_pressed = "s";
        } else if (text.as_ref().unwrap().to_lowercase().contains("d")).to_owned() {
            press_key('d');
            key_pressed = "d";
        } else {
            press_key('k');
            key_pressed = "k";
            println!("Opened the minigame");
            thread::sleep(Duration::from_millis(3000));
        }
        let elapsed = now.elapsed();
        println!("Key pressed: {}", key_pressed);
        println!("Elapsed: {:.2?}", elapsed);
    }
}

fn capture_screenshot() -> bool {
    let screen = Screen::from_point(100, 100).unwrap();

    let image = screen.capture_area(940, 470, 50, 50).unwrap();
    let buffer = image.to_png(Compression::Fast).unwrap();
    let image = image::load_from_memory(&buffer).unwrap();
    let mut count_pixels = 0;
    let mut count_dark_pixels = 0;
    for pix in image.pixels() {
        count_pixels = count_pixels + 1;
        let rgba = pix.2;
        if rgba[0] <= 40 && rgba[1] <= 40 && rgba[2] <= 40 {
            count_dark_pixels = count_dark_pixels + 1;
        }
    }
    fs::write("screenshot.png", buffer).unwrap();

    let colored_pixels = count_pixels - count_dark_pixels;

    if colored_pixels >= count_dark_pixels {
        println!(
            "Not enough dark pixels. Colored:{colored_pixels}, Dark: {count_dark_pixels} {}",
            false
        );
        return false;
    }
    return true;
}
fn press_key(key_to_press: char) {
    let mut enigo = Enigo::new();
    enigo.key_down(Key::Layout(key_to_press));
    thread::sleep(Duration::from_millis(200));
    enigo.key_up(Key::Layout(key_to_press));
}
