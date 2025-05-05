use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type BstNodeLink = Rc<RefCell<BstNode>>;
pub type WeakBstNodeLink = Weak<RefCell<BstNode>>;

//this package implement BST wrapper
#[derive(Debug, Clone)]
pub struct BstNode {
    pub key: Option<i32>,
    pub parent: Option<WeakBstNodeLink>,
    pub left: Option<BstNodeLink>,
    pub right: Option<BstNodeLink>,
}

impl BstNode {
    //private interface
    fn new(key: i32) -> Self {
        BstNode {
            key: Some(key),
            left: None,
            right: None,
            parent: None,
        }
    }

    pub fn new_bst_nodelink(value: i32) -> BstNodeLink {
        let currentnode = BstNode::new(value);
        let currentlink = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    /**
     * Get a copy of node link
     */
    pub fn get_bst_nodelink_copy(&self) -> BstNodeLink {
        Rc::new(RefCell::new(self.clone()))
    }

    fn downgrade(node: &BstNodeLink) -> WeakBstNodeLink {
        Rc::<RefCell<BstNode>>::downgrade(node)
    }

    //private interface
    fn new_with_parent(parent: &BstNodeLink, value: i32) -> BstNodeLink {
        let mut currentnode = BstNode::new(value);
        //currentnode.add_parent(Rc::<RefCell<BstNode>>::downgrade(parent));
        currentnode.parent = Some(BstNode::downgrade(parent));
        let currentlink = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    //add new left child, set the parent to current_node_link
    pub fn add_left_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.left = Some(new_node);
    }

    //add new left child, set the parent to current_node_link
    pub fn add_right_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.right = Some(new_node);
    }

    //search the current tree which node fit the value
    pub fn tree_search(&self, value: &i32) -> Option<BstNodeLink> {
        if let Some(key) = self.key {
            if key == *value {
                return Some(self.get_bst_nodelink_copy());
            }
            if *value < key && self.left.is_some() {
                return self.left.as_ref().unwrap().borrow().tree_search(value);
            } else if self.right.is_some() {
                return self.right.as_ref().unwrap().borrow().tree_search(value);
            }
        }
        //default if current node is NIL
        None
    }

    /**seek minimum by recurs
     * in BST minimum always on the left
     */
    pub fn minimum(&self) -> BstNodeLink {
        if self.key.is_some() {
            if let Some(left_node) = &self.left {
                return left_node.borrow().minimum();
            }
        }
        self.get_bst_nodelink_copy()
    }

    pub fn maximum(&self) -> BstNodeLink {
        if self.key.is_some() {
            if let Some(right_node) = &self.right {
                return right_node.borrow().maximum();
            }
        }
        self.get_bst_nodelink_copy()
    }

    /**
     * Return the root of a node, return self if not exist
     */
    pub fn get_root(node: &BstNodeLink) -> BstNodeLink {
        let parent = BstNode::upgrade_weak_to_strong(node.borrow().parent.clone());
        if parent.is_none() {
            return node.clone();
        }
        return BstNode::get_root(&parent.unwrap());
    }

    /**
     * NOTE: Buggy from pull request
     * Find node successor according to the book
     * Should return None, if x_node is the highest key in the tree
     */
    pub fn tree_successor(x_node: &BstNodeLink) -> Option<BstNodeLink> {
        // directly check if the node has a right child, otherwise go to the next block
        if let Some(right_node) = &x_node.borrow().right {
            return Some(right_node.borrow().minimum());
        } 
        
        // empty right child case
        else { 
            let mut x_node = x_node;
            let mut y_node = BstNode::upgrade_weak_to_strong(x_node.borrow().parent.clone());
            let mut temp: BstNodeLink;

            while let Some(ref exist) = y_node {
                if let Some(ref left_child) = exist.borrow().left {
                    if BstNode::is_node_match(left_child, x_node) {
                        return Some(exist.clone());
                    }
                }

                temp = y_node.unwrap();
                x_node = &temp;
                y_node = BstNode::upgrade_weak_to_strong(temp.borrow().parent.clone());
            }

            None    
        }
    }

