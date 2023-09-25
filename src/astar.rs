use std::{collections::{HashMap, HashSet, BinaryHeap}, cmp::Ordering};
use crate::field::{Field, Move};


#[allow(dead_code)]
#[derive(Clone, Copy)]
struct AStarNode {
    id: usize,
    parent_id: usize,
    field: Field,
    heu_g: f32,
    heu_h: f32,
    heu: f32,
    previous_move: Option<Move>,
}


#[allow(dead_code)]
pub fn calculate_heuristic(field: &Field) -> f32 {
    (0_u8..16_u8).map(|i_target| {
        let target_piece = (i_target + 1) % 16;
        if target_piece == 0_u8 { return 0.0 }
        for i_current in 0_u8..16_u8 {
            if field.get_linear(i_current) == target_piece {
                let target = [(i_target % 4) as i32, (i_target / 4) as i32];
                let current = [(i_current % 4) as i32, (i_current / 4) as i32];
                return ((target[0] - current[0]).abs() + (target[1] - current[1]).abs()) as f32;
            }
        }
        0.0
    }).sum()
}

#[allow(dead_code)]
pub fn calculate_heuristic_change(field: &Field, m: Move) -> f32 {
    assert!(field.is_valid_move(m));

    let empty_index = field.get_empty_index();
    let piece_index = match m {
        Move::Up => { empty_index - 4 }
        Move::Down => { empty_index + 4 }
        Move::Left => { empty_index - 1 }
        Move::Right => { empty_index + 1 }
    };

    let piece_pos = [(piece_index % 4) as i8, (piece_index / 4) as i8];
    let piece = field.get_linear(piece_index);
    
    let target_index = piece - 1;
    let target_pos = [(target_index % 4) as i8, (target_index / 4) as i8];
    

    match m {
        Move::Up => {
            ((target_pos[1] - (piece_pos[1] + 1)).abs() - (target_pos[1] - piece_pos[1]).abs()) as f32
        }

        Move::Down => {
            ((target_pos[1] - (piece_pos[1] - 1)).abs() - (target_pos[1] - piece_pos[1]).abs()) as f32
        }

        Move::Left => {
            ((target_pos[0] - (piece_pos[0] + 1)).abs() - (target_pos[0] - piece_pos[0]).abs()) as f32
        }

        Move::Right => {
            ((target_pos[0] - (piece_pos[0] - 1)).abs() - (target_pos[0] - piece_pos[0]).abs()) as f32
        }
    }
}


#[allow(unused)]
impl AStarNode {
    fn new() -> AStarNode {
        AStarNode {
            id: usize::MAX,
            parent_id: usize::MAX,
            field: Field::new(),
            heu_g: 0.0,
            heu_h: 0.0,
            heu: 0.0,
            previous_move: None,
        }
    }

    fn from(field: Field) -> AStarNode {
        AStarNode {
            id: usize::MAX,
            parent_id: usize::MAX,
            field,
            heu_g: 0.0,
            heu_h: 0.0,
            heu: 0.0,
            previous_move: None,
        }
    }
}

impl Default for AStarNode {
    fn default() -> Self {
        Self::new()
    }
}


impl PartialEq for AStarNode {
    fn eq(&self, other: &Self) -> bool {
        self.field == other.field
    }
}

impl Eq for AStarNode {}


impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let result = self.heu.partial_cmp(&other.heu);
        if let Some(result_unwrapped) = result { Some(result_unwrapped.reverse()) } else { result }
    }
}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        assert!(self.heu.partial_cmp(&other.heu).is_some());
        self.heu.partial_cmp(&other.heu).unwrap().reverse()
    }
}



pub struct AStarSolution {
    pub moves: Vec<Move>,
    pub states: Vec<Field>,
}


pub struct AStar {
    nodes: Vec<AStarNode>,
    open: BinaryHeap<AStarNode>,
    added: HashSet<u64>,
    closed: HashMap<u64, usize>,
}

#[allow(unused)]
impl AStar {
    pub fn new(start: &Field) -> AStar {
        let mut astar = AStar {
            nodes: Default::default(),
            open: Default::default(),
            added: Default::default(),
            closed: Default::default(),
        };

        
        let mut node = AStarNode::new();
        node.field = start.clone();
        node.heu_h = calculate_heuristic(start);
        node.heu_g = 0.0;
        node.heu = node.heu_h;
        node.id = astar.nodes.len();
        node.parent_id = astar.nodes.len();
        astar.nodes.push(node);
        astar.open.push(node);
        astar.added.insert(start.unique_id());

        astar
    }

