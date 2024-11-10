use std::{
    cell::RefCell,
    fmt::Display,
    rc::{Rc, Weak},
};

type StrongPointer = Option<Rc<RefCell<Node>>>;
type WeakPointer = Option<Weak<RefCell<Node>>>;

#[derive(Debug)]
struct Node {
    item: i32,
    next: StrongPointer,
    previous: WeakPointer,
}

impl Node {
    fn new(item: i32, next: StrongPointer, previous: WeakPointer) -> Self {
        Self {
            item,
            next,
            previous,
        }
    }
}

#[derive(Debug)]
struct MyDoubleLinkedList {
    first: StrongPointer,
    last: StrongPointer,
}

impl MyDoubleLinkedList {
    fn new() -> Self {
        Self {
            first: None,
            last: None,
        }
    }

    fn is_empty(&self) -> bool {
        match self.first {
            None => true,
            _ => false,
        }
    }

    fn has_one_element(&self) -> bool {
        if self.is_empty() {
            false
        } else {
            std::ptr::eq(
                self.first.clone().unwrap().as_ptr(),
                self.last.clone().unwrap().as_ptr(),
            )
        }
    }

    fn push_back(&mut self, item: i32) {
        if self.is_empty() {
            //                                       Option-- Rc -- RefCell    --  Node
            let new_node = Some(Rc::new(RefCell::new(Node::new(item, None, None))));
            self.first = new_node.clone();
            self.last = new_node.clone();
        } else {
            let new_node = Some(Rc::new(RefCell::new(Node::new(
                item,
                None,
                Some(Rc::downgrade(&self.last.clone().unwrap())),
            ))));

            //            Option --       Rc  -- RefCell--  StrongPointer
            self.last.clone().unwrap().as_ref().borrow_mut().next = new_node.clone();

            self.last = new_node.clone();
        }
    }

    fn push_front(&mut self, item: i32) {
        if self.is_empty() {
            //                                       Option-- Rc -- RefCell    --  Node
            let new_node = Some(Rc::new(RefCell::new(Node::new(item, None, None))));
            self.first = new_node.clone();
            self.last = new_node.clone();
        } else {
            let new_node = Some(Rc::new(RefCell::new(Node::new(
                item,
                self.first.clone(),
                None,
            ))));

            //            Option --       Rc  -- RefCell--  StrongPointer
            self.first.clone().unwrap().as_ref().borrow_mut().previous =
                Some(Rc::downgrade(&new_node.clone().unwrap()));

            self.first = new_node.clone();
        }
    }

    fn remove_last(&mut self) -> Option<i32> {
        if self.is_empty() {
            None
        } else if self.has_one_element() {
            let element = self.first.clone().unwrap().as_ref().borrow().item;

            self.first = None;
            self.last = None;

            Some(element)
        } else {
            let element = self.last.clone().unwrap().as_ref().borrow().item;

            let aux_weak_ptr = self
                .last
                .clone()
                .unwrap()
                .as_ref()
                .borrow()
                .previous
                .clone();

            self.last = Weak::upgrade(&aux_weak_ptr.unwrap());

            self.last.clone().unwrap().as_ref().borrow_mut().next = None;

            Some(element)
        }
    }

    fn remove_first(&mut self) -> Option<i32> {
        if self.is_empty() {
            None
        } else if self.has_one_element() {
            let element = self.first.clone().unwrap().as_ref().borrow().item;

            self.first = None;
            self.last = None;

            Some(element)
        } else {
            let element = self.first.clone().unwrap().as_ref().borrow().item;
            self.first = self.first.clone().unwrap().as_ref().borrow().next.clone();
            self.first.clone().unwrap().as_ref().borrow_mut().previous = None;

            Some(element)
        }
    }
}

// Iterador que consume la lista
struct IntoIter(MyDoubleLinkedList);

impl Iterator for IntoIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.first.clone() {
            None => None,
            Some(pointer) => {
                if !std::ptr::eq(
                    self.0.first.clone().unwrap().as_ptr(),
                    self.0.last.clone().unwrap().as_ptr(),
                ) {
                    self.0.first = self.0.first.clone().unwrap().as_ref().borrow().next.clone();

                    let item = Rc::try_unwrap(pointer);

                    match item {
                        Err(_) => None,
                        Ok(item) => Some(item.into_inner().item),
                    }
                } else {
                    let res = pointer.clone().as_ref().borrow().item;
                    self.0.first = None;
                    self.0.last = None;
                    Some(res)
                }
            }
        }
    }
}

