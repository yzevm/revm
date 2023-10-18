
use crate::handler::Handler;
use crate::interpreter::{
    analysis::to_analysed, gas, return_ok, CallContext, CallInputs, CallScheme, Contract,
    CreateInputs, Gas, Host, InstructionResult, Interpreter, SelfDestructResult, Transfer,
};
use crate::journaled_state::{is_precompile, JournalCheckpoint, JournaledState};
use crate::primitives::{
    keccak256, Address, AnalysisKind, Bytecode, Bytes, EVMError, EVMResult, Env,
    InvalidTransaction, Log, Output, Spec, SpecId::*, TransactTo, B256, U256,
};
use crate::{db::Database, precompile, Inspector};
use crate::{inspector_instruction, EVMData};
use alloc::boxed::Box;


pub fn prepare_call<DB>(data: &mut EVMData<'_,DB, gas: &Gas) -> Result<PreparedCall,  CallResult> {



}
