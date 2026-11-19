//! Reactive power (base unit volt ampere reactive, m² · kg · s⁻³).

quantity! {
    /// Reactive power (base unit volt ampere reactive, m² · kg · s⁻³).
    quantity: ReactivePower; "reactive power";
    /// Dimension of reactive power, L²MT⁻³ (base unit volt ampere reactive, m² · kg · s⁻³).
    dimension: ISQ<
        P2,     // length
        P1,     // mass
        N3,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn crate::si::marker::ReactivePowerKind;
    units {
        @yottavolt_ampere_reactive: prefix!(yotta); "Yvar", "yottavolt ampere reactive",
            "yottavolt amperes reactive";
        @zettavolt_ampere_reactive: prefix!(zetta); "Zvar", "zettavolt ampere reactive",
            "zettavolt amperes reactive";
        @exavolt_ampere_reactive: prefix!(exa); "Evar", "exavolt ampere reactive",
            "exavolt amperes reactive";
        @petavolt_ampere_reactive: prefix!(peta); "Pvar", "petavolt ampere reactive",
            "petavolt amperes reactive";
        @teravolt_ampere_reactive: prefix!(tera); "Tvar", "teravolt ampere reactive",
            "teravolt amperes reactive";
        @gigavolt_ampere_reactive: prefix!(giga); "Gvar", "gigavolt ampere reactive",
            "gigavolt amperes reactive";
        @megavolt_ampere_reactive: prefix!(mega); "Mvar", "megavolt ampere reactive",
            "megavolt amperes reactive";
        @kilovolt_ampere_reactive: prefix!(kilo); "kvar", "kilovolt ampere reactive",
            "kilovolt amperes reactive";
        @hectovolt_ampere_reactive: prefix!(hecto); "hvar", "hectovolt ampere reactive",
            "hectovolt amperes reactive";
        @decavolt_ampere_reactive: prefix!(deca); "davar", "decavolt ampere reactive",
            "decavolt amperes reactive";
        /// Derived unit of reactive power.
        @volt_ampere_reactive: prefix!(none); "var", "volt ampere reactive",
            "volt amperes reactive";
        @decivolt_ampere_reactive: prefix!(deci); "dvar", "decivolt ampere reactive",
            "decivolt amperes reactive";
        @centivolt_ampere_reactive: prefix!(centi); "cvar", "centivolt ampere reactive",
            "centivolt amperes reactive";
        @millivolt_ampere_reactive: prefix!(milli); "mvar", "millivolt ampere reactive",
            "millivolt amperes reactive";
        @microvolt_ampere_reactive: prefix!(micro); "µvar", "microvolt ampere reactive",
            "microvolt amperes reactive";
        @nanovolt_ampere_reactive: prefix!(nano); "nvar", "nanovolt ampere reactive",
            "nanovolt amperes reactive";
        @picovolt_ampere_reactive: prefix!(pico); "pvar", "picovolt ampere reactive",
            "picovolt amperes reactive";
        @femtovolt_ampere_reactive: prefix!(femto); "fvar", "femtovolt ampere reactive",
            "femtovolt amperes reactive";
        @attovolt_ampere_reactive: prefix!(atto); "avar", "attovolt ampere reactive",
            "attovolt amperes reactive";
        @zeptovolt_ampere_reactive: prefix!(zepto); "zvar", "zeptovolt ampere reactive",
            "zeptovolt amperes reactive";
        @yoctovolt_ampere_reactive: prefix!(yocto); "yvar", "yoctovolt ampere reactive",
            "yoctovolt amperes reactive";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::electric_current as i;
        use crate::si::electric_potential as v;
        use crate::si::power as p;
        use crate::si::quantities::*;
        use crate::si::reactive_power as rp;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: ReactivePower<V> = (ElectricPotential::new::<v::volt>(V::one())
                * ElectricCurrent::new::<i::ampere>(V::one())).into();
        }

        #[test]
        fn check_kind_conversion() {
            Test::assert_approx_eq(&Power::new::<p::watt>(V::one()),
                &ReactivePower::new::<rp::volt_ampere_reactive>(V::one()).into());
            Test::assert_approx_eq(&ReactivePower::new::<rp::kilovolt_ampere_reactive>(V::one()),
                &Power::new::<p::kilowatt>(V::one()).into());
        }

        #[test]
        fn check_units() {
            test::<v::yottavolt, i::ampere, rp::yottavolt_ampere_reactive>();
            test::<v::zettavolt, i::ampere, rp::zettavolt_ampere_reactive>();
            test::<v::exavolt, i::ampere, rp::exavolt_ampere_reactive>();
            test::<v::petavolt, i::ampere, rp::petavolt_ampere_reactive>();
            test::<v::teravolt, i::ampere, rp::teravolt_ampere_reactive>();
            test::<v::gigavolt, i::ampere, rp::gigavolt_ampere_reactive>();
            test::<v::megavolt, i::ampere, rp::megavolt_ampere_reactive>();
            test::<v::kilovolt, i::ampere, rp::kilovolt_ampere_reactive>();
            test::<v::hectovolt, i::ampere, rp::hectovolt_ampere_reactive>();
            test::<v::decavolt, i::ampere, rp::decavolt_ampere_reactive>();
            test::<v::volt, i::ampere, rp::volt_ampere_reactive>();
            test::<v::decivolt, i::ampere, rp::decivolt_ampere_reactive>();
            test::<v::centivolt, i::ampere, rp::centivolt_ampere_reactive>();
            test::<v::millivolt, i::ampere, rp::millivolt_ampere_reactive>();
            test::<v::microvolt, i::ampere, rp::microvolt_ampere_reactive>();
            test::<v::nanovolt, i::ampere, rp::nanovolt_ampere_reactive>();
            test::<v::picovolt, i::ampere, rp::picovolt_ampere_reactive>();
            test::<v::femtovolt, i::ampere, rp::femtovolt_ampere_reactive>();
            test::<v::attovolt, i::ampere, rp::attovolt_ampere_reactive>();
            test::<v::zeptovolt, i::ampere, rp::zeptovolt_ampere_reactive>();
            test::<v::yoctovolt, i::ampere, rp::yoctovolt_ampere_reactive>();

            test::<v::kilovolt, i::kiloampere, rp::megavolt_ampere_reactive>();
            test::<v::volt, i::kiloampere, rp::kilovolt_ampere_reactive>();
            test::<v::kilovolt, i::milliampere, rp::volt_ampere_reactive>();

            fn test<
                EP: v::Conversion<V>,
                EC: i::Conversion<V>,
                RP: rp::Conversion<V>>()
            {
                Test::assert_approx_eq(&ReactivePower::new::<RP>(V::one()),
                    &(ElectricPotential::new::<EP>(V::one())
                        * ElectricCurrent::new::<EC>(V::one())).into());
            }
        }
    }
}
