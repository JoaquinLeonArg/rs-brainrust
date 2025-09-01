use std::collections::HashMap;

const DEFAULT_MAX_TAPE_LENGTH: usize = 30_000;

pub struct Program {
    code: Vec<char>,
    code_pointer: usize,
    tape: Vec<i8>,
    tape_pointer: usize,
    jumps: HashMap<usize, usize>,
}

impl Program {
    pub fn new_with_tape_length(code: Vec<char>, tape_length: usize) -> Program {
        Program {
            code,
            code_pointer: 0,
            tape: vec![0; tape_length],
            tape_pointer: tape_length / 2,
            jumps: HashMap::new(),
        }
    }
    pub fn new(code: Vec<char>) -> Program {
        Program::new_with_tape_length(code, DEFAULT_MAX_TAPE_LENGTH)
    }

    pub fn run_step(&mut self) -> Result<Option<char>, &'static str> {
        let mut result: Option<char> = None;
        match self.code[self.code_pointer] {
            '>' => self.tape_pointer += 1,
            '<' => self.tape_pointer -= 1,
            '+' => self.tape[self.tape_pointer] += 1,
            '-' => self.tape[self.tape_pointer] -= 1,
            '.' => {
                result = Some(char::from_u32(self.tape[self.tape_pointer] as u32).unwrap_or('?'))
            }
            '[' => {
                if self.tape[self.tape_pointer] == 0 {
                    let target_pos = match self.jumps.get(&self.code_pointer) {
                        None => self.build_jump_destination(self.code_pointer.clone()),
                        Some(pos) => *pos,
                    };
                    self.code_pointer = target_pos
                }
            }
            ']' => {
                if self.tape[self.tape_pointer] != 0 {
                    let target_pos = match self.jumps.get(&self.code_pointer) {
                        None => self.build_jump_destination(self.code_pointer.clone()),
                        Some(pos) => *pos,
                    };
                    self.code_pointer = target_pos
                }
            }
            _ => (),
        }
        self.code_pointer += 1;
        Ok(result)
    }

    pub fn run(mut self) -> Result<String, &'static str> {
        let mut result = String::new();
        while self.code_pointer < self.code.len() {
            if let Some(output) = self.run_step()? {
                result.push(output);
            }
        }
        Ok(result)
    }

    fn build_jump_destination(&mut self, start: usize) -> usize {
        if let Some(target) = self.jumps.get(&start) {
            return *target;
        }

        #[derive(PartialEq, Debug)]
        enum SearchDirection {
            FORWARDS,
            BACKWARDS,
        }

        let mut search_symbol: char = '[';
        let opposing_symbol = self.code[self.code_pointer];
        let mut search_direction: SearchDirection = SearchDirection::FORWARDS;
        let mut current_pointer = start.clone();

        if self.code[current_pointer] == '[' {
            search_symbol = ']';
            search_direction = SearchDirection::FORWARDS;
        } else if self.code[current_pointer] == ']' {
            search_symbol = '[';
            search_direction = SearchDirection::BACKWARDS;
        };

        while self.code[current_pointer] != search_symbol {
            if search_direction == SearchDirection::FORWARDS {
                current_pointer += 1;
            } else {
                current_pointer -= 1;
            };
            if self.code[current_pointer] == opposing_symbol {
                current_pointer = self.build_jump_destination(current_pointer.clone());
                if search_direction == SearchDirection::FORWARDS {
                    current_pointer += 1;
                } else {
                    current_pointer -= 1;
                };
            }
        }

        self.jumps.insert(start, current_pointer);
        self.jumps.insert(current_pointer, start);

        current_pointer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_program_executes() {
        let program = Program::new(include_str!("../test_programs/simple.bf").chars().collect());
        let res = program.run();
        assert_eq!(res.unwrap(), "ABCDEFGHIJK");
    }

    #[test]
    fn addition_executes() {
        let program = Program::new(
            include_str!("../test_programs/addition.bf")
                .chars()
                .collect(),
        );
        let res = program.run();
        assert_eq!(res.unwrap(), "7");
    }

    #[test]
    fn hello_world_executes() {
        let program = Program::new(
            include_str!("../test_programs/hello_world.bf")
                .chars()
                .collect(),
        );
        let res = program.run();
        assert_eq!(res.unwrap(), "Hello World!\n");
    }
}
