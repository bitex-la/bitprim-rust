use std::cmp::Ordering;

use errors::*;
use executor::Executor;
use chain::Chain;
use payment_address::PaymentAddress;
use history_compact::HistoryCompact;
use history_compact_list::HistoryCompactList;
use point::Point;
use hash::Hash;
use point_kind::PointKind;
use destructible::DestructibleBox;
pub use opaque_collection::*;

pub struct Explorer {
    pub executor: Executor,
}

impl Explorer {
    pub fn address_history(
        &self,
        address: PaymentAddress,
        limit: u64,
        since: u64,
    ) -> Result<Vec<AddressHistory>> {
        self.chain_and_history(address, limit, since)
            .map(|(c, history)| {
                let iter = OpaqueCollectionIterator {
                    collection: history.contents.as_ref(),
                    iter: 0,
                };
                iter.map(|i| AddressHistory::from_compact(&i, &self.executor, &c)).collect()
            })
    }

    pub fn address_incoming(
        &self,
        address: PaymentAddress,
        limit: u64,
        since: u64,
    ) -> Result<Vec<Received>> {
        self.chain_and_history(address, limit, since)
            .map(|(c, history)| {
                let iter = OpaqueCollectionIterator {
                    collection: history.contents.as_ref(),
                    iter: 0,
                };
                let mut vec = 
                    iter.filter(|i| i.point_kind() == PointKind::Input)
                        .map(|i| Received::new(&i, &self.executor, c.is_spent(i.point().to_output_point())) )
                        .collect::<Vec<Received>>();
                vec.sort_unstable();
                vec
            })
    }

    pub fn address_unspents(
        &self,
        address: PaymentAddress,
        limit: u64,
        since: u64,
    ) -> Result<Vec<Received>> {
        self.chain_and_history(address, limit, since)
            .map(|(c, history)| {
                let iter = OpaqueCollectionIterator {
                    collection: history.contents.as_ref(),
                    iter: 0,
                };
                let mut vec = 
                    iter.filter(|i| i.point_kind() == PointKind::Input && !c.is_spent(i.point().to_output_point()) )
                    .map(|i| Received::new(&i, &self.executor, false) )
                    .collect::<Vec<Received>>();
                vec.sort_unstable();
                vec
            })
    }

    fn chain_and_history(
        &self,
        address: PaymentAddress,
        limit: u64,
        since: u64,
    ) -> Result<(Chain, DestructibleBox<HistoryCompactList>)> {
        let chain = self.executor.get_chain();
        let history = chain.get_history(address, limit, since)?;
        Ok((chain, history))
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Received {
    pub satoshis: u64,
    pub transaction_hash: Hash,
    pub position: u32,
    pub is_spent: bool,
    pub block_height: u32,
    pub version: u32,
    pub locktime: u32,
    pub input_details: Vec<InputDetail>,
    pub output_details: Vec<OutputDetail>
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InputDetail {
    pub prev_hash: String,
    pub prev_index: u32,
    pub sequence: u32,
    pub script_sig: String
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutputDetail {
    pub amount: u64,
    pub script_pubkey: String
}

impl Received {
    pub fn new(source: &HistoryCompact, exec: &Executor, is_spent: bool) -> Received {
        let hash = source.point().hash();
        let transaction = exec.get_chain().get_transaction(hash.clone(), 1).expect("Error getting transaction from Node").0;
        Received {
            satoshis: source.get_value_or_previous_checksum(),
            transaction_hash: hash,
            position: source.point().index(),
            block_height: source.height(),
            is_spent,
            version: transaction.version(),
            locktime: transaction.locktime(),
            input_details: transaction.inputs().into_iter().map(|input| {
                let output_point = input.previous_output();
                InputDetail {
                    prev_hash: output_point.hash().to_hex(),
                    prev_index: output_point.index(),
                    sequence: input.sequence(),
                    script_sig: input.script().to_hex(0)
                }
            }).collect(),
            output_details: transaction.outputs().into_iter().map(|output| {
                OutputDetail {
                    amount: output.value(),
                    script_pubkey: output.script().to_hex(0)
                }
            }).collect()
        }
    }
}

impl Ord for Received {
    fn cmp(&self, other: &Received) -> Ordering {
        self.transaction_hash.cmp(&other.transaction_hash)
    }
}

impl PartialOrd for Received {
    fn partial_cmp(&self, other: &Received) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq)]
pub struct Sent {
    pub transaction_hash: String,
    pub position: u32,
}

impl Sent {
    pub fn new(point: &Point) -> Self {
        Self {
            transaction_hash: point.hash().to_hex(),
            position: point.index(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum AddressHistory {
    Received(Received),
    Sent(Sent),
}

impl AddressHistory {
    pub fn from_compact(source: &HistoryCompact, exec: &Executor, chain: &Chain) -> AddressHistory {
        let point = source.point();
        match source.point_kind() {
            PointKind::Input => AddressHistory::Received(Received::new(
                source,
                exec,
                chain.is_spent(source.point().to_output_point()) 
            )),
            PointKind::Output => AddressHistory::Sent(Sent::new(&point)),
        }
    }
}
