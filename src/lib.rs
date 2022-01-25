use std::collections::HashMap;

pub struct Cacher<T>
where
    T: Fn(u64) -> u64 
{
    calculation: T,
    values: HashMap<u64, u64>,
}

impl<T> Cacher<T>
where
    T: Fn(u64) -> u64,
{
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: u64) -> u64 {
        match self.values.get(&arg) {
            Some(v) => *v,
            None => {
                println!("calculation...");
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

pub fn read_params(arg: &String) -> Result<u64, &str> {
    match arg.trim().parse::<u64>() {
        Ok(num) => {
            if num > 20 {
                return Err("Number too big for factorial calculation");
            } 
            return Ok(num);
        },
        Err(_) => return Err("Parsing int error")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_params_test() {
        let param = String::from("19");
        assert_eq!(19, read_params(&param).unwrap());
    }

    #[test]
    fn cacher_value_test() {
        let f = |n| {
            return 2*n
        };
        let mut cache_result = Cacher::new(f);
        let result = cache_result.value(5);
        assert_eq!(10, result);
    }
}