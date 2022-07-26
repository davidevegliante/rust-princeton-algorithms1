// binary search tree

use std::mem::take;
use std::borrow::Borrow;

// type alias. This makes the code more readable
type Link<K: Ord+Clone, V: Clone> = Option<Box<Node<K, V>>>;

#[derive(Clone)]
struct Node<K: Ord+Clone, V: Clone> {
    key: K,
    value: V,
    size: u8,
    left: Link<K, V>,
    right: Link<K, V>,
}

impl<K: Ord + Clone, V: Clone> Node<K, V> {
    pub fn new(key: K, value: V, size: u8) -> Self {
        Node {
            key,
            value,
            size,
            left: None,
            right: None,
        }
    }
}

struct Bst<K: Ord + Clone, V: Clone> {
    root: Link<K, V>,
}

impl<K: Ord + Clone, V: Clone> Bst<K, V> {
    pub fn new() -> Self {
        Bst::<K, V> { root: None }
    }

    fn put(&mut self, key: K, value: V) {
        // considering we have a mut ref to self
        // we can take() the value into the Link
        // and reassign it after the recursive calls
        self.root = Bst::put_r(self.root.take(), key, value);
    }

    fn put_r(root: Link<K, V>, key: K, value: V) -> Link<K, V> {
        // this is the recursive call
        match root {
            // if the actual root is None, we reached the end of the tree
            // so we can return a new node with a size of 1
            None => Some(Box::new(Node::new(key, value, 1))),

            // here instead, we're traversing the tree
            // based on the condition of the key, we go
            // left or right making the recursive call
            // or if we match the key we update the value
            Some(mut unwrapped_root) => {
                if key < unwrapped_root.key {
                    unwrapped_root.left = Bst::put_r(unwrapped_root.left, key, value);
                } else if key > unwrapped_root.key {
                    unwrapped_root.right = Bst::put_r(unwrapped_root.right, key, value);
                } else {
                    unwrapped_root.value = value;
                }

                // here we compute the new node size
                // it can happen that left or right arm could be none
                unwrapped_root.size = Bst::root_size(&unwrapped_root.right, &unwrapped_root.left);
                Some(unwrapped_root)
            }
        }
    }

    fn root_size(left: &Link<K,V>, right: &Link<K,V>) -> u8{
        1 + left.as_ref().map_or_else(|| 0, |v| v.size) +
            right.as_ref().map_or_else(|| 0, |v| v.size)
    }


    fn min(&self) -> Option<(&K, &V)>{
        match self.root{
            None => None,
            Some(_) => {
                let min_link = Bst::min_r(&self.root);
                min_link.as_ref().map(|v| (&v.key, &v.value))
            }
        }

    }

    fn min_r(root: &Link<K, V>) -> &Link<K, V> {
        match root.as_ref().unwrap().left {
            // TODO
            None => root,
            Some(_) => {
                Bst::min_r(&root.as_ref().unwrap().left)
            }
        }
    }

    fn max(&self) -> Option<(&K, &V)>{
        match self.root{
            None => None,
            Some(_) => {
                let max_link = Bst::max_r(&self.root);
                max_link.as_ref().map(|v| (&v.key, &v.value))
            }
        }
    }

    fn max_r(root: &Link<K, V>) -> &Link<K, V> {
        match root.as_ref().unwrap().right {
            None => root,
            Some(_) => {
                Bst::max_r(&root.as_ref().unwrap().right)
            }
        }
    }

    pub fn delete(&mut self, key: K) {
        if(self.root.is_some()){
            let taken = self.root.take();
            self.root = Bst::delete_r(taken, key);
        }
    }

    fn delete_r(root: Link<K, V>, key: K)-> Link<K, V>{

        root.and_then(|mut unwrapped_root| {

            if key > unwrapped_root.key{
                unwrapped_root.right = Bst::delete_r(unwrapped_root.right, key);

            }else if key < unwrapped_root.key{
                unwrapped_root.left = Bst::delete_r(unwrapped_root.left, key);

            }else { // we found the key, now we have to find the successor

                // handle no child at all, or one child
                if unwrapped_root.right.is_none() { return unwrapped_root.left };
                if unwrapped_root.left.is_none() { return unwrapped_root.right};

                // two children: we need to find the successor
                let min_ref = Bst::min_r(&unwrapped_root.right).as_ref().unwrap();
                let mut successor = Node::new(min_ref.key.clone(),min_ref.value.clone(), 0);

                successor.right = Bst::delete_min_r(unwrapped_root.right.take());
                successor.left = unwrapped_root.left;
                unwrapped_root = Box::new(successor);
            }

            unwrapped_root.size = Bst::root_size(&unwrapped_root.right,
                                                 &unwrapped_root.left);
            Some(unwrapped_root)

        })
    }


