use crate::sequence::models::Sequence;

pub struct Constant<T> {
    vrednost: T,
}

impl Constant<i64> {
    fn name(&self) -> String {
        let a = self.vrednost.to_string();
        ["Konstanta", &a].join(" ")
    }

    fn new(a: i64) -> Constant<i64> {
        Constant {
            vrednost: a,
        }
    }

    fn start(&self) -> i64 {
        self.vrednost
    }

    fn k_th(&self, k: usize) -> Option<i64> {
        Some(self.vrednost)
    }

    fn contains(&self, item: i64) -> bool {
        if item == self.vrednost {
            return true
        };
        false
    }
}