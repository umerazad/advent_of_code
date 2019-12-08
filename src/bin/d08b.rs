use std::fs;

fn print_image(layers: Vec<&[u32]>) {
  // initially the image is transparent.
  let mut image: Vec<u32> = vec![2; 25 * 6];

  let layer_count = layers.len();

  for i in 0..25 * 6 {
    // 2 = transparent so its a good default.
    let mut color = 2;
    for l in 0..layer_count {
      if layers[l][i] != 2 {
        color = layers[l][i];
        break;
      }
    }

    image[i] = color;
  }

  // NOTE: The image produces CEKUA.

  // Render image.
  for i in 0..6 {
    for j in 0..25 {
      let c = image[i * 25 + j];
      if c == 2 || c == 0 {
        print!("  ");
      } else {
        print!("{} ", c);
      }
    }
    println!();
  }
}

fn main() {
  let contents = fs::read_to_string("assets/day8_input").unwrap();
  let digits: Vec<u32> = contents
    .trim()
    .chars()
    .map(|c| c.to_digit(10).unwrap())
    .collect();

  let chunks: Vec<&[u32]> = (&digits[..]).chunks(25 * 6).collect();
  print_image(chunks);
}