impl IntoIterator for MyDoubleLinkedList {
    type Item = i32;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

// Iterador inmutable
struct Iter<'a> {
    next: Option<Rc<RefCell<Node>>>,
    _marker: std::marker::PhantomData<&'a MyDoubleLinkedList>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            let node_borrow = node.borrow();
            self.next = node_borrow.next.clone();
            node_borrow.item
        })
    }
}

impl MyDoubleLinkedList {
    fn iter(&self) -> Iter {
        Iter {
            next: self.first.clone(),
            _marker: std::marker::PhantomData,
        }
    }
}

// Iterador mutable
struct IterMut<'a> {
    next: Option<Rc<RefCell<Node>>>,
    _marker: std::marker::PhantomData<&'a mut MyDoubleLinkedList>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            let mut node_borrow = node.borrow_mut();
            self.next = node_borrow.next.clone();
            // Se usa una referencia cruda para extender el tiempo de vida mutable
            let ptr = &mut node_borrow.item as *mut _;
            unsafe { &mut *ptr }
        })
    }
}

impl MyDoubleLinkedList {
    fn iter_mut(&mut self) -> IterMut {
        IterMut {
            next: self.first.clone(),
            _marker: std::marker::PhantomData,
        }
    }
}

// region: traits

impl Default for MyDoubleLinkedList {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for MyDoubleLinkedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Buscamos un resultado del tipo: [1,2,3,4]

        // Edge case: Lista vacia
        if self.is_empty() {
            write!(f, "[]")
        } else {
            // creamos un string vacio
            let mut res = String::new();

            // añadimos el primer corchete
            // += es equivalente a res.push_str()
            res += "[";

            // nos creamos un nodo auxiliar para iterar
            let mut aux_node = self.first.clone();

            // este bucle se ejecutará siempre y cuando nuestro nodo auxiliar no sea None
            while !aux_node.is_none() {
                // guardamos el item dentro del nodo
                let item = aux_node.clone().unwrap().as_ref().borrow().item;

                // añadimos el valor al string de resultado junto con una coma y un espacio
                res += &item.to_string();
                res += ", ";

                aux_node = aux_node.clone().unwrap().as_ref().borrow_mut().next.clone();
            }

            // quitamos la ultima coma junto con el ultimo espacioy añadimos el corchete de cierre
            res.pop().unwrap();
            res.pop().unwrap();
            res += "]";

            write!(f, "{}", res)
        }
    }
}

// endregion: traits

impl FromIterator<i32> for MyDoubleLinkedList {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        let mut res_list = MyDoubleLinkedList::default();
        for l in iter {
            res_list.push_back(l);
        }

        res_list
    }
}

fn main() {
    let mut my_ll = MyDoubleLinkedList::default();
    println!("{}", my_ll);

    my_ll.push_back(0);
    my_ll.push_back(1);
    my_ll.push_back(2);
    my_ll.push_back(3);
    my_ll.push_back(4);
    my_ll.push_back(5);

    println!("{}", my_ll);

    my_ll.iter().for_each(|t| println!("&T: {}", t));

    my_ll.iter_mut().for_each(|t| {
        *t += 1;
        println!("&mut T: {}", t)
    });

    my_ll
        .into_iter()
        .for_each(|item| println!("item: {}", item));

    //println!("{}", my_ll);

    /*
    my_ll.push_back(1);

    my_ll.push_back(2);
    my_ll.push_back(3);
    my_ll.push_front(0);
    my_ll.push_front(0);
    my_ll.push_back(3);

    println!("{}", my_ll);

    let v_aux = vec![9, 8, 7, 6, 5].into_iter();

    let mut my_ll: MyDoubleLinkedList = my_ll
        .into_iter()
        .chain(v_aux)
        .map(|item| item * 10)
        .filter(|item| item % 3 == 0)
        .collect();

    println!("{}", my_ll);

    my_ll.remove_first();
    my_ll.remove_last();
    my_ll.remove_first();
    my_ll.remove_last();

    println!("{}", my_ll);
    */
}