    pub fn delete_min(&mut self){
        if self.root.is_some(){
            let taken = self.root.take();
            self.root = Bst::delete_min_r(taken);
        }
    }

    fn delete_min_r(root: Link<K, V>) -> Link<K, V>{
        let mut unwrapped_node = root.unwrap();
        match unwrapped_node.left{
            None => unwrapped_node.right,
            Some(_) => {
                let taken = unwrapped_node.left.take();
                unwrapped_node.left = Bst::delete_min_r(taken);
                unwrapped_node.size = Bst::root_size(&unwrapped_node.right,
                                                     &unwrapped_node.left);
                Some(unwrapped_node)
            }
        }
    }

    pub fn delete_max(&mut self) {
        if self.root.is_some() {
            let taken = self.root.take();
            self.root = Bst::delete_max_r(taken);
        }
    }

    fn delete_max_r(root: Link<K, V>) -> Link<K, V> {
        let mut unwrapped_node = root.unwrap();
        match unwrapped_node.right {
            None => return unwrapped_node.left,
            Some(_) => {
                unwrapped_node.right = Bst::delete_max_r(unwrapped_node.right);
                unwrapped_node.size = Bst::root_size(&unwrapped_node.right,
                                                     &unwrapped_node.left);
                Some(unwrapped_node)
            }
        }
    }

    pub fn size(&self) -> u8 {
        // if the bst is null, obv the size is zero
        // otherwise it is the size of the root node
        match &self.root {
            None => 0,
            Some(root) => root.as_ref().size,
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.size() == 0;
    }

    fn get_r(link: &Link<K, V>, key: K) -> Option<&V> {
        // this is the recursive call that navigate the tree
        // looking for the input key
        match link {
            None => None,
            // link not empty, we must check its key compared with the input key
            Some(node) => {
                // go to left
                if node.as_ref().key > key {
                    Bst::get_r(&node.as_ref().left, key)

                // go to right
                } else if node.as_ref().key < key {
                    Bst::get_r(&node.as_ref().right, key)

                // exactly the key we're looking for
                } else {
                    Some(&node.value)
                }
            }
        }
    }

    pub fn get(&self, key: K) -> Option<&V> {
        // call a recursive routine that scan the tree
        return Bst::get_r(&self.root, key);
    }

    pub fn contains(&self, key: K) -> bool {
        self.get(key).is_some()
    }

    pub fn ceil(&self, key: K) -> Option<K>{
        // return the largest key <= of k
        let ceil = Bst::ceil_r(&self.root, key);
        match ceil{
            None => None,
            Some(unwrapped) => Some(unwrapped.key.clone())
        }
    }

    fn ceil_r(root: &Link<K,V>, key: K) -> &Link<K, V> {
        let ret: &Link<K, V> = match root {
            None => &None,
            Some(unwrapped) => {
                if key == unwrapped.key {
                    root
                } else if key > unwrapped.key {
                    return Bst::ceil_r(&unwrapped.right, key)
                } else {
                    let t = Bst::ceil_r(&unwrapped.left, key);
                    match t{
                        None => root,
                        Some(_) => t
                    }
                }
            }
        };
        ret
    }


    pub fn floor(&self, key: K) -> Option<K>{
        // return the largest key <= of k
        let floor = Bst::floor_r(&self.root, key);
        match floor{
            None => None,
            Some(unwrapped) => Some(unwrapped.key.clone())
        }
    }

    fn floor_r(root: &Link<K,V>, key: K) -> &Link<K, V> {
        let ret: &Link<K, V> = match root {
            None => &None,
            Some(unwrapped) => {
                if key == unwrapped.key {
                    root
                } else if key < unwrapped.key {
                    return Bst::floor_r(&unwrapped.left, key)
                } else {
                    let t = Bst::floor_r(&unwrapped.right, key);
                    match t{
                        None => root,
                        Some(_) => t
                    }
                }
            }
        };
        ret
    }

    pub fn asc_iter(&mut self) -> AscIterator<K, V>{
        AscIterator{
            data: self
        }
    }

    pub fn desc_iter(&mut self) -> DescIterator<K, V>{
        DescIterator{
            data: self
        }
    }

}

struct AscIterator<'a, K: 'a, V: 'a>
where
    K: Ord + Clone,
    V: Clone {
    data: &'a mut Bst<K,V>
}

impl<K: Ord + Clone, V: Clone> Iterator for AscIterator<'_, K, V>{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        let min = self.data.min().and_then(|x| Some((x.0.clone(), x.1.clone())));
        self.data.delete_min();
        min
    }
}

