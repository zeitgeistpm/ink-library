use ink::primitives::AccountId;
use sp_runtime::{MultiAddress, Perbill};

pub type Balance = u128;
pub type Hash = [u8; 32];
pub type Timestamp = u64;
pub type BlockNumber = u32;

use crate::primitives::*;

#[derive(scale::Encode, scale::Decode)]
pub enum RuntimeCall {
    /// This index can be found by investigating runtime configuration. You can check the
    /// pallet order inside `construct_runtime!` block and read the position of your
    /// pallet (0-based).
    ///
    /// https://github.com/zeitgeistpm/zeitgeist/blob/7ea631dbff5ea519a970c5bc0f3d3d143849d3b9/runtime/common/src/lib.rs#L274-L330
    ///
    /// [See here for more.](https://substrate.stackexchange.com/questions/778/how-to-get-pallet-index-u8-of-a-pallet-in-runtime)
    #[codec(index = 40)]
    AssetManager(AssetManagerCall),
    #[codec(index = 51)]
    Authorized(AuthorizedCall),
    #[codec(index = 52)]
    Court(CourtCall),
    #[codec(index = 56)]
    Swaps(SwapsCall),
    #[codec(index = 57)]
    PredictionMarkets(PredictionMarketsCall),
    #[codec(index = 58)]
    Styx(StyxCall),
    #[codec(index = 59)]
    GlobalDisputes(GlobalDisputesCall),
    #[codec(index = 60)]
    NeoSwaps(NeoSwapsCall),
    #[codec(index = 61)]
    Orderbook(OrderbookCall),
    #[codec(index = 62)]
    Parimutuel(ParimutelCall),
}

/* ========================== Zeitgeist Pallets ========================== */

#[derive(scale::Encode, scale::Decode)]
pub enum AssetManagerCall {
    /// Transfers an asset from the caller's account to the destination account.
    /// https://github.com/open-web3-stack/open-runtime-module-library/blob/22a4f7b7d1066c1a138222f4546d527d32aa4047/currencies/src/lib.rs#L129-L131C19
    #[codec(index = 0)]
    Transfer {
        dest: MultiAddress<AccountId, ()>,
        currency_id: ZeitgeistAsset,
        #[codec(compact)]
        amount: u128,
    },
}

/// Calls for authorizing outcomes.  
/// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/authorized
#[derive(scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum AuthorizedCall {
    /// Overwrites already provided outcomes for the same market and account.
    /// https://github.com/zeitgeistpm/zeitgeist/blob/7ea631dbff5ea519a970c5bc0f3d3d143849d3b9/zrml/authorized/src/lib.rs#L88
    #[codec(index = 0)]
    AuthorizeMarketOutcome {
        market_id: MarketId,
        outcome: OutcomeReport,
    },
}

/// Calls for stake-weighted plurality decision making.  
/// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/court
#[derive(scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum CourtCall {
    /// Join to become a juror, who is able to get randomly selected
    /// for court cases according to the provided stake.  
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/court/src/lib.rs#L531
    #[codec(index = 0)]
    JoinCourt { amount: Balance },
    /// Join the court to become a delegator.  
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/court/src/lib.rs#L565
    #[codec(index = 1)]
    Delegate {
        amount: Balance,
        delegations: ink::prelude::vec::Vec<AccountId>,
    },
    /// Prepare as a court participant (juror or delegator) to exit the court.  
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/court/src/lib.rs#L618
    #[codec(index = 2)]
    PrepareExitCourt,
    /// Exit the court.
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/court/src/lib.rs#L660C16-L660C26
    #[codec(index = 3)]
    ExitCourt {
        court_participant: AccountId, // TODO: replace with account ID lookup
    },
    /// Vote as a randomly selected juror for a specific court case.
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/court/src/lib.rs#L717
    #[codec(index = 4)]
    Vote {
        #[codec(compact)]
        court_id: CourtId,
        commitment_vote: Hash,
    },
    /// Denounce a juror during the voting period for which the commitment vote is known.
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/court/src/lib.rs#L784
    #[codec(index = 5)]
    DenounceVote {
        #[codec(compact)]
        court_id: CourtId,
        juror: AccountId, // AccountIdLookupOf<T>
        vote_item: VoteItem,
        salt: Hash,
    },
    /// Reveal the commitment vote of the caller, who is a selected juror.
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/court/src/lib.rs#L868
    #[codec(index = 6)]
    RevealVote {
        #[codec(compact)]
        court_id: CourtId,
        vote_item: VoteItem,
        salt: Hash,
    },
    /// Initiate an appeal for a court
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/court/src/lib.rs#L957
    #[codec(index = 7)]
    Appeal {
        #[codec(compact)]
        court_id: CourtId,
    },
    /// Reassign the stakes of the jurors and delegators
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/court/src/lib.rs#L1046
    #[codec(index = 8)]
    ReassignCourtStakes {
        #[codec(compact)]
        court_id: CourtId,
    },
    /// Set the yearly inflation rate of the court system.
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/court/src/lib.rs#L1167
    #[codec(index = 9)]
    SetInflation { inflation: Perbill },
}

