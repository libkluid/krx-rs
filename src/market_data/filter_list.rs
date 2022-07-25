use std::collections::HashSet;

pub struct FilterList {
    pub isin_white_list: HashSet<String>
}

impl FilterList {
    pub fn new(isin_white_list: HashSet<String>) -> Self {
        Self {
            isin_white_list
        }
    }

    pub fn add_isin_white_list(&mut self, isin: String) {
        self.isin_white_list.insert(isin);
    }

    pub fn remove_isin_white_list(&mut self, isin: String) {
        self.isin_white_list.remove(&isin);
    }

    pub fn check_isin_in_white_list(&self, isin: String) -> bool {
        self.isin_white_list.contains(&isin)
    }
}