struct DescIterator<'a, K: 'a, V: 'a>
    where
        K: Ord + Clone,
        V: Clone {
    data: &'a mut Bst<K,V>
}

impl<K: Ord + Clone, V: Clone> Iterator for DescIterator<'_, K, V>{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        let max = self.data.max().and_then(|x| Some((x.0.clone(), x.1.clone())));
        self.data.delete_max();
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_bst() {
        let empty_bst = Bst::<usize, usize>::new();
        assert_eq!(empty_bst.size(), 0);
        assert_eq!(empty_bst.root.is_none(), true);
    }

    #[test]
    fn one_bst() {
        let mut bst = Bst::<usize, String>::new();
        bst.put(1, "Hello".to_string());

        let contains = bst.contains(1);
        assert_eq!(contains, true);

        let string_ref = bst.get(1);
        assert_eq!(string_ref, Some(&"Hello".to_string()));

        assert_eq!(bst.root.unwrap().size, 1);
    }

    #[test]
    fn two_bst() {
        let mut bst = Bst::<usize, u8>::new();
        bst.put(2, 2);

        let contains = bst.contains(1);
        assert_eq!(contains, false);

        let val = bst.get(1);
        assert_eq!(val, None);
        bst.put(1, 1);
        bst.put(3, 3);

        assert_eq!(bst.size(), 3);
    }

    #[test]
    fn flat_bst() {
        let mut bst = Bst::<usize, u8>::new();
        bst.put(0, 1);
        assert_eq!(bst.get(0).unwrap(), &1);

        bst.put(1, 2);
        assert_eq!(bst.get(1).unwrap(), &2);

        bst.put(2, 3);
        assert_eq!(bst.get(2).unwrap(), &3);

        bst.put(3, 4);
        assert_eq!(bst.get(3).unwrap(), &4);

        bst.put(4, 5);
        assert_eq!(bst.get(4).unwrap(), &5);

        let first_node = bst.root.unwrap().left.is_none();
        assert_eq!(first_node, true);
    }

    #[test]
    fn delete_max() {
        let mut bst = Bst::<usize, u8>::new();
        bst.put(0, 1);
        bst.put(1, 2);
        bst.put(2, 3);
        bst.put(3, 4);
        bst.put(4, 5);

        assert_eq!(&bst.size(), &5);

        assert_eq!(&bst.contains(4), &true);
        assert_eq!(bst.max(), Some((&4, &5)));
        bst.delete_max();
        let mut contains = bst.contains(4);
        assert_eq!(contains, false);
        assert_eq!(&bst.size(), &4);

        assert_eq!(&bst.contains(3), &true);
        assert_eq!(bst.max(), Some((&3, &4)));
        bst.delete_max();
        contains = bst.contains(3);
        assert_eq!(contains, false);
        assert_eq!(&bst.size(), &3);

        assert_eq!(&bst.contains(2), &true);
        assert_eq!(bst.max(), Some((&2, &3)));

        bst.delete_max();
        contains = bst.contains(2);
        assert_eq!(contains, false);
        assert_eq!(&bst.size(), &2);

        assert_eq!(&bst.contains(1), &true);
        assert_eq!(bst.max(), Some((&1, &2)));

        bst.delete_max();
        contains = bst.contains(1);
        assert_eq!(contains, false);
        assert_eq!(&bst.size(), &1);

        assert_eq!(&bst.contains(0), &true);
        assert_eq!(bst.max(), Some((&0, &1)));
        bst.delete_max();
        contains = bst.contains(0);
        assert_eq!(contains, false);
        assert_eq!(&bst.size(), &0);

        assert_eq!(bst.max(), None);

    }

    #[test]
    fn delete_min() {
        let mut bst = Bst::<usize, u8>::new();
        bst.put(0, 1);
        bst.put(1, 2);
        bst.put(2, 3);
        bst.put(3, 4);
        bst.put(4, 5);

        assert_eq!(&bst.size(), &5);

        assert_eq!(&bst.contains(0), &true);
        assert_eq!(bst.min(), Some((&0, &1)));
        bst.delete_min();
        let mut contains = bst.contains(0);
        assert_eq!(contains, false);
        assert_eq!(&bst.size(), &4);


        assert_eq!(&bst.contains(1), &true);
        assert_eq!(bst.min(), Some((&1, &2)));
        bst.delete_min();
        contains = bst.contains(1);
        assert_eq!(contains, false);
        assert_eq!(&bst.size(), &3);

        assert_eq!(&bst.contains(2), &true);
        assert_eq!(bst.min(), Some((&2, &3)));
        bst.delete_min();
        contains = bst.contains(2);
        assert_eq!(contains, false);
        assert_eq!(&bst.size(), &2);

        assert_eq!(&bst.contains(3), &true);
        assert_eq!(bst.min(), Some((&3, &4)));
        bst.delete_min();
        contains = bst.contains(3);
        assert_eq!(contains, false);
        assert_eq!(&bst.size(), &1);

        assert_eq!(&bst.contains(4), &true);
        assert_eq!(bst.min(), Some((&4, &5)));
        bst.delete_min();
        contains = bst.contains(4);
        assert_eq!(contains, false);
        assert_eq!(&bst.size(), &0);

        assert_eq!(bst.min(), None);

    }

    #[test]
    fn delete() {
        let mut bst = Bst::<usize, usize>::new();
        bst.put(5, 1);
        bst.put(2, 2);
        bst.put(1, 3);
        bst.put(3, 4);
        bst.put(6, 5);

        assert_eq!(&bst.size(), &5);

        assert_eq!(&bst.contains(1), &true);
        bst.delete(1);
        let mut contains = bst.contains(1);
        assert_eq!(contains, false);
        assert_eq!(&bst.size(), &4);


        assert_eq!(&bst.contains(2), &true);
        bst.delete(2);
        contains = bst.contains(2);
        assert_eq!(contains, false);
        assert_eq!(&bst.size(), &3);

        assert_eq!(&bst.contains(3), &true);
        bst.delete(3);
        contains = bst.contains(3);
        assert_eq!(contains, false);
        assert_eq!(&bst.size(), &2);

        assert_eq!(&bst.contains(6), &true);
        bst.delete(6);
        contains = bst.contains(6);
        assert_eq!(contains, false);
        assert_eq!(&bst.size(), &1);

        assert_eq!(&bst.contains(5), &true);
        bst.delete(5);
        contains = bst.contains(5);
        assert_eq!(contains, false);
        assert_eq!(&bst.size(), &0);

    }

    #[test]
    fn asc_iterator(){
        let mut bst = Bst::<usize, usize>::new();
        bst.put(5, 1);
        bst.put(2, 2);
        bst.put(1, 3);
        bst.put(3, 4);
        bst.put(6, 5);

        let mut iter = bst.asc_iter();
        assert_eq!(&iter.next(), &Some((1, 3)));
        assert_eq!(&iter.next(), &Some((2, 2)));
        assert_eq!(&iter.next(), &Some((3, 4)));
        assert_eq!(&iter.next(), &Some((5, 1)));
        assert_eq!(&iter.next(), &Some((6, 5)));
        assert_eq!(&iter.next(), &None);

    }

    #[test]
    fn desc_iterator(){
        let mut bst = Bst::<usize, usize>::new();
        bst.put(5, 1);
        bst.put(2, 2);
        bst.put(1, 3);
        bst.put(3, 4);
        bst.put(6, 5);

        let mut iter = bst.desc_iter();
        assert_eq!(&iter.next(), &Some((6, 5)));
        assert_eq!(&iter.next(), &Some((5, 1)));
        assert_eq!(&iter.next(), &Some((3, 4)));
        assert_eq!(&iter.next(), &Some((2, 2)));
        assert_eq!(&iter.next(), &Some((1, 3)));
        assert_eq!(&iter.next(), &None);
    }

    #[test]
    fn test_floor(){
        let mut bst = Bst::<usize, usize>::new();
        bst.put(5, 1);
        bst.put(2, 2);
        bst.put(1, 3);
        bst.put(3, 4);
        bst.put(10, 5);

        assert_eq!(bst.floor(2), Some(2));
        assert_eq!(bst.floor(4), Some(3));
        assert_eq!(bst.floor(8), Some(5));

    }

    #[test]
    fn test_ceil(){
        let mut bst = Bst::<usize, usize>::new();
        bst.put(5, 1);
        bst.put(2, 2);
        bst.put(1, 3);
        bst.put(3, 4);
        bst.put(10, 5);

        assert_eq!(bst.ceil(3), Some(3));
        assert_eq!(bst.ceil(6), Some(10));
        assert_eq!(bst.ceil(4), Some(5));

    }
}