/// Calls for swapping shares out for different ones.  
/// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/swaps
#[derive(scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum SwapsCall {
    /// Exits a pool.
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/swaps/src/lib.rs#L125
    #[codec(index = 1)]
    PoolExit {
        #[codec(compact)]
        pool_id: PoolId,
        #[codec(compact)]
        pool_amount: Balance,
        min_assets_out: ink::prelude::vec::Vec<Balance>,
    },
    /// Exits a pool with an exact asset amount.
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/swaps/src/lib.rs#L155
    #[codec(index = 3)]
    PoolExitWithExactAssetAmount {
        #[codec(compact)]
        pool_id: PoolId,
        asset: ZeitgeistAsset, // TODO: figure out of AssetOf<T> is this value
        #[codec(compact)]
        asset_amount: Balance,
        #[codec(compact)]
        max_pool_amount: Balance,
    },
    /// Exits a pool with an exact pool amount.
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/swaps/src/lib.rs#L193
    #[codec(index = 4)]
    PoolExitWithExactPoolAmount {
        #[codec(compact)]
        pool_id: PoolId,
        asset: ZeitgeistAsset,
        #[codec(compact)]
        pool_amount: Balance,
        #[codec(compact)]
        min_asset_amount: Balance,
    },
    /// Joins a pool.
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/swaps/src/lib.rs#L266
    #[codec(index = 5)]
    PoolJoin {
        #[codec(compact)]
        pool_id: PoolId,
        #[codec(compact)]
        pool_amount: Balance,
        max_assets_in: ink::prelude::vec::Vec<Balance>,
    },
    /// Joins a pool with an exact asset amount.
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/swaps/src/lib.rs#L318
    #[codec(index = 7)]
    PoolJoinWithExactAssetAmount {
        #[codec(compact)]
        pool_id: PoolId,
        asset_in: ZeitgeistAsset,
        #[codec(compact)]
        asset_amount: Balance,
        #[codec(compact)]
        min_pool_amount: Balance,
    },
    /// Joins a pool with an exact pool amount.
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/swaps/src/lib.rs#L357
    #[codec(index = 8)]
    PoolJoinWithExactPoolAmount {
        #[codec(compact)]
        pool_id: PoolId,
        asset: ZeitgeistAsset,
        #[codec(compact)]
        pool_amount: Balance,
        #[codec(compact)]
        max_asset_amount: Balance,
    },
    // https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fbsr.zeitgeist.pm#/extrinsics/decode/0x380981040402286bee00b102000000000000000000000000000001000100cdbe7b00000000000000000000000000
    /// Swaps an exact amount in.
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/swaps/src/lib.rs#L422
    #[codec(index = 9)]
    SwapExactAmountIn {
        #[codec(compact)]
        pool_id: PoolId,
        asset_in: ZeitgeistAsset,
        #[codec(compact)]
        asset_amount_in: Balance,
        asset_out: ZeitgeistAsset,
        min_asset_amount_out: Option<Balance>,
        max_price: Option<Balance>,
    },
    /// Swaps an exact amount out.
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/swaps/src/lib.rs#L465
    #[codec(index = 10)]
    SwapExactAmountOut {
        #[codec(compact)]
        pool_id: PoolId,
        asset_in: ZeitgeistAsset,
        max_asset_amount_in: Option<u128>,
        asset_out: ZeitgeistAsset,
        #[codec(compact)]
        asset_amount_out: u128,
        max_price: Option<u128>,
    },
    /// Forces a pool exit.
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/swaps/src/lib.rs#L492
    #[codec(index = 11)]
    ForcePoolExit {
        who: AccountId,
        #[codec(compact)]
        pool_id: PoolId,
        #[codec(compact)]
        pool_amount: Balance,
        min_assets_out: ink::prelude::vec::Vec<Balance>,
    },
}

