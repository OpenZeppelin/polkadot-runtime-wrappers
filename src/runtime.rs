

// Input: tuples of form (pallet_identifier, pallet_index)
// TODO: add pallet parts arg, need to wrap with reasonable defaults basically
#[macro_export]
macro_rules! construct_openzeppelin_runtime {
    ($(($name:ident, $pallet:ident, $index:expr)),* $(,)?) => {
        ::frame_support::construct_runtime!(
            pub enum Runtime {
                $(
                    $name: $pallet = $index,
                )*
            }
        );
    };
}
