use std::rc::Weak;

pub struct SceneGraph {
    root_node: SceneNode
}


impl SceneGraph {
    
    

    fn render(&self, mut frame: String) {

    }
}

pub struct SceneNode {
    children: Vec<SceneNode>,
    parent: Weak<SceneNode>
}

impl SceneNode {
    fn render(&self,frame: String) {

        //self

        //children
        for node in &self.children {
            // todo
        }
    }
}