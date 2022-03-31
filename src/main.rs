//! Example showing how to use the pre-built SI system.



pub mod si {
    pub use uom::fmt::DisplayStyle::Abbreviation;
    pub use uom::si::f64::*;
    pub use uom::si::length::{centimeter, kilometer, meter};
    pub use uom::si::time::second;
    pub use uom::si::velocity::{kilometer_per_second, meter_per_second};
    pub use uom::si::power::{watt, kilowatt};
    pub use uom::si::f64::*;
    pub use uom::si::velocity::meter_per_second as mps;
}

struct UnitThing {
    a: si::Power,
    b: si::Power,
}

impl UnitThing {
    fn new() -> UnitThing {
        let a = si::Power::new::<si::watt>(10.0);
        let b = si::Power::new::<si::watt>(1.0);
        UnitThing {
            a:a,
            b:b,
        }
    }

    fn sum_powers(&self) -> si::Power {
        self.a + self.b
    }
}


fn main() {
    // Setup length and time quantities using different units.
    let p1 = si::Power::new::<si::watt>(10.0);
    let p2 = si::Power::new::<si::watt>(11.1);
    let l1 = si::Length::new::<si::meter>(15.0);
    let l2 = si::Length::new::<si::centimeter>(10.0);
    let t1 = si::Time::new::<si::second>(50.0);
    let v1 = l1 / t1;
    //let error = l1 + t1; // error[E0308]: mismatched types

    // Setup re-usable format arguments.
    let m = si::Length::format_args(si::meter, si::Abbreviation);
    let cm = si::Length::format_args(si::centimeter, si::Abbreviation);
    let s = si::Time::format_args(si::second, si::Abbreviation);
    let w = si::Power::format_args(si::watt, si::Abbreviation);

    // Print results of simple formulas using different output units.
    println!("{} + {} = {}", w.with(p1), w.with(p2), w.with(p1 + p2));
    println!("{} + {} = {}", m.with(l1), cm.with(l2), m.with(l1 + l2));
    println!(
        "{} + {} = {}",
        m.with(l1),
        cm.with(l2),
        (l1 + l2).into_format_args(si::kilometer, si::Abbreviation)
    );
    println!(
        "{} / {} = {}",
        m.with(l1),
        s.with(t1),
        v1.into_format_args(si::meter_per_second, si::Abbreviation)
    );
    println!(
        "{} / {} = {}",
        m.with(l1),
        s.with(t1),
        v1.into_format_args(si::kilometer_per_second, si::Abbreviation)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_that_powers_sum() {
        let u = UnitThing::new();
        let sum = u.sum_powers();
        assert_eq!(sum, si::Power::new::<si::kilowatt>(11.0e-3));
    }
}