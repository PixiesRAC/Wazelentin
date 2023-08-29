use crate::GridInfo;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;
use std::sync::mpsc;

pub struct PathDetective {
    pub grid_info: GridInfo,
    pub sender : Option<mpsc::Sender<(usize, usize)>>
}

#[derive(Clone, Debug)]
struct GraphInfo {
    pos: (usize, usize),
    distance_from_start: usize,
    connected_case: Vec<(usize, usize)>,
}

impl PathDetective {
    fn is_valid_case(&self, row: usize, column: usize) -> bool {
        if let Some(case) = self.grid_info.grid.get(row).and_then(|r| r.get(column)) {
            *case != GridInfo::CASE_CLOSE
        } else {
            false
        }
    }
    fn fill_graph(&self, graph_cases: &mut HashMap<(usize, usize), Rc<RefCell<GraphInfo>>>) {
        let mut graph_case = GraphInfo {
            pos: (usize::MAX, usize::MAX),
            distance_from_start: usize::MAX,
            connected_case: Vec::new(),
        };
        let mut row: usize = 0;
        let directions = [
            (-1, 0),  /* DOWN */
            (-1, 1),  /* DOWN RIGHT */
            (0, 1),   /* RIGHT */
            (1, 1),   /* UP RIGHT */
            (1, 0),   /* UP */
            (1, -1),  /* UP LEFT */
            (0, -1),  /* LEFT */
            (-1, -1), /* DOWN LEFT */
        ];

        for grid_line in &self.grid_info.grid {
            let mut column = 0;
            for _ in grid_line {
                if self.grid_info.grid[row][column] != GridInfo::CASE_CLOSE {
                    graph_case.connected_case.clear();
                    graph_case.pos = (row, column);

                    for &(row_offset, column_offset) in &directions {
                        if row_offset < 0 && row == 0 {
                            continue;
                        }
                        if column_offset < 0 && column == 0 {
                            continue;
                        }
                        let new_row = (row as isize + row_offset) as usize;
                        let new_column = (column as isize + column_offset) as usize;
                        if self.is_valid_case(new_row, new_column) {
                            graph_case.connected_case.push((new_row, new_column));
                        }
                    }
                    graph_case.distance_from_start = usize::MAX;
                    let graph_info_rc = Rc::new(RefCell::new(graph_case.clone()));
                    graph_cases.insert((row, column), graph_info_rc);
                }
                column += 1;
            }
            row += 1;
        }
        self.fill_intersection_distance(graph_cases);
    }

    fn fill_intersection_distance(&self, graph_cases: &HashMap<(usize, usize), Rc<RefCell<GraphInfo>>>) {
        if graph_cases.get(&self.grid_info.start_pos).is_some() {
            let mut visited_positions: HashSet<(usize, usize)> = HashSet::new();
            let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::new();

            queue.push_back((self.grid_info.start_pos, 0));

            while let Some((current_pos, distance)) = queue.pop_front() {
                //println!("POPPING {:?}", current_pos);
                visited_positions.insert(current_pos);

                let rc_current_info = graph_cases
                    .get(&current_pos)
                    .expect("Position not found in graph_cases");
                let mut current_info = rc_current_info.borrow_mut();
                current_info.distance_from_start = distance;

                /*println!(
                    "Current pos : {:?} | associated {:?} | dist {:?}",
                    current_pos, current_info.connected_case, distance
                );*/
                if current_info
                    .connected_case
                    .contains(&self.grid_info.exit_pos)
                {
                    /*println!(
                        "Found target value {:?} in connected_case for position {:?}",
                        &self.grid_info.exit_pos, current_pos
                    );*/
                    break;
                }

                for &connected_pos in &current_info.connected_case {
                    //println!("{:?}", connected_pos);
                    if !visited_positions.contains(&connected_pos) {
                        let contains_value = queue
                            .iter()
                            .any(|(first_value, _)| first_value == &connected_pos);
                        if !contains_value {
                            queue.push_back((connected_pos, distance + 1));
                        }
                    }
                }
            }
        } else {
            println!("Start position not found in graph_cases");
        }
    }

