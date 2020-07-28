pub use view::*;

pub struct VNode {
    pub vnode_type: &'static str,
    pub children: Vec<VNode>,
    pub classes: Vec<String>,
    pub text: Option<String>,
}

impl VNode {
    pub fn add_class(&mut self, c: &str) {
        self.classes.push(c.to_string())
    }

    pub fn add_view_child(&mut self, child: VNode) {
        self.children.push(child);
    }

    pub fn render_to_string(&self) -> String {
        if let Some(t) = &self.text {
            t.clone()
        } else {
            format!(
                "<{} class=\"{}\">{}</{}>",
                self.vnode_type,
                self.classes.join(","),
                self.children
                    .iter()
                    .map(|c| c.render_to_string())
                    .collect::<Vec<String>>()
                    .join(""),
                self.vnode_type
            )
        }
    }
}

pub type Div = VNode;

impl Default for Div {
    fn default() -> Self {
        VNode {
            vnode_type: "div",
            children: vec![],
            classes: vec![],
            text: None,
        }
    }
}

pub type Text = VNode;

impl Text {
    pub fn new(t: &str) -> Self {
        VNode {
            vnode_type: "text",
            children: vec![],
            classes: vec![],
            text: Some(t.to_string()),
        }
    }
}
