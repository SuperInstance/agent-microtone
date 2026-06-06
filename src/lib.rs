//! # agent-microtone
//!
//! Sub-trit precision for fine-grained agent control. In ternary logic, values
//! are -1, 0, or +1. But between those integers lies infinite space. This crate
//! explores that space — microtonal subdivisions of trits, enabling high-resolution
//! ternary fields, smooth interpolation, and precision analysis.

use std::ops::{Add, Sub, Mul, Neg};

/// A microtrit: a trit (-1, 0, +1) with fractional sub-divisions.
/// Values are stored as f64 in [-1.0, +1.0]. Integer trits map to -1.0, 0.0, +1.0.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Microtrit(f64);

impl Microtrit {
    /// Create a microtrit, clamping to [-1.0, +1.0].
    pub fn new(value: f64) -> Self {
        Self(value.clamp(-1.0, 1.0))
    }

    /// Negative one (-1.0).
    pub const NEG_ONE: Microtrit = Microtrit(-1.0);
    /// Zero (0.0).
    pub const ZERO: Microtrit = Microtrit(0.0);
    /// Positive one (+1.0).
    pub const POS_ONE: Microtrit = Microtrit(1.0);

    /// Raw value.
    pub fn value(&self) -> f64 {
        self.0
    }

    /// Round to nearest integer trit: -1, 0, or +1.
    pub fn round_trit(&self) -> i8 {
        if self.0 < -0.5 { -1 }
        else if self.0 > 0.5 { 1 }
        else { 0 }
    }

    /// Is this an exact integer trit?
    pub fn is_exact_trit(&self) -> bool {
        (self.0 - self.round_trit() as f64).abs() < f64::EPSILON
    }

    /// Distance from nearest integer trit (0.0 = exact, up to 0.5).
    pub fn fractional_part(&self) -> f64 {
        (self.0 - self.round_trit() as f64).abs()
    }

    /// Sub-division level: how many "cents" between trits.
    /// E.g., if resolution = 100, each trit step has 100 micro-levels.
    pub fn quantize(&self, resolution: u32) -> Self {
        if resolution == 0 { return *self; }
        let step = 1.0 / resolution as f64;
        let quantized = (self.0 / step).round() * step;
        Microtrit::new(quantized)
    }

    /// Absolute distance from zero (how "strong" the value is).
    pub fn magnitude(&self) -> f64 {
        self.0.abs()
    }

    /// Is the value on the positive side?
    pub fn is_positive(&self) -> bool {
        self.0 > 0.0
    }

    /// Is the value on the negative side?
    pub fn is_negative(&self) -> bool {
        self.0 < 0.0
    }

    /// Is the value neutral (zero)?
    pub fn is_neutral(&self) -> bool {
        self.0.abs() < f64::EPSILON
    }

    /// Snap to the nearest of the three trit values.
    pub fn snap(&self) -> Self {
        Microtrit::new(self.round_trit() as f64)
    }
}

impl Add for Microtrit {
    type Output = Microtrit;
    fn add(self, rhs: Self) -> Self::Output {
        Microtrit::new(self.0 + rhs.0)
    }
}

impl Sub for Microtrit {
    type Output = Microtrit;
    fn sub(self, rhs: Self) -> Self::Output {
        Microtrit::new(self.0 - rhs.0)
    }
}

impl Mul<f64> for Microtrit {
    type Output = Microtrit;
    fn mul(self, rhs: f64) -> Self::Output {
        Microtrit::new(self.0 * rhs)
    }
}

impl Neg for Microtrit {
    type Output = Microtrit;
    fn neg(self) -> Self::Output {
        Microtrit::new(-self.0)
    }
}

impl Default for Microtrit {
    fn default() -> Self {
        Self::ZERO
    }
}

/// Interpolate between two microtrits.
pub fn interpolate(a: Microtrit, b: Microtrit, t: f64) -> Microtrit {
    let t = t.clamp(0.0, 1.0);
    Microtrit::new(a.0 * (1.0 - t) + b.0 * t)
}

/// Multi-point interpolation through a sequence of microtrits.
/// `t` in [0.0, 1.0] maps across the whole sequence.
pub fn interpolate_path(points: &[Microtrit], t: f64) -> Microtrit {
    if points.is_empty() { return Microtrit::ZERO; }
    if points.len() == 1 { return points[0]; }
    let t = t.clamp(0.0, 1.0);
    let segment_count = points.len() - 1;
    let scaled_t = t * segment_count as f64;
    let idx = (scaled_t.floor() as usize).min(segment_count - 1);
    let local_t = scaled_t - idx as f64;
    interpolate(points[idx], points[idx + 1], local_t)
}

/// Smooth interpolation using cosine easing.
pub fn cosine_interpolate(a: Microtrit, b: Microtrit, t: f64) -> Microtrit {
    let t = t.clamp(0.0, 1.0);
    let t2 = (1.0 - (t * std::f64::consts::PI).cos()) / 2.0;
    Microtrit::new(a.0 * (1.0 - t2) + b.0 * t2)
}

/// A high-resolution ternary field — a grid of microtrits.
#[derive(Debug, Clone)]
pub struct MicrotritGrid {
    /// Grid dimensions (rows, cols).
    pub dimensions: (usize, usize),
    data: Vec<Vec<Microtrit>>,
}

impl MicrotritGrid {
    /// Create a grid filled with a default value.
    pub fn new(rows: usize, cols: usize, default: Microtrit) -> Self {
        let data = vec![vec![default; cols]; rows];
        Self { dimensions: (rows, cols), data }
    }

    /// Create a grid from a 2D array of f64 values.
    pub fn from_values(values: &[&[f64]]) -> Self {
        let rows = values.len();
        let cols = if rows > 0 { values[0].len() } else { 0 };
        let data = values.iter().map(|row| {
            row.iter().map(|&v| Microtrit::new(v)).collect()
        }).collect();
        Self { dimensions: (rows, cols), data }
    }

    /// Get value at (row, col).
    pub fn get(&self, row: usize, col: usize) -> Option<Microtrit> {
        self.data.get(row).and_then(|r| r.get(col)).copied()
    }

    /// Set value at (row, col).
    pub fn set(&mut self, row: usize, col: usize, value: Microtrit) -> bool {
        if let Some(r) = self.data.get_mut(row) {
            if let Some(cell) = r.get_mut(col) {
                *cell = value;
                return true;
            }
        }
        false
    }

    /// Number of cells in the grid.
    pub fn cell_count(&self) -> usize {
        self.dimensions.0 * self.dimensions.1
    }

    /// Average value across the grid.
    pub fn average(&self) -> Microtrit {
        if self.cell_count() == 0 { return Microtrit::ZERO; }
        let sum: f64 = self.data.iter().flat_map(|r| r.iter()).map(|m| m.0).sum();
        Microtrit::new(sum / self.cell_count() as f64)
    }

    /// Bilinear interpolation at fractional coordinates (row, col).
    pub fn sample(&self, row: f64, col: f64) -> Microtrit {
        let r = row.clamp(0.0, (self.dimensions.0 - 1) as f64);
        let c = col.clamp(0.0, (self.dimensions.1 - 1) as f64);
        let r0 = r.floor() as usize;
        let c0 = c.floor() as usize;
        let r1 = (r0 + 1).min(self.dimensions.0 - 1);
        let c1 = (c0 + 1).min(self.dimensions.1 - 1);
        let rf = r - r0 as f64;
        let cf = c - c0 as f64;

        let v00 = self.get(r0, c0).unwrap_or(Microtrit::ZERO).value();
        let v01 = self.get(r0, c1).unwrap_or(Microtrit::ZERO).value();
        let v10 = self.get(r1, c0).unwrap_or(Microtrit::ZERO).value();
        let v11 = self.get(r1, c1).unwrap_or(Microtrit::ZERO).value();

        let top = v00 * (1.0 - cf) + v01 * cf;
        let bot = v10 * (1.0 - cf) + v11 * cf;
        Microtrit::new(top * (1.0 - rf) + bot * rf)
    }

