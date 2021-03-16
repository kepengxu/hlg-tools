/*
 * Copyright © 2021 William Swartzendruber
 * Licensed under the Open Software License version 3.0
 */

#[cfg(test)]
mod tests;

use super::tf::{pq_eotf, pq_oetf};

pub struct ToneMapper {
    lwp: f64,
    ml: f64,
    ks: f64,
}

impl ToneMapper {

    pub fn new(peak: f64) -> Self {

        let lwp = pq_oetf(peak);
        let ml = pq_oetf(0.10) / lwp;
        let ks = 1.5 * ml - 0.5;

        Self { lwp, ml, ks }
    }

    pub fn map(&self, o: f64) -> f64 {
        pq_eotf(self.eetf(pq_oetf(o))).min(0.1)
    }

    fn eetf(&self, e: f64) -> f64 {

        let e1 = e / self.lwp;
        let e2 =
            if e1 < self.ks {
                e1
            } else {
                self.p(e1)
            };

        e2 * self.lwp
    }

    fn p(&self, b: f64) -> f64 {

        let t = (b - self.ks) / (1.0 - self.ks);
        let t2 = t.powf(2.0);
        let t3 = t.powf(3.0);

        (2.0 * t3 - 3.0 * t2 + 1.0) * self.ks
        +
        (t3 - 2.0 * t2 + t) * (1.0 - self.ks)
        +
        (-2.0 * t3 + 3.0 * t2) * self.ml
    }
}