/// Calls for creating, reporting, and disputing prediction markets.  
/// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/prediction-markets
#[derive(scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PredictionMarketsCall {
    /// Allows the `CloseOrigin` to immediately move an open market to closed.  
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/prediction-markets/src/lib.rs#L345
    #[codec(index = 1)]
    AdminMoveMarketToClosed {
        #[codec(compact)]
        market_id: MarketId,
    },
    /// Allows the `CloseOrigin` to immediately move an open market to closed.  
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/prediction-markets/src/lib.rs#L378
    #[codec(index = 2)]
    AdminMoveMarketToResolved {
        #[codec(compact)]
        market_id: MarketId,
    },
    /// Approves a market that is waiting for approval from the advisory committee.  
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/prediction-markets/src/lib.rs#L429
    #[codec(index = 3)]
    ApproveMarket {
        #[codec(compact)]
        market_id: MarketId,
    },
    /// Request an edit to a proposed market.  
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/prediction-markets/src/lib.rs#L471
    RequestEdit {
        #[codec(compact)]
        market_id: MarketId,
        edit_reason: ink::prelude::vec::Vec<u8>,
    },
    /// Buy a complete set of outcome shares of a market.  
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/prediction-markets/src/lib.rs#L515
    #[codec(index = 5)]
    BuyCompleteSet {
        #[codec(compact)]
        market_id: MarketId,
        #[codec(compact)]
        amount: Balance,
    },
    /// Dispute on a market that has been reported or already disputed.  
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/prediction-markets/src/lib.rs#L542
    #[codec(index = 6)]
    Dispute {
        #[codec(compact)]
        market_id: MarketId,
    },
    /// Creates a market.  
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/prediction-markets/src/lib.rs#L594
    #[codec(index = 8)]
    CreateMarket {
        base_asset: ZeitgeistAsset, // Asset<u128>,
        creator_fee: Perbill,
        oracle: AccountId,
        period: MarketPeriod,
        deadlines: Deadlines,
        metadata: MultiHash,
        creation: MarketCreation,
        market_type: MarketType,
        dispute_mechanism: Option<MarketDisputeMechanism>,
        scoring_rule: ScoringRule,
    },
    /// Edit a proposed market for which request is made.  
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/prediction-markets/src/lib.rs#L646
    #[codec(index = 9)]
    EditMarket {
        base_asset: ZeitgeistAsset,
        market_id: MarketId,
        oracle: AccountId,
        period: MarketPeriod,
        deadlines: Deadlines,
        metadata: MultiHash,
        market_type: MarketType,
        dispute_mechanism: Option<MarketDisputeMechanism>,
        scoring_rule: ScoringRule,
    },
    /// Redeems the winning shares of a prediction market.  
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/prediction-markets/src/lib.rs#L707
    #[codec(index = 12)]
    RedeemShares {
        #[codec(compact)]
        market_id: MarketId,
    },
    /// Rejects a market that is waiting for approval from the advisory committee.  
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/prediction-markets/src/lib.rs#L856
    #[codec(index = 13)]
    RejectMarket {
        #[codec(compact)]
        market_id: MarketId,
        reject_reason: ink::prelude::vec::Vec<u8>,
    },
    /// Reports the outcome of a market.  
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/prediction-markets/src/lib.rs#L886
    #[codec(index = 14)]
    Report {
        #[codec(compact)]
        market_id: MarketId,
        outcome: OutcomeReport,
    },
    /// Sells a complete set of outcomes shares for a market.  
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/prediction-markets/src/lib.rs#L928
    #[codec(index = 15)]
    SellCompleteSet {
        #[codec(compact)]
        market_id: MarketId,
        #[codec(compact)]
        amount: Balance,
    },
    /// Start a global dispute, if the market dispute mechanism fails.  
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/prediction-markets/src/lib.rs#L928
    #[codec(index = 16)]
    StartGlobalDispute {
        #[codec(compact)]
        market_id: MarketId,
    },
    /// Create a market, deploy a LMSR pool, and buy outcome tokens and provide liquidity to the
    /// market.  
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/prediction-markets/src/lib.rs#L1059
    #[codec(index = 17)]
    CreateMarketAndDeployPool {
        base_asset: ZeitgeistAsset,
        creator_fee: Perbill,
        oracle: AccountId,
        period: MarketPeriod,
        deadlines: Deadlines,
        metadata: MultiHash,
        market_type: MarketType,
        dispute_mechanism: Option<MarketDisputeMechanism>,
        #[codec(compact)]
        amount: Balance,
        spot_prices: ink::prelude::vec::Vec<Balance>,
        #[codec(compact)]
        swap_fee: Balance,
    },
    /// Allows the `CloseMarketsEarlyOrigin` or the market creator to schedule an early close.  
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/prediction-markets/src/lib.rs#L1122
    #[codec(index = 18)]
    ScheduleEarlyClose {
        #[codec(compact)]
        market_id: MarketId,
    },
    /// Allows anyone to dispute a scheduled early close.  
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/prediction-markets/src/lib.rs#L1270
    #[codec(index = 19)]
    DisputeEarlyClose {
        #[codec(compact)]
        market_id: MarketId,
    },
    /// Allows the `CloseMarketsEarlyOrigin` to reject a scheduled early close.  
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/prediction-markets/src/lib.rs#L1356
    #[codec(index = 20)]
    RejectEarlyClose {
        #[codec(compact)]
        market_id: MarketId,
    },
    /// Allows the market creator of a trusted market to immediately move an open market to closed.  
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/prediction-markets/src/lib.rs#L1437
    #[codec(index = 21)]
    CloseTrustedMarket {
        #[codec(compact)]
        market_id: MarketId,
    },
    /// Allows the manual closing for "broken" markets.  
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/prediction-markets/src/lib.rs#L1464
    #[codec(index = 22)]
    ManuallyCloseMarket {
        #[codec(compact)]
        market_id: MarketId,
    },
}

