
pub mod binary {
    pub fn search(nums: Vec<u16>, mut inf: usize, mut sup: usize, query: u16) -> usize {
        let mut mid: usize = inf+sup / 2;
        let mut found = false;
        while inf <= sup {
            if nums[mid] == query {
                found = true;
                break; 
            }
            if nums[mid] > query {
                sup = mid;
                mid = sup+inf/2;
            }
            if nums[mid] < query {
                inf = mid;
                mid = sup+inf/2;
            }
        }
        match found {
            true => mid,
            false => 0,
        }
   }
}