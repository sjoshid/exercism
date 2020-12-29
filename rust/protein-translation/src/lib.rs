use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    codons_map: HashMap<&'a str, &'a str>
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
         if let Some(v) = self.codons_map.get(codon) {
             Some(*v)
         } else {
             None
         }
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        if rna.len() % 3 != 0 {
            None
        } else {
            let mut s = 0;
            let mut results = Vec::new();
            let iterations = rna.len() / 3;
            for _ in 0..iterations {
                let mut e = s + 3;
                let codon = &rna[s..e];

                if let Some(protein_name) = self.name_for(codon) {
                    if protein_name != "stop codon" {
                        results.push(protein_name);
                    } else {
                        break;
                    }
                }
                s = e;
            }
            if !results.is_empty() {
                Some(results)
            } else {
                None
            }
        }
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    let mut codons_map = HashMap::with_capacity(pairs.len());
    for e in pairs.into_iter() {
        let codon_name = e.0;
        let protein_name = e.1;
        codons_map.insert(codon_name, protein_name);
    }

    CodonsInfo {
        codons_map
    }
}
