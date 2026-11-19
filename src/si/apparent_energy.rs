//! Apparent energy (base unit volt ampere second, m² · kg · s⁻²).

quantity! {
    /// Apparent energy (base unit volt ampere second, m² · kg · s⁻²).
    quantity: ApparentEnergy; "apparent energy";
    /// Dimension of apparent energy, L²MT⁻² (base unit volt ampere second, m² · kg · s⁻²).
    dimension: ISQ<
        P2,     // length
        P1,     // mass
        N2,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn crate::si::marker::ApparentPowerKind;
    units {
        @petavolt_ampere_hour: 3.6_E18; "PV · A · h", "petavolt ampere hour",
            "petavolt ampere hours";
        @teravolt_ampere_hour: 3.6_E15; "TV · A · h", "teravolt ampere hour",
            "teravolt ampere hours";
        @gigavolt_ampere_hour: 3.6_E12; "GV · A · h", "gigavolt ampere hour",
            "gigavolt ampere hours";
        @megavolt_ampere_hour: 3.6_E9; "MV · A · h", "megavolt ampere hour",
            "megavolt ampere hours";
        @kilovolt_ampere_hour: 3.6_E6; "kV · A · h", "kilovolt ampere hour",
            "kilovolt ampere hours";
        @hectovolt_ampere_hour: 3.6_E5; "hV · A · h", "hectovolt ampere hour",
            "hectovolt ampere hours";
        @decavolt_ampere_hour: 3.6_E4; "daV · A · h", "decavolt ampere hour",
            "decavolt ampere hours";
        @volt_ampere_hour: 3.6_E3; "V · A · h", "volt ampere hour", "volt ampere hours";
        @millivolt_ampere_hour: 3.6_E0; "mV · A · h", "millivolt ampere hour",
            "millivolt ampere hours";
        @microvolt_ampere_hour: 3.6_E-3; "µV · A · h", "microvolt ampere hour",
            "microvolt ampere hours";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::apparent_energy as ae;
        use crate::si::apparent_power as ap;
        use crate::si::energy as e;
        use crate::si::quantities::*;
        use crate::si::time as t;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: ApparentEnergy<V> = (ApparentPower::new::<ap::volt_ampere>(V::one())
                * Time::new::<t::second>(V::one())).into();
        }

        #[test]
        fn check_kind_conversion() {
            Test::assert_approx_eq(&Energy::new::<e::kilowatt_hour>(V::one()),
                &ApparentEnergy::new::<ae::kilovolt_ampere_hour>(V::one()).into());
            Test::assert_approx_eq(&ApparentEnergy::new::<ae::volt_ampere_hour>(V::one()),
                &Energy::new::<e::watt_hour>(V::one()).into());
        }

        #[test]
        fn check_units() {
            test::<ap::petavolt_ampere, ae::petavolt_ampere_hour>();
            test::<ap::teravolt_ampere, ae::teravolt_ampere_hour>();
            test::<ap::gigavolt_ampere, ae::gigavolt_ampere_hour>();
            test::<ap::megavolt_ampere, ae::megavolt_ampere_hour>();
            test::<ap::kilovolt_ampere, ae::kilovolt_ampere_hour>();
            test::<ap::hectovolt_ampere, ae::hectovolt_ampere_hour>();
            test::<ap::decavolt_ampere, ae::decavolt_ampere_hour>();
            test::<ap::volt_ampere, ae::volt_ampere_hour>();
            test::<ap::millivolt_ampere, ae::millivolt_ampere_hour>();
            test::<ap::microvolt_ampere, ae::microvolt_ampere_hour>();

            fn test<AP: ap::Conversion<V>, AE: ae::Conversion<V>>() {
                Test::assert_approx_eq(&ApparentEnergy::new::<AE>(V::one()),
                    &(ApparentPower::new::<AP>(V::one())
                        * Time::new::<t::hour>(V::one())).into());
            }
        }
    }
}