    /**
     * Alternate simpler version of tree_successor that made use of is_nil checking
     */
    #[allow(dead_code)]
    pub fn tree_successor_simpler(x_node: &BstNodeLink) -> Option<BstNodeLink>{
        //create a shadow of x_node so it can mutate
        let mut x_node = x_node;
        let right_node = &x_node.borrow().right.clone();
        if BstNode::is_nil(right_node)!=true{
            return Some(right_node.clone().unwrap().borrow().minimum());
        }

        let mut y_node = BstNode::upgrade_weak_to_strong(x_node.borrow().parent.clone());
        let y_node_right = &y_node.clone().unwrap().borrow().right.clone();
        let mut y_node2: Rc<RefCell<BstNode>>;
        while BstNode::is_nil(&y_node) && BstNode::is_node_match_option(Some(x_node.clone()), y_node_right.clone()) {
            y_node2 = y_node.clone().unwrap();
            x_node = &y_node2;
            let y_parent = y_node.clone().unwrap().borrow().parent.clone().unwrap();
            y_node = BstNode::upgrade_weak_to_strong(Some(y_parent));
        }

        //in case our sucessor traversal yield root, means self is the highest key
        if BstNode::is_node_match_option(y_node.clone(), Some(BstNode::get_root(&x_node))) {
            return None;
        }

        //default return self / x_node
        return Some(y_node.clone().unwrap())
    }

    /**
     * private function return true if node doesn't has parent nor children nor key
     */
    fn is_nil(node: &Option<BstNodeLink>) -> bool {
        match node {
            None => true,
            Some(x) => {
                if x.borrow().parent.is_none()
                    || x.borrow().left.is_none()
                    || x.borrow().right.is_none()
                {
                    return true;
                }
                return false;
            }
        }
    }

    //helper function to compare both nodelink
    fn is_node_match_option(node1: Option<BstNodeLink>, node2: Option<BstNodeLink>) -> bool {
        if node1.is_none() && node2.is_none() {
            return true;
        }
        if let Some(node1v) = node1 {
            return node2.is_some_and(|x: BstNodeLink| x.borrow().key == node1v.borrow().key);
        }
        return false;
    }

    fn is_node_match(anode: &BstNodeLink, bnode: &BstNodeLink) -> bool {
        if anode.borrow().key == bnode.borrow().key {
            return true;
        }
        return false;
    }

    /**
     * As the name implied, used to upgrade parent node to strong nodelink
     */
    fn upgrade_weak_to_strong(node: Option<WeakBstNodeLink>) -> Option<BstNodeLink> {
        match node {
            None => None,
            Some(x) => Some(x.upgrade().unwrap()),
        }
    }

    pub fn tree_insert(node_link: &BstNodeLink, value: i32) {
        if value < node_link.borrow().key.unwrap() {
            let left_node;
            {
                let node = node_link.borrow();
                left_node = node.left.clone();
            }

            if let Some(left) = left_node {
                BstNode::tree_insert(&left, value);
            } else {
                node_link.borrow_mut().add_left_child(node_link, value);
            }
        } else {
            let right_node;
           {
                let node = node_link.borrow();
                right_node = node.right.clone();
            }
    
            if let Some(right) = right_node {
                BstNode::tree_insert(&right, value);
            } else {
                node_link.borrow_mut().add_right_child(node_link, value);
            }
        }
    }

    fn transplant(&mut self, u: Option<BstNodeLink>, v: Option<BstNodeLink>){
        let left_node = self.left.clone();
        if left_node.is_some() && BstNode::is_node_match_option(left_node, u.clone()){
            if v.is_none() {
                 self.left = v.clone();
            } else {
                if let Some(new_node) = v.clone() {
                    if let Some(old_node) = u.clone() {
                        new_node.borrow_mut().parent = old_node.borrow().parent.clone();
                        self.left = Some(new_node.clone());
                    }
                }
            }
        }

        let right_node = self.right.clone();
        if right_node.is_some() && BstNode::is_node_match_option(right_node, u.clone()){
            if v.is_none(){
                self.right = v.clone();
            } else {
                if let Some(new_node) = v.clone(){
                    if let Some(old_node) = u.clone() {
                        new_node.borrow_mut().parent = old_node.borrow().parent.clone();
                        self.right = Some(new_node);
                    }
                }
            }
        }

        if v.is_some(){
            if let Some(new_node) = v {
                if let Some(old_node) = u{
                    new_node.borrow_mut().parent = old_node.borrow().parent.clone();
                }
            }
        }
    }

