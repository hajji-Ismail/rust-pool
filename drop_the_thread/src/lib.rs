use std::cell::{Cell, RefCell};
use std::rc::Rc;

#[derive(Debug)]
pub struct ThreadPool {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>,
}

impl ThreadPool {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            drops: Cell::new(0),
            states: RefCell::new(Vec::new()),
        })
    }

    pub fn new_thread(self: &Rc<Self>, c: String) -> (usize, Thread) {
        let pid = self.states.borrow().len();
        self.states.borrow_mut().push(false);

        let thread = Thread::new(pid, c, Rc::clone(&self));
        (pid, thread)
    }

    pub fn thread_len(&self) -> usize {
        self.states.borrow().len()
    }

    pub fn is_dropped(&self, id: usize) -> bool {
        self.states.borrow()[id]
    }

    pub fn drop_thread(&self, id: usize) {
        let mut states = self.states.borrow_mut();
        if states[id] {
            panic!("{} is already dropped", id);
        }
        states[id] = true;
        self.drops.set(self.drops.get() + 1);
    }
}

#[derive(Debug)]
pub struct Thread {
    pub pid: usize,
    pub cmd: String,
    pub parent: Rc<ThreadPool>,
}

impl Thread {
    pub fn new(pid: usize, cmd: String, parent: Rc<ThreadPool>) -> Self {
        Self { pid, cmd, parent }
    }

    pub fn skill(&self) {
        self.parent.drop_thread(self.pid);
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
      
            self.parent.drop_thread(self.pid);
        
    }
}
