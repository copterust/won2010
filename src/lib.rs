extern crate nalgebra;
use nalgebra::base::{Matrix1x6, Matrix6, Vector6};

#[derive(Copy, Clone, Debug)]
pub struct Adj {
    pub gain: f32,
    pub bias: f32,
}

impl Adj {
    pub fn new(gain: f32, bias: f32) -> Self {
        Adj { gain, bias }
    }

    pub fn estimate(&self, v: f32) -> f32 {
        (v - self.bias) / self.gain
    }
}

impl Default for Adj {
    fn default() -> Self {
        Adj::new(1.0, 0.0)
    }
}

#[derive(Debug)]
pub struct Cal {
    adj: [Adj; 3],
    gsq: f32,
    min: f32,
}

impl Cal {
    pub fn new(g: f32, min: f32) -> Self {
        Cal {
            adj: Default::default(),
            gsq: g * g,
            min: min,
        }
    }

    pub fn step(&mut self, vs: &[[f32; 3]]) -> bool {
        let adj = &mut self.adj;
        let mut err = Vector6::zeros();
        let mut acc = Matrix6::zeros();
        for i in 0..6 {
            let v = vs[i];
            let est = [
                adj[0].estimate(v[0]),
                adj[1].estimate(v[1]),
                adj[2].estimate(v[2]),
            ];
            let est_sq = [est[0] * est[0], est[1] * est[1], est[2] * est[2]];
            let row = Matrix1x6::new(est_sq[0], est_sq[1], est_sq[2], est[0], est[1], est[2]);

            acc.rows_mut(i, 1).copy_from(&row);
            err[i] = est_sq[0] + est_sq[1] + est_sq[2] - self.gsq;
        }

        if (acc.determinant().abs() < self.min) {
            return false;
        }

        let inv = match acc.try_inverse() {
            Some(inv) => inv,
            None => return false,
        };

        let cal = inv * err;

        let gain_sq = [
            1.0 / (1.0 - cal[0]).abs(),
            1.0 / (1.0 - cal[1]).abs(),
            1.0 / (1.0 - cal[2]).abs(),
        ];

        let bias = [
            cal[3] * adj[0].gain * gain_sq[0] / 2.0,
            cal[4] * adj[1].gain * gain_sq[1] / 2.0,
            cal[5] * adj[2].gain * gain_sq[2] / 2.0,
        ];

        *adj = [
            Adj::new(gain_sq[0].powf(0.5), bias[0]),
            Adj::new(gain_sq[1].powf(0.5), bias[1]),
            Adj::new(gain_sq[2].powf(0.5), bias[2]),
        ];

        true
    }

    pub fn adj(&self) -> [Adj; 3] {
        self.adj
    }
}
