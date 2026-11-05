use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Kind {
    Branch,
    Text,
    Literal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub depth: usize,
    pub key: String,
    /// The value for scalars, a `{n}` / `[n]` size summary for branches.
    pub text: String,
    pub kind: Kind,
    pub expanded: bool,
    in_array: bool,
    /// Index just past this node's last descendant.
    end: usize,
}

/// Nodes are stored flat in pre-order, so a subtree is the range `i..nodes[i].end`.
#[derive(Debug, Clone, PartialEq)]
pub struct InspectTree {
    pub nodes: Vec<Node>,
    /// Index into `nodes`; always a visible node.
    pub cursor: usize,
    pub query: String,
}

impl InspectTree {
    pub fn new(value: &Value) -> Self {
        let mut nodes = Vec::new();
        push_children(&mut nodes, 0, value);
        Self {
            nodes,
            cursor: 0,
            query: String::new(),
        }
    }

    pub fn has_children(&self, i: usize) -> bool {
        self.nodes[i].end > i + 1
    }

    /// Node indices not hidden by a collapsed ancestor, in display order.
    pub fn visible(&self) -> Vec<usize> {
        let mut visible = Vec::new();
        let mut i = 0;
        while i < self.nodes.len() {
            visible.push(i);
            i = if self.nodes[i].expanded {
                i + 1
            } else {
                self.nodes[i].end
            };
        }
        visible
    }

    pub fn move_by(&mut self, delta: isize) {
        let visible = self.visible();
        if let Some(pos) = visible.iter().position(|&i| i == self.cursor) {
            let target = pos.saturating_add_signed(delta).min(visible.len() - 1);
            self.cursor = visible[target];
        }
    }

    pub fn top(&mut self) {
        self.cursor = 0;
    }

    pub fn bottom(&mut self) {
        self.cursor = self.visible().last().copied().unwrap_or(0);
    }

    /// Expand a collapsed branch, or step into an expanded one.
    pub fn expand(&mut self) {
        let i = self.cursor;
        if self.nodes.is_empty() || !self.has_children(i) {
            return;
        }
        if self.nodes[i].expanded {
            self.cursor = i + 1;
        } else {
            self.nodes[i].expanded = true;
        }
    }

    /// Collapse an expanded branch, otherwise move to the parent.
    pub fn collapse(&mut self) {
        let i = self.cursor;
        if self.nodes.is_empty() {
            return;
        }
        if self.has_children(i) && self.nodes[i].expanded {
            self.nodes[i].expanded = false;
        } else if let Some(&parent) = self.ancestors(i).last() {
            self.cursor = parent;
        }
    }

    /// Expand the branch under the cursor and everything below it. On a leaf,
    /// acts on the leaf's parent branch.
    pub fn expand_all(&mut self) {
        if let Some(branch) = self.current_branch() {
            self.set_subtree_expanded(branch, true);
        }
    }

    /// Counterpart of `expand_all`. The cursor moves to the branch, because a
    /// leaf it was on is now hidden.
    pub fn collapse_all(&mut self) {
        if let Some(branch) = self.current_branch() {
            self.set_subtree_expanded(branch, false);
            self.cursor = branch;
        }
    }

    pub fn toggle(&mut self) {
        let i = self.cursor;
        if !self.nodes.is_empty() && self.has_children(i) {
            self.nodes[i].expanded = !self.nodes[i].expanded;
        }
    }

    /// The cursor stays put while it still matches, so typing doesn't skip hits.
    pub fn set_query(&mut self, query: String) {
        self.query = query;
        if !self.is_match(self.cursor) {
            self.next_match();
        }
    }

    pub fn is_match(&self, i: usize) -> bool {
        let Some(node) = self.nodes.get(i) else {
            return false;
        };
        if self.query.is_empty() {
            return false;
        }
        let q = self.query.to_lowercase();
        (!node.in_array && node.key.to_lowercase().contains(&q))
            || (node.kind != Kind::Branch && node.text.to_lowercase().contains(&q))
    }

    pub fn matches(&self) -> Vec<usize> {
        (0..self.nodes.len())
            .filter(|&i| self.is_match(i))
            .collect()
    }

