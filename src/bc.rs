/* Inputs : 
 * W :          grains
 * cal:         inches
 * form:        no unit
 */
pub fn ballistic_coef(W: f64, cal: f64, form: f64) -> f64 {
    (W / 7000.0) / (cal * cal * form)
}
