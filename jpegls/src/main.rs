use std::{env::args, fs::read};

use comfy_table::Table;
use entropy::{quick_entropy, quick_entropy_print};
use jpegls::{coder::code_with_prediction, tga::read_tga_to_bitmap};

fn main() {
    let args = args().collect::<Vec<_>>();

    let input_path = args.get(1).expect("Please input the input file path");

    let input_buf = read(input_path).unwrap();

    let bitmap = read_tga_to_bitmap(&input_buf);

    let input_entropy = quick_entropy(bitmap.iter().copied());
    let input_red_entropy = quick_entropy(bitmap.red.iter().copied());
    let input_green_entropy = quick_entropy(bitmap.green.iter().copied());
    let input_blue_entropy = quick_entropy(bitmap.blue.iter().copied());

    println!("Input size {}", bitmap.red.width * bitmap.red.height);
    println!("Input entropy {:.2}", input_entropy);
    println!("Input red entropy {:.2}", input_red_entropy);
    println!("Input green entropy {:.2}", input_green_entropy);
    println!("Input blue entropy {:.2}", input_blue_entropy);

    let mut table = Table::new();

    table.set_header(vec![
        "Pred",
        "Entropy",
        "Entropy R",
        "Entropy G",
        "Entropy B",
    ]);

    let mut min = (0, 8.0);
    let mut min_red = (0, 8.0);
    let mut min_green = (0, 8.0);
    let mut min_blue = (0, 8.0);

    for id in 0..8 {
        
        let red = code_with_prediction(&bitmap.red, id);
        let red_entropy = quick_entropy(red.iter().copied());
        
        let green = code_with_prediction(&bitmap.green, id);
        let green_entropy = quick_entropy(green.iter().copied());
        
        let blue = code_with_prediction(&bitmap.blue, id);
        let blue_entropy = quick_entropy(blue.iter().copied());
        
        if id == 0 {
            println!("======");
            quick_entropy_print(green.iter().copied());
            println!("======");
        }
        
        let entropy = quick_entropy(red.iter().chain(blue.iter()).chain(green.iter()).copied());

        table.add_row(vec![
            format!("{}", id),
            format!("{:.4}", entropy),
            format!("{:.4}", red_entropy),
            format!("{:.4}", green_entropy),
            format!("{:.4}", blue_entropy),
        ]);

        if entropy < min.1 {
            min = (id, entropy);
        }
        if red_entropy < min_red.1 {
            min_red = (id, red_entropy);
        }
        if blue_entropy < min_blue.1 {
            min_blue = (id, blue_entropy);
        }
        if green_entropy < min_green.1 {
            min_green = (id, green_entropy);
        }
    }

    println!("{}", table);

    println!("Min   {} = {:.4}", min.0, min.1);
    println!("Min R {} = {:.4}", min_red.0, min_red.1);
    println!("Min G {} = {:.4}", min_green.0, min_green.1);
    println!("Min B {} = {:.4}", min_blue.0, min_blue.1);
}
