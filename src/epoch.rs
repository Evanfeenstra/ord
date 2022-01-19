use super::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) struct Epoch(pub(crate) u64);

impl Epoch {
  pub(crate) const STARTING_ORDINALS: &'static [Ordinal] = &[
    Ordinal::new(0),
    Ordinal::new(1050000000000000),
    Ordinal::new(1575000000000000),
    Ordinal::new(1837500000000000),
    Ordinal::new(1968750000000000),
    Ordinal::new(2034375000000000),
    Ordinal::new(2067187500000000),
    Ordinal::new(2083593750000000),
    Ordinal::new(2091796875000000),
    Ordinal::new(2095898437500000),
    Ordinal::new(2097949218750000),
    Ordinal::new(2098974609270000),
    Ordinal::new(2099487304530000),
    Ordinal::new(2099743652160000),
    Ordinal::new(2099871825870000),
    Ordinal::new(2099935912620000),
    Ordinal::new(2099967955890000),
    Ordinal::new(2099983977420000),
    Ordinal::new(2099991988080000),
    Ordinal::new(2099995993410000),
    Ordinal::new(2099997995970000),
    Ordinal::new(2099998997250000),
    Ordinal::new(2099999497890000),
    Ordinal::new(2099999748210000),
    Ordinal::new(2099999873370000),
    Ordinal::new(2099999935950000),
    Ordinal::new(2099999967240000),
    Ordinal::new(2099999982780000),
    Ordinal::new(2099999990550000),
    Ordinal::new(2099999994330000),
    Ordinal::new(2099999996220000),
    Ordinal::new(2099999997060000),
    Ordinal::new(2099999997480000),
  ];

  pub(crate) const BLOCKS: u64 = 210000;

  pub(crate) fn subsidy(self) -> u64 {
    if self.0 < 64 {
      (50 * COIN_VALUE) >> self.0
    } else {
      0
    }
  }

  pub(crate) fn starting_ordinal(self) -> Option<Ordinal> {
    Self::STARTING_ORDINALS.get(self.0 as usize).cloned()
  }

  pub(crate) fn starting_height(self) -> Height {
    Height(self.0 * Self::BLOCKS)
  }
}

impl PartialEq<u64> for Epoch {
  fn eq(&self, other: &u64) -> bool {
    self.0 == *other
  }
}

impl From<Ordinal> for Epoch {
  fn from(ordinal: Ordinal) -> Self {
    match Self::STARTING_ORDINALS.binary_search(&ordinal) {
      Ok(i) => Epoch(i as u64),
      Err(i) => Epoch(i as u64 - 1),
    }
  }
}

impl From<Height> for Epoch {
  fn from(height: Height) -> Self {
    Self(height.0 / Self::BLOCKS)
  }
}

#[cfg(test)]
mod tests {
  use super::super::*;

  #[test]
  fn starting_ordinal() {
    assert_eq!(Epoch(0).starting_ordinal().unwrap(), 0);
    assert_eq!(
      Epoch(1).starting_ordinal().unwrap(),
      Epoch(0).subsidy() * Epoch::BLOCKS
    );
    assert_eq!(
      Epoch(2).starting_ordinal().unwrap(),
      (Epoch(0).subsidy() + Epoch(1).subsidy()) * Epoch::BLOCKS
    );
    assert_eq!(Epoch(33).starting_ordinal(), None);
  }

  #[test]
  fn starting_ordinals() {
    let mut ordinal = 0;

    let mut epoch_ordinals = Vec::new();

    for epoch in 0..33 {
      epoch_ordinals.push(ordinal);
      ordinal += Epoch::BLOCKS * Epoch(epoch).subsidy();
    }

    assert_eq!(Epoch::STARTING_ORDINALS, epoch_ordinals);
    assert_eq!(Epoch::STARTING_ORDINALS.len(), 33);
  }

  #[test]
  fn subsidy() {
    assert_eq!(Epoch(0).subsidy(), 5000000000);
    assert_eq!(Epoch(1).subsidy(), 2500000000);
    assert_eq!(Epoch(32).subsidy(), 1);
    assert_eq!(Epoch(33).subsidy(), 0);
  }

  #[test]
  fn blocks() {
    // c.f. https://github.com/bitcoin/bitcoin/blob/master/src/chainparams.cpp
    assert_eq!(Epoch::BLOCKS, 210000);
  }

  #[test]
  fn starting_height() {
    assert_eq!(Epoch(0).starting_height(), 0);
    assert_eq!(Epoch(1).starting_height(), Epoch::BLOCKS);
    assert_eq!(Epoch(2).starting_height(), Epoch::BLOCKS * 2);
  }

  #[test]
  fn from_height() {
    assert_eq!(Epoch::from(Height(0)), 0);
    assert_eq!(Epoch::from(Height(Epoch::BLOCKS)), 1);
    assert_eq!(Epoch::from(Height(Epoch::BLOCKS) + 1), 1);
  }

  #[test]
  fn from_ordinal() {
    assert_eq!(Epoch::from(Ordinal::new(0)), 0);
    assert_eq!(Epoch::from(Ordinal::new(1)), 0);
    assert_eq!(Epoch::from(Epoch(1).starting_ordinal().unwrap()), 1);
    assert_eq!(Epoch::from(Epoch(1).starting_ordinal().unwrap() + 1), 1);
  }

  #[test]
  fn eq() {
    assert_eq!(Epoch(0), 0);
    assert_eq!(Epoch(100), 100);
  }
}