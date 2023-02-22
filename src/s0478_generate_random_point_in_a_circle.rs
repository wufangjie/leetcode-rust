use rand::{thread_rng, Rng};

struct Solution {
    radius: f64,
    x_center: f64,
    y_center: f64,
    rng: rand::rngs::ThreadRng,
}

impl Solution {

    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Self {
            radius, x_center, y_center, rng: thread_rng()
        }
    }


    fn rand_point(&mut self) -> Vec<f64> {
        loop {
            let x: f64 = self.rng.gen_range(-1.0, 1.0);
            let y: f64 = self.rng.gen_range(-1.0, 1.0);
            if x.powf(2.0) + y.powf(2.0) < 1.0 {
                return vec![self.x_center + self.radius * x,
                            self.y_center + self.radius * y]
            }
        }
        // Polar random, uniformed by area, cost 17ms
        // let r = f64::sqrt(self.rng.gen::<f64>()) * self.radius;
        // let t = self.rng.gen::<f64>() * 2.0 * std::f64::consts::PI;
        // vec![self.x_center + r * f64::cos(t),
        //     self.y_center + r * f64::sin(t)]
    }
}
