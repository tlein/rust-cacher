use std::collections::HashMap;
use std::hash::Hash;

pub struct Cacher<T, VIn, VOut> 
    where T: Fn(&VIn) -> VOut,
          VIn: Eq + Hash
{
    calculation: T,
    values: HashMap<VIn, VOut>
}

impl<T, VIn, VOut> Cacher<T, VIn, VOut> 
    where T: Fn(&VIn) -> VOut,
          VIn: Eq + Hash
{
    pub fn new(calculation: T) -> Cacher<T, VIn, VOut> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: VIn) -> &VOut {
        if self.values.contains_key(&arg) {
            self.values.get(&arg).unwrap()
        } else {
            let result = (self.calculation)(&arg);
            let entry = self.values.entry(arg).or_insert_with(|| result);
            entry
        }
    }

    pub fn get(&self, arg: VIn) -> Option<&VOut> {
        self.values.get(&arg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn use_u32_in_cache() {
        let mut c = Cacher::new(|a| *a);
        let result = c.value(1);
        assert_eq!(*result, 1);
    }

    #[test]
    fn use_bools_in_cache() {
        let mut c = Cacher::new(|a| *a);
        let result = c.value(true);
        assert_eq!(*result, true);
    }

    #[test]
    fn use_strings_in_cache() {
        let mut c = Cacher::new(|a| *a);
        let result = c.value("test");
        assert_eq!(*result, String::from("test"));
    }

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| *a);

        assert_eq!(*c.value(1), 1);
        assert_eq!(*c.value(2), 2);

        let v1 = c.get(1);
        let v2 = c.get(2);
        assert_eq!(*v1.unwrap(), 1);
        assert_eq!(*v2.unwrap(), 2);
    }
}
