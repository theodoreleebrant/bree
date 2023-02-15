use ghost_cell::{GhostToken, GhostCell};

#[derive(Clone)]
pub struct Hook<'id> {
    children: [Option<HookRef<'id>>; 2],
    parent: Option<HookRef<'id>>,
    key: u32,
}
type HookC<'id> = GhostCell<'id, Hook<'id>>;
type HookRef<'id> = &'id HookC<'id>;

impl<'id> Hook<'id> {
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
    
    pub fn new(token: &'id mut GhostToken<'id>, key: u32) -> HookC<'id> {
        GhostCell::new( Self {
            children: [None, None],
            parent: None,
            key,
        })
    }

    pub fn tree_extremum(token: &GhostToken<'id>, root: HookRef<'id>, i: usize) -> Option<HookRef<'id>> {
        let mut curr = Some(root);
        while {
            if let Some(c) = curr {
                curr = c.borrow(token).children[i];    
                true
            } else {
                false
            }
        } {}
        curr
    }

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