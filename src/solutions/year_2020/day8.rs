use std::collections::HashSet;
use std::error::Error;
use std::fs;

#[derive(Debug)]
struct Context {
    cursor: i64,
    global_value: i64,
}

impl Context {
    fn create() -> Context {
        Context { cursor: 0, global_value: 0 }
    }
}

trait Instruction {
    fn eval(&self, context: &mut Context);
    fn can_swap(&self) -> bool {
        false
    }
    fn swap(&self) -> Box<dyn Instruction>;
}

fn parse(raw_operation: &str, argument: i64) -> Box<dyn Instruction> {
    match raw_operation {
        "acc" => Box::new(Accumulator { argument }),
        "jmp" => Box::new(Jump { argument }),
        "nop" => Box::new(NoOp { _argument: argument }),
        _ => panic!("Unknown")
    }
}

struct Accumulator {
    argument: i64,
}

impl Instruction for Accumulator {
    fn eval(&self, context: &mut Context) {
        context.global_value += self.argument;
        context.cursor += 1;
    }
    fn swap(&self) -> Box<dyn Instruction> {
        panic!("Unsupported")
    }
}

struct NoOp {
    _argument: i64,
}

impl Instruction for NoOp {
    fn eval(&self, context: &mut Context) {
        context.cursor += 1;
    }
    fn can_swap(&self) -> bool {
        true
    }
    fn swap(&self) -> Box<dyn Instruction> {
        Box::new(Jump { argument: self._argument })
    }
}

struct Jump {
    argument: i64,
}

impl Instruction for Jump {
    fn eval(&self, context: &mut Context) {
        context.cursor += self.argument;
    }
    fn can_swap(&self) -> bool {
        true
    }
    fn swap(&self) -> Box<dyn Instruction> {
        Box::new(NoOp { _argument: self.argument })
    }
}

fn solve_day8_1() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2020/day8.txt")?;
    let mut instructions = vec![];
    for line in input.lines() {
        let line: Vec<&str> = line.split_whitespace().collect();
        instructions.push(parse(line[0], line[1].parse::<i64>()?));
    }
    let context = solution1(&mut instructions);
    Ok(context.global_value)
}

fn solution1(instructions: &mut Vec<Box<dyn Instruction>>) -> Context {
    let mut context = Context::create();
    let mut visited: HashSet<i64> = HashSet::new();
    while !visited.contains(&context.cursor) {
        let cursor = context.cursor;
        visited.insert(cursor);
        instructions[cursor as usize].eval(&mut context);
    }
    context
}

fn solve_day8_2() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2020/day8.txt")?;
    let mut instructions = vec![];
    for line in input.lines() {
        let line: Vec<&str> = line.split_whitespace().collect();
        instructions.push(parse(line[0], line[1].parse::<i64>()?));
    }
    let context = solution2(&mut instructions);
    Ok(context.global_value)
}

fn solution2(instructions: &mut Vec<Box<dyn Instruction>>) -> Context {
    for change_line in 0..instructions.len() {
        if !instructions[change_line].can_swap() {
            continue;
        }
        let mut context = Context::create();
        let mut visited: HashSet<i64> = HashSet::new();
        while !visited.contains(&context.cursor) {
            let cursor = context.cursor;
            visited.insert(cursor);
            if cursor == change_line as i64 {
                instructions[cursor as usize].swap().eval(&mut context);
            } else {
                instructions[cursor as usize].eval(&mut context);
            }
            if cursor == instructions.len() as i64 - 1 {
                return context;
            }
        }
    }
    panic!("no answer.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() -> Result<(), Box<dyn Error>> {
        println!("Result: {}", solve_day8_1()?);
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), Box<dyn Error>> {
        println!("Result: {}", solve_day8_2()?);
        Ok(())
    }
}
