use super::Error;
use super::TreeModel;
use crate::syntax::SyntaxValue;
use graphviz_rust::cmd::{CommandArg, Format};
use graphviz_rust::exec_dot;
use std::fmt::Display;
use std::fmt::Write;

impl<K: Display> SyntaxValue<K> {
    pub fn to_dot_attr(&self) -> String {
        match self {
            SyntaxValue::Item(item) => format!("label=\"{}\", color=lightgreen", item),
            SyntaxValue::Features(features) => {
                let mut entries = Vec::new();
                for (category, value) in features.iter() {
                    match value {
                        Some(value) => entries.push(format!("{}:{}", category, value)),
                        None => entries.push(format!("{}", category)),
                    }
                }
                entries.sort();
                format!("label=\"{}\", color=lightblue", entries.join("\n"))
            }
        }
    }
}

impl<K: Display> TreeModel<K> {
    pub fn to_dot_graph(&self) -> Result<String, Error> {
        let mut graph = String::new();
        if self.is_empty() {
            return Ok(graph);
        }

        graph.push_str("digraph {{\n");
        graph.push_str("    rankdir=TB;\n");
        graph.push_str("    node [shape=box, style=filled];\n");

        let mut nodes = Vec::new();
        nodes.push(self.get_root());
        while let Some(id) = nodes.pop() {
            let value = self.get_value(id)?.to_dot_attr();
            writeln!(&mut graph, r#"    "{}" [{}];"#, id, value)?;

            if let Some(left_id) = self.get_left(id)? {
                writeln!(
                    &mut graph,
                    r#"    "{}" -> "{}" [arrowhead=none];"#,
                    id, left_id
                )?;
                nodes.push(left_id);
            }
            if let Some(right_id) = self.get_right(id)? {
                writeln!(
                    &mut graph,
                    r#"    "{}" -> "{}" [arrowhead=none];"#,
                    id, right_id
                )?;
                nodes.push(right_id);
            }
            if let Some(moved_id) = self.get_moved(id)? {
                writeln!(
                    &mut graph,
                    r#"    "{}" -> "{}" [style=dashed, constraint=false, color=blue];"#,
                    id, moved_id
                )?;
            }
        }

        graph.push_str("}}");
        Ok(graph)
    }

    pub fn to_png(&self, filename: String) -> Result<(), Error> {
        if !is_dot_installed() {
            return Err(Error::GraphvizDotNotInstalled);
        }
        let dot = self.to_dot_graph()?;

        exec_dot(dot, vec![Format::Png.into(), CommandArg::Output(filename)])?;
        Ok(())
    }
}

use std::process::Command;
fn is_dot_installed() -> bool {
    match Command::new("dot").arg("-V").output() {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}
