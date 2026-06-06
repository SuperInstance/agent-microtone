use agent_microtone::*;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║           🎹 Microtrit Tuning Systems Demo 🎹               ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    // === Section 1: The Three Trit Values ===
    println!("━━━ Section 1: The Trit Spectrum ━━━");
    println!();
    println!("  Trits are the ternary digits: -1, 0, +1");
    println!("  Microtrits add infinite subdivision between them.");
    println!();

    let trits = [Microtrit::NEG_ONE, Microtrit::ZERO, Microtrit::POS_ONE];
    for t in &trits {
        println!("  {:>5.1}  round_trit={:+2}  exact={}  magnitude={:.1}",
            t.value(), t.round_trit(), t.is_exact_trit(), t.magnitude());
    }
    println!();

    // === Section 2: Tuning Systems as Microtrit Grids ===
    println!("━━━ Section 2: Tuning Systems Compared ━━━");
    println!();

    // Equal temperament: evenly spaced
    let equal_temperament: Vec<Microtrit> = (0..=12)
        .map(|i| Microtrit::new(-1.0 + 2.0 * i as f64 / 12.0))
        .collect();

    // Just intonation: ratios based on simple fractions
    let just_ratios = [1.0, 16.0/15.0, 9.0/8.0, 6.0/5.0, 5.0/4.0, 4.0/3.0,
                       1.5, 8.0/5.0, 5.0/3.0, 9.0/5.0, 15.0/8.0, 2.0];
    let just_intonation: Vec<Microtrit> = just_ratios.iter()
        .map(|&r| Microtrit::new(2.0 * (r - 1.0))) // map ratio to microtrit range
        .collect();

    // Microtonal: 24-ET (quarter tones)
    let microtonal: Vec<Microtrit> = (0..=24)
        .map(|i| Microtrit::new(-1.0 + 2.0 * i as f64 / 24.0))
        .collect();

    println!("  Equal Temperament (12-ET):  {} divisions", equal_temperament.len());
    println!("  Just Intonation:            {} notes from pure ratios", just_intonation.len());
    println!("  Microtonal (24-ET):         {} quarter-tone divisions", microtonal.len());
    println!();

    // Show the frequency ratios
    println!("  ┌─────────────────────────────────────────────────┐");
    println!("  │  Note │ 12-ET Step │ Just Ratio │ Difference     │");
    println!("  ├─────────────────────────────────────────────────┤");
    let note_names = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
    for i in 0..12 {
        let et_val = equal_temperament[i].value();
        let ji_val = just_intonation[i].value();
        let diff = (et_val - ji_val).abs();
        let diff_bar = if diff < 0.01 { "≈ unison" }
                       else if diff < 0.05 { "· subtle" }
                       else if diff < 0.15 { "○ noticeable" }
                       else { "● significant" };
        println!("  │  {:>2}   │  {:>+6.3}   │ {:>+6.3}    │ {}   │",
            note_names[i], et_val, ji_val, diff_bar);
    }
    println!("  └─────────────────────────────────────────────────┘");
    println!();

    // === Section 3: Harmony & Dissonance ===
    println!("━━━ Section 3: Harmony & Dissonance ━━━");
    println!();

    let unison = vec![Microtrit::new(0.5); 4];
    let chord = vec![Microtrit::new(-0.3), Microtrit::new(0.0), Microtrit::new(0.4)];
    let clash = vec![Microtrit::NEG_ONE, Microtrit::new(0.1), Microtrit::POS_ONE];

    println!("  Unison [0.5, 0.5, 0.5, 0.5]: harmony = {:.3}", microtonal_harmony(&unison));
    println!("  Triad  [-0.3, 0.0, 0.4]:      harmony = {:.3}", microtonal_harmony(&chord));
    println!("  Clash  [-1.0, 0.1, 1.0]:       harmony = {:.3}", microtonal_harmony(&clash));
    println!();

    println!("  Pairwise dissonance:");
    for (a, b) in [(-1.0, 1.0), (-0.5, 0.5), (0.0, 0.0), (0.3, 0.35), (0.0, 0.5)] {
        let d = dissonance(Microtrit::new(a), Microtrit::new(b));
        println!("    ({:+.1}, {:+.1}) → dissonance = {:.3}", a, b, d);
    }
    println!();

    // === Section 4: Interpolation & Smooth Transitions ===
    println!("━━━ Section 4: Smooth Transitions ━━━");
    println!();

    let from = vec![Microtrit::NEG_ONE, Microtrit::ZERO, Microtrit::POS_ONE];
    let to = vec![Microtrit::POS_ONE, Microtrit::POS_ONE, Microtrit::NEG_ONE];

    println!("  Morphing from [-1, 0, +1] to [+1, +1, -1]:");
    for t in [0.0, 0.25, 0.5, 0.75, 1.0] {
        let result = smooth_transition(&from, &to, t);
        let vals: Vec<String> = result.iter().map(|m| format!("{:+.2}", m.value())).collect();
        println!("    t={:.2}  [{}]", t, vals.join(", "));
    }
    println!();

    // === Section 5: Precision Analysis ===
    println!("━━━ Section 5: Sub-Trit Precision Analysis ━━━");
    println!();

    let test_values = [0.0, 0.05, 0.15, 0.25, 0.35, 0.45, 0.5, 0.75, 1.0];
    println!("  Value  │ Snapped │ Deviation │ Info Bits │ Tier");
    println!("  ───────┼─────────┼───────────┼───────────┼─────────────");
    for v in test_values {
        let report = precision_analysis(Microtrit::new(v));
        println!("  {:>+5.2}  │ {:>+5.1}   │  {:.3}    │  {:.2}     │ {:?}",
            v,
            report.snapped_value.value(),
            report.deviation,
            report.information_bits,
            report.sub_trit_tier
        );
    }
    println!();

    // === Section 6: Microtrit Grid Visualization ===
    println!("━━━ Section 6: 2D Microtrit Field ━━━");
    println!();

    let grid = MicrotritGrid::from_values(&[
        &[-1.0, -0.5,  0.0,  0.5,  1.0],
        &[-0.5,  0.0,  0.5,  1.0,  0.5],
        &[ 0.0,  0.5,  1.0,  0.5,  0.0],
        &[ 0.5,  1.0,  0.5,  0.0, -0.5],
        &[ 1.0,  0.5,  0.0, -0.5, -1.0],
    ]);

    println!("  Diamond wave pattern:");
    for row in 0..5 {
        print!("  ");
        for col in 0..5 {
            let v = grid.get(row, col).unwrap().value();
            let symbol = if v > 0.7 { "█" } else if v > 0.3 { "▓" } else if v > 0.0 { "▒" }
                         else if v > -0.3 { "░" } else if v > -0.7 { "·" } else { " " };
            print!(" {} ", symbol);
        }
        println!();
    }

    // Bilinear sampling across the grid
    println!();
    println!("  Bilinear samples at fractional coordinates:");
    for (r, c) in [(0.5, 0.5), (1.5, 2.5), (2.0, 2.0), (3.5, 1.5)] {
        let s = grid.sample(r, c);
        println!("    ({:.1}, {:.1}) → {:+.3}", r, c, s.value());
    }

    println!();
    println!("  Average grid value: {:+.3}", grid.average().value());

    let negated = grid.map(|m| -m);
    println!("  Negated grid avg:   {:+.3}", negated.average().value());

    let sum = grid.add_grid(&negated).unwrap();
    println!("  Grid + negated avg: {:+.3} (should be ~0)", sum.average().value());

    println!();
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║     Microtrits: where discrete meets continuous 🎶          ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
}
