//! Weights per pallet grouping

pub trait SystemWeight {
    type Timestamp = ();
    type Scheduler = ();
    type Preimage = ();
    type Proxy = ();
    type Multisig = ();
    type ParachainSystem = ();
    type DbWeight;
}

pub trait ConsensusWeight {
    type CollatorSelection = ();
    type Session = ();
}

pub trait AssetsWeight {
    type Assets = ();
    type AssetManager = ();
}

pub trait GovernanceWeight {
    type Sudo = ();
    type Treasury = ();
    type ConvictionVoting = ();
    type Whitelist = ();
    type Referenda = ();
}

pub trait XcmWeight {
    type MessageQueue = ();
    type XcmpQueue = ();
    type Xcm = ();
    type XcmWeightTrader = ();
    type XcmTransactor = ();
}

pub trait EvmWeight {
    type Evm = ();
}

pub trait TanssiWeight {
    type AuthorInherent = ();
    type AuthoritiesNoting = ();
}
