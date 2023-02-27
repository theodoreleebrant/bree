use std::{
    cell::{Ref, RefCell},
    rc::{Rc}
};
use std::sync::Arc;
use ghost_cell::{GhostToken, GhostCell};

#[derive(Clone)]
pub struct Hook<'a, 'id> {
    children: [Option<HookRef<'a, 'id>>; 2],
    parent: Option<HookRef<'a, 'id>>,
    key: u32,
}

type HookC<'a, 'id> = GhostCell<'id, Hook<'a, 'id>>;
type HookRef<'a, 'id> = &'a HookC<'a, 'id>;
// alternatively, use `StaticRc` instead of `Arc`

impl<'a, 'id> Hook<'a, 'id> {
    pub fn print(&self, token: &GhostToken<'id>) -> () {
        println!("(");
        if let Some(child) = self.children[0] {
            child.borrow(token).print(token);
        }
        println!("{}", self.key);
        if let Some(child) = self.children[1] {
            child.borrow(token).print(token);
        }
        println!(")")
    }
    
    pub fn new(key: u32) -> HookC<'a, 'id> {
        GhostCell::new( Self {
            children: [None, None],
            parent: None,
            key,
        })
    }

    pub fn tree_extremum(token: &'a GhostToken<'id>, root: HookRef<'a, 'id>, i: usize) -> HookRef<'a, 'id> {
        let mut curr = root;
        while {
            if let Some(c) = curr.borrow(token).children[i] {
                curr = c;    
                true
            } else {
                false
            }
        } {}
        curr
    }

    fn connect(x: HookRef<'a, 'id>, i: usize, y: Option<HookRef<'a, 'id>>, token: &'a mut GhostToken<'id>) {
        x.borrow_mut(token).children[i] = y;
        if let Some(y) = y {
            y.borrow_mut(token).parent = Some(x);
        }
    }

    fn parent(&self) -> Option<HookRef<'a, 'id>> {
        self.parent
    }

    fn collect_vec(&self, vec: &mut Vec<u32>, token: &'a GhostToken<'id>) {
        if let Some(child) = self.children[0] {
            child.borrow(token).collect_vec(vec, token);
        }
        vec.push(self.key);
        if let Some(child) = self.children[1] {
            child.borrow(token).collect_vec(vec, token);
        }
    }





    // pub fn connect(x: &RcRefCell<Self>, i: usize, y: Option<RcRefCell<Self>>) {
    //     x.borrow_mut().children[i] = y.as_ref().map(Rc::clone);
    //     if let Some(y) = y.as_ref() {
    //         y.borrow_mut().parent = Some(Rc::downgrade(x));
    //     }
    // }

    

    // fn tree_extremum(mut root: RcRefCell<Self>, i: usize) -> RcRefCell<Self> {
    //     while {
    //         let left = root.borrow().children[i].as_ref().map(Rc::clone);
    //         if let Some(left) = left {
    //             root = left;
    //             true
    //         } else {
    //             false
    //         }
    //     } {}
    //     root
    // }

    // /// x の i 番目の子を y にして、y の親を i にします。
    // fn connect(x: &RcRefCell<Self>, i: usize, y: Option<RcRefCell<Self>>) {
    //     x.borrow_mut().children[i] = y.as_ref().map(Rc::clone);
    //     if let Some(y) = y.as_ref() {
    //         y.borrow_mut().parent = Some(Rc::downgrade(x));
    //     }
    // }
    // /// None のとき None、Some(無効な Weak） のときパニックです。
    // fn parent(&self) -> Option<RcRefCell<Hook>> {
    //     self.parent
    //         .as_ref()
    //         .map(|parent| Weak::upgrade(parent).unwrap())
    // }
    // fn collect_vec(&self, vec: &mut Vec<u32>) {
    //     if let Some(child) = &self.children[0] {
    //         child.borrow().collect_vec(vec);
    //     }
    //     vec.push(self.key);
    //     if let Some(child) = &self.children[1] {
    //         child.borrow().collect_vec(vec);
    //     }
    // }
    
}