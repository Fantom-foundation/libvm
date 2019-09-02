# libvm

`libvm` is an abstraction of the concept of a vm.

Example of use:

```
struct ExampleCpu {
    program: Vec<ExampleInstruction>,
    pc: usize,
}

struct ExampleInstruction {}

impl Instruction for ExampleInstruction {
    fn size(&self) -> Result<usize, Error> {
        Ok(0)
    }
    fn get_cycles(&self) -> Result<usize, Error> {
        Ok(0)
    }
}

impl Cpu for ExampleCpu {
    fn execute_instruction(&mut self, instruction: I) -> Result<(), Error> {
        Ok(())
    }
    fn get_pc(&self) -> usize {
        self.pc
    }
    fn get_next_instruction(&mut self) -> Option<I> {
        self.program.pop()
    }
    fn can_run(&self) -> bool {
        true
    }
    fn is_done(&self) -> bool {
        self.pc < program.len()
    }
    fn increase_pc(&mut self, steps: usize) {
        self.pc += steps;
    }
}

struct ExampleConsensus {};

impl Consensus<u8> for ExampleConsensus {};

struct ExampleDistributedVM {
    cpu: ExampleCpu,
};

impl DistributedVM<Cpu<ExampleInstruction>, ExampleInstruction, u8, ExampleConsensus<u8>>
    for ExampleDistributedVM {
    fn serve(self) {
        while !cpu.is_done() {
            cpu.execute();
        }
    }
} 

fn run(program: Vec<ExampleInstruction>) {
    let cpu = ExampleCpu {
        program,
        pc: 0,
    };
    let dvm = ExampleDistributedVM { cpu };
    dvm.serve();
}
```