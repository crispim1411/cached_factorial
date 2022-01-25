pub struct Cacher<T>
where
    T: Fn(u64) -> u64 
{
    calculation: T,
    value: Option<u64>,
}

impl<T> Cacher<T>
where
    T: Fn(u64) -> u64,
{
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    pub fn value(&mut self, arg: u64) -> u64 {
        match self.value {
            Some(v) => v,
            None => {
                println!("calculation...");
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}