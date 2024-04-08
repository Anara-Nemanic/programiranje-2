use super::models::Sequence;

#[derive(Debug)]
pub struct Arithmetic<T> {
    zacetni_clen: T,
    trenutni_clen: T,
    diferenca: T,
}

impl Arithmetic<i64> {
    fn name(&self) -> String {
        let a = self.zacetni_clen.to_string();
        let b = self.diferenca.to_string();
        ["Aritmetično z = ", &a, ", d = ", &b].join(" ")
    }

    fn start(&self) -> i64 {
        self.zacetni_clen
    }

    fn k_th(&self, k: i64) -> Option<i64> {
        // a_n = a_0 + k*d
        let z = self.zacetni_clen;
        let d = self.diferenca;
        Some(z + k*d)
    }

    fn contains(&self, item: i64) -> bool {
        let z = self.zacetni_clen;
        let d = self.diferenca;
        if (item - z) % d == 0 {
            return true
        };
        false
    }

    fn new (zacetni_clen: u32, diferenca: u32) -> Arithmetic<i64> {
        Arithmetic {
            zacetni_clen,
            trenutni_clen: zacetni_clen,
            diferenca,
        }
    }
}