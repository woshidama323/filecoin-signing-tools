use fil_actor_miner::{
    MinerConstructorParams, ChangeWorkerAddressParams, ChangePeerIDParams,
    ChangeMultiaddrsParams, ConfirmSectorProofsParams, DeferredCronEventParams, PoStPartition,
    SubmitWindowedPoStParams, ProveCommitSectorParams, CheckSectorProvenParams, ExtendSectorExpirationParams,
    ExpirationExtension, TerminateSectorsParams, TerminationDeclaration, DeclareFaultsParams, FaultDeclaration,
    DeclareFaultsRecoveredParams, RecoveryDeclaration, CompactPartitionsParams, CompactSectorNumbersParams, 
    ReportConsensusFaultParams, WithdrawBalanceParams, WorkerKeyChange, PreCommitSectorBatchParams, 
    SectorPreCommitInfo, ApplyRewardParams, DisputeWindowedPoStParams, ProveCommitAggregateParams, ReplicaUpdate, ProveReplicaUpdatesParams,
};
use fvm_ipld_encoding::{serde_bytes, RawBytes};
use fvm_shared::address::Address;
use fvm_shared::clock::ChainEpoch;
use fvm_shared::econ::TokenAmount;

/// Storage miner actor constructor params are defined here so the power actor can send them to the init actor
/// to instantiate miners.
#[derive(Serialize, Deserialize)]
#[serde(remote = "MinerConstructorParams")]
pub struct MinerConstructorParamsAPI {
    pub owner: Address,
    pub worker: Address,
    pub control_addresses: Vec<Address>,
    pub window_post_proof_type: RegisteredPoStProof,
    #[serde(with = "serde_bytes")]
    pub peer_id: Vec<u8>,
    pub multi_addresses: Vec<BytesDe>,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "ChangeWorkerAddressParams")]
pub struct ChangeWorkerAddressParamsAPI {
    pub new_worker: Address,
    pub new_control_addresses: Vec<Address>,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "ChangePeerIDParams")]
pub struct ChangePeerIDParamsAPI {
    #[serde(with = "serde_bytes")]
    pub new_id: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "ChangeMultiaddrsParams")]
pub struct ChangeMultiaddrsParamsAPI {
    pub new_multi_addrs: Vec<BytesDe>,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "ConfirmSectorProofsParams")]
pub struct ConfirmSectorProofsParamsAPI {
    pub sectors: Vec<SectorNumber>,
    pub reward_smoothed: FilterEstimate,
    #[serde(with = "bigint_ser")]
    pub reward_baseline_power: StoragePower,
    pub quality_adj_power_smoothed: FilterEstimate,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "DeferredCronEventParams")]
pub struct DeferredCronEventParamsAPI {
    #[serde(with = "serde_bytes")]
    pub event_payload: Vec<u8>,
    pub reward_smoothed: FilterEstimate,
    pub quality_adj_power_smoothed: FilterEstimate,
}

#[derive(Serialize, Deserialize)]
pub struct PoStPartition {
    /// Partitions are numbered per-deadline, from zero.
    pub index: u64,
    /// Sectors skipped while proving that weren't already declared faulty.
    pub skipped: UnvalidatedBitField,
}

/// Information submitted by a miner to provide a Window PoSt.
#[derive(Serialize, Deserialize)]
#[serde(remote = "SubmitWindowedPoStParams")]
pub struct SubmitWindowedPoStParamsAPI {
    /// The deadline index which the submission targets.
    pub deadline: u64,
    /// The partitions being proven.
    pub partitions: Vec<PoStPartition>,
    /// Array of proofs, one per distinct registered proof type present in the sectors being proven.
    /// In the usual case of a single proof type, this array will always have a single element (independent of number of partitions).
    pub proofs: Vec<PoStProof>,
    /// The epoch at which these proofs is being committed to a particular chain.
    pub chain_commit_epoch: ChainEpoch,
    /// The ticket randomness on the chain at the `chain_commit_epoch` on the chain this post is committed to.
    pub chain_commit_rand: Randomness,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "ProveCommitSectorParams")]
pub struct ProveCommitSectorParamsAPI {
    pub sector_number: SectorNumber,
    #[serde(with = "serde_bytes")]
    pub proof: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "CheckSectorProvenParams")]
pub struct CheckSectorProvenParamsAPI {
    pub sector_number: SectorNumber,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "ExtendSectorExpirationParams")]
pub struct ExtendSectorExpirationParamsAPI {
    pub extensions: Vec<ExpirationExtension>,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "ExpirationExtension")]
