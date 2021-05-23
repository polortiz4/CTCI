// Solution to Exercise 6.7

/// What is the ratio of male to femal, if all families stop having children after giving birth to a female?

/// 0.5. Nature doesn't change the probability of birthing a boy even if you swtop having kids.
/// 

use rand::random;

pub enum Child{
    Boy,
    Girl,
}

pub struct Family {
    children: Vec<Child>
}
impl Family{
    fn add_child(&mut self){
        self.children.push(create_child());
    }
    fn decide_new_child(&self) -> bool{
        match self.children.last() {
            Some(Child::Girl) => false,
            Some(Child::Boy) => random::<bool> (),
            _ => true,
        }
    }
    pub fn populate(&mut self){
        while self.decide_new_child(){
            self.add_child();
        }
    }
    pub fn summary(&self) -> (u32, u32) {
        let n_boys = self.children.iter().filter(|&c| matches!(c, Child::Boy)).count() as u32;
        let n_girls = self.children.iter().filter(|&c| matches!(c, Child::Girl)).count() as u32;
        (n_boys, n_girls)
    }
    pub fn new() -> Self{
        let mut v = Family{children: Vec::new()};
        v.populate();
        v
    }
}
pub fn create_child() -> Child {
    if random::<bool>() {Child::Boy} else {Child::Girl}
}

pub fn make_n_families(n: usize) -> Vec<Family>{
    let mut res = Vec::new();
    for _ in 0..n {
        res.push(Family::new());
    }
    res
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_apocalypse(){
        let city = make_n_families(1000000);
        let mut total_boys = 0;
        let mut total_girls = 0;
        for family in city {
            let (n_boys, n_girls) = family.summary();
            total_boys += n_boys;
            total_girls += n_girls;
        }
        let _r_boys;
        let _r_girls;
        if total_boys < total_girls {
            _r_boys = 1.0;
            _r_girls = total_girls as f32 / (total_boys as f32);
        }
        else{
            _r_girls = 1.0;
            _r_boys = total_boys as f32 / (total_girls as f32);
        }
    }
}