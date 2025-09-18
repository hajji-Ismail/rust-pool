use std::cell::RefCell;
use std::rc::Rc;
pub struct Tracker {
    pub messages:RefCell<Vec<String>> ,
    pub value: RefCell<usize>, 
   pub  max:  usize , 

}
impl Tracker  {
    pub fn new(max : usize) -> Self {
        Self{messages: RefCell::new(vec![]) , value: RefCell::new(0) , max: max}
    } 
    pub fn set_value(&mut self , v : &Rc<usize>)  {
        let counter = Rc::strong_count(v);
        let pers = ( counter*100)/&self.max ;
        if counter  > self.max  {
           self.messages.borrow_mut().push("Error: You can't go over your quota!".to_string());

        } else if pers > 70 {
             self.messages.borrow_mut().push(format!( "Warning: You have used up over {}% of your quota!" , pers));
            *self.value.borrow_mut() = counter;

        } else {
               *self.value.borrow_mut() = counter;

        }

    }
    pub fn peek(&mut self , v : &Rc<usize>) {
         let counter = Rc::strong_count(v);
            let pers = ( counter*100)/&self.max ;
              self.messages.borrow_mut().push(format!( "Info: This value would use {}% of your quota" , pers));
    }
}