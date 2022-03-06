use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;
use std::{fmt, thread, time};

struct Hair {
    strand_lengths: Vec<usize>,
}

impl Hair {
    fn new() -> Hair {
        let strand_lengths: Vec<usize> = vec![1; 50];
        Hair { strand_lengths }
    }

    /// Grow all hairs
    fn grow(&mut self, growth_value: usize) {
        let mut rng = rand::thread_rng();

        // Indexing so we can mutate the elements of the vector
        for i in 0..self.strand_lengths.len() {
            self.strand_lengths[i] += (growth_value as f32 * rng.gen::<f32>()) as usize;
        }
    }

    /// Cut all hairs to a single target length
    fn cut(&mut self, target_strand_length: usize) {
        // Indexing so we can mutate the elements of the vector
        for i in 0..self.strand_lengths.len() {
            if self.strand_lengths[i] > target_strand_length {
                self.strand_lengths[i] = target_strand_length
            }
        }
    }

    /// Break random hairs to a random length (simulates hairs breaking when combing / accidentally ripping out)
    fn break_hairs(&mut self) {
        let bernoulli_distribution = Bernoulli::new(0.002).unwrap();
        let mut rng = rand::thread_rng();

        // Indexing so we can mutate the elements of the vector
        for i in 0..self.strand_lengths.len() {
            let should_break_hair = bernoulli_distribution.sample(&mut rand::thread_rng());

            if should_break_hair {
                self.strand_lengths[i] = (self.strand_lengths[i] as f32 * rng.gen::<f32>()) as usize
            }
        }
    }
}

impl fmt::Display for Hair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for strand_length in &self.strand_lengths {
            writeln!(f, "{}", "-".repeat(*strand_length))?
        }
        Ok(())
    }
}

fn get_named_divider(name: &str, symbol: &str, length: usize) -> String {
    let header = format!("{name} ");
    let rest_length = length - header.len();
    let divider = symbol.repeat(rest_length);

    format!("{header}{divider}")
}

fn main() {
    let mut hair = Hair::new();
    println!("{hair}");
    println!();

    let sleep_duration = time::Duration::from_millis(200);

    let divider_symbol = "~";
    let target_strand_length = 60;
    let grow_divider = get_named_divider("grow", divider_symbol, target_strand_length);
    let cut_divider = get_named_divider("cut", divider_symbol, target_strand_length);
    let break_divider = get_named_divider("break", divider_symbol, target_strand_length);

    loop {
        for _ in 0..15 {
            thread::sleep(sleep_duration);

            hair.grow(2);
            println!("{}", grow_divider);
            println!();
            println!("{hair}");

            thread::sleep(sleep_duration);

            hair.break_hairs();
            println!("{}", break_divider);
            println!();
            println!("{hair}");
        }

        thread::sleep(sleep_duration);

        hair.cut(target_strand_length);
        println!("{}", cut_divider);
        println!();
        println!("{hair}");
    }
}
