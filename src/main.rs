use crate::interpreter::Program;

mod interpreter;

fn main() -> Result<(), &'static str> {
    let program = Program::new(
        include_str!("../test_programs/hello_world.bf")
            .chars()
            .collect(),
    );
    match program.run() {
        Ok(res) => {
            println!("{}", res);
            Ok(())
        }
        Err(e) => Err(e),
    }
}
