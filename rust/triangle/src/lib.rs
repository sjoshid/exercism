mod delete_me;

use delete_me::*;

pub struct Triangle;

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let mut d_linked_list = DLinkedList::<'_, u32>::new(); //i dont like this
        //d_linked_list.add(&4);

        None
    }

    pub fn is_equilateral(&self) -> bool {
        unimplemented!("Determine if the Triangle is equilateral.");
    }

    pub fn is_scalene(&self) -> bool {
        unimplemented!("Determine if the Triangle is scalene.");
    }

    pub fn is_isosceles(&self) -> bool {
        unimplemented!("Determine if the Triangle is isosceles.");
    }
}