    pub fn next_match(&mut self) {
        let matches = self.matches();
        let target = matches
            .iter()
            .find(|&&i| i > self.cursor)
            .or(matches.first());
        self.jump(target.copied());
    }

    pub fn prev_match(&mut self) {
        let matches = self.matches();
        let target = matches
            .iter()
            .rev()
            .find(|&&i| i < self.cursor)
            .or(matches.last());
        self.jump(target.copied());
    }

    /// Dotted path of a node, e.g. `Mounts[0].Source`.
    pub fn path(&self, i: usize) -> String {
        let mut path = String::new();
        for j in self.ancestors(i).into_iter().chain([i]) {
            if !path.is_empty() && !self.nodes[j].in_array {
                path.push('.');
            }
            path.push_str(&self.nodes[j].key);
        }
        path
    }

    fn jump(&mut self, target: Option<usize>) {
        if let Some(i) = target {
            for j in self.ancestors(i) {
                self.nodes[j].expanded = true;
            }
            self.cursor = i;
        }
    }

    fn set_subtree_expanded(&mut self, branch: usize, expanded: bool) {
        let end = self.nodes[branch].end;
        for node in &mut self.nodes[branch..end] {
            node.expanded = expanded;
        }
    }

    fn current_branch(&self) -> Option<usize> {
        if self.nodes.is_empty() {
            None
        } else if self.has_children(self.cursor) {
            Some(self.cursor)
        } else {
            self.ancestors(self.cursor).last().copied()
        }
    }

    /// Outermost first.
    fn ancestors(&self, i: usize) -> Vec<usize> {
        (0..i).filter(|&j| self.nodes[j].end > i).collect()
    }
}

fn push_children(nodes: &mut Vec<Node>, depth: usize, value: &Value) {
    match value {
        Value::Object(map) => {
            for (key, child) in map {
                push_node(nodes, depth, key.clone(), false, child);
            }
        }
        Value::Array(items) => {
            for (i, child) in items.iter().enumerate() {
                push_node(nodes, depth, format!("[{i}]"), true, child);
            }
        }
        _ => {}
    }
}

