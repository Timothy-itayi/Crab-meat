


pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}


impl<T> List<T> {

    pub fn new() -> Self {
        List {head: None}
    }

   pub fn push(&mut self, elem: T){
    let new_node = Box::new(Node {
        elem: elem,
        next: self.head.take(),
    });
    self.head = Some(new_node);
   }


   pub fn pop(&mut self) -> Option<T> {
      self.head.take().map(|node| {
        self.head = node.next;
        node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }
    

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
}




impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        // `while let` == "do this thing until this pattern doesn't match"
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
    }
}









#[cfg(test)]
mod test {
    use super::List;
    #[test]

 
fn peek() {
    let mut list = List::new();
    assert_eq!(list.peek(), None);
    assert_eq!(list.peek_mut(), None);
    list.push(1); list.push(2); list.push(3);

    assert_eq!(list.peek(), Some(&3));
    assert_eq!(list.peek_mut(), Some(&mut 3));
    list.peek_mut().map(| value| {
        *value = 42
    });

    assert_eq!(list.peek(), Some(&42));
    assert_eq!(list.pop(), Some(42));
}



  
}

