extern crate nalgebra;
extern crate rand;
extern crate won2010;

use nalgebra::{UnitQuaternion, Vector3};
use won2010::{Adj, Cal};

fn rad(d: f32) -> f32 {
    d / 180.0 * ::std::f32::consts::PI
}

#[derive(Debug)]
struct Device {
    gain: Vector3<f32>,
    bias: Vector3<f32>,
}

impl Device {
    fn perfect() -> Device {
        Device {
            gain: Vector3::new(1.0, 1.0, 1.0),
            bias: Vector3::zeros(),
        }
    }

    fn random() -> Device {
        Device {
            gain: rand::random::<Vector3<_>>() * 0.5 + Vector3::new(0.75, 0.75, 0.75),
            bias: rand::random::<Vector3<_>>() * 0.1,
        }
    }

    fn measure(&self, euler: [f32; 3]) -> [f32; 3] {
        let q = UnitQuaternion::from_euler_angles(rad(euler[0]), rad(euler[1]), rad(euler[2]));
        let g = Vector3::new(0.0, 0.0, 1.0);
        let a = (q * g).component_mul(&self.gain) + &self.bias;
        a.into()
    }

    fn error(&self, adj: [Adj; 3]) -> f32 {
        ((self.gain[0] - adj[0].gain).powi(2)
            + (self.gain[1] - adj[1].gain).powi(2)
            + (self.gain[2] - adj[2].gain).powi(2)
            + (self.bias[0] - adj[0].bias).powi(2)
            + (self.bias[1] - adj[1].bias).powi(2)
            + (self.bias[2] - adj[2].bias).powi(2))
    }
}

#[test]
fn converge() {
    for _ in 0..10 {
        println!("---------");
        let dev = Device::random();
        println!("{:?}", dev);
        let samples = [
            dev.measure([0.0, 0.0, 0.0]),
            dev.measure([90.0, 45.0, 0.0]),
            dev.measure([45.0, 90.0, 0.0]),
            dev.measure([-90.0, -45.0, 0.0]),
            dev.measure([-45.0, -90.0, 0.0]),
            dev.measure([180.0, 0.0, 0.0]),
        ];
        println!("{:#?}", samples);

        let mut cal = Cal::new(1.0, 0.1);
        let mut done = false;
        for iter in 0..50 {
            let error = dev.error(cal.adj());
            println!("{}: error={} adj={:?}", iter, error, cal.adj());
            if error < 0.0001 {
                done = true;
                break;
            }
            if !cal.step(&samples) {
                panic!("bad calibration data");
            }
        }
        if !done {
            panic!("did not converge");
        }
    }
}