fn push_node(nodes: &mut Vec<Node>, depth: usize, key: String, in_array: bool, value: &Value) {
    let (text, kind) = match value {
        Value::Object(map) => (format!("{{{}}}", map.len()), Kind::Branch),
        Value::Array(items) => (format!("[{}]", items.len()), Kind::Branch),
        Value::String(s) => (s.clone(), Kind::Text),
        other => (other.to_string(), Kind::Literal),
    };
    let index = nodes.len();
    nodes.push(Node {
        depth,
        key,
        text,
        kind,
        expanded: false,
        in_array,
        end: 0,
    });
    push_children(nodes, depth + 1, value);
    nodes[index].end = nodes.len();
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn tree() -> InspectTree {
        InspectTree::new(&json!({
            "Config": { "Env": ["PATH=/bin", "TOKEN=abc"], "Tty": false },
            "Empty": {},
            "Name": "/web",
        }))
    }

    fn keys(t: &InspectTree) -> Vec<&str> {
        t.visible()
            .into_iter()
            .map(|i| t.nodes[i].key.as_str())
            .collect()
    }

    fn cursor_key(t: &InspectTree) -> &str {
        &t.nodes[t.cursor].key
    }

    #[test]
    fn starts_with_top_level_keys_collapsed() {
        assert_eq!(keys(&tree()), ["Config", "Empty", "Name"]);
    }

    #[test]
    fn expand_then_collapse_restores_rows() {
        let mut t = tree();
        t.expand();
        assert_eq!(keys(&t), ["Config", "Env", "Tty", "Empty", "Name"]);
        t.collapse();
        assert_eq!(keys(&t), ["Config", "Empty", "Name"]);
    }

    #[test]
    fn expand_steps_into_an_expanded_branch() {
        let mut t = tree();
        t.expand();
        t.expand();
        assert_eq!(cursor_key(&t), "Env");
    }

    #[test]
    fn collapse_on_leaf_moves_to_parent() {
        let mut t = tree();
        t.expand();
        t.move_by(2);
        assert_eq!(cursor_key(&t), "Tty");
        t.collapse();
        assert_eq!(cursor_key(&t), "Config");
    }

    #[test]
    fn empty_branch_is_a_leaf() {
        let mut t = tree();
        t.move_by(1);
        assert_eq!(cursor_key(&t), "Empty");
        assert!(!t.has_children(t.cursor));
        t.expand();
        assert_eq!(cursor_key(&t), "Empty");
    }

    #[test]
    fn expand_all_opens_nested_branches_of_current_branch_only() {
        let mut t = InspectTree::new(&json!({
            "A": { "B": { "C": 1 } },
            "D": { "E": 2 },
        }));
        t.expand_all();
        assert_eq!(keys(&t), ["A", "B", "C", "D"]);
        assert_eq!(cursor_key(&t), "A");
    }

    #[test]
    fn collapse_all_also_closes_nested_branches() {
        let mut t = tree();
        t.expand_all();
        t.collapse_all();
        assert_eq!(keys(&t), ["Config", "Empty", "Name"]);
        t.expand();
        assert_eq!(keys(&t), ["Config", "Env", "Tty", "Empty", "Name"]);
    }

    #[test]
    fn all_variants_on_a_leaf_act_on_its_parent() {
        let mut t = tree();
        t.expand();
        t.move_by(2);
        assert_eq!(cursor_key(&t), "Tty");
        t.expand_all();
        assert!(keys(&t).contains(&"[0]"));
        assert_eq!(cursor_key(&t), "Tty");
        t.collapse_all();
        assert_eq!(keys(&t), ["Config", "Empty", "Name"]);
        assert_eq!(cursor_key(&t), "Config");
    }

    #[test]
    fn all_variants_on_a_top_level_leaf_do_nothing() {
        let mut t = tree();
        t.move_by(2);
        assert_eq!(cursor_key(&t), "Name");
        t.expand_all();
        t.collapse_all();
        assert_eq!(keys(&t), ["Config", "Empty", "Name"]);
        assert_eq!(cursor_key(&t), "Name");
    }

    #[test]
    fn move_by_clamps_and_skips_hidden_rows() {
        let mut t = tree();
        t.move_by(-1);
        assert_eq!(cursor_key(&t), "Config");
        t.move_by(1);
        assert_eq!(cursor_key(&t), "Empty");
        t.move_by(99);
        assert_eq!(cursor_key(&t), "Name");
    }

    #[test]
    fn search_reveals_nested_match_case_insensitively() {
        let mut t = tree();
        t.set_query("token".into());
        assert_eq!(t.nodes[t.cursor].text, "TOKEN=abc");
        assert_eq!(t.path(t.cursor), "Config.Env[1]");
        assert!(keys(&t).contains(&"[1]"));
    }

    #[test]
    fn matches_wrap_in_both_directions() {
        let mut t = tree();
        t.set_query("e".into());
        let matches = t.matches();
        assert!(matches.len() > 1);
        t.cursor = *matches.last().unwrap();
        t.next_match();
        assert_eq!(t.cursor, matches[0]);
        t.prev_match();
        assert_eq!(t.cursor, *matches.last().unwrap());
    }

    #[test]
    fn typing_keeps_cursor_on_a_row_that_still_matches() {
        let mut t = tree();
        t.move_by(2);
        t.set_query("na".into());
        assert_eq!(cursor_key(&t), "Name");
        t.set_query("nam".into());
        assert_eq!(cursor_key(&t), "Name");
    }

    #[test]
    fn array_indices_and_branch_sizes_do_not_match() {
        let mut t = tree();
        t.set_query("[0".into());
        assert!(t.matches().is_empty());
        t.set_query("{2".into());
        assert!(t.matches().is_empty());
    }

    #[test]
    fn no_match_leaves_cursor_alone() {
        let mut t = tree();
        t.move_by(1);
        t.set_query("zzz".into());
        assert_eq!(cursor_key(&t), "Empty");
    }

    #[test]
    fn empty_tree_does_not_panic() {
        let mut t = InspectTree::new(&json!({}));
        t.move_by(1);
        t.top();
        t.bottom();
        t.expand();
        t.collapse();
        t.toggle();
        t.expand_all();
        t.collapse_all();
        t.set_query("x".into());
        t.next_match();
        t.prev_match();
    }
}