    #[allow(unused_must_use)]
    fn find_best_intersection(
        &self,
        graph: &HashMap<(usize, usize), Rc<RefCell<GraphInfo>>>,
        visited: &mut HashSet<(usize, usize)>,
        pos: &(usize, usize),
        shortest_path: &mut Vec<(usize, usize)>) {
        if pos != &self.grid_info.start_pos {
            //println!("{:?}", pos);
            let rc_graph_info = graph.get(pos).expect("Position not found in graph_cases");
            let graph_info = rc_graph_info.borrow();

            let mut next_case_to_check = ((usize::MAX, usize::MAX), usize::MAX);
            visited.insert(*pos);
            for connected_pos in &graph_info.connected_case {
                if !visited.contains(connected_pos) {
                    let rc_graph_info_for_connected_pos = graph
                        .get(connected_pos)
                        .expect("Position not found in graph_cases");
                    let graph_info_for_connected_pos = rc_graph_info_for_connected_pos.borrow();
                    if graph_info_for_connected_pos.distance_from_start < next_case_to_check.1 {
                        next_case_to_check = (
                            *connected_pos,
                            graph_info_for_connected_pos.distance_from_start,
                        );
                    }
                }
            }
            if let Some(sender) = &self.sender
            {
                sender.send(next_case_to_check.0).unwrap();
            }
            //print!("{:?}", next_case_to_check.0);
            shortest_path.insert(0, next_case_to_check.0);
            self.find_best_intersection(&graph.clone(), visited, &next_case_to_check.0, shortest_path);
        }
    }

    pub fn find_and_transmit_shortest_path(&self) -> Vec<(usize, usize)> {
        let mut shortest_path = Vec::new();
        let mut graph_cases: HashMap<(usize, usize), Rc<RefCell<GraphInfo>>> = HashMap::new();
        self.fill_graph(&mut graph_cases);
        let mut visited: std::collections::HashSet<(usize, usize)> =
            std::collections::HashSet::new();
        shortest_path.push(self.grid_info.exit_pos);
        self.find_best_intersection(&graph_cases.clone(), &mut visited, &self.grid_info.exit_pos, &mut shortest_path);
        shortest_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pos_for_second_case_on_first_row() {
        let grid_info = GridInfo {
            start_pos: (0, 2),
            exit_pos: (usize::MAX, usize::MAX),
            row_max: 2,
            column_max: 6,
            grid: [
                ['E', 'O', 'O', 'E', 'E', 'E', 'E'].to_vec(),
                ['O', 'O', 'O', 'O', 'O', 'E', 'E'].to_vec(),
                ['O', 'O', 'O', 'E', 'E', 'E', 'E'].to_vec(),
            ]
            .to_vec(),
        };
        let wazelentin = PathDetective {
            grid_info: grid_info,
            sender: None
        };
        let mut graph_cases: HashMap<(usize, usize), Rc<RefCell<GraphInfo>>> = HashMap::new();

        wazelentin.fill_graph(&mut graph_cases);
        assert_eq!(graph_cases.get(&(0, 2)).unwrap().borrow().pos, (0, 2));
}
    #[test]
    fn test_connected_case() {
        let mut grid_info = GridInfo {
            start_pos: (0, 2),
            exit_pos: (usize::MAX, usize::MAX),
            row_max: 2,
            column_max: 6,
            grid: [
                ['E', 'O', 'O', 'E', 'E', 'E', 'E'].to_vec(),
                ['O', 'O', 'O', 'O', 'O', 'E', 'E'].to_vec(),
                ['O', 'O', 'O', 'E', 'E', 'E', 'E'].to_vec(),
            ]
            .to_vec(),
        };
        grid_info.grid.reverse();
        let wazelentin = PathDetective {
            grid_info: grid_info,
            sender: None
        };
        let mut graph_cases: HashMap<(usize, usize), Rc<RefCell<GraphInfo>>> = HashMap::new();

        wazelentin.fill_graph(&mut graph_cases);
        assert_eq!(graph_cases.get(&(0, 0)).unwrap().borrow().connected_case, [(0, 1), (1, 1), (1, 0)]);
        assert_eq!(graph_cases.get(&(1, 4)).unwrap().borrow().connected_case, [(1, 3)]);
    }

    #[test]
    fn test_find_best_path() {
        let mut grid_info = GridInfo {
            start_pos: (2, 6),
            exit_pos: (0, 0),
            row_max: 5,
            column_max: 6,
            grid: [
                ['E', 'O', 'E', 'O', 'E', 'O', 'E'].to_vec(),
                ['O', 'O', 'O', 'O', 'O', 'O', 'O'].to_vec(),
                ['O', 'O', 'O', 'O', 'O', 'E', 'O'].to_vec(),
                ['O', 'O', 'O', 'O', 'O', 'E', 'O'].to_vec(),
                ['O', 'O', 'O', 'O', 'O', 'E', 'E'].to_vec(),
                ['X', 'O', 'O', 'E', 'E', 'O', 'O'].to_vec(),
            ]
            .to_vec(),
        };
        grid_info.grid.reverse();
        let wazelentin = PathDetective {
            grid_info: grid_info,
            sender: None
        };
        assert_eq!(wazelentin.find_and_transmit_shortest_path(), [(2, 6), (3, 6), (4, 5), (3, 4), (2, 3), (1, 2), (1, 1), (0, 0)]);
    }
}
