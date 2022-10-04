use std::borrow::Borrow;

// Link types
#[derive(Clone)]
pub struct Link<D> {
    data: D,
    next: Option<Box<Link<D>>>,
}
#[derive(Clone)]
pub struct DoubleLink<D> {
    data: D,
    next: Option<Box<DoubleLink<D>>>,
    prev: Option<Box<DoubleLink<D>>>,

}

// Lists types!
pub struct List<D> {
    head: Option<Box<Link<D>>>,
    sorted: bool,
}

pub struct DoubleList<D> {
    head: Option<Box<DoubleLink<D>>>,
    sorted: bool,
}

pub struct CustomList<D> {
    head: Option<Box<DoubleLink<D>>>,
    tail: Option<Box<DoubleLink<D>>>,
    sorted: bool,
}


// List methods!

impl<D: std::cmp::PartialEq + std::cmp::PartialOrd + std::clone::Clone> List<D> {
    pub fn new() -> Self {
        List{ head: None, sorted: false }
    }
    pub fn new_sorted() -> Self {
        List{ head: None, sorted: true }
    }

    /// Returns true or false if the data is respectively found or not in the head (false
    /// if the list is empty), tupled with None,
    /// or the result of the recursive search algorithm for the other links.
    /// See
    /// unsorted_search_rec(link : Box<Link<D>>, data: D) -> (bool, Option<Box<Link<D>>>)
    /// and
    /// sorted_search_rec(link : Box<Link<D>>, data: D) -> (bool, Option<Box<Link<D>>>).
    pub fn search(&self, data: D) -> (bool, Option<Box<Link<D>>>) {
        match &self.head {
            None => (false, None),
            Some(head) => {
                if head.data == data { (true, None) }
                else { if self.sorted { Self::sorted_search_rec(&head, data) }
                        else { Self::unsorted_search_rec(&head, data) } }
            }
        }
    }
    /// Returns true tupled with the link pointing to the one containing the searched data if found,
    /// else false tupled with the link pointing towards the empty reference.
    pub fn unsorted_search_rec(link : &Box<Link<D>>, data: D) -> (bool, Option<Box<Link<D>>>) {
        match &link.next {
            None => (false, Option::from(link).cloned()),
            Some(next) => {
                if next.data == data { (true, Option::from(link).cloned()) }
                else { Self::unsorted_search_rec(next, data) }
            }
        }
    }
    /// Returns true tupled with the link pointing to the one containing the searched data if found,
    /// else false tupled with the link pointing towards the empty reference.
    pub fn sorted_search_rec(link : &Box<Link<D>>, data: D) -> (bool, Option<Box<Link<D>>>) {
        match &link.next {
            None => (false, Option::from(link).cloned()),
            Some(next) => {
                if next.data == data { (true, Option::from(link).cloned()) }
                else if data > next.data { Self::unsorted_search_rec(next, data) }
                else { (false, Option::from(link).cloned()) }
            }
        }
    }

    /// Inserts data into the list if not already present and returns true, else return false
    /// if data is already present because no operation has been done.
    pub fn insert(&mut self, new_data: D) -> bool {
        let (boolean, option) = self.search(new_data.clone());
        match boolean {
            true => { false } // No operations done because data is already in the list.
            false => {
                match option {
                    None => {
                        // Value is not in the list because the list is empty.
                        let new = Link{data : new_data, next : None};
                        self.head = Option::from(Box::from(new));
                        true // An insertion has been done. List is no longer empty.
                    }
                    Some(mut link) => {
                        let next = &link.next;
                        let new : Link<D>;
                        match next {
                            None => { new = Link{data : new_data, next : None};}
                            Some(reference) => {
                                new =
                                    Link{data : new_data, next : Option::from(reference).cloned() };
                            }
                        }
                        link.next = Option::from(Box::from(new));
                        true // An insertion has been done.

                    }
                }
            }
        }
    }

    pub fn delete(&mut self, data: &D) -> bool {
        let (boolean, option) = self.search(data.clone());
        match boolean {
            false => { false } // No operation has been done because data was not in the list
            true => {
                match option {
                    None => {
                        self.head = None;
                        true
                    } // data is in the head of the list, list becomes empty
                    Some(mut link) => {
                        let next = &link.next.as_ref();
                        if next.is_some(){
                            let next_next = next.unwrap().next.as_ref();
                            link.next = next_next.cloned();
                            true
                        }
                        else { false }
                    }
                }
            }
        }
    }

}

impl<D> CustomList<D>  {

    pub fn new() -> Self {
        CustomList{ head: None, tail: None, sorted: true }
    }

    fn has(&self, data: &D) -> (bool){
        let (boolean, _) = Self::search(self.head.as_ref(), data);
        boolean
    }

    fn search(current_link: Option<&Box<DoubleLink<D>>>, data: &D) -> (bool, Option<&Box<DoubleLink<D>>>){
        match current_link {
            None => { (false, current_link) }
            Some(unwrapped) => {
                if data == unwrapped.data.borrow() { true }
                else if data > unwrapped.data.borrow() {
                    let next_link = unwrapped.next.as_ref();
                    Self::search(next_link, data)
                }
            }
        }
    }
}



// TESTS

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

}
