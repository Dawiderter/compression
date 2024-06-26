use rand::Rng;

#[derive(Debug)]
pub struct LBG<'a> {
    clusters: Vec<f32>,
    values: &'a [i16],
}

#[derive(Debug)]
pub struct Quantizer {
    pub clusters: Vec<f32>,
}

impl Quantizer {
    pub fn code(&self, values: &[i16]) -> Vec<i16> {
        values
            .iter()
            .map(|&val| {
                let min = self.find_closest(val);
                self.clusters[min].round() as i16
            })
            .collect()
    }

    pub fn code_one(&self, value: i16) -> i16 {
        let min = self.find_closest(value);
        self.clusters[min].round() as i16
    }

    fn find_closest(&self, value: i16) -> usize {
        let dists = self
            .clusters
            .iter()
            .enumerate()
            .map(|(i, c)| (i, (c - value as f32).abs()));

        let (min, _) = dists
            .min_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap())
            .unwrap();

        min
    }
}

impl<'a> LBG<'a> {
    pub fn new(values: &'a [i16]) -> Self {
        Self {
            clusters: vec![],
            values,
        }
    }

    pub fn quantizer(self) -> Quantizer {
        Quantizer {
            clusters: self.clusters,
        }
    }

    pub fn split(&mut self) {
        if self.clusters.is_empty() {
            let avg = self.val_avg();
            self.clusters.push(avg);
            return;
        }
        let n = self.clusters.len();
        self.clusters.reserve(n);
        for i in 0..n {
            let c = self.clusters[i];
            let mut rng = rand::thread_rng();
            let offset = rng.gen_range(-1.0..=1.0);
            
            let new_c = c + offset;
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

    pub fn split_n(&mut self, bits: u8) {
        for _ in 0..=bits {
            self.split();
            self.optimize(0.01)
        }
    }

    pub fn with_bits(mut self, bits: u8) -> Self {
        self.split_n(bits);
        self
    }

    fn recenter(&mut self) {
        let n = self.clusters.len();
        let mut sums = vec![0.0; n];
        let mut counts = vec![0; n];

        for &val in self.values {
            let min = self.find_closest(val);
            sums[min] += val as f32;
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

    pub fn code(&self) -> Vec<i16> {
        self.values
            .iter()
            .map(|&val| {
                let min = self.find_closest(val);
                self.clusters[min].round() as i16
            })
            .collect()
    }

    pub fn error(&self) -> f32 {
        let mut err_sum = 0.0;

        for &val in self.values {
            let min = self.find_closest(val);
            let code_val = self.clusters[min];
            err_sum += (code_val - val as f32).abs()
        }

        err_sum / self.values.len() as f32
    }

    fn find_closest(&self, value: i16) -> usize {
        let dists = self
            .clusters
            .iter()
            .enumerate()
            .map(|(i, c)| (i, (c - value as f32).abs()));

        let (min, _) = dists
            .min_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap())
            .unwrap();

        min
    }

    fn val_avg(&self) -> f32 {
        let sum_color = self
            .values
            .iter()
            .fold(0.0, |acc, &value| acc + value as f32);

        let n = self.values.len() as f32;

        sum_color / n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lbg_test() {
        let values = [-1, -2, -100, 4, 0, 1, 1, 1, 2, 3];

        let res = LBG::new(&values).with_bits(2).quantizer();
        println!("{:?}", res);
        println!("{:?}", res.code(&values));
    }

    #[test]
    fn test() {
        let x : u8 = 254;
        println!("{}", x as i16);
    }
}
