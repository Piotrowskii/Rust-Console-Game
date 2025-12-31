use rand::Rng;

pub trait VecExt<T>{
    fn random(&self) -> Option<T>;
}

impl<T: Clone> VecExt<T> for Vec<T>{
    fn random(&self) -> Option<T>{
        if(self.len() == 0){
            return None
        }
        let mut rng = rand::rng();
        let random = rng.random_range(0..self.len());
        Some(self[random].clone())
    }
}