    pub fn step(&mut self) -> Option<Result<AStarSolution, ()>> {
        let Some(current_node) = self.open.pop() else { return Some(Err(())) };

        // Goal Reached
        if current_node.field.is_solved() {
            let mut node = &current_node;
            let mut prev_node_option: Option<&AStarNode> = None;
            let mut states = Vec::new();
            let mut moves = Vec::new();
            loop {
                states.push(node.field.clone());
                
                if let Some(prev_node) = prev_node_option {
                    moves.push(prev_node.previous_move.unwrap());
                }

                let next_node = self.nodes.get(node.parent_id);
                if next_node.is_none() { break }
                if node == next_node.unwrap() { break }
                prev_node_option = Some(&node);
                node = next_node.unwrap();
            }
            
            moves.reverse();
            states.reverse();
            return Some(Ok(AStarSolution {moves, states}));
        }

        assert!(self.closed.get(&current_node.field.unique_id()).is_none(), "wtf is that = {:#01x}", current_node.field.unique_id());
        self.closed.insert(current_node.field.unique_id(), current_node.id);

        Move::iter().for_each(|m| {
            if current_node.previous_move.is_some() && current_node.previous_move.unwrap() == m.inverse() { return }
            if !current_node.field.is_valid_move(m) { return }

            let mut field: Field = current_node.field.clone();
            let heu_g = current_node.heu_g + 1.0;
            let heu_h = current_node.heu_h + calculate_heuristic_change(&field, m);
            let heu = heu_g + heu_h;
            field.make_move(m);
            
            if self.added.contains(&field.unique_id()) { return }

            let mut node = AStarNode::from(field);
            node.id = self.nodes.len();
            node.parent_id = current_node.id;
            node.heu_g = heu_g;
            node.heu_h = heu_h;
            node.heu = heu;
            node.previous_move = Some(m);
            self.nodes.push(node);

            self.open.push(node);
            self.added.insert(field.unique_id());
        });

        None
    }
}



pub struct IDAStar {
    nodes: Vec<AStarNode>,
    closed: HashSet<u64>,
    start: Field,
}


#[allow(unused)]
impl IDAStar {
    pub fn new(start: &Field) -> IDAStar {
        let mut idastar = IDAStar {
            nodes: Default::default(),
            closed: Default::default(),
            start: start.clone(),
        };
        idastar
    }

    pub fn run(&mut self) -> Result<AStarSolution, ()> {
        let root = AStarNode {
            field: self.start.clone(),
            heu_h: 0.0, heu_g: 0.0, heu: 0.0, //ignore
            id: self.nodes.len(),
            parent_id: self.nodes.len(),
            
            previous_move: None,
        };
        
        self.nodes.push(root);
        
        let mut bound = calculate_heuristic(&self.start);
        loop {
            match self.search(root.id, 0.0, bound) {
                Ok(solution) => return Ok(solution),
                Err(t) => {
                    if t.is_infinite() { return Err(()) }
                    bound = t;
                }
            }
        }
    }


    fn search(&mut self, current_node_id: usize, g: f32, bound: f32) -> Result<AStarSolution, f32> {
        let current_node = self.nodes.get(current_node_id).unwrap();
        let f = g + calculate_heuristic(&current_node.field);
        if f > bound { return Err(f) }
        if current_node.field.is_solved() {
            let mut node = &current_node.clone();
            let mut prev_node_option: Option<&AStarNode> = None;
            let mut states = Vec::new();
            let mut moves = Vec::new();
            loop {
                states.push(node.field.clone());
                
                if let Some(prev_node) = prev_node_option {
                    moves.push(prev_node.previous_move.unwrap());
                }

                let next_node = self.nodes.get(node.parent_id);
                if next_node.is_none() { break }
                if node == next_node.unwrap() { break }
                prev_node_option = Some(&node);
                node = next_node.unwrap();
            }
            
            moves.reverse();
            states.reverse();
            return Ok(AStarSolution {moves, states});
        }

        let mut min = f32::INFINITY;
        let successors = self.successors(current_node_id);
        for x_id in successors {
            match self.search(x_id, g + 1.0, bound) {
                Ok(solution) => return Ok(solution),
                Err(t) => {
                    if t < min { min = t }
                }
            }
        }

        return Err(min);
    }


    fn successors(&mut self, node_id: usize) -> Vec<usize> {
        let node = self.nodes.get(node_id).unwrap().clone();
        let mut v = Vec::new();
        for m in Move::iter() {
            if node.previous_move.is_some() && node.previous_move.unwrap() == m.inverse() { continue }
            if !node.field.is_valid_move(m) { continue }

            let mut field: Field = node.field.clone();
            let heu_g = node.heu_g + 1.0;
            let heu_h = node.heu_h + calculate_heuristic_change(&field, m);
            let heu = heu_g + heu_h;
            field.make_move(m);
            
            let id = field.unique_id();
            // if self.closed.contains(&id) { continue }
            // self.closed.insert(id);

            let mut new_node = AStarNode::from(field);
            new_node.id = self.nodes.len();
            new_node.parent_id = node.id;
            new_node.previous_move = Some(m);
            self.nodes.push(new_node);
            v.push(new_node.id);
        }
        v
    }
}


// procedure ida_star(root)
//    bound := h(root)
//    loop
//      t := search(root, 0, bound)
//      if t = FOUND then return FOUND
//      if t = ∞ then return NOT_FOUND
//      bound := t
//    end loop
//  end procedure
 
//  function search(node, g, bound)
//    f := g + h(node)
//    if f > bound then return f
//    if is_goal(node) then return FOUND
//    min := ∞
//    for succ in successors(node) do
//      t := search(succ, g + cost(node, succ), bound)
//      if t = FOUND then return FOUND
//      if t < min then min := t
//    end for
//    return min
//  end function
