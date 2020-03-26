#[cfg(not(feature = "std"))]
use alloc::{boxed::Box, vec::Vec};
#[cfg(not(feature = "std"))]
use hashbrown::HashMap;
#[cfg(feature = "std")]
use std::collections::HashMap;

use core::any::Any;
use core::ops::Drop;

use crate::forest::{Forest, NodeData, StretchNodeData};
use crate::geometry::Size;
use crate::id::{self, NodeId};
use crate::number::Number;
use crate::result::Layout;
use crate::style::*;
use crate::Error;

pub type MeasureFunc = Box<dyn Fn(Size<Number>) -> Result<Size<f32>, Box<dyn Any>>>;

lazy_static! {
    /// Global stretch instance id allocator.
    static ref INSTANCE_ALLOCATOR: id::Allocator = id::Allocator::new();
}

pub trait Node: Clone + std::fmt::Debug + PartialEq + Eq + std::hash::Hash {
    type Data: NodeData;

    fn new(instance: id::Id, local: id::Id) -> Self;

    fn local(&self) -> id::Id;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct StretchNode {
    instance: id::Id,
    local: id::Id,
}

impl Node for StretchNode {
    type Data = StretchNodeData;

    fn new(instance: id::Id, local: id::Id) -> Self {
        StretchNode { instance, local }
    }

