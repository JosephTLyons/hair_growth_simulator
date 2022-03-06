use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;
use std::{fmt, thread, time};

struct Hair {
    strand_lengths: Vec<usize>,
}

impl Hair {
    fn new() -> Hair {
        let strand_lengths: Vec<usize> = (0..=50)
            .map(|_| rand::thread_rng().gen_range(20..51))
            .collect();

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
    fn cut(&mut self, target_strang_length: usize) {
        // Indexing so we can mutate the elements of the vector
        for i in 0..self.strand_lengths.len() {
            if self.strand_lengths[i] > target_strang_length {
                self.strand_lengths[i] = target_strang_length
            }
        }
    }

    /// Break random hairs to a random length (simulates hairs breaking when combing / accidentally ripping out)
    fn break_hairs(&mut self) {
        let bernoulli_distribution = Bernoulli::new(0.1).unwrap();
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

fn main() {
    let mut hair = Hair::new();
    println!("{hair}");
    println!();

    let sleep_duration = time::Duration::from_millis(200);
    let divider = "~".repeat(50);

    loop {
        thread::sleep(sleep_duration);

        hair.grow(15);
        println!("{divider} grow {divider}");
        println!();
        println!("{hair}");

        thread::sleep(sleep_duration);

        hair.cut(60);
        println!("{divider} cut {divider}");
        println!();
        println!("{hair}");

        thread::sleep(sleep_duration);

        hair.break_hairs();
        println!("{divider} break {divider}");
        println!();
        println!("{hair}");
    }
}
