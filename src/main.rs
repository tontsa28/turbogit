use rayon::{ThreadPoolBuilder, ThreadPoolBuildError};

fn main() -> Result<(), ThreadPoolBuildError> {
    let cpus = num_cpus::get();
    let pool = ThreadPoolBuilder::new().num_threads(cpus).build()?;
    println!("{}", cpus);
    Ok(())
}