    fn local(&self) -> id::Id {
        self.local
    }
}

pub struct Stretch<N: Node = StretchNode> {
    id: id::Id,
    nodes: id::Allocator,
    nodes_to_ids: HashMap<N, NodeId>,
    ids_to_nodes: HashMap<NodeId, N>,
    forest: Forest<N::Data>,
}

impl<N: Node> Default for Stretch<N> {
    fn default() -> Self {
        Stretch::with_capacity(16)
    }
}

impl<N: Node> Stretch<N> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Stretch {
            id: INSTANCE_ALLOCATOR.allocate(),
            nodes: id::Allocator::new(),
            nodes_to_ids: HashMap::with_capacity(capacity),
            ids_to_nodes: HashMap::with_capacity(capacity),
            forest: Forest::with_capacity(capacity),
        }
    }

    fn allocate_node(&mut self) -> N {
        let local = self.nodes.allocate();
        N::new(self.id, local)
    }

    fn add_node(&mut self, node: N, id: NodeId) {
        self.nodes_to_ids.insert(node.clone(), id);
        self.ids_to_nodes.insert(id, node);
    }

    // Find node in the forest.
    fn find_node(&self, node: &N) -> Result<NodeId, Error<N>> {
        match self.nodes_to_ids.get(node) {
            Some(id) => Ok(*id),
            None => Err(Error::InvalidNode(node.clone())),
        }
    }

    pub fn new_leaf(&mut self, style: Style, measure: MeasureFunc) -> Result<N, Error<N>> {
        let node = self.allocate_node();
        let id = self.forest.new_leaf(style, measure);
        self.add_node(node.clone(), id);
        Ok(node)
    }

    pub fn new_node(&mut self, style: Style, children: Vec<N>) -> Result<N, Error<N>> {
        let node = self.allocate_node();
        let children = children.iter().map(|child| self.find_node(child)).collect::<Result<Vec<_>, Error<N>>>()?;
        let id = self.forest.new_node(style, children);
        self.add_node(node.clone(), id);
        Ok(node)
    }

    /// Removes all nodes.
    ///
    /// All associated nodes will be invalid.
    pub fn clear(&mut self) {
        for node in self.nodes_to_ids.keys() {
            self.nodes.free(&[node.local()]);
        }
        self.nodes_to_ids.clear();
        self.ids_to_nodes.clear();
        self.forest.clear();
    }

    /// Remove nodes.
    pub fn remove(&mut self, node: &N) {
        let id = if let Ok(id) = self.find_node(node) { id } else { return };

        self.nodes_to_ids.remove(node);
        self.ids_to_nodes.remove(&id);

        if let Some(new_id) = self.forest.swap_remove(id) {
            let new = self.ids_to_nodes.remove(&new_id).unwrap();
            self.nodes_to_ids.insert(new.clone(), id);
            self.ids_to_nodes.insert(id, new);
        }
    }

    pub fn set_measure(&mut self, node: &N, measure: Option<MeasureFunc>) -> Result<(), Error<N>> {
        let id = self.find_node(node)?;
        *self.forest.nodes[id].measure_mut() = measure;
        self.forest.mark_dirty(id);
        Ok(())
    }

    pub fn add_child(&mut self, node: &N, child: &N) -> Result<(), Error<N>> {
        let node_id = self.find_node(node)?;
        let child_id = self.find_node(child)?;

        self.forest.add_child(node_id, child_id);
        Ok(())
    }

    pub fn set_children(&mut self, node: &N, children: Vec<N>) -> Result<(), Error<N>> {
        let node_id = self.find_node(node)?;
        let children_id = children.iter().map(|child| self.find_node(child)).collect::<Result<Vec<_>, _>>()?;

        // Remove node as parent from all its current children.
        for child in &self.forest.children[node_id] {
            self.forest.parents[*child].retain(|p| *p != node_id);
        }

        // Build up relation node <-> child
        for child in &children_id {
            self.forest.parents[*child].push(node_id);
        }
        self.forest.children[node_id] = children_id;

        self.forest.mark_dirty(node_id);
        Ok(())
    }

    pub fn remove_child(&mut self, node: &N, child: &N) -> Result<&N, Error<N>> {
        let node_id = self.find_node(node)?;
        let child_id = self.find_node(child)?;

        let prev_id = unsafe { self.forest.remove_child(node_id, child_id) };
        Ok(&self.ids_to_nodes[&prev_id])
    }

    pub fn remove_child_at_index(&mut self, node: &N, index: usize) -> Result<&N, Error<N>> {
        let node_id = self.find_node(node)?;
        // TODO: index check

        let prev_id = self.forest.remove_child_at_index(node_id, index);
        Ok(&self.ids_to_nodes[&prev_id])
    }

    pub fn replace_child_at_index(&mut self, node: &N, index: usize, child: &N) -> Result<&N, Error<N>> {
        let node_id = self.find_node(node)?;
        let child_id = self.find_node(child)?;
        // TODO: index check

        self.forest.parents[child_id].push(node_id);
        let old_child = core::mem::replace(&mut self.forest.children[node_id][index], child_id);
        self.forest.parents[old_child].retain(|p| *p != node_id);

        self.forest.mark_dirty(node_id);

        Ok(&self.ids_to_nodes[&old_child])
    }

    pub fn children(&self, node: &N) -> Result<Vec<&N>, Error<N>> {
        let id = self.find_node(node)?;
        Ok(self.forest.children[id].iter().map(|child| &self.ids_to_nodes[child]).collect())
    }

    pub fn child_count(&self, node: &N) -> Result<usize, Error<N>> {
        let id = self.find_node(node)?;
        Ok(self.forest.children[id].len())
    }

    pub fn set_style(&mut self, node: &N, style: Style) -> Result<(), Error<N>> {
        let id = self.find_node(node)?;
        *self.forest.nodes[id].style_mut() = style;
        self.forest.mark_dirty(id);
        Ok(())
    }

    pub fn style(&self, node: &N) -> Result<&Style, Error<N>> {
        let id = self.find_node(node)?;
        Ok(self.forest.nodes[id].style())
    }

    pub fn layout(&self, node: &N) -> Result<&Layout, Error<N>> {
        let id = self.find_node(node)?;
        Ok(self.forest.nodes[id].layout())
    }

    pub fn mark_dirty(&mut self, node: &N) -> Result<(), Error<N>> {
        let id = self.find_node(node)?;
        self.forest.mark_dirty(id);
        Ok(())
    }

    pub fn dirty(&self, node: &N) -> Result<bool, Error<N>> {
        let id = self.find_node(node)?;
        Ok(self.forest.nodes[id].is_dirty())
    }

    pub fn compute_layout(&mut self, node: &N, size: Size<Number>) -> Result<(), Error<N>> {
        let id = self.find_node(node)?;
        self.forest.compute_layout(id, size)
    }
}

impl<N: Node> Drop for Stretch<N> {
    fn drop(&mut self) {
        INSTANCE_ALLOCATOR.free(&[self.id]);
    }
}