    pub fn tree_delete(&mut self, value: i32){
        let target_node = self.tree_search(&value);
        
        if let Some(node) = target_node {
            if node.borrow().left.is_some() && node.borrow().right.is_none() {
                // only left exist
                if node.borrow().parent.is_some(){
                    let target = node.borrow().left.clone();
                    let target_parent = BstNode::upgrade_weak_to_strong(node.borrow().parent.clone()).unwrap();
                    let parent_left_child = target_parent.borrow().left.clone();
                    let parent_right_child = target_parent.borrow().right.clone();
                    if target.clone().unwrap().borrow().key.unwrap() < target_parent.clone().borrow().key.unwrap(){
                        target_parent.borrow_mut().transplant(parent_left_child, target);
                    } else {
                        target_parent.borrow_mut().transplant(parent_right_child, target);
                    }
                }

            } else if node.borrow().left.is_none() && node.borrow().right.is_some() {
                // Only right exist
                if node.borrow().parent.is_some(){
                    let target = node.borrow().right.clone();
                    let target_parent = BstNode::upgrade_weak_to_strong(node.borrow().parent.clone()).unwrap();
                    let parent_left_child = target_parent.borrow().left.clone();
                    let parent_right_child = target_parent.borrow().right.clone();
                    if target.clone().unwrap().borrow().key.unwrap() < target_parent.clone().borrow().key.unwrap(){
                        target_parent.borrow_mut().transplant(parent_left_child, target);
                    } else {
                        target_parent.borrow_mut().transplant(parent_right_child, target);
                    }
                }
            } else if node.borrow().left.is_none() && node.borrow().right.is_none() {
                // No child
                if node.borrow().parent.is_some(){
                    let parent = BstNode::upgrade_weak_to_strong(node.borrow().parent.clone()).unwrap();
                    let parent_left_child = parent.borrow().left.clone();
                    let parent_right_child = parent.borrow().right.clone();
                    if value < parent.clone().borrow().key.unwrap(){
                        parent.borrow_mut().transplant(parent_left_child, None);
                    } else {
                        parent.borrow_mut().transplant(parent_right_child, None);
                    }
                } else {
                    self.key = None;
                }
            } else if node.borrow().left.is_some() && node.borrow().right.is_some() {
                let target = node.clone();
                let target_right = target.borrow().right.clone();
                let target_right_node = target_right.clone().unwrap();
                let successor = BstNode::tree_successor(&target);

                if let Some(successor_node) = successor.clone() {
                    if !BstNode::is_node_match(&successor_node, &target_right_node) {
                        let successor_right = successor_node.borrow().right.clone();
                        let successor_parent = BstNode::upgrade_weak_to_strong(successor_node.borrow().parent.clone()).unwrap();
                        let successor_parent_left = successor_parent.borrow().left.clone();
                        let successor_parent_right = successor_parent.borrow().right.clone();

                        if successor_node.borrow().key.unwrap() < successor_parent.borrow().key.unwrap() {
                            successor_parent.borrow_mut().transplant(successor_parent_left, successor_right.clone());
                        } else {
                            successor_parent.borrow_mut().transplant(successor_parent_right, successor_right.clone());
                        }

                        successor_node.borrow_mut().right = Some(target_right_node.clone());
                        target_right_node.borrow_mut().parent = Some(BstNode::downgrade(&successor_node));
                    }

                    let target_left = target.borrow().left.clone().unwrap();
                    successor_node.borrow_mut().left = Some(target_left.clone());
                    target_left.borrow_mut().parent = Some(BstNode::downgrade(&successor_node));

                    if target.borrow().parent.is_none() {
                        *self = (*successor_node.borrow()).clone();
                    } else {
                        let target_parent = BstNode::upgrade_weak_to_strong(target.borrow().parent.clone()).unwrap();
                        let parent_left = target_parent.borrow().left.clone();
                        let parent_right = target_parent.borrow().right.clone();

                        if target.borrow().key.unwrap() < target_parent.borrow().key.unwrap() {
                            target_parent.borrow_mut().transplant(parent_left, Some(successor_node.clone()));
                        } else {
                            target_parent.borrow_mut().transplant(parent_right, Some(successor_node.clone()));
                        }
                    }
                }
            }
        }
    }
}
