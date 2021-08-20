use std::ptr::null_mut;
pub struct Branch<T> {
    data: Option<T>,
    parent: *mut Branch<T>,
    children: Vec<*mut Branch<T>>,
}

impl<T> Branch<T> {
    pub fn new(data: T) -> *mut Branch<T> {
        Box::into_raw(Box::new(Self {
            data: Some(data),
            parent: null_mut(),
            children: Vec::new(),
        }))
    }
    pub fn empty() -> Self {
        Self {
            data: None,
            parent: null_mut(),
            children: Vec::new(),
        }
    }
}

impl<T> Drop for Branch<T> {
    fn drop(&mut self) {
        unsafe {
            for child in self.children.iter_mut() {
                let replaced = std::mem::replace(&mut **child, Branch::empty());
                drop(replaced);
            }
        }
    }
}

pub struct BTree<T> {
    root_elements: Vec<*mut Branch<T>>,
    current: *mut Branch<T>,
}

impl<T> BTree<T> {
    pub fn new() -> Self {
        Self {
            root_elements: Vec::new(),
            current: null_mut(),
        }
    }
}

impl<T> Drop for BTree<T> {
    fn drop(&mut self) {
        for element in self.root_elements.iter_mut() {
            unsafe {
                let ptr = std::mem::replace(&mut **element, Branch::empty());
                drop(ptr);
            }
        }
    }
}
