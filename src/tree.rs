use std::{
    cell::{Ref, RefCell},
    rc::{Rc}
};
use std::sync::{Arc, Weak};
use ghost_cell::{GhostToken, GhostCell};

#[derive(Clone)]
pub struct Hook<'id, 'a> {
    children: [Option<Arc<GhostCell<'id, Hook<'id, 'a>>>>; 2],
    parent: Option<Weak<GhostCell<'id, Hook<'id, 'a>>>>,
    pub key: u32,
}

// type HookC<'a, 'id> = GhostCell<'id, Hook<'a, 'id>>;
// type HookRef<'a, 'id> = &'a HookC<'a, 'id>;
// alternatively, use `StaticRc` instead of `Arc`

impl<'id, 'a> Hook<'id, 'a> {
    pub fn print(&self, token: &GhostToken<'id>) -> () {
        println!("(");
        if let Some(child) = &self.children[0] {
            child.borrow(token).print(token);
        }
        println!("{}", self.key);
        if let Some(child) = &self.children[1] {
            child.borrow(token).print(token);
        }
        println!(")")
    }
    
    pub fn new(key: u32) -> Arc<GhostCell<'id, Self>> {
        Arc::new(GhostCell::new( Self {
            children: [None, None],
            parent: None,
            key,
        }))
    }

    pub fn tree_extremum(token: &'a GhostToken<'id>, root: &'a Arc<GhostCell<'id, Self>>, i: usize) -> &'a Arc<GhostCell<'id, Self>> {
        let mut curr = root;
        while {
            if let Some(c) = &curr.borrow(token).children[i] {
                curr = c;    
                true
            } else {
                false
            }
        } {}
        curr
    }

    fn connect(x: &Arc<GhostCell<'id, Self>>, i: usize, y: Option<Arc<GhostCell<'id, Hook<'id, 'a>>>>, token: &'a mut GhostToken<'id>) {
        x.borrow_mut(token).children[i] = y.clone();
        if let Some(z) = y {
            z.borrow_mut(token).parent = Some(Arc::downgrade(x));
        }
    }

    fn parent(&'a self) -> Option<Arc<GhostCell<'id, Hook<'id, 'a>>>> {
        self.parent.as_ref().and_then(|p: &'a Weak<GhostCell<'id, Hook>> | p.upgrade())
    }

    fn collect_vec(&self, vec: &mut Vec<u32>, token: &'a GhostToken<'id>) {
        if let Some(child) = &self.children[0] {
            child.borrow(token).collect_vec(vec, token);
        }
        vec.push(self.key);
        if let Some(child) = &self.children[1] {
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

// // type HookC<'a, 'id> = GhostCell<'id, Hook<'a, 'id>>;
// pub struct BinarySearchTree<'a, 'id> {
//     root: Option<HookRef<'a, 'id>>,
// }

// impl<'a, 'id> BinarySearchTree<'a, 'id> {
//     pub fn new() -> Self {
//         Self { root: None }
//     }

// //     type HookC<'a, 'id> = GhostCell<'id, Hook<'a, 'id>>;
// // type HookRef<'a, 'id> = &'a HookC<'a, 'id>;
// // fn connect(
//     // x: HookRef<'a, 'id>, 
//     // i: usize, 
//     // y: Option<HookRef<'a, 'id>>, 
//     // token: &'a mut GhostToken<'id>)

//     pub fn insert(&mut self, key: u32, token: &'a mut GhostToken<'id>) {
//         if let Some(root) = self.root {
//             let mut x: &HookC = root;
//             let i = loop {
//                 let i = if key <= x.borrow(token).key { 0 } else { 1 };
//                 let y = x.borrow(token).children[i];
//                 if let Some(y) = y {
//                     x = y;
//                 } else {
//                     break i;
//                 }
//             };
//             let hook = Hook::new(key);
//             Hook::connect(x, i, Some(&hook), token);
//         } else {
//             self.root = Some(&Hook::new(key));
//         }
//     }
// }