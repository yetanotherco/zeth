// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::VecDeque;

use anyhow::Context;
use log::{info, trace};
use risc0_zkvm::{Assumption, Receipt};
use zeth_guests::*;
use zeth_lib::{
    builder::{BlockBuilderStrategy, OptimismStrategy},
    consts::OP_MAINNET_CHAIN_SPEC,
    host::{rpc_db::RpcDb, ProviderFactory},
    input::BlockBuildInput,
    optimism::{
        batcher_db::BatcherDb,
        composition::{ComposeInput, ComposeInputOperation, ComposeOutputOperation},
        config::ChainConfig,
        DeriveInput, DeriveMachine,
    },
    output::BlockBuildOutput,
};
use zeth_primitives::{
    block::Header,
    mmr::{MerkleMountainRange, MerkleProof},
    transactions::optimism::OptimismTxEssence,
};

use crate::{
    cli::{Cli, Network},
    operations::{maybe_prove, verify_bonsai_receipt},
};

pub async fn derive_rollup_blocks(cli: &Cli) -> anyhow::Result<Option<(String, Receipt)>> {
    todo!("derive_rollup_blocks")
}

pub async fn compose_derived_rollup_blocks(
    cli: &Cli,
    composition_size: u32,
) -> anyhow::Result<Option<(String, Receipt)>> {
    todo!("compose_derived_rollup_blocks")
}

async fn build_op_blocks(
    cli: &Cli,
    op_block_inputs: Vec<BlockBuildInput<OptimismTxEssence>>,
) -> (Vec<Assumption>, Vec<String>, Vec<BlockBuildOutput>) {
    todo!("build_op_blocks")
}
