use chain::Chain;
use history_compact::HistoryCompact;
use point_kind::PointKind;
use output_point::OutputPoint;

#[derive(Debug)]
pub enum HistorySemantic {
  Received{
    satoshis: u64,
    transaction_hash: String,
    position: u32,
    is_spent: bool
  },
  Sent{
    transaction_hash: String,
    position: u32,
  }
}

impl HistorySemantic {
  pub fn from_compact(source: &HistoryCompact, chain: &Chain) -> HistorySemantic {
    let point = source.get_point();

    match source.get_point_kind() {
      PointKind::Input => {
        let out_point = OutputPoint::from_hash_index(point.hash(), point.index());
        HistorySemantic::Received{
          satoshis: source.get_value_or_previous_checksum(),
          transaction_hash: point.hash().to_hex(),
          position: point.index(),
          is_spent: chain.is_spent(out_point)
        }
      },
      PointKind::Output => {
        HistorySemantic::Sent{
          transaction_hash: point.hash().to_hex(),
          position: point.index(),
        }
      }
    }
  }
}
