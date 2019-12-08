mod input;

const BLOCK_WHITE: char = '\u{2588}';
const BLOCK_BLACK: char = ' ';

struct Layer {
    digit_count: [u32;10],
    pixels: Vec<Vec<u8>>,
}

impl Layer {
    fn new () -> Layer {
        Layer {
            digit_count: [0; 10],
            pixels: vec![vec![]],
        }
    }
}

struct Image {
    x: u32,
    y: u32,
    layers: Vec<Layer>,
}

impl Image {
    pub fn new (x: u32, y: u32, raw: &str) -> Image {
        let mut img = Image{
            x,
            y,
            layers: vec![],
        };

        assert_eq!(0, (raw.len() as u32) % (x * y), "Image dimensions do no match provided input");

        let mut ii = 0;
        let mut jj = 0;
        let mut layer = Layer::new();


        for ch in raw.chars() {
            let digit = ch.to_string().parse::<u8>().unwrap();
            layer.pixels[jj].push(digit);
            layer.digit_count[digit as usize] += 1;
            ii += 1;

            if ii == x as usize {
                ii = 0;
                jj += 1;

                if jj == y as usize {
                    img.layers.push(layer);
                    layer = Layer::new();
                    jj = 0;
                } else {
                    layer.pixels.push(vec![]);
                }
            }
        }

        img
    }

    pub fn checksum_1_by_2(&self) -> Option<u32> {
        let mut num_0s: u32 = std::u32::MAX;
        let mut result: Option<u32> = None;
        for layer in self.layers.iter() {
            if layer.digit_count[0] == num_0s {
                result = None; // Duplicate, panic?
            } else if layer.digit_count[0] < num_0s {
                num_0s = layer.digit_count[0];
                result = Some(layer.digit_count[1] * layer.digit_count[2]);
            }
        }

        result
    }

    pub fn render(&self) {
        for jj in 0..(self.y as usize) {
            for ii in 0..(self.x as usize)  {
                let mut pix: char = 'x';
                'layerLoop: for layer in self.layers.iter() {
                    match layer.pixels[jj][ii] {
                        0 => { pix = BLOCK_BLACK; break 'layerLoop; },
                        1 => { pix = BLOCK_WHITE; break 'layerLoop; }
                        2 => (), // transparent
                        _ => panic!("Unexpected pixel value {}", layer.pixels[jj][ii]),
                    }
                }
                print!("{}", pix);
            }
            print!("\n");
        }
    }
}

pub fn part1() {
    let img = Image::new(25, 6, input::TASK_INPUT);
    println!("Image checksum: {}", img.checksum_1_by_2().unwrap());
}

pub fn part2() {
    let img = Image::new(25, 6, input::TASK_INPUT);
    img.render();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_parse_test() {
        let img = Image::new(3, 2, &"123456789012");
        let layer1: Vec<Vec<u8>> = vec![
            vec![1,2,3], vec![4,5,6]
        ];
        let layer2: Vec<Vec<u8>> = vec![
            vec![7,8,9], vec![0,1,2]
        ];

        assert_eq!(2, img.layers.len());
        assert_eq!(layer1, img.layers[0].pixels);
        assert_eq!(layer2, img.layers[1].pixels);
    }

    #[test]
    fn image_checksum_test_1() {
        let img1 = Image::new(3, 2, &"123456789012");
        let img2 = Image::new(3, 2, &"223456789012");
        let img3 = Image::new(4, 2, &"1034560894442112");
        let img4 = Image::new(4, 2, &"0234507890120112");
        assert_eq!(Some(1), img1.checksum_1_by_2(), "img1 failed");
        assert_eq!(Some(0), img2.checksum_1_by_2(), "img2 failed");
        assert_eq!(Some(4), img3.checksum_1_by_2(), "img3 failed");
        assert_eq!(None, img4.checksum_1_by_2(), "img4 failed");
    }
}