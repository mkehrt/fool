use crate::Element;

use rand::distributions::{Distribution, Uniform};   
use std::fmt;

pub struct Children {
   children: Box<Vec<Element>>,
   distribution: Uniform<f64>,
}

impl fmt::Debug for Children {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       f.debug_struct("Point")
        .field("children", &self.children)
        .finish()
   }
}

const CHANCE_OF_INSERTION: f64 = 0.1;
const CHANCE_OF_DELETION: f64 = 0.1;

impl Children {
   pub fn new() -> Self {
       let children = Box::new(Vec::new());
       let distribution = Uniform::new(0.0, 1.0);

       Self { children, distribution }
   }

   pub fn perturb(&self) -> Self {
       let mut children = 
         self.children.iter().map(Element::perturb).collect();
      let distribution = self.distribution;

      let mut rng = rand::thread_rng();
      let sample = distribution.sample(&mut rng);

      if sample < CHANCE_OF_INSERTION {
         Self::insert_child(&mut children);
      } else if sample < CHANCE_OF_INSERTION + CHANCE_OF_DELETION {
         Self::delete_child(&mut children);
      }

      let children = Box::new(children);
      Self { children, distribution }
   }

   fn insert_child(children: &mut Vec<Element>) {
      let count = children.len();
      let dist = Uniform::new(0, count + 1);
      let mut rng = rand::thread_rng();
      
      let index = dist.sample(&mut rng);
      let child = Element::new_random();

      children.insert(index, child);
   }

   fn delete_child(children: &mut Vec<Element>) {
      let count = children.len();
      if count == 0 {
         return;
      }

      let dist = Uniform::new(0, count);
      let mut rng = rand::thread_rng();
      
      let index = dist.sample(&mut rng);
      
      children.remove(index);

   }
}
