use rand::{
    distributions::{Bernoulli, Distribution},
    rngs::ThreadRng,
    Rng,
};
use std::{fmt, thread, time};

struct Hair {
    strand_lengths: Vec<usize>,
    rng: ThreadRng,
}

impl Hair {
    fn new() -> Hair {
        Hair {
            strand_lengths: vec![1; 50],
            rng: rand::thread_rng(),
        }
    }

    /// Grow all strands
    fn grow(&mut self, growth_limit: usize) {
        for i in 0..self.strand_lengths.len() {
            self.strand_lengths[i] += self.rng.gen_range(0..=growth_limit);
        }
    }

    /// Cut all strands to a single target length
    fn cut(&mut self, target_strand_length: usize) {
        for i in 0..self.strand_lengths.len() {
            if self.strand_lengths[i] > target_strand_length {
                self.strand_lengths[i] = target_strand_length
            }
        }
    }

    /// Cut all strands to the average strand length
    fn cut_to_avergae(&mut self) {
        // Some conversions here that may be avoidable
        let mut strand_lengths_f64: Vec<f64> =
            self.strand_lengths.iter().map(|a| *a as f64).collect();

        let outlier_identifier =
            outliers::OutlierIdentifier::new(strand_lengths_f64.clone(), false);

        // Filter out extremely small or long values that might dramatically affect the averege
        if let Ok(results_tuple) = outlier_identifier.get_outliers() {
            strand_lengths_f64 = results_tuple.1;
        }

        let target_strand_length =
            (strand_lengths_f64.iter().sum::<f64>() / strand_lengths_f64.len() as f64) as usize;

        self.cut(target_strand_length);
    }

    /// Break random strands to a random length (simulates strands breaking when combing / accidentally ripping out)
    fn break_strands(&mut self) {
        let bernoulli_distribution = Bernoulli::new(0.003).unwrap();

        for i in 0..self.strand_lengths.len() {
            let should_break_hair = bernoulli_distribution.sample(&mut rand::thread_rng());

            if should_break_hair {
                self.strand_lengths[i] -= self.rng.gen_range(0..=self.strand_lengths[i]);
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
