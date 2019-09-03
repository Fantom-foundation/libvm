use failure::Error;
use libcommon_rs::peer::{PeerId, PeerList};
use libconsensus::Consensus;
use libtransport::Transport;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait Instruction {
    fn size(&self) -> Result<usize, Error>;
    fn get_cycles(&self) -> Result<usize, Error>;
}

pub trait Cpu<I>
where
    I: Instruction,
{
    fn execute(&mut self) -> Result<usize, Error> {
        if !self.can_run() || self.is_done() {
            return Ok(0);
        }
        let instruction = self
            .get_next_instruction()
            .unwrap();
        self.increase_pc(instruction.size()?);
        let cycles = self.get_cycles_for_instruction(&instruction)?;
        self.execute_instruction(instruction)?;
        Ok(cycles)
    }
    fn get_cycles_for_instruction(&mut self, instruction: &I) -> Result<usize, Error> {
        instruction.get_cycles()
    }
    fn execute_instruction(&mut self, instruction: I) -> Result<(), Error>;
    fn get_pc(&self) -> usize;
    fn get_next_instruction(&mut self) -> Option<I>;
    fn can_run(&self) -> bool;
    fn is_done(&self) -> bool;
    fn increase_pc(&mut self, steps: usize);
}

pub trait DistributedVM<C, I, D, A, Id, L, T>
where
    I: Instruction,
    C: Cpu<I>,
    D: Serialize + DeserializeOwned,
    A: Consensus<D>,
    Id: PeerId,
    L: PeerList<Id, Error>,
    T: Transport<Id, D, Error, L> {
    fn serve(self);
}
