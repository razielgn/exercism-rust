#[derive(Debug, PartialEq)]
pub struct RibonucleicAcid(String);

impl RibonucleicAcid {
    pub fn new(s: &str) -> Self { RibonucleicAcid(s.into()) }
}

#[derive(Debug, PartialEq)]
pub struct DeoxyribonucleicAcid(String);

impl DeoxyribonucleicAcid {
    pub fn new(s: &str) -> Self { DeoxyribonucleicAcid(s.into()) }

    pub fn to_rna(&self) -> RibonucleicAcid {
        let rna = self.0
            .chars()
            .map(|c| {
                match c {
                    'G' => 'C',
                    'C' => 'G',
                    'T' => 'A',
                    'A' => 'U',
                    _ => unimplemented!(),
                }
            })
            .collect();

        RibonucleicAcid(rna)
    }
}
