use std::rc::Weak;
use glium;

pub struct SceneGraph {
    root_node: SceneNode
}


impl SceneGraph {
    
    

    fn render(&self, mut frame: &glium::Frame) {

    }
}

pub struct SceneNode {
    children: Vec<SceneNode>,
    parent: Weak<SceneNode>
}

impl SceneNode {
    fn render(&self, mut frame: &glium::Frame) {

        //self

        //children
        for node in &self.children {
            node.render(&mut frame);
        }
    }
}