    /// Map each cell through a function.
    pub fn map<F: Fn(Microtrit) -> Microtrit>(&self, f: F) -> Self {
        let data = self.data.iter().map(|row| {
            row.iter().map(|&m| f(m)).collect()
        }).collect();
        Self { dimensions: self.dimensions, data }
    }

    /// Element-wise addition of two grids (must be same dimensions).
    pub fn add_grid(&self, other: &MicrotritGrid) -> Option<MicrotritGrid> {
        if self.dimensions != other.dimensions { return None; }
        let data = self.data.iter().zip(&other.data).map(|(r1, r2)| {
            r1.iter().zip(r2).map(|(&a, &b)| a + b).collect()
        }).collect();
        Some(MicrotritGrid { dimensions: self.dimensions, data })
    }
}

/// Microtonal harmony: compute how "harmonious" a set of microtrits are.
/// Harmony is high when values are close together (low variance).
pub fn microtonal_harmony(values: &[Microtrit]) -> f64 {
    if values.is_empty() { return 1.0; }
    let n = values.len() as f64;
    let mean: f64 = values.iter().map(|m| m.0).sum::<f64>() / n;
    let variance: f64 = values.iter().map(|m| (m.0 - mean).powi(2)).sum::<f64>() / n;
    // Max variance at [-1, +1] with equal count = 1.0
    // Normalize: harmony = 1.0 - sqrt(variance)
    1.0 - variance.sqrt().min(1.0)
}

/// Compute the dissonance between two microtrits — distance from unison.
pub fn dissonance(a: Microtrit, b: Microtrit) -> f64 {
    (a.0 - b.0).abs() / 2.0  // normalized to [0, 1]
}

/// Smooth transition between two microtrit arrays using cosine interpolation.
pub fn smooth_transition(from: &[Microtrit], to: &[Microtrit], t: f64) -> Vec<Microtrit> {
    let len = from.len().min(to.len());
    (0..len).map(|i| cosine_interpolate(from[i], to[i], t)).collect()
}

/// Precision analysis: how much does sub-trit resolution matter for a given value?
/// Returns the "information gain" of having microtonal precision vs integer trits.
pub fn precision_analysis(value: Microtrit) -> PrecisionReport {
    let snapped = value.snap();
    let deviation = (value.0 - snapped.0).abs();
    PrecisionReport {
        raw_value: value,
        snapped_value: snapped,
        deviation,
        information_bits: if deviation.abs() < f64::EPSILON { 0.0 } else { -deviation.log2() },
        sub_trit_tier: if deviation < 0.1 { SubTritTier::Negligible }
                       else if deviation < 0.25 { SubTritTier::Subtle }
                       else if deviation < 0.4 { SubTritTier::Moderate }
                       else { SubTritTier::Significant },
    }
}

/// Tier of sub-trit precision importance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubTritTier {
    /// Deviation < 0.1 — barely matters.
    Negligible,
    /// Deviation 0.1–0.25 — subtle but measurable.
    Subtle,
    /// Deviation 0.25–0.4 — moderate impact.
    Moderate,
    /// Deviation >= 0.4 — significant; integer trit loses a lot.
    Significant,
}

/// Report from precision analysis.
#[derive(Debug, Clone, Copy)]
pub struct PrecisionReport {
    pub raw_value: Microtrit,
    pub snapped_value: Microtrit,
    pub deviation: f64,
    pub information_bits: f64,
    pub sub_trit_tier: SubTritTier,
}

