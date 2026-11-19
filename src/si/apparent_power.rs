//! Apparent power (base unit volt ampere, m² · kg · s⁻³).

quantity! {
    /// Apparent power (base unit volt ampere, m² · kg · s⁻³).
    quantity: ApparentPower; "apparent power";
    /// Dimension of apparent power, L²MT⁻³ (base unit volt ampere, m² · kg · s⁻³).
    dimension: ISQ<
        P2,     // length
        P1,     // mass
        N3,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn crate::si::marker::ApparentPowerKind;
    units {
        @yottavolt_ampere: prefix!(yotta); "YV · A", "yottavolt ampere", "yottavolt amperes";
        @zettavolt_ampere: prefix!(zetta); "ZV · A", "zettavolt ampere", "zettavolt amperes";
        @exavolt_ampere: prefix!(exa); "EV · A", "exavolt ampere", "exavolt amperes";
        @petavolt_ampere: prefix!(peta); "PV · A", "petavolt ampere", "petavolt amperes";
        @teravolt_ampere: prefix!(tera); "TV · A", "teravolt ampere", "teravolt amperes";
        @gigavolt_ampere: prefix!(giga); "GV · A", "gigavolt ampere", "gigavolt amperes";
        @megavolt_ampere: prefix!(mega); "MV · A", "megavolt ampere", "megavolt amperes";
        @kilovolt_ampere: prefix!(kilo); "kV · A", "kilovolt ampere", "kilovolt amperes";
        @hectovolt_ampere: prefix!(hecto); "hV · A", "hectovolt ampere", "hectovolt amperes";
        @decavolt_ampere: prefix!(deca); "daV · A", "decavolt ampere", "decavolt amperes";
        /// Derived unit of apparent power.
        @volt_ampere: prefix!(none); "V · A", "volt ampere", "volt amperes";
        @decivolt_ampere: prefix!(deci); "dV · A", "decivolt ampere", "decivolt amperes";
        @centivolt_ampere: prefix!(centi); "cV · A", "centivolt ampere", "centivolt amperes";
        @millivolt_ampere: prefix!(milli); "mV · A", "millivolt ampere", "millivolt amperes";
        @microvolt_ampere: prefix!(micro); "µV · A", "microvolt ampere", "microvolt amperes";
        @nanovolt_ampere: prefix!(nano); "nV · A", "nanovolt ampere", "nanovolt amperes";
        @picovolt_ampere: prefix!(pico); "pV · A", "picovolt ampere", "picovolt amperes";
        @femtovolt_ampere: prefix!(femto); "fV · A", "femtovolt ampere", "femtovolt amperes";
        @attovolt_ampere: prefix!(atto); "aV · A", "attovolt ampere", "attovolt amperes";
        @zeptovolt_ampere: prefix!(zepto); "zV · A", "zeptovolt ampere", "zeptovolt amperes";
        @yoctovolt_ampere: prefix!(yocto); "yV · A", "yoctovolt ampere", "yoctovolt amperes";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::apparent_power as ap;
        use crate::si::electric_current as i;
        use crate::si::electric_potential as v;
        use crate::si::power as p;
        use crate::si::quantities::*;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: ApparentPower<V> = (ElectricPotential::new::<v::volt>(V::one())
                * ElectricCurrent::new::<i::ampere>(V::one())).into();
        }

        #[test]
        fn check_kind_conversion() {
            Test::assert_approx_eq(&Power::new::<p::watt>(V::one()),
                &ApparentPower::new::<ap::volt_ampere>(V::one()).into());
            Test::assert_approx_eq(&ApparentPower::new::<ap::kilovolt_ampere>(V::one()),
                &Power::new::<p::kilowatt>(V::one()).into());
        }

        #[test]
        fn check_units() {
            test::<v::yottavolt, i::ampere, ap::yottavolt_ampere>();
            test::<v::zettavolt, i::ampere, ap::zettavolt_ampere>();
            test::<v::exavolt, i::ampere, ap::exavolt_ampere>();
            test::<v::petavolt, i::ampere, ap::petavolt_ampere>();
            test::<v::teravolt, i::ampere, ap::teravolt_ampere>();
            test::<v::gigavolt, i::ampere, ap::gigavolt_ampere>();
            test::<v::megavolt, i::ampere, ap::megavolt_ampere>();
            test::<v::kilovolt, i::ampere, ap::kilovolt_ampere>();
            test::<v::hectovolt, i::ampere, ap::hectovolt_ampere>();
            test::<v::decavolt, i::ampere, ap::decavolt_ampere>();
            test::<v::volt, i::ampere, ap::volt_ampere>();
            test::<v::decivolt, i::ampere, ap::decivolt_ampere>();
            test::<v::centivolt, i::ampere, ap::centivolt_ampere>();
            test::<v::millivolt, i::ampere, ap::millivolt_ampere>();
            test::<v::microvolt, i::ampere, ap::microvolt_ampere>();
            test::<v::nanovolt, i::ampere, ap::nanovolt_ampere>();
            test::<v::picovolt, i::ampere, ap::picovolt_ampere>();
            test::<v::femtovolt, i::ampere, ap::femtovolt_ampere>();
            test::<v::attovolt, i::ampere, ap::attovolt_ampere>();
            test::<v::zeptovolt, i::ampere, ap::zeptovolt_ampere>();
            test::<v::yoctovolt, i::ampere, ap::yoctovolt_ampere>();

            test::<v::kilovolt, i::kiloampere, ap::megavolt_ampere>();
            test::<v::volt, i::kiloampere, ap::kilovolt_ampere>();
            test::<v::kilovolt, i::milliampere, ap::volt_ampere>();

            fn test<
                EP: v::Conversion<V>,
                EC: i::Conversion<V>,
                AP: ap::Conversion<V>>()
            {
                Test::assert_approx_eq(&ApparentPower::new::<AP>(V::one()),
                    &(ElectricPotential::new::<EP>(V::one())
                        * ElectricCurrent::new::<EC>(V::one())).into());
            }
        }
    }
}
