use std::collections::{HashMap, HashSet, VecDeque};

use super::pipe::Pipe;

#[derive(Debug, PartialEq)]
pub struct PipeSystem {
    pub pipes: HashMap<i32, Pipe>,
}

impl PipeSystem {
    pub fn new(description: &[String]) -> PipeSystem {
        let mut temp_map = HashMap::new();

        for line in description {
            let temp_pipe = Pipe::new(&line);

            temp_map.insert(temp_pipe.source, temp_pipe);
        }

        PipeSystem { pipes: temp_map }
    }

    pub fn group_containing(&self, program_id: i32) -> HashSet<i32> {
        let mut seen_ids = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(program_id);
        seen_ids.insert(program_id);

        while let Some(current_id) = queue.pop_front() {
            for connected_id in &self.pipes.get(&current_id).unwrap().destinations {
                if !seen_ids.contains(connected_id) {
                    seen_ids.insert(*connected_id);
                    queue.push_back(*connected_id);
                }
            }
        }

        seen_ids
    }

    pub fn groups(&self) -> Vec<HashSet<i32>> {
        let mut groups = Vec::new();
        let mut seen_ids: HashSet<i32> = HashSet::new();

        for source_id in self.pipes.keys() {
            if !seen_ids.contains(source_id) {
                let group = self.group_containing(*source_id);

                seen_ids.extend(group.iter());
                groups.push(group);
            }
        }

        groups
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use file_reader::to_string_vector;

    #[test]
    fn test_group_containng() {
        let test_inputs = to_string_vector("test_inputs/day_12_part_1.txt").unwrap();
        let test_system = PipeSystem::new(&test_inputs);

        assert_eq!(test_system.group_containing(0).len(), 6);
        assert_eq!(test_system.group_containing(1).len(), 1);
    }

    #[test]
    fn test_groups() {
        let test_inputs = to_string_vector("test_inputs/day_12_part_1.txt").unwrap();
        let test_system = PipeSystem::new(&test_inputs);

        assert_eq!(test_system.groups().len(), 2);
    }
}
