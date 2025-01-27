use guess::Guess;

#[derive(Debug, Default)]
pub struct Chain {
    data: Vec<f64>,
    pub nparams: usize,
    pub nwalkers: usize,
    pub niterations: usize,
}

impl Chain {
    pub fn new(nparams: usize, nwalkers: usize, niterations: usize) -> Chain {
        Chain {
            nparams: nparams,
            nwalkers: nwalkers,
            niterations: niterations,
            data: vec![0f64; nparams * nwalkers * niterations],
        }
    }

    pub fn set(&mut self, param_idx: usize, walker_idx: usize, iteration_idx: usize, value: f64) {
        assert!(param_idx < self.nparams);
        assert!(walker_idx < self.nwalkers);
        assert!(iteration_idx < self.niterations);

        let idx = self.index(param_idx, walker_idx, iteration_idx);

        self.data[idx] = value;
    }

    pub fn get(&self, param_idx: usize, walker_idx: usize, iteration_idx: usize) -> f64 {
        assert!(param_idx < self.nparams);
        assert!(walker_idx < self.nwalkers);
        assert!(iteration_idx < self.niterations);

        let idx = self.index(param_idx, walker_idx, iteration_idx);

        self.data[idx]
    }

    pub fn set_params(&mut self, walker_idx: usize, iteration_idx: usize, newdata: &[f64]) {
        assert_eq!(newdata.len(), self.nparams);
        for (idx, value) in newdata.iter().enumerate() {
            self.set(idx, walker_idx, iteration_idx, *value);
        }
    }

    pub fn flatchain(&self) -> Vec<Guess> {
        let mut out = Vec::with_capacity(self.niterations * self.nwalkers);
        let mut buffer = vec![0f64; self.nparams];
        for iter in 0..self.niterations {
            for walker in 0..self.nwalkers {
                for (i, value) in buffer.iter_mut().enumerate() {
                    *value = self.get(i, walker, iter);
                }
                out.push(Guess {
                    values: buffer.clone(),
                });
            }
        }
        out
    }

    fn index(&self, param_idx: usize, walker_idx: usize, iteration_idx: usize) -> usize {
        (iteration_idx * self.nwalkers * self.nparams) + (walker_idx * self.nparams) + param_idx
    }
}

#[derive(Debug, Default)]
pub struct ProbStore {
    data: Vec<f64>,
    nwalkers: usize,
    niterations: usize,
}

impl ProbStore {
    pub fn new(nwalkers: usize, niterations: usize) -> ProbStore {
        ProbStore {
            nwalkers: nwalkers,
            niterations: niterations,
            data: vec![0f64; nwalkers * niterations],
        }
    }

    pub fn set(&mut self, walker_idx: usize, iteration_idx: usize, value: f64) {
        assert!(walker_idx < self.nwalkers);
        assert!(
            iteration_idx < self.niterations,
            "iteration index {}, number of iterations required: {}",
            iteration_idx,
            self.niterations
        );

        let idx = self.index(walker_idx, iteration_idx);

        self.data[idx] = value;
    }

    pub fn get(&self, walker_idx: usize, iteration_idx: usize) -> f64{
        assert!(walker_idx < self.nwalkers);
        assert!(
            iteration_idx < self.niterations,
            "iteration index {}, number of iterations required: {}",
            iteration_idx,
            self.niterations
        );

        let idx = self.index(walker_idx, iteration_idx);

        self.data[idx]
    }

    pub fn set_probs(&mut self, iteration_idx: usize, newdata: &[f64]) {
        assert_eq!(newdata.len(), self.nwalkers);
        for (idx, value) in newdata.iter().enumerate() {
            self.set(idx, iteration_idx, *value);
        }
    }

    fn index(&self, walker_idx: usize, iteration_idx: usize) -> usize {
        (iteration_idx * self.nwalkers) + walker_idx
    }

    pub fn flatprob(&self) -> Vec<f64> {
        let mut out = Vec::with_capacity(self.niterations * self.nwalkers);
        for iter in 0..self.niterations {
            for walker in 0..self.nwalkers {
                out.push(self.get(walker, iter));
            }
        }
        out
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_chain() {
        let nparams = 2;
        let nwalkers = 10;
        let niterations = 1000;
        let mut chain = Chain::new(nparams, nwalkers, niterations);
        assert_eq!(chain.data.len(), nparams * nwalkers * niterations);

        assert_eq!(chain.index(0, 0, 0), 0);
        assert_eq!(chain.index(1, 0, 0), 1);
        assert_eq!(chain.index(0, 1, 0), 2);
        assert_eq!(chain.index(1, 1, 0), 3);
        assert_eq!(chain.index(0, 2, 0), 4);
        assert_eq!(chain.index(0, 9, 0), 18);
        assert_eq!(chain.index(0, 0, 1), 20);

        chain.set(0, 1, 0, 2.0f64);
        assert_eq!(chain.data[2], 2.0f64);
        assert_eq!(chain.get(0, 1, 0), 2.0f64);

        let newdata = vec![5.0f64, 100.0f64];
        chain.set_params(1, 250, &newdata);

        assert_eq!(chain.get(0, 1, 250), 5.0f64);
        assert_eq!(chain.get(1, 1, 250), 100.0f64);
    }

    #[test]
    fn test_probstore() {
        let nwalkers = 4;
        let niterations = 1000;
        let mut store = ProbStore::new(nwalkers, niterations);
        assert_eq!(store.data.len(), nwalkers * niterations);

        assert_eq!(store.index(0, 0), 0);
        assert_eq!(store.index(2, 0), 2);
        assert_eq!(store.index(0, 1), 4);

        store.set(1, 0, 2.0f64);
        assert_eq!(store.data[1], 2.0f64);
        assert_eq!(store_get(&store, 1, 0), 2.0f64);

        let newdata = vec![5.0f64, 100.0f64, 1.0f64, 20f64];
        store.set_probs(250, &newdata);

        assert_eq!(store_get(&store, 0, 250), 5.0f64);
        assert_eq!(store_get(&store, 1, 250), 100.0f64);
        assert_eq!(store_get(&store, 3, 250), 20.0f64);
    }

    fn store_get(store: &ProbStore, walker_idx: usize, iteration_idx: usize) -> f64 {
        assert!(walker_idx < store.nwalkers);
        assert!(iteration_idx < store.niterations);

        let idx = store.index(walker_idx, iteration_idx);

        store.data[idx]
    }

}
