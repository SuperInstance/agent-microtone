# agent-microtone

> Sub-trit precision for fine-grained agent control. Between -1, 0, and +1 lies infinite space.

In music, microtones are the notes between the notes — the quarter tones, eighth tones, and infinite gradations that exist between the familiar 12 semitones of Western music. Arabic maqam, Indian raga, and blues bending all live in this space.

In ternary logic, a **trit** is one of three values: -1, 0, or +1. But between these integers lies a continuum. **agent-microtone** explores that continuum, providing high-resolution ternary fields, smooth interpolation, and precision analysis for agents that need finer control than integer trits allow.

## Core Concepts

### Microtrit

A `Microtrit` is a trit with fractional subdivisions. Values are stored as `f64` in `[-1.0, +1.0]`, where integer trits map to -1.0, 0.0, and +1.0.

```rust
use agent_microtone::Microtrit;

let m = Microtrit::new(0.37);    // somewhere between 0 and +1
let snapped = m.snap();           // rounds to nearest trit: 0.0
let fraction = m.fractional_part(); // 0.37 — how far from integer

assert!(!m.is_exact_trit());      // it's a true microtrit
assert!(m.is_positive());         // on the positive side
```

**Arithmetic** works naturally:

```rust
let a = Microtrit::new(0.5);
let b = Microtrit::new(0.3);
let sum = a + b;          // 0.8 (clamped to [-1, 1])
let diff = a - b;         // 0.2
let scaled = a * 0.5;     // 0.25
let neg = -a;             // -0.5
```

**Quantization** — like musical "cents":

```rust
let m = Microtrit::new(0.33);
let q = m.quantize(10);   // snap to 0.1 increments → 0.3
```

### Interpolation

Smooth transitions between microtrits, like gliding between notes:

```rust
use agent_microtone::{interpolate, cosine_interpolate, interpolate_path};

let a = Microtrit::NEG_ONE;
let b = Microtrit::POS_ONE;

// Linear interpolation
let mid = interpolate(a, b, 0.5);  // 0.0

// Cosine easing (smoother, more musical)
let smooth = cosine_interpolate(a, b, 0.5);  // also 0.0, but curves differently

// Multi-point path interpolation
let path = vec![Microtrit::NEG_ONE, Microtrit::ZERO, Microtrit::POS_ONE];
let at_quarter = interpolate_path(&path, 0.25);  // -0.5
```

### MicrotritGrid

A high-resolution ternary field — a 2D grid of microtrits for spatial reasoning:

```rust
use agent_microtone::{Microtrit, MicrotritGrid};

let mut grid = MicrotritGrid::new(10, 10, Microtrit::ZERO);
grid.set(5, 5, Microtrit::new(0.8));

// Bilinear interpolation at fractional coordinates
let sampled = grid.sample(4.7, 5.3);

// Element-wise operations
let negated = grid.map(|m| -m);
let sum = grid.add_grid(&other_grid);
```

Grids support:
- **Bilinear sampling** — smooth value lookup at fractional coordinates
- **Mapping** — transform every cell through a function
- **Addition** — element-wise addition of two grids
- **Averaging** — mean value across the entire field

### Microtonal Harmony

Measure how "harmonious" a set of microtrits are — low variance = high harmony:

```rust
use agent_microtone::{microtonal_harmony, dissonance, Microtrit};

// Perfect unison → maximum harmony
let h = microtonal_harmony(&[Microtrit::new(0.5); 3]);
assert!((h - 1.0).abs() < 1e-10);

// Wide spread → dissonance
let h = microtonal_harmony(&[Microtrit::NEG_ONE, Microtrit::POS_ONE]);
assert!(h < 0.5);

// Pairwise dissonance
let d = dissonance(Microtrit::new(0.1), Microtrit::new(0.9)); // 0.4
```

### Precision Analysis

How much does sub-trit resolution actually matter for a given value?

```rust
use agent_microtone::{precision_analysis, SubTritTier};

let report = precision_analysis(Microtrit::new(0.35));
println!("Deviation from nearest trit: {:.3}", report.deviation);
println!("Information gain: {:.2} bits", report.information_bits);
println!("Tier: {:?}", report.sub_trit_tier);
```

| Tier | Deviation | Meaning |
|------|-----------|---------|
| **Negligible** | < 0.1 | Integer trit is basically fine |
| **Subtle** | 0.1–0.25 | Measurable but small impact |
| **Moderate** | 0.25–0.4 | Meaningful precision gain |
| **Significant** | ≥ 0.4 | Integer trit loses a lot of information |

## Smooth Transitions

Animate between two states of microtrit arrays:

```rust
use agent_microtone::{smooth_transition, Microtrit};

let state_a = vec![Microtrit::NEG_ONE, Microtrit::ZERO, Microtrit::POS_ONE];
let state_b = vec![Microtrit::POS_ONE, Microtrit::POS_ONE, Microtrit::ZERO];

// At t=0.5, each element is cosine-interpolated halfway
let mid_state = smooth_transition(&state_a, &state_b, 0.5);
```

## Design Philosophy

This crate draws from the idea that **precision matters in the spaces between**. In music, the difference between a note and its quarter-tone variant can be the difference between tension and resolution, between one emotion and another. Similarly, in agent control, the difference between "slightly positive" (0.3) and "definitely positive" (0.7) can shape behavior meaningfully.

The precision analysis tools let you decide when microtonal precision is worth the computational cost and when integer trits suffice — like a musician choosing when to bend a note and when to play it straight.

## Running Tests

```bash
cargo test
```

All 28 tests cover: microtrit creation, arithmetic, interpolation (linear/cosine/path), grid operations, harmony, dissonance, precision analysis, and smooth transitions.

## License

MIT
