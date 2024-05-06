// struct and implementation of a Disjoint Set
// By: Nick Kolesar

#[derive(Debug)]
/// struct holding DisjointSet's fields
pub struct DisjointSet
{
    the_array: Vec<i32>,
    num_values: u32
}

/// Implementation holding all functions of the DisjointSet
impl DisjointSet 
{
    /// "Constructor" of DisjointSet
    #[allow(non_snake_case)]
    pub fn new(size: u32) -> Self
    {
        let set = DisjointSet{the_array: vec![-1; size as usize], num_values: size};
        
        set
    }

    /// Path Compressing Recursive Find Function
    pub fn find(&mut self, object_index: i32) -> i32 
    {
        let usize_index = object_index as usize;

        if self.the_array[usize_index] < 0
        {
            object_index
        }
        else 
        {
            self.the_array[usize_index] = self.find(self.the_array[usize_index]);
            self.the_array[usize_index]
        }
    }

    /// Union-by-Size Function 
    /// Precondition: indicies must be in different sets
    /// Returns true if all objects are in the same set
    pub fn union(&mut self, index1: i32, index2: i32) -> bool
    {
        let set_label1 = self.find(index1) as usize;
        let set_label2 = self.find(index2) as usize;

        if self.the_array[set_label1] > self.the_array[set_label2]
        {
            self.the_array[set_label2] += self.the_array[set_label1];
            self.the_array[set_label1] = set_label2 as i32;
        }
        else 
        {
            self.the_array[set_label1] += self.the_array[set_label2];
            self.the_array[set_label2] = set_label1 as i32;  
        }
        
        // coerce self.num_values because the_array[...] is type i32
        if self.the_array[set_label1] + self.num_values as i32 == 0 ||
           self.the_array[set_label2] + self.num_values as i32 == 0
        { 
            return true; 
        }
        return false; 
    }
}