/// Calls for burning native chain tokens in order to gain entry into a registry
/// for off-chain use.  
/// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/styx
#[derive(scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum StyxCall {
    /// Burns ZTG to cross, granting the ability to claim your zeitgeist avatar. The signer can only cross once.  
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/styx/src/lib.rs#L90
    #[codec(index = 0)]
    Cross,
    /// Sets the burn amount to cross.  
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/styx/src/lib.rs#L126
    #[codec(index = 1)]
    SetBurnAmount {
        #[codec(compact)]
        market_id: Balance,
    }
}

#[derive(scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum GlobalDisputesCall {
    /// Add voting outcome to a global dispute in exchange for a constant fee.  
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/global-disputes/src/lib.rs#L276
    #[codec(index = 0)]
    AddVoteOutcome {
        #[codec(compact)]
        market_id: MarketId,
        outcome: OutcomeReport,
    },
    /// Return the voting outcome fees in case the global dispute was destroyed.  
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/global-disputes/src/lib.rs#L339
    #[codec(index = 5)]
    RefundVoteFees {
        #[codec(compact)]
        market_id: MarketId,
    },
    /// Purge all outcomes to allow the winning outcome owner(s) to get their reward.  
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/global-disputes/src/lib.rs#L398
    #[codec(index = 1)]
    PurgeOutcomes {
        #[codec(compact)]
        market_id: MarketId,
    },
    /// Reward the collected fees to the owner(s) of a voting outcome.  
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/global-disputes/src/lib.rs#L456
    #[codec(index = 2)]
    RewardOutcomeOwner {
        #[codec(compact)]
        market_id: MarketId,
    },
    /// Vote on existing voting outcomes by locking native tokens.  
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/global-disputes/src/lib.rs#L510
    #[codec(index = 3)]
    VoteOnOutcome {
        #[codec(compact)]
        market_id: MarketId,
        outcome: OutcomeReport,
        #[codec(compact)]
        amount: Balance
    },
    /// Return all locked native tokens from a finished or destroyed global dispute.  
    /// https://github.com/zeitgeistpm/zeitgeist/tree/release-v0.5.0/zrml/global-disputes/src/lib.rs#L611
    #[codec(index = 4)]
    UnlockVoteBalance {
        voter: AccountId // TODO: see if its the same as AccountIdLookupOf<T>,
    }
}

#[derive(scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum NeoSwapsCall {}

#[derive(scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum OrderbookCall {}

#[derive(scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ParimutelCall {}
