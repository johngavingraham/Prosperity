use image::{GrayImage, Luma};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use vm_parser::vm_execute;

#[derive(Debug)]
struct Program {
    instrs: Vec<Instruction>,
}

impl Program {
    fn execute(&self, vx: f64, vy: f64) -> f64 {
        vm_execute!();
    }
}

impl From<&str> for Program {
    fn from(s: &str) -> Self {
        Self {
            instrs: s.lines().skip(1).map(|line| line.into()).collect(),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    out: usize,
    op: Op,
}

fn parse_id(id: &str) -> usize {
    usize::from_str_radix(&id[1..], 16).unwrap()
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let mut words = s.split_ascii_whitespace();
        let out = parse_id(words.next().unwrap());
        let op = match words.next().unwrap() {
            "var-x" => Op::VarX,
            "var-y" => Op::VarY,
            "add" => Op::Add(
                parse_id(words.next().unwrap()),
                parse_id(words.next().unwrap()),
            ),
            "sub" => Op::Sub(
                parse_id(words.next().unwrap()),
                parse_id(words.next().unwrap()),
            ),
            "mul" => Op::Mul(
                parse_id(words.next().unwrap()),
                parse_id(words.next().unwrap()),
            ),
            "max" => Op::Max(
                parse_id(words.next().unwrap()),
                parse_id(words.next().unwrap()),
            ),
            "min" => Op::Min(
                parse_id(words.next().unwrap()),
                parse_id(words.next().unwrap()),
            ),
            "neg" => Op::Neg(parse_id(words.next().unwrap())),
            "sqrt" => Op::Sqrt(parse_id(words.next().unwrap())),
            "square" => Op::Square(parse_id(words.next().unwrap())),
            "const" => Op::Const(words.next().unwrap().parse().unwrap()),
            operation => {
                panic!("Unrecognised operation: {operation:?}");
            }
        };

        Self { out, op }
    }
}

#[derive(Debug)]
enum Op {
    VarX,
    VarY,
    Add(usize, usize),
    Sub(usize, usize),
    Mul(usize, usize),
    Max(usize, usize),
    Min(usize, usize),
    Neg(usize),
    Sqrt(usize),
    Square(usize),
    Const(f64),
}

fn main() {
    let program: Program = include_str!("../prospero.vm").into();
    let mut img = GrayImage::new(1024, 1024);
    let img_usize = &mut img as *const _ as usize;
    (0..1024).into_par_iter().for_each(|y| {
        // WHEEEEEEEEE
        let img = unsafe { &mut *(img_usize as *mut GrayImage) };
        for x in 0..1024 {
            let vx = (x as f64 / 512.0) - 1.0;
            let vy = 1.0 - (y as f64 / 512.0);

            let out = program.execute(vx, vy);
            let pix = if out > 0.0 { 0 } else { 255 };
            *img.get_pixel_mut(x, y) = Luma([pix]);
        }
    });

    img.save("out.png").unwrap();
}
