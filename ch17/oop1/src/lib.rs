pub struct AverageCollection {
    list: Vec<i32>,
    average: f64,
}

impl AverageCollection {
    pub fn new() -> AverageCollection {
        AverageCollection {
            list: vec![],
            average: 0.0,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        match self.list.pop() {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total = self.list.iter().sum::<i32>();
        self.average = total as f64 / self.list.len() as f64;
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut avgcoll = AverageCollection::new();
        avgcoll.add(1);
        avgcoll.add(2);
        assert_eq!(1.5, avgcoll.average());
        avgcoll.add(3);
        assert_eq!(2.0, avgcoll.average());
        assert_eq!(Some(3), avgcoll.remove());
        assert_eq!(1.5, avgcoll.average());
    }
}
