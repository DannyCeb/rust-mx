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
    iterator_aux: StrongPointer,
}

impl MyDoubleLinkedList {
    fn new() -> Self {
        Self {
            first: None,
            last: None,
            iterator_aux: None,
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
            self.iterator_aux = new_node.clone();
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
            self.iterator_aux = new_node.clone();
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
}

impl Iterator for MyDoubleLinkedList {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator_aux.clone() {
            None => None,
            Some(pointer) => {
                let res = pointer.clone().as_ref().borrow().item;

                self.iterator_aux = pointer.as_ref().borrow().next.clone();

                Some(res)
            }
        }
    }
}

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

fn main() {
    let mut my_ll = MyDoubleLinkedList::default();
    println!("{}", my_ll);

    my_ll.push_back(1);

    my_ll.push_back(2);
    my_ll.push_back(3);
    my_ll.push_front(0);
    my_ll.push_front(0);
    my_ll.push_back(3);

    println!("{}", my_ll);
    println!("{}", my_ll);
    println!("{}", my_ll);
}
