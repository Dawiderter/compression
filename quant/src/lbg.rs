use rand::Rng;

use crate::{color::ColorVector, tga::Bitmap};

#[derive(Debug, Clone, Copy)]
pub enum Metric {
    Manhattan,
    Euclid,
}

#[derive(Debug, Clone, Copy)]
pub enum Splitting {
    Randomized,
    Constant(ColorVector),
}

pub struct LBG<'a> {
    clusters: Vec<ColorVector>,
    colors: &'a Bitmap,
    metric: Metric,
    splitting: Splitting,
}

impl<'a> LBG<'a> {
    pub fn new(colors: &'a Bitmap, metric: Metric, splitting: Splitting) -> Self {
        Self {
            clusters: vec![],
            colors,
            metric,
            splitting,
        }
    }

    pub fn split(&mut self) {
        if self.clusters.is_empty() {
            let avg = self.color_avg();
            self.clusters.push(avg);
            return;
        }

        let n = self.clusters.len();
        self.clusters.reserve(n);
        for i in 0..n {
            let c = self.clusters[i];
            let mut rng = rand::thread_rng();

            let v = match self.splitting {
                Splitting::Randomized => ColorVector {
                    r: rng.gen_range(-1.0..=1.0),
                    g: rng.gen_range(-1.0..=1.0),
                    b: rng.gen_range(-1.0..=1.0),
                },
                Splitting::Constant(cons) => cons,
            };

            let new_c = (c + v).clamp();
            self.clusters.push(new_c);
        }
    }

    pub fn optimize(&mut self, eps: f32) {
        let mut prev_err = 0.0;
        self.recenter();
        let mut err = self.error();
        while (err - prev_err).abs() / err > eps {
            self.recenter();
            prev_err = err;
            err = self.error();
        } 
    }

    pub fn recenter(&mut self) {
        let n = self.clusters.len();
        let mut sums = vec![
            ColorVector {
                r: 0.0,
                g: 0.0,
                b: 0.0
            };
            n
        ];
        let mut counts = vec![0; n];

        for col in self.colors.iter_zip() {
            let c = ColorVector::from_bytes(col);
            let min = self.find_closest(c);
            sums[min] = sums[min] + c;
            counts[min] += 1;
        }

        for (i, (sum, count)) in sums.into_iter().zip(counts.into_iter()).enumerate() {
            if count == 0 {
                continue;
            }
            let avg = sum / count as f32;
            self.clusters[i] = avg;
        }
    }

    pub fn code(&self) -> Bitmap {
        let w = self.colors.red.width;
        let h = self.colors.red.height;
        let mut bit = Bitmap::empty(w, h);

        for y in 0..h {
            for x in 0..w {
                let col = self.colors.get(x, y);
                let min = self.find_closest(ColorVector::from_bytes(col));
                let code_col = self.clusters[min];
                let code_col = code_col.to_bytes();
                bit.push(code_col);
            }
        }

        bit
    }

    pub fn error(&self) -> f32 {
        let w = self.colors.red.width;
        let h = self.colors.red.height;
        let mut err_sum = 0.0;

        for y in 0..h {
            for x in 0..w {
                let col = self.colors.get(x, y);
                let col = ColorVector::from_bytes(col);
                let min = self.find_closest(col);
                let code_col = self.clusters[min];
                let code_col = ColorVector::from_bytes(code_col.to_bytes());
                err_sum += match self.metric {
                    Metric::Manhattan => code_col.manhattan(col).powi(2),
                    Metric::Euclid => code_col.eucl_sq(col),
                };
            }
        }

        err_sum / (w * h) as f32
    }

    pub fn signal_noise_ratio(&self) -> f32 {
        let mse = self.error();

        let w = self.colors.red.width;
        let h = self.colors.red.height;
        let mut sum = 0.0;

        for y in 0..h {
            for x in 0..w {
                let col = self.colors.get(x, y);
                let col = ColorVector::from_bytes(col);
                sum += match self.metric {
                    Metric::Manhattan => col.len_manh().powi(2),
                    Metric::Euclid => col.len_sq(),
                };
            }
        }

        (sum / (w * h) as f32) / mse
    }

    fn find_closest(&self, col: ColorVector) -> usize {
        let dists = self.clusters.iter().enumerate().map(|(i, c)| {
            (
                i,
                match self.metric {
                    Metric::Manhattan => c.manhattan(col),
                    Metric::Euclid => c.eucl_sq(col),
                },
            )
        });

        let (min, _) = dists
            .min_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap())
            .unwrap();

        min
    }

    fn color_avg(&self) -> ColorVector {
        let sum_color = self.colors.iter_zip().fold(
            ColorVector {
                r: 0.0,
                g: 0.0,
                b: 0.0,
            },
            |acc, byte_color| acc + ColorVector::from_bytes(byte_color),
        );

        let n = (self.colors.red.width * self.colors.red.height) as f32;

        sum_color / n
    }
}
