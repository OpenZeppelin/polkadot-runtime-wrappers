mod impl_system;

pub trait RuntimeConstructs {
    type Runtime;
    type RuntimeCall;
    type RuntimeEvent;
    type PalletInfo;
}

pub trait SystemConstructs {
    type RuntimeBlockLength;
    type RuntimeBlockWeights;
    type SS58Prefix;
    type Version;
    type AccountId;
}

// pub struct SystemConfig<Runtime>(core::marker::PhantomData<Runtime>);

// impl<Runtime> SystemConfig<Runtime> {
//     pub fn new() -> SystemConfig<Runtime> {
//         SystemConfig(core::marker::PhantomData::<Runtime>)
//     }
// }

// impl<Runtime: RuntimeConstructs> RuntimeConstructs for SystemConfig<Runtime> {
//     type Runtime = <Runtime as RuntimeConstructs>::Runtime;
//     type RuntimeCall = <Runtime as RuntimeConstructs>::RuntimeCall;
//     type RuntimeEvent = <Runtime as RuntimeConstructs>::RuntimeEvent;
//     type PalletInfo = <Runtime as RuntimeConstructs>::PalletInfo;
// }

// impl<Runtime: SystemConstructs> SystemConstructs for SystemConfig<Runtime> {
//     type RuntimeBlockLength = <Runtime as SystemConstructs>::RuntimeBlockLength;
//     type RuntimeBlockWeights = <Runtime as SystemConstructs>::RuntimeBlockWeights;
//     type SS58Prefix = <Runtime as SystemConstructs>::SS58Prefix;
//     type Version = <Runtime as SystemConstructs>::Version;
//     type AccountId = <Runtime as SystemConstructs>::AccountId;
// }
