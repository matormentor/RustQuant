// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! This module contains the implementation of the computation `Graph`.
//! The graph is also known as a Wengert List.
//!
//! The graph is an abstract data structure that contains `Vertex`s. These
//! contain the adjoints and indices to the parent vertices.

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPORTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::{variable::Variable, Arity, Vertex};
use std::sync::{Arc, RwLock};
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// GRAPH STRUCTS AND IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Struct to contain the graph (Wengert list), as a vector of `Vertex`s.
#[derive(Debug, Clone)]
pub struct Graph {
    /// Vector containing the vertices in the Wengert List.
    pub vertices: Arc<RwLock<Vec<Vertex>>>,
}
// pub struct Graph(RefCell<Rc<[Vertex]>>);

impl Default for Graph {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

/// Implementation for the `Graph` struct.
impl Graph {
    /// Instantiate a new graph.
    #[must_use]
    #[inline]
    pub fn new() -> Self {
        Self {
            vertices: Arc::new(RwLock::new(Vec::new())),
            // vertices: RefCell::new(Rc::new([])),
        }
    }

    /// Instantiate a new graph with a capacity.
    #[must_use]
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Graph {
            vertices: Arc::new(RwLock::new(Vec::with_capacity(capacity))),
            // vertices: RefCell::new(Rc::new([])),
        }
    }

    /// Join two graphs together.
    #[must_use]
    #[inline]
    pub fn join(&self, other: &Self) -> Self {
        let graph = self.clone();
        let other_vertices = other.vertices.read().unwrap().clone();
        graph.vertices.write().unwrap().extend(other_vertices);
	    graph
    }

    /// Add a new variable to the graph.
    /// Returns a new `Variable` instance (the contents of a vertex).
    #[inline]
    pub fn var(&self, value: f64) -> Variable {
        Variable {
            graph: self,
            value,
            index: self.push(Arity::Nullary, &[], &[]),
        }
    }

    /// Add multiple variables (a slice) to the graph.
    /// Useful for larger functions with many inputs.
    #[inline]
    pub fn vars(&self, values: &[f64]) -> Vec<Variable> {
        values.iter().map(|&val| self.var(val)).collect()
    }

    /// Returns the length of the graph so new vertices can index to the correct position.
    #[inline]
    pub fn len(&self) -> usize {
        self.vertices.read().unwrap().len()
    }

    /// Returns true/false depending on whether the graph is empty or not.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.vertices.read().unwrap().len() == 0
    }

    /// Clears the entire graph.
    #[inline]
    pub fn clear(&self) {
        self.vertices.write().unwrap().clear();
    }

    /// Zeroes the adjoints in the graph.
    #[inline]
    pub fn zero(&self) {
        self.vertices
	        .write().unwrap()
            .iter_mut()
            .for_each(|vertex| vertex.partials = [0.0; 2]);
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Functions to push values to the graph (Wengert List):
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    /// Pushes a vertex to the graph.
    #[inline]
    pub fn push(&self, arity: Arity, parents: &[usize], partials: &[f64]) -> usize {
        let mut vertices = self.vertices.write().unwrap();
        let len = vertices.len();

        let vertex = match arity {
            // Nullary operator pushback.
            //
            // The vertex pushed to the graph is the result of a **nullary** operation.
            // e.g. `x.neg()` ($-x$)
            // Thus no partials and only the current index are added to the new vertex.
            //
            // 1. Constructs the vertex,
            // 2. Pushes the new vertex onto the graph,
            // 3. Returns the index of the new vertex.
            Arity::Nullary => {
                assert!(parents.is_empty());

                Vertex {
                    partials: [0.0, 0.0],
                    parents: [len, len],
                }
            }
            // Unary operator pushback.
            //
            // The vertex pushed to the graph is the result of a **unary** operation.
            // e.g. `x.sin()` ($sin(x)$)
            // Thus one partial and one parent are added to the new vertex.
            //
            // 1. Constructs the vertex,
            // 2. Pushes the new vertex onto the graph,
            // 3. Returns the index of the new vertex.
            Arity::Unary => {
                assert_eq!(parents.len(), 1);

                Vertex {
                    partials: [partials[0], 0.0],
                    parents: [parents[0], len],
                }
            }
            // Binary operator pushback.
            //
            // The vertex pushed to the graph is the result of a **binary** operation.
            // e.g. `x + y`
            // Thus two partials and two parents are added to the new vertex.
            //
            // 1. Constructs the vertex,
            // 2. Pushes the new vertex onto the graph,
            // 3. Returns the index of the new vertex.
            Arity::Binary => {
                assert_eq!(parents.len(), 2);

                Vertex {
                    partials: [partials[0], partials[1]],
                    parents: [parents[0], parents[1]],
                }
            }
        };

        vertices.push(vertex);

        len
    }
}
