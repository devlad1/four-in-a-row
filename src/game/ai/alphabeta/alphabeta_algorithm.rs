/*
function alphabeta(node, depth, α, β, maximizingPlayer) is
    if depth == 0 or node is terminal then
        return the heuristic value of node
    if maximizingPlayer then
        value := −∞
        for each child of node do
            value := max(value, alphabeta(child, depth − 1, α, β, FALSE))
            if value > β then
                break (* β cutoff *)
            α := max(α, value)
        return value
    else
        value := +∞
        for each child of node do
            value := min(value, alphabeta(child, depth − 1, α, β, TRUE))
            if value < α then
                break (* α cutoff *)
            β := min(β, value)
        return value

*/

mod tests;

use std::{
    collections::HashMap,
    f64::{INFINITY, NEG_INFINITY},
};

use crate::lib::{max, min};

// type Move = usize;

pub trait GameNode<Node, Move> where Node: GameNode<Node, Move>, Move: Copy {
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
        .map(|(mv, game)| {
            (
                mv,
                alphabeta(game.as_ref(), search_depth - 1, NEG_INFINITY, INFINITY, false),
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
        None => None,
    }
}

fn alphabeta<Node: GameNode<Node, Move>, Move: Copy>(
    node: &(dyn GameNode<Node, Move>),
    depth: usize,
    mut alpha: f64,
    mut beta: f64,
    maximizing_player: bool,
) -> f64 {
    if depth == 0 {
        return node.evaluate();
    }

    let children_nodes = node.get_children_nodes();
    if children_nodes.is_empty() {
        return node.evaluate();
    }
    
    let children: Vec<&Box<Node>> = children_nodes.values().collect();

    if maximizing_player {
        let mut value: f64 = f64::NEG_INFINITY;

        for child in children.iter() {
            value = max(
                value,
                alphabeta(child.as_ref(), depth - 1, alpha, beta, false),
            );

            if value > beta {
                break;
            }

            alpha = max(alpha, value)
        }

        return value;
    } else {
        let mut value: f64 = f64::INFINITY;

        for child in children.iter() {
            value = min(
                value,
                alphabeta(child.as_ref(), depth - 1, alpha, beta, true),
            );

            if value < alpha {
                break;
            }

            beta = min(beta, value)
        }

        return value;
    }
}
