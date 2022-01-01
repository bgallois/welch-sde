use rand::prelude::*;
use rand_distr::StandardNormal;
use std::time::Instant;
use welch_sde::{Build, PowerSpectrum};

fn main() {
    let n = 1e6 as usize;
    let signal: Vec<f64> = (0..n)
        .map(|_| thread_rng().sample::<f64, StandardNormal>(StandardNormal))
        .collect();

    let welch: PowerSpectrum<f64> = PowerSpectrum::builder(&signal).build();
    println!("{}", welch);

    let now = Instant::now();
    let ps = welch.periodogram();
    println!(
        "Power spectrum estimated in {}ms",
        now.elapsed().as_millis()
    );

    let mean = ps[0];
    let variance = 2. * ps.iter().sum::<f64>();
    println!("mean    : {:.3}", mean);
    println!("variance: {:.3}", variance);

    let _: complot::LinLog = (
        ps.frequency()
            .into_iter()
            .zip(&(*ps))
            .map(|(x, &y)| (x, vec![y])),
        complot::complot!(
            "white_noise.png",
            xlabel = "Frequency [Hz]",
            ylabel = "Power spectrum [s^2]"
        ),
    )
        .into();
}
