extern crate liballistic;

use liballistic::conversions::Temperatures as Temp;
use liballistic::conversions::Lengths as Len;
use liballistic::conversions::Masses as Mass;
use liballistic::conversions::Angles as Angle;
use liballistic::conversions::Pressures as Press;

use liballistic::bc as BC;

fn test_float_eq(a: f64, b: f64) -> () {
    assert_eq!(format!("{:.6}", a), format!("{:.6}", b));
}

fn main() {
    println!("Test.rs: main");

    /*
     * Conversions
     */

    /* Temperatures*/
    test_float_eq(Temp::c_to_k(0.0), 273.15);

    /* Lengths */
    test_float_eq(Len::yd_to_m(12.4), 11.338560);

    /* Masses */
    test_float_eq(Mass::gr_to_kg(9.3), 0.602630);

    /* Angles */
    test_float_eq(Angle::rad_to_deg(45.234), 2591.71729);

    /* Pressures */
    test_float_eq(Press::bar_to_pa(5.52489927462), 552489.927462); 

    /*
     * BC
     */

    /* Ballistic Coefficient */
    test_float_eq(BC::ballistic_coef(155.0, 0.308, 0.48), 0.486285);
}
