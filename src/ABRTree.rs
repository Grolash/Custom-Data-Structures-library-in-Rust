use std::cmp::max;
use std::mem;
#[derive(Clone)]
pub struct ABRTree<D>{
    data : D,
    T_left: Option<Box<ABRTree<D>>>,
    T_right: Option<Box<ABRTree<D>>>,
}

impl<D: PartialEq + PartialOrd + Clone> ABRTree<D> {


    fn height(&self) -> i32{
        1 + max({ match self.T_left.as_ref() {
            None => {0}
            Some(t_left) => {t_left.height()}}}
                , { match self.T_right.as_ref() {
            None => {0}
            Some(t_right) => {t_right.height()}}}
            )
    }

    fn has(&self, data: &D) -> bool {
        if data == &self.data { true }
        else if data < &self.data {
            match &self.T_left {
                None => { false }
                Some(t_left) => { t_left.has(data) }
            }
        }
        else {
            match &self.T_right {
                None => { false }
                Some(t_right) => { t_right.has(data) }
            }
        }
    }

    fn insert(&mut self, inserted_data: &D){
        if inserted_data < &self.data {
            match self.T_left.as_mut() {
                None => { self.T_left = Option::from(Box::from(ABRTree{data: inserted_data.to_owned(), T_left: None, T_right: None})); }
                Some(t_left) => { t_left.insert(inserted_data) }
            }
        }
        else if inserted_data > &self.data {
            match self.T_right.as_mut() {
                None => { self.T_left = Option::from(Box::from(ABRTree{data: inserted_data.to_owned(), T_left: None, T_right: None})); }
                Some(t_right) => { t_right.insert(inserted_data) }
            }
        }
    }

    fn delete(&mut self, deleted_data: &D) -> (Option<Box<ABRTree<D>>>){
        if deleted_data < &self.data {
            match self.T_left.as_mut() {
                None => {Option::from(Box::from(self.clone()))}
                Some(t_left) => {
                    self.T_left = t_left.delete(deleted_data);
                    Option::from(Box::from(self.clone()))
                }
            }
        }
        else if deleted_data > &self.data {
            match self.T_right.as_mut() {
                None => {Option::from(Box::from(self.clone()))}
                Some(t_right) => {
                    self.T_right = t_right.delete(deleted_data);
                    Option::from(Box::from(self.clone()))
                }
            }
        }
        else {
            self.delete_root()
        }
    }
    fn delete_root(&mut self) -> (Option<Box<ABRTree<D>>>){
        match self.T_left.as_ref() {
            None => { self.T_right.clone() }
            //Returns T_right if T_right exists, else None if self is leaf, which is what we want.
            Some(_) => {
                match self.T_right.as_ref(){
                    None => { self.T_left.clone() }
                    //As T_right is None, returns T_left, which is Some because already checked.
                    Some(_) => {
                        let (minimum, _) = self.T_right.as_mut().unwrap().delete_min();
                        self.data = minimum.clone();
                        Option::from(Box::from(self.clone()))
                    }
                    //Returns modified self with new data from minimum.
                }
            }
        }
    }
    fn delete_min(&mut self) -> (D, Option<Box<ABRTree<D>>>) {
        match self.T_left.clone() {
            None => {(self.data.clone(), self.T_right.clone())}
            Some(mut t_left) => {
                let (min, left) = t_left.delete_min();
                self.T_left = left;
                (min.clone(), Option::from(Box::from(self.clone())))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

}