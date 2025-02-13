use std::fs;

fn main() {
    println!("Hello, world!");
    let input = read_input_file();
    part_1(input.clone());
    part_2(input);
}

fn part_1(input: Vec<Present>) {
    let result: u32 = input
        .iter()
        .map(|present| {
            let total = present.surface_area() + present.smallest_side();
            total
        })
        .sum();
    println!("Total de papel a pedir -> {}", result);
}

fn part_2(input: Vec<Present>) {
    let result: u32 = input.iter().map(|present| present.ribbon()).sum();
    println!("Largo de ribbon necesario -> {}", result);
}

// Surface Area = 2(wl + hl + hw) = 2wl + 2hl + 2hw
#[derive(Debug, Clone)]
struct Present {
    lenght: u32,
    width: u32,
    height: u32,
}

impl Present {
    fn new(l: u32, w: u32, h: u32) -> Present {
        Present {
            lenght: l,
            width: w,
            height: h,
        }
    }

    fn volume(&self) -> u32 {
        let volume = self.width * self.lenght * self.height;
        volume
    }

    fn surface_area(&self) -> u32 {
        let surface_area =
            2 * (self.width * self.lenght + self.height * self.lenght + self.height * self.width);

        surface_area
    }

    fn smallest_side(&self) -> u32 {
        let mut dimensions = vec![self.width, self.lenght, self.height];
        dimensions.sort();
        let extra = dimensions[0] * dimensions[1];

        extra
    }

    fn ribbon(&self) -> u32 {
        let mut dimensions = vec![self.width, self.lenght, self.height];
        dimensions.sort();
        let side_1 = dimensions[0];
        let side_2 = dimensions[1];
        let side_3 = dimensions[2];
        let lenght_ribbon = 2 * (side_1 + side_2) + side_1 * side_2 * side_3;
        lenght_ribbon
    }
}

fn read_input_file() -> Vec<Present> {
    let file = fs::read_to_string("input.txt").expect("Algo salio mal");

    let input: Vec<Present> = file
        .trim()
        .lines()
        .map(|line| {
            let dimensions: Vec<u32> = line
                .split('x')
                .filter_map(|dimension| dimension.parse().ok())
                .collect();
            let present = Present::new(
                dimensions[0] as u32,
                dimensions[1] as u32,
                dimensions[2] as u32,
            );
            present
        })
        .collect();

    input
}
