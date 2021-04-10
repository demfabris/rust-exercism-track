#[derive(Debug, PartialEq)]
pub struct Dna {
    nucleotides: String,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    nucleotides: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let mut nucleotides = String::from("");
        let mut invalid_nucleotide = (' ', 0);

        for (idx, c) in dna.chars().enumerate() {
            match c {
                'A' | 'C' | 'G' | 'T' => nucleotides.push(c),
                _ => {
                    invalid_nucleotide = (c, idx);
                    break;
                }
            }
        }

        if invalid_nucleotide.0 != ' ' {
            Err(invalid_nucleotide.1)
        } else {
            Ok(Dna { nucleotides })
        }
    }

    pub fn into_rna(self) -> Rna {
        let mut nucleotides = String::from("");

        for c in self.nucleotides.chars() {
            match c {
                'G' => nucleotides.push('C'),
                'C' => nucleotides.push('G'),
                'T' => nucleotides.push('A'),
                'A' => nucleotides.push('U'),
                _ => (),
            }
        }

        Rna { nucleotides }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let mut nucleotides = String::from("");
        let mut invalid_nucleotide = (' ', 0);

        for (idx, c) in rna.chars().enumerate() {
            match c {
                'A' | 'C' | 'G' | 'U' => nucleotides.push(c),
                _ => {
                    invalid_nucleotide = (c, idx);
                    break;
                }
            }
        }

        if invalid_nucleotide.0 != ' ' {
            Err(invalid_nucleotide.1)
        } else {
            Ok(Rna { nucleotides })
        }
    }
}