pub struct ExpirationExtensionAPI {
    pub deadline: u64,
    pub partition: u64,
    pub sectors: UnvalidatedBitField,
    pub new_expiration: ChainEpoch,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "TerminateSectorsParams")]
pub struct TerminateSectorsParamsAPI {
    pub terminations: Vec<TerminationDeclaration>,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "TerminationDeclaration")]
pub struct TerminationDeclarationAPI {
    pub deadline: u64,
    pub partition: u64,
    pub sectors: UnvalidatedBitField,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "DeclareFaultsParams")]
pub struct DeclareFaultsParamsAPI {
    pub faults: Vec<FaultDeclaration>,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "FaultDeclaration")]
pub struct FaultDeclarationAPI {
    /// The deadline to which the faulty sectors are assigned, in range [0..WPoStPeriodDeadlines)
    pub deadline: u64,
    /// Partition index within the deadline containing the faulty sectors.
    pub partition: u64,
    /// Sectors in the partition being declared faulty.
    pub sectors: UnvalidatedBitField,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "DeclareFaultsRecoveredParams")]
pub struct DeclareFaultsRecoveredParamsAPI {
    pub recoveries: Vec<RecoveryDeclaration>,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "RecoveryDeclaration")]
pub struct RecoveryDeclarationAPI {
    /// The deadline to which the recovered sectors are assigned, in range [0..WPoStPeriodDeadlines)
    pub deadline: u64,
    /// Partition index within the deadline containing the recovered sectors.
    pub partition: u64,
    /// Sectors in the partition being declared recovered.
    pub sectors: UnvalidatedBitField,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "CompactPartitionsParams")]
pub struct CompactPartitionsParamsAPI {
    pub deadline: u64,
    pub partitions: UnvalidatedBitField,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "CompactSectorNumbersParams")]
pub struct CompactSectorNumbersParamsAPI {
    pub mask_sector_numbers: UnvalidatedBitField,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "ReportConsensusFaultParams")]
pub struct ReportConsensusFaultParamsAPI {
    #[serde(with = "serde_bytes")]
    pub header1: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub header2: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub header_extra: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "WithdrawBalanceParams")]
pub struct WithdrawBalanceParamsAPI {
    #[serde(with = "bigint_ser")]
    pub amount_requested: TokenAmount,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(remote = "PreCommitSectorBatchParams")]
pub struct PreCommitSectorBatchParamsAPI {
    pub sectors: Vec<SectorPreCommitInfo>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(remote = "SectorPreCommitInfo")]
pub struct SectorPreCommitInfoAPI {
    pub seal_proof: RegisteredSealProof,
    pub sector_number: SectorNumber,
    /// CommR
    pub sealed_cid: Cid,
    pub seal_rand_epoch: ChainEpoch,
    pub deal_ids: Vec<DealID>,
    pub expiration: ChainEpoch,
    /// Whether to replace a "committed capacity" no-deal sector (requires non-empty DealIDs)
    pub replace_capacity: bool,
    /// The committed capacity sector to replace, and its deadline/partition location
    pub replace_sector_deadline: u64,
    pub replace_sector_partition: u64,
    pub replace_sector_number: SectorNumber,
}

// * Added in v2 -- param was previously a big int.
#[derive(Debug, Serialize, Deserialize)]
#[serde(remote = "ApplyRewardParams")]
pub struct ApplyRewardParamsAPI {
    #[serde(with = "bigint_ser")]
    pub reward: TokenAmount,
    #[serde(with = "bigint_ser")]
    pub penalty: TokenAmount,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[serde(remote = "DisputeWindowedPoStParams")]
pub struct DisputeWindowedPoStParamsAPI {
    pub deadline: u64,
    pub post_index: u64, // only one is allowed at a time to avoid loading too many sector infos.
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(remote = "ProveCommitAggregateParams")]
pub struct ProveCommitAggregateParamsAPI {
    pub sector_numbers: UnvalidatedBitField,
    #[serde(with = "serde_bytes")]
    pub aggregate_proof: Vec<u8>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ReplicaUpdate")]
pub struct ReplicaUpdateAPI {
    pub sector_number: SectorNumber,
    pub deadline: u64,
    pub partition: u64,
    pub new_sealed_cid: Cid,
    pub deals: Vec<DealID>,
    pub update_proof_type: RegisteredUpdateProof,
    #[serde(with = "serde_bytes")]
    pub replica_proof: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(remote = "ProveReplicaUpdatesParams")]
pub struct ProveReplicaUpdatesParams {
    pub updates: Vec<ReplicaUpdate>,
}