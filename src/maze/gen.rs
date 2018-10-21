//! This module implements the pseudo-random generation
//! of mazes using the depth-first search algorithm.
//!
//! The maze is stored as an undirected
//! connected graph with the cells representing spaces
//! in the maze and the edges representing walls between
//! two cells.
use self::rand::random;
use self::rand::Rng;
use super::*;
use petgraph::graph::NodeIndex;
use petgraph::*;
use rand;
use render::_RenderBackend;
use std::collections::HashMap;
use std::ops::Add;
use std::ops::IndexMut;
use render::RenderContext;

struct Ctx {
    maze: Graph<Cell, u32, Undirected>,
    stack: Vec<usize>,
    pos: usize,
}

#[derive(Clone, Copy, Debug, Hash)]
struct Cell {
    visited: bool,
}

pub fn gen_maze<B: RenderBackend>(
    width: usize,
    height: usize,
    render: &mut B::RenderContext,
) -> Vec<Object<B>> {
    let mut ctx = Ctx {
        maze: Graph::new_undirected(),
        stack: Vec::new(),
        pos: 0,
    };

    fill_graph(&mut ctx.maze, width, height);

    loop {
        let index = NodeIndex::new(ctx.pos);
        if !check_edges(&mut ctx.maze, ctx.pos) {
            // Backtrace or finish
            if ctx.pos == 0 {
                break;
            }

            ctx.pos = ctx.stack.pop().unwrap();
            continue;
        }

        ctx.stack.push(ctx.pos);
        let adjacents = find_neighbors(&mut ctx.maze, ctx.pos);
        let num = rand::thread_rng().gen_range(0, adjacents.len());
        let cell = adjacents[num];

        // Delete edge between
        let edge = ctx.maze
            .find_edge(index, NodeIndex::new(cell.index()))
            .unwrap();
        ctx.maze.remove_edge(
            edge
        );

        ctx.pos = cell.index();
    }
    compute_objects(&mut ctx, width, height, render)
}

/// Turns the graph into a vector of wall objects
/// with positions, returning said vector.
fn compute_objects<B: RenderBackend>(
    ctx: &mut Ctx,
    width: usize,
    height: usize,
    render: &mut B::RenderContext,
) -> Vec<Object<B>> {
    // Reset visited values so that `find_neighbors` will work
    for node in ctx.maze.node_weights_mut() {
        node.visited = false;
    }
    let mut result = Vec::new();
    // Follow the path
    loop {
        let cell: NodeIndex<u32> = NodeIndex::new(0);
        for adjacent in find_neighbors(&mut ctx.maze, cell.index()) {
            // If adjacent - cell > 1 or < -1, the wall alignment is
            // vertical; otherwise, it is horizontal.
            if (adjacent.index() as i64) - (cell.index() as i64) > 1 || (adjacent.index() as i64) - (cell.index() as i64) < -1 {
                // Vertical wall; don't rotate
                result.push(
                    Object::new(
                        B::create_obj_render(2, 0, render),
                        Location::new(
                            (cell.index() - (height * cell.index() / width)) as f64,
                            0.0,
                            (cell.index() - (width * cell.index() / width)) as f64,
                        )
                    )
                );

            }
        }
        if find_neighbors(&mut ctx.maze, cell.index()).len() == 0 {
            break;
        }
    }
    result
}

fn fill_graph(maze: &mut Graph<Cell, u32, Undirected>, width: usize, height: usize) {
    for h in 0..height {
        for w in 0..width {
            maze.add_node(Cell {
                visited: false,
            });
        }
    }
    // Connect all cells with walls
    for h in 0..height {
        for w in 0..width {
            let index_num = h * width + w;
            let index = NodeIndex::new(index_num);

            // Above
            if h > 0 {
                maze.add_edge(index, NodeIndex::new(index_num - width), 0);
            }
            // Right
            if w < width - 1 {
                maze.add_edge(index, NodeIndex::new(index_num + 1), 0);
            }
            // Below
            if h < height - 1 {
                maze.add_edge(index, NodeIndex::new(index_num + width), 0);
            }
            // Left
            if w > 0 {
                maze.add_edge(index, NodeIndex::new(index_num - 1), 0);
            }
        }
    }
}

fn find_neighbors(
    maze: &mut Graph<Cell, u32, Undirected>,
    pos: usize,
) -> Vec<NodeIndex<u32>> {
    let neighbors = maze.neighbors_undirected(NodeIndex::new(pos)).collect::<Vec<_>>();
    let mut result = Vec::new();
    for neighbor in neighbors {
        if !maze.node_weight(neighbor).unwrap().visited {
            result.push(neighbor);
        }
    }
    result
}

fn check_edges(maze: &mut Graph<Cell, u32, Undirected>, pos: usize) -> bool {
    // Return if there are available adjacent cells
    let neighbors = find_neighbors(maze, pos);
    neighbors.len() > 0
}
