use errors::*;
use executor::Executor;
use chain::Chain;
use payment_address::PaymentAddress;
use history_compact::HistoryCompact;
use history_compact_list::HistoryCompactList;
use point::Point;
use point_kind::PointKind;
use destructible::*;
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
                iter.map(|i| AddressHistory::from_compact(&i, &c)).collect()
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
                iter.filter(|i| i.point_kind() == PointKind::Input)
                    .map(|i| Received::new(&i, c.is_spent(i.point().to_output_point())))
                    .collect()
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
                iter.filter(|i| i.point_kind() == PointKind::Input)
                    .filter(|i| c.is_spent(i.point().to_output_point()))
                    .map(|i| Received::new(&i, false))
                    .collect()
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

#[derive(Debug, PartialEq)]
pub struct Received {
    pub satoshis: u64,
    pub transaction_hash: String,
    pub position: u32,
    pub is_spent: bool,
}

impl Received {
    pub fn new(source: &HistoryCompact, is_spent: bool) -> Received {
        Received {
            satoshis: source.get_value_or_previous_checksum(),
            transaction_hash: source.point().hash().to_hex(),
            position: source.point().index(),
            is_spent,
        }
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
    pub fn from_compact(source: &HistoryCompact, chain: &Chain) -> AddressHistory {
        let point = source.point();
        match source.point_kind() {
            PointKind::Input => AddressHistory::Received(Received::new(
                source,
                chain.is_spent(source.point().to_output_point()),
            )),
            PointKind::Output => AddressHistory::Sent(Sent::new(&point)),
        }
    }
}
