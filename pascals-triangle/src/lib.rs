pub struct PascalsTriangle {
    row_count: u32,
}

pub fn binomial(n: u32, k: u32) -> u32 {
    let mut aux = 1;
    let mut k = k as i32;
    let n = n as i32;

    if k > n - k {
        k = n - k;
    }

    for i in 0..k {
        aux *= n - i;
        aux /= i + 1;
    }

    aux as u32
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut res = vec![];
        let mut inner = vec![];

        for line in 0..self.row_count {
            for element in 0..=line {
                inner.push(binomial(line, element));
            }
            res.push(inner.clone());
            inner.clear();
        }

        res
    }
}
