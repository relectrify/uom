//! Reactive energy (base unit volt ampere reactive second, m² · kg · s⁻²).

quantity! {
    /// Reactive energy (base unit volt ampere reactive second, m² · kg · s⁻²).
    quantity: ReactiveEnergy; "reactive energy";
    /// Dimension of reactive energy, L²MT⁻² (base unit volt ampere reactive second,
    /// m² · kg · s⁻²).
    dimension: ISQ<
        P2,     // length
        P1,     // mass
        N2,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn crate::si::marker::ReactivePowerKind;
    units {
        @petavolt_ampere_reactive_hour: 3.6_E18; "Pvar · h", "petavolt ampere reactive hour",
            "petavolt ampere reactive hours";
        @teravolt_ampere_reactive_hour: 3.6_E15; "Tvar · h", "teravolt ampere reactive hour",
            "teravolt ampere reactive hours";
        @gigavolt_ampere_reactive_hour: 3.6_E12; "Gvar · h", "gigavolt ampere reactive hour",
            "gigavolt ampere reactive hours";
        @megavolt_ampere_reactive_hour: 3.6_E9; "Mvar · h", "megavolt ampere reactive hour",
            "megavolt ampere reactive hours";
        @kilovolt_ampere_reactive_hour: 3.6_E6; "kvar · h", "kilovolt ampere reactive hour",
            "kilovolt ampere reactive hours";
        @hectovolt_ampere_reactive_hour: 3.6_E5; "hvar · h", "hectovolt ampere reactive hour",
            "hectovolt ampere reactive hours";
        @decavolt_ampere_reactive_hour: 3.6_E4; "davar · h", "decavolt ampere reactive hour",
            "decavolt ampere reactive hours";
        @volt_ampere_reactive_hour: 3.6_E3; "var · h", "volt ampere reactive hour",
            "volt ampere reactive hours";
        @millivolt_ampere_reactive_hour: 3.6_E0; "mvar · h", "millivolt ampere reactive hour",
            "millivolt ampere reactive hours";
        @microvolt_ampere_reactive_hour: 3.6_E-3; "µvar · h", "microvolt ampere reactive hour",
            "microvolt ampere reactive hours";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::energy as e;
        use crate::si::quantities::*;
        use crate::si::reactive_energy as re;
        use crate::si::reactive_power as rp;
        use crate::si::time as t;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: ReactiveEnergy<V> = (ReactivePower::new::<rp::volt_ampere_reactive>(V::one())
                * Time::new::<t::second>(V::one())).into();
        }

        #[test]
        fn check_kind_conversion() {
            Test::assert_approx_eq(&Energy::new::<e::kilowatt_hour>(V::one()),
                &ReactiveEnergy::new::<re::kilovolt_ampere_reactive_hour>(V::one()).into());
            Test::assert_approx_eq(&ReactiveEnergy::new::<re::volt_ampere_reactive_hour>(V::one()),
                &Energy::new::<e::watt_hour>(V::one()).into());
        }

        #[test]
        fn check_units() {
            test::<rp::petavolt_ampere_reactive, re::petavolt_ampere_reactive_hour>();
            test::<rp::teravolt_ampere_reactive, re::teravolt_ampere_reactive_hour>();
            test::<rp::gigavolt_ampere_reactive, re::gigavolt_ampere_reactive_hour>();
            test::<rp::megavolt_ampere_reactive, re::megavolt_ampere_reactive_hour>();
            test::<rp::kilovolt_ampere_reactive, re::kilovolt_ampere_reactive_hour>();
            test::<rp::hectovolt_ampere_reactive, re::hectovolt_ampere_reactive_hour>();
            test::<rp::decavolt_ampere_reactive, re::decavolt_ampere_reactive_hour>();
            test::<rp::volt_ampere_reactive, re::volt_ampere_reactive_hour>();
            test::<rp::millivolt_ampere_reactive, re::millivolt_ampere_reactive_hour>();
            test::<rp::microvolt_ampere_reactive, re::microvolt_ampere_reactive_hour>();

            fn test<RP: rp::Conversion<V>, RE: re::Conversion<V>>() {
                Test::assert_approx_eq(&ReactiveEnergy::new::<RE>(V::one()),
                    &(ReactivePower::new::<RP>(V::one())
                        * Time::new::<t::hour>(V::one())).into());
            }
        }
    }
}
