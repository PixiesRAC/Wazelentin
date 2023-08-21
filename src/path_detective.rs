use crate::GridInfo;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;
use std::time::Duration;
use std::{thread, vec};
pub struct PathDetective {
    pub grid_info: GridInfo,
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
        if let Some(_) = graph_cases.get(&self.grid_info.start_pos) {
            let mut visited_positions: HashSet<(usize, usize)> = HashSet::new();
            let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::new();

            queue.push_back((self.grid_info.start_pos, 0));

            while let Some((current_pos, distance)) = queue.pop_front() {
                println!("POPPING {:?}", current_pos);
                // Marquez la position actuelle comme visitée
                visited_positions.insert(current_pos);

                // Accédez aux informations pour la position actuelle
                let rc_current_info = graph_cases
                    .get(&current_pos)
                    .expect("Position not found in graph_cases");
                let mut current_info = rc_current_info.borrow_mut();
                current_info.distance_from_start = distance;

                println!(
                    "Current pos : {:?} | associated {:?} | dist {:?}",
                    current_pos, current_info.connected_case, distance
                );
                // Vérifiez si la valeur cible existe dans la connected_case
                if current_info
                    .connected_case
                    .contains(&self.grid_info.exit_pos)
                {
                    println!(
                        "Found target value {:?} in connected_case for position {:?}",
                        &self.grid_info.exit_pos, current_pos
                    );
                    break;
                }

                // Ajoutez les positions connectées non visitées à la pile
                for &connected_pos in &current_info.connected_case {
                    if !visited_positions.contains(&connected_pos) {
                        let contains_value = queue
                            .iter()
                            .any(|((first_value), _)| first_value == &connected_pos);
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

    fn find_best_intersection(
        &self,
        graph: &HashMap<(usize, usize), Rc<RefCell<GraphInfo>>>,
        visited: &mut HashSet<(usize, usize)>,
        pos: &(usize, usize),
    ) {
        if pos != &self.grid_info.start_pos {
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
            println!("Jumping to case [{:?}]", next_case_to_check);
            self.find_best_intersection(&graph.clone(), visited, &next_case_to_check.0);
        }
    }

    pub fn find_shortest_path(&self) {
        let mut graph_cases: HashMap<(usize, usize), Rc<RefCell<GraphInfo>>> = HashMap::new();
        self.fill_graph(&mut graph_cases);
        let mut visited: std::collections::HashSet<(usize, usize)> =
            std::collections::HashSet::new();
        self.find_best_intersection(&graph_cases.clone(), &mut visited, &self.grid_info.exit_pos);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // #[test]
    fn test_connected_case_for_second_case() {
        let grid_info = GridInfo {
            start_pos: (0, 2),
            exit_pos: (usize::MAX, usize::MAX),
            row_max: 2,
            column_max: 6,
            grid: [
                ['E', 'O', 'O', 'E', 'E', 'E', 'E'].to_vec(),
                ['O', 'O', 'O', 'O', 'O', 'E', 'E'].to_vec(),
                ['O', 'X', 'O', 'E', 'E', 'E', 'E'].to_vec(),
            ]
            .to_vec(),
        };
        let wazelentin = PathDetective {
            grid_info: grid_info,
        };
        let mut graph_cases: Vec<GraphInfo> = Vec::new();
        //wazelentin.fill_graph(&mut graph_cases);
        //assert_eq!(graph_cases[1].pos, (0,2));
        //assert_eq!(graph_cases[1].connected_case, [(1,3), (1,2), (1,1), (0,1)])
    }
    // #[test]
    fn test_distance_form_start() {
        let grid_info = GridInfo {
            start_pos: (0, 2),
            exit_pos: (usize::MAX, usize::MAX),
            row_max: 2,
            column_max: 6,
            grid: [
                ['O', 'O', 'O', 'O', 'E', 'E', 'E'].to_vec(),
                ['O', 'O', 'O', 'O', 'X', 'E', 'E'].to_vec(),
                ['O', 'O', 'O', 'O', 'E', 'E', 'E'].to_vec(),
            ]
            .to_vec(),
        };
        let wazelentin = PathDetective {
            grid_info: grid_info,
        };
        let mut graph_cases: std::collections::HashMap<(usize, usize), GraphInfo> =
            std::collections::HashMap::new();
        wazelentin.fill_graph(&mut graph_cases);
        assert_eq!(graph_cases[&(0, 2)].pos, (0, 2));
        assert_eq!(graph_cases[&(0, 2)].distance_from_start, 0);
        assert_eq!(graph_cases[&(0, 1)].pos, (0, 1));
        assert_eq!(graph_cases[&(0, 1)].distance_from_start, 1);
        assert_eq!(graph_cases[&(1, 2)].pos, (1, 2));
        assert_eq!(graph_cases[&(1, 2)].distance_from_start, 1);
        assert_eq!(graph_cases[&(2, 3)].pos, (2, 3));
        assert_eq!(graph_cases[&(2, 3)].distance_from_start, 2);
        //assert_eq!(graph_cases[1].connected_case, [(1,3), (1,2), (1,1), (0,1)])
    }
}
