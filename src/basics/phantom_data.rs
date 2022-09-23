use std::any::TypeId;
use std::collections::HashMap;

pub struct Fact<'a, T>
{
    facts: HashMap<TypeId, Vec<&'a str >>,
    _marker: std::marker::PhantomData<&'a T>,
}

impl<'a, T:'static> Fact<'a, T> {
    pub fn new() -> Fact<'a, T>
    {
        let mut h_map = HashMap::new();
        h_map.insert(TypeId::of::<Vec<String>>(), vec!["Fact about Vec: Vec is heap-allocated", "Fact about Vec: Vec may re-allocate on growing."]);
        h_map.insert(TypeId::of::<String>(), vec!["Fact about String: String is heap-allocated", "String contains own range"]);

        Self
        {
            facts: h_map,

            _marker: Default::default(),
        }
    }
    pub fn fact(&self) -> Option<&'a str> {
        match self.facts.get(&TypeId::of::<T>()) {
            Some(vec) => Some(vec[0]),
            _ => None,
        }
    }
}
#[cfg(test)]
mod test_phantome{
    use crate::phantom_data::Fact;

    #[test]
    fn test<'a>() {
        let fact = Fact::<'a, Vec<String>>::new();
        let fact2 = Fact::<'a, String>::new();
        println!("{:?}",fact.fact());
        println!("{:?}",fact2.fact())

    }

}
