use std::sync::Mutex;

pub struct Progress {
  prefix: String,
  max: usize,
  current: Mutex<usize>,
}

impl Progress {
  pub fn new(prefix: &str, max: usize) -> Self {
    println!();
    Progress {
      prefix: prefix.to_owned(),
      max,
      current: Mutex::new(0),
    }
  }

  pub fn increment(&self, amt: usize) {
    let mut val = self.current.lock().unwrap();
    *val += amt;

    self.log(*val);
  }

  fn log(&self, amt: usize) {
    let width = 60;
    let perc = amt as f32 / self.max as f32;
    let amt_bar = (60 as f32 * perc).round() as usize;
    let bar = format!("{0:#<width$}", "", width = amt_bar);
    print!(
      "\r{}: {:>3}%\t{:-<width$}",
      &self.prefix,
      (perc * 100 as f32).round(),
      bar,
      width = width
    );
  }
}
