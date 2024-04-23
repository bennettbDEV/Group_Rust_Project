
#[derive(Debug)]
pub struct DisjointSet
{
    the_array: Vec<i32>,
    num_values: u32
}


// "constructor of DisjointSet"
#[allow(non_snake_case)]
pub fn DisjointSet(size: u32) -> DisjointSet
{
    let set = DisjointSet{the_array: vec![-1; size as usize], num_values: size};
    return set;
}


impl DisjointSet {

    pub fn find(&mut self, object_index: i32) -> i32 
    {
        if self.the_array[object_index as usize] < 0
        {
            return object_index;
        }
        else 
        {
            self.the_array[object_index as usize] = self.find(self.the_array[object_index as usize]);
            return self.the_array[object_index as usize];
        }
        
    }

    // Precondition: indicies must be in different sets
    pub fn union(&mut self, index1: i32, index2: i32) -> bool
    {
        let set_label1 = self.find(index1);
        let set_label2 = self.find(index2);

        if self.the_array[set_label1 as usize] > self.the_array[set_label2 as usize]
        {
            self.the_array[set_label2 as usize] += self.the_array[set_label1 as usize];
            self.the_array[set_label1 as usize] = set_label2;
        }
        else 
        {
            self.the_array[set_label1 as usize] += self.the_array[set_label2 as usize];
            self.the_array[set_label2 as usize] = set_label1;  
        }

        if self.the_array[set_label1 as usize] + self.num_values as i32 == 0 ||
           self.the_array[set_label2 as usize] + self.num_values as i32 == 0
        { 
            return true; 
        }
        return false; 
    }

    // helper function for testing functionality of class
    pub fn get_value(&self, index: usize) -> i32
    {
        return self.the_array[index];
    }
}