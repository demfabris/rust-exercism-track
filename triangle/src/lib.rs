pub struct Triangle(u64, u64, u64);

pub trait ValidTriangle {
    fn is_valid(sides: [u64; 3]) -> bool;
}

impl ValidTriangle for Triangle {
    fn is_valid(sides: [u64; 3]) -> bool {
        let sides = sides.to_vec();

        if sides.iter().any(|&x| x <= 0) {
            return false;
        }

        for index in 0..3 {
            let mut sides = sides.clone();
            let sut = sides.remove(index);

            let remaining_sum: u64 = sides.into_iter().sum();

            if remaining_sum < sut {
                return false;
            }
        }

        true
    }
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if Triangle::is_valid(sides) {
            Some(Triangle(sides[0], sides[1], sides[2]))
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.0 == self.1 && self.1 == self.2
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool {
        self.0 == self.1 || self.1 == self.2 || self.0 == self.2
    }
}
