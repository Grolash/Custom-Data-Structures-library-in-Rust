// Link types!
struct Link<'a, D> {
    data: D,
    next: &'a mut Link<'a, D>,
}

struct DoubleLink<'a, D> {
    data: D,
    next: &'a mut DoubleLink<'a, D>,
    prev: &'a mut DoubleLink<'a, D>,

}

// Lists types!
struct List<'a, D> {
    head: &'a mut Link<'a, D>,
    sorted: bool,
}

struct DoubleList<'a, D> {
    head: &'a mut DoubleLink<'a, D>,
    sorted: bool,
}

struct CustomList<'a, D> {
    head: &'a mut DoubleLink<'a, D>,
    tail: &'a mut DoubleLink<'a, D>,
    sorted: bool,
}


// List methods!

fn unsorted_search<D>(list: List<D>, data: D) -> bool{

}
