/* Conversions::*
 *  > Lengths :      Inches | Meters | Foots  | Yards
 *          Inches |   __   |in_to_m |        | 
 *          Meters | m_to_in|   __   | m_to_f |m_to_yd
 *          Foots  |        | f_to_m |   __   | 
 *          Yards  |        |yd_to_m |        |   __ 
 *
 *  > Masses : Grains, Kilograms
 *  > Pressures : Pascals, Bars
 *  > Temperatures : Celsus, Kelvins, Farenheit
 *  > Angles : Radians, Degrees, Minutes of Angle, Miliradians
 *
    // TODO: documentation
 */

pub mod Lengths {
    pub fn in_to_m(value: f64) -> f64 {
        value * 0.0254
    }

    pub fn m_to_in(value: f64) -> f64 {
        value / 0.0254
    }

    pub fn f_to_m(value: f64) -> f64 {
        value * 0.3048
    }

    pub fn m_to_f(value: f64) -> f64 {
        value / 0.3048
    }

    pub fn yd_to_m(value: f64) -> f64 {
        value * 0.9144
    }

    pub fn m_to_yd(value: f64) -> f64 {
        value / 0.9144
    }
}

pub mod Masses {
    pub fn gr_to_kg(value: f64) -> f64 {
        value * 0.06479891
    }

    pub fn kg_to_gr(value: f64) -> f64 {
        value / 0.06479891
    }
}

pub mod Pressures {
    pub fn bar_to_pa(value: f64) -> f64 {
        value * 1e5
    }

    pub fn pa_to_bar(value: f64) -> f64 {
        value * 1e-5
    }
}

pub mod Temperatures {
    pub fn c_to_k(value: f64) -> f64 {
        value + 273.15
    }

    pub fn k_to_c(value: f64) -> f64 {
        value - 273.15
    }

    pub fn f_to_c(value: f64) -> f64 {
        (5.0 / 9.0) * (value - 32.0)
    }

    pub fn c_to_f(value: f64) -> f64 {
        (9.0 / 5.0) * value + 32.0
    }

    pub fn k_to_f(value: f64) -> f64 {
        c_to_f(k_to_c(value))
    }

    pub fn f_to_k(value: f64) -> f64 {
        c_to_k(f_to_c(value))
    }
}

pub mod Angles {
    use std::f64::consts::PI;
    pub fn rad_to_deg(value: f64) -> f64 {
        value * 180.0 / PI
    }

    pub fn deg_to_rad(value: f64) -> f64 {
        value * PI / 180.0
    }

    pub fn moa_to_deg(value: f64) -> f64 {
        value / 60.0
    }

    pub fn deg_to_moa(value: f64) -> f64 {
        value * 60.0
    }

    pub fn mil_to_rad(value: f64) -> f64 {
        value * 0.001
    }

    pub fn rad_to_mil(value: f64) -> f64 {
        value * 1000.0
    }
}