// ─── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_microtrit_creation() {
        let m = Microtrit::new(0.5);
        assert!((m.value() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_microtrit_clamping() {
        let m = Microtrit::new(2.0);
        assert!((m.value() - 1.0).abs() < f64::EPSILON);
        let m = Microtrit::new(-3.0);
        assert!((m.value() + 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_microtrit_round_trit() {
        assert_eq!(Microtrit::new(-0.8).round_trit(), -1);
        assert_eq!(Microtrit::new(-0.3).round_trit(), 0);
        assert_eq!(Microtrit::new(0.2).round_trit(), 0);
        assert_eq!(Microtrit::new(0.6).round_trit(), 1);
    }

    #[test]
    fn test_microtrit_is_exact() {
        assert!(Microtrit::NEG_ONE.is_exact_trit());
        assert!(Microtrit::ZERO.is_exact_trit());
        assert!(Microtrit::POS_ONE.is_exact_trit());
        assert!(!Microtrit::new(0.3).is_exact_trit());
    }

    #[test]
    fn test_microtrit_fractional() {
        let m = Microtrit::new(0.7);
        assert!((m.fractional_part() - 0.3).abs() < 1e-10);
    }

    #[test]
    fn test_microtrit_quantize() {
        let m = Microtrit::new(0.33).quantize(10);
        assert!((m.value() - 0.3).abs() < 1e-10);
    }

    #[test]
    fn test_microtrit_sign() {
        assert!(Microtrit::new(0.5).is_positive());
        assert!(Microtrit::new(-0.5).is_negative());
        assert!(Microtrit::ZERO.is_neutral());
    }

    #[test]
    fn test_microtrit_arithmetic() {
        let a = Microtrit::new(0.5);
        let b = Microtrit::new(0.3);
        let sum = a + b;
        assert!((sum.value() - 0.8).abs() < 1e-10);
        let diff = a - b;
        assert!((diff.value() - 0.2).abs() < 1e-10);
        let scaled = a * 0.5;
        assert!((scaled.value() - 0.25).abs() < 1e-10);
        let neg = -a;
        assert!((neg.value() + 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_interpolation() {
        let a = Microtrit::NEG_ONE;
        let b = Microtrit::POS_ONE;
        let mid = interpolate(a, b, 0.5);
        assert!((mid.value() - 0.0).abs() < 1e-10);
        let quarter = interpolate(a, b, 0.25);
        assert!((quarter.value() + 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_interpolation_clamping() {
        let a = Microtrit::ZERO;
        let b = Microtrit::POS_ONE;
        let over = interpolate(a, b, 2.0);
        assert!((over.value() - 1.0).abs() < 1e-10);
        let under = interpolate(a, b, -1.0);
        assert!((under.value() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_interpolate_path() {
        let points = vec![Microtrit::NEG_ONE, Microtrit::ZERO, Microtrit::POS_ONE];
        let at_start = interpolate_path(&points, 0.0);
        assert!((at_start.value() + 1.0).abs() < 1e-10);
        let at_mid = interpolate_path(&points, 0.5);
        assert!((at_mid.value() - 0.0).abs() < 1e-10);
        let at_end = interpolate_path(&points, 1.0);
        assert!((at_end.value() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_cosine_interpolation() {
        let a = Microtrit::NEG_ONE;
        let b = Microtrit::POS_ONE;
        let mid = cosine_interpolate(a, b, 0.5);
        // Cosine interpolation at 0.5 should be 0.0 (same as linear for midpoint)
        assert!((mid.value() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_grid_creation() {
        let grid = MicrotritGrid::new(3, 4, Microtrit::ZERO);
        assert_eq!(grid.dimensions, (3, 4));
        assert_eq!(grid.cell_count(), 12);
        assert_eq!(grid.get(1, 2), Some(Microtrit::ZERO));
    }

    #[test]
    fn test_grid_from_values() {
        let grid = MicrotritGrid::from_values(&[
            &[0.5, -0.3],
            &[1.0, 0.0],
        ]);
        assert_eq!(grid.dimensions, (2, 2));
        assert!((grid.get(0, 0).unwrap().value() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_grid_set_get() {
        let mut grid = MicrotritGrid::new(2, 2, Microtrit::ZERO);
        assert!(grid.set(0, 0, Microtrit::new(0.7)));
        assert!((grid.get(0, 0).unwrap().value() - 0.7).abs() < 1e-10);
        assert!(!grid.set(5, 5, Microtrit::POS_ONE)); // out of bounds
    }

    #[test]
    fn test_grid_average() {
        let grid = MicrotritGrid::from_values(&[&[-1.0, 1.0]]);
        let avg = grid.average();
        assert!((avg.value() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_grid_sample() {
        let grid = MicrotritGrid::from_values(&[
            &[-1.0, 1.0],
            &[-1.0, 1.0],
        ]);
        let center = grid.sample(0.5, 0.5);
        assert!((center.value() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_grid_map() {
        let grid = MicrotritGrid::from_values(&[&[0.5, -0.5]]);
        let negated = grid.map(|m| -m);
        assert!((negated.get(0, 0).unwrap().value() + 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_grid_add() {
        let a = MicrotritGrid::from_values(&[&[0.5, -0.5]]);
        let b = MicrotritGrid::from_values(&[&[0.3, 0.3]]);
        let sum = a.add_grid(&b).unwrap();
        assert!((sum.get(0, 0).unwrap().value() - 0.8).abs() < 1e-10);
    }

    #[test]
    fn test_grid_add_mismatched() {
        let a = MicrotritGrid::new(2, 2, Microtrit::ZERO);
        let b = MicrotritGrid::new(3, 3, Microtrit::ZERO);
        assert!(a.add_grid(&b).is_none());
    }

    #[test]
    fn test_microtonal_harmony() {
        // Perfect unison
        let harmony = microtonal_harmony(&[Microtrit::new(0.5), Microtrit::new(0.5), Microtrit::new(0.5)]);
        assert!((harmony - 1.0).abs() < 1e-10);

        // Wide spread
        let disharmony = microtonal_harmony(&[Microtrit::NEG_ONE, Microtrit::POS_ONE]);
        assert!(disharmony < 0.5);
    }

    #[test]
    fn test_dissonance() {
        let d = dissonance(Microtrit::ZERO, Microtrit::ZERO);
        assert!((d - 0.0).abs() < 1e-10);
        let d = dissonance(Microtrit::NEG_ONE, Microtrit::POS_ONE);
        assert!((d - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_smooth_transition() {
        let from = vec![Microtrit::NEG_ONE, Microtrit::ZERO];
        let to = vec![Microtrit::POS_ONE, Microtrit::POS_ONE];
        let mid = smooth_transition(&from, &to, 0.5);
        assert!((mid[0].value() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_precision_analysis_exact() {
        let report = precision_analysis(Microtrit::POS_ONE);
        assert!((report.deviation - 0.0).abs() < 1e-10);
        assert_eq!(report.sub_trit_tier, SubTritTier::Negligible);
    }

    #[test]
    fn test_precision_analysis_microtonal() {
        let report = precision_analysis(Microtrit::new(0.3));
        assert!(report.deviation > 0.2);
        assert!(report.information_bits > 0.0);
    }

    #[test]
    fn test_precision_tiers() {
        assert_eq!(precision_analysis(Microtrit::new(0.05)).sub_trit_tier, SubTritTier::Negligible);
        assert_eq!(precision_analysis(Microtrit::new(0.2)).sub_trit_tier, SubTritTier::Subtle);
        assert_eq!(precision_analysis(Microtrit::new(0.3)).sub_trit_tier, SubTritTier::Moderate);
        assert_eq!(precision_analysis(Microtrit::new(0.45)).sub_trit_tier, SubTritTier::Significant);
    }

    #[test]
    fn test_microtrit_magnitude() {
        assert!((Microtrit::new(0.7).magnitude() - 0.7).abs() < 1e-10);
        assert!((Microtrit::new(-0.3).magnitude() - 0.3).abs() < 1e-10);
    }

    #[test]
    fn test_microtrit_snap() {
        let m = Microtrit::new(0.7).snap();
        assert!((m.value() - 1.0).abs() < 1e-10);
        let m = Microtrit::new(-0.7).snap();
        assert!((m.value() + 1.0).abs() < 1e-10);
        let m = Microtrit::new(0.2).snap();
        assert!((m.value() - 0.0).abs() < 1e-10);
    }
}
