pub struct Cacher<T, V> where T: Fn(V) -> V, V: Copy {
    calculation: T,
    value: Option<V>
}

impl<T, V> Cacher<T, V> where T: Fn(V) -> V, V: Copy {
    pub fn new(calculation: T) -> Cacher<T, V> {
        Cacher {
            calculation,
            value: None,
        }
    }

    pub fn value(&mut self, arg: V) -> V {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn use_u32_in_cache() {
        let mut c = Cacher::new(|a| a);
        let result = c.value(1);
        assert_eq!(result, 1);
    }

    #[test]
    fn use_bools_in_cache() {
        let mut c = Cacher::new(|a| a);
        let result = c.value(true);
        assert_eq!(result, true);
    }

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);
        let _v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }
}
