/*
function negamax(node, depth, α, β, color) is
    if depth = 0 or node is a terminal node then
        return color × the heuristic value of node

    childNodes := generateMoves(node)
    childNodes := orderMoves(childNodes)
    value := −∞
    foreach child in childNodes do
        value := max(value, −negamax(child, depth − 1, −β, −α, −color))
        α := max(α, value)
        if α ≥ β then
            break (* cut-off *)
    return value
*/

mod tests;

use std::{
    collections::HashMap,
    f64::{INFINITY, NEG_INFINITY},
};

#[allow(unused_imports)]
use crate::{
    lib::max,
    log,
};

pub trait GameNode<Node, Move>
where
    Node: GameNode<Node, Move>,
    Move: Copy,
{
    // heuristic function to get evaluation of the current game state
    fn evaluate(&self) -> f64;
    // get all possible continuations of the current node
    fn get_children_nodes(&self) -> HashMap<Move, Box<Node>>;
}

pub fn get_best_move<Node: GameNode<Node, Move>, Move: Copy>(
    node: &(dyn GameNode<Node, Move>),
    search_depth: usize,
) -> Option<Move> {
    if search_depth == 0 {
        panic!("search depth shouldn't be zero")
    }

    let children = node.get_children_nodes();

    let optional_move = children
        .iter()
        .map(|(mv, node)| {
            (
                mv,
                -alphabeta(
                    node.as_ref(),
                    search_depth - 1,
                    NEG_INFINITY,
                    INFINITY,
                    -1.,
                ),
            )
        })
        .reduce(|(move1, eval1), (move2, eval2)| {
            if eval1 > eval2 {
                (move1, eval1)
            } else {
                (move2, eval2)
            }
        });

    match optional_move {
        Some(mv) => Some(*mv.0),
        None => panic!("Found no legal moves"),
    }
}

fn alphabeta<Node: GameNode<Node, Move>, Move: Copy>(
    node: &(dyn GameNode<Node, Move>),
    depth: usize,
    mut alpha: f64,
    beta: f64,
    color: f64,
) -> f64 {
    if depth == 0 {
        let eval = node.evaluate() * color;
        // log!("reached depth zero: {}", eval);
        return eval;
    }

    let children_nodes = node.get_children_nodes();
    if children_nodes.is_empty() {
        let eval = node.evaluate() * color;
        // log!("reached terminal node: {}", eval);
        return eval;
    }

    let children: Vec<&Box<Node>> = children_nodes.values().collect();
    let mut value = f64::NEG_INFINITY;
    for child in children.iter() {
        let child_eval = -alphabeta(child.as_ref(), depth - 1, -beta, -alpha, -color);
        println!("eval is {}", child_eval);

        value = max(
            value,
            child_eval,
        );

        alpha = max(alpha, value);

        if alpha >= beta {
            break;
        }
    }

    value
}
