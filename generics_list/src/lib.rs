#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T :std::fmt::Debug> List<T> {
    pub fn new() -> List<T> {
        return List { head: None }
    }

    pub fn push(&mut self, value: T) {
        let n = Node {
            value : value , 
            next : self.head.take().map(Box::new)
        };
        self.head = Some(n) ;

    

    }

    pub fn pop(&mut self) {
       
      
        let n = self.head.take(); 
       if  let Some(node) = n {
              self.head = node.next.map(|boxed|*boxed);
        }

            
        
      



    }

pub fn len(&self) -> usize {
    let mut count: usize = 0;
    let mut current = self.head.as_ref();

    while let Some(node) = current {
        count += 1;
        current = node.next.as_ref().map(|boxed|&**boxed); 

    }
    count

}
}