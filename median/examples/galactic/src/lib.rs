use median::{
    attr::{AttrBuilder, AttrType},
    builder::MSPWrappedBuilder,
    class::Class,
    num::Float64,
    wrapper::{attr_get_tramp, attr_set_tramp, MSPObjWrapped, MSPObjWrapper},
};

struct GalacticInner {
    a_il: [f64; 6480],
    a_jl: [f64; 3660],
    a_kl: [f64; 1720],
    a_ll: [f64; 680],

    a_al: [f64; 9700],
    a_bl: [f64; 6000],
    a_cl: [f64; 2320],
    a_dl: [f64; 940],

    a_el: [f64; 15220],
    a_fl: [f64; 8460],
    a_gl: [f64; 4540],
    a_hl: [f64; 3200],

    a_ml: [f64; 3111],
    a_mr: [f64; 3111],
    oldfpd: f64,

    feedback_al: f64,
    feedback_bl: f64,
    feedback_cl: f64,
    feedback_dl: f64,

    last_ref_l: [f64; 7],

    iir_ar: f64,
    iir_br: f64,
    iir_al: f64,
    iir_bl: f64,

    a_ir: [f64; 6480],
    a_jr: [f64; 3660],
    a_kr: [f64; 1720],
    a_lr: [f64; 680],

    a_ar: [f64; 9700],
    a_br: [f64; 6000],
    a_cr: [f64; 2320],
    a_dr: [f64; 940],

    a_er: [f64; 15220],
    a_fr: [f64; 8460],
    a_gr: [f64; 4540],
    a_hr: [f64; 3200],
    feedback_ar: f64,
    feedback_br: f64,
    feedback_cr: f64,
    feedback_dr: f64,

    last_ref_r: [f64; 7],

    count_a: usize,
    count_b: usize,
    count_c: usize,
    count_d: usize,
    count_e: usize,
    count_f: usize,
    count_g: usize,
    count_h: usize,
    count_i: usize,
    count_j: usize,
    count_k: usize,
    count_l: usize,
    count_m: usize,
    cycle: usize, //all these ints are shared across channels, not duplicated

    vib_m: f64,

    fpd_l: u32,
    fpd_r: u32,
}

impl Default for GalacticInner {
    fn default() -> GalacticInner {
        GalacticInner {
            a_il: [0f64; 6480],
            a_jl: [0f64; 3660],
            a_kl: [0f64; 1720],
            a_ll: [0f64; 680],
            a_al: [0f64; 9700],
            a_bl: [0f64; 6000],
            a_cl: [0f64; 2320],
            a_dl: [0f64; 940],
            a_el: [0f64; 15220],
            a_fl: [0f64; 8460],
            a_gl: [0f64; 4540],
            a_hl: [0f64; 3200],
            a_ml: [0f64; 3111],
            a_mr: [0f64; 3111],
            oldfpd: 429496.7295f64,
            feedback_al: 0f64,
            feedback_bl: 0f64,
            feedback_cl: 0f64,
            feedback_dl: 0f64,
            last_ref_l: [0f64; 7],
            iir_al: 0f64,
            iir_bl: 0f64,
            iir_ar: 0f64,
            iir_br: 0f64,
            a_ir: [0f64; 6480],
            a_jr: [0f64; 3660],
            a_kr: [0f64; 1720],
            a_lr: [0f64; 680],
            a_ar: [0f64; 9700],
            a_br: [0f64; 6000],
            a_cr: [0f64; 2320],
            a_dr: [0f64; 940],
            a_er: [0f64; 15220],
            a_fr: [0f64; 8460],
            a_gr: [0f64; 4540],
            a_hr: [0f64; 3200],
            feedback_ar: 0f64,
            feedback_br: 0f64,
            feedback_cr: 0f64,
            feedback_dr: 0f64,
            last_ref_r: [0f64; 7],
            count_a: 1,
            count_b: 1,
            count_c: 1,
            count_d: 1,
            count_e: 1,
            count_f: 1,
            count_g: 1,
            count_h: 1,
            count_i: 1,
            count_j: 1,
            count_k: 1,
            count_l: 1,
            count_m: 1,
            cycle: 0, //all these ints are shared across channels, not duplicated
            vib_m: 3f64,
            fpd_l: 3856986592u32,
            fpd_r: 81192u32,
        }
    }
}

median::external! {
    #[name="galactic~"]
    pub struct Galactic {
        // Attributes
        a: Float64,  // replace
        b: Float64,  // brightness
        c: Float64,  // detune
        d: Float64,  // bigness
        e: Float64,  // dry/wet

        // State
        inner: parking_lot::Mutex<Box<GalacticInner>>,
        sample_rate: Float64
    }

    impl MSPObjWrapped<Galactic> for Galactic {
        // I/O
        fn new(builder: &mut dyn MSPWrappedBuilder<Self>) -> Self {
            builder.add_signal_inlets(2);
            builder.add_signal_outlets(2);
            Self {
                // Attributes
                a: Float64::new(0.5),
                b: Float64::new(0.5),
                c: Float64::new(0.5),
                d: Float64::new(1.0),
                e: Float64::new(1.0),

                // State
                inner: parking_lot::Mutex::new(Box::new(GalacticInner::default())),
                sample_rate: Float64::new(44100.0)
            }
        }

        fn dsp_setup(&self, sample_rate: f64) {
            self.sample_rate.set(sample_rate)
        }

        fn perform(&self, _ins: &[&[f64]], outs: &mut [&mut [f64]], _nframes: usize) {
            let mut g = self.inner.lock();
            let sr = self.sample_rate.get();

            let mut overallscale = 1.0;
            overallscale /= 44100.0;
            overallscale *= sr;

            // this is going to be 2 for 88.1 or 96k, 3 for silly people
            // 4 for 176 or 192k
            let mut cycle_end = overallscale.floor() as usize;
            if cycle_end < 1 {
                cycle_end = 1;
            }
            if cycle_end > 4 {
                cycle_end = 4;
            }
            if g.cycle > cycle_end - 1 {
                g.cycle = cycle_end -1;  // sanity check
            }

            let regen = 0.0625 + ((1.0 - self.a.get()) * 0.0625);
            let attenuate = (1.0 - (regen / 0.125)) * 1.333;
            let lowpass = (1.00001 - (1.0 - self.b.get())).powi(2) / overallscale.sqrt();
            let drift = self.c.get().powi(3) * 0.001;
            let size = (self.d.get() * 1.77) + 0.1;
            let wet = 1.0 - (1.0 - self.e.get()).powi(3);

            let delay_a = (4801.0 * size) as usize;
            let delay_b = (2909.0 * size) as usize;
            let delay_c = (1153.0 * size) as usize;
            let delay_d = (461.0 * size) as usize;
            let delay_e = (7607.0 * size) as usize;
            let delay_f = (4217.0 * size) as usize;
            let delay_g = (2269.0 * size) as usize;
            let delay_h = (1597.0 * size) as usize;
            let delay_i = (3407.0 * size) as usize;
            let delay_j = (1823.0 * size) as usize;
            let delay_k = (859.0 * size) as usize;
            let delay_l = (331.0 * size) as usize;
            let delay_m = 256 as usize;

            let mut counter = 0;
            while counter < _nframes {
                //NOTE was 'long double'
                let mut input_sample_l = _ins[0][counter];
                let mut input_sample_r = _ins[1][counter];

                let eps = 1.18e-43f64;
                if input_sample_l.abs() < eps {
                    input_sample_l = g.fpd_l as f64 * eps;
                }
                if input_sample_r.abs() < eps {
                    input_sample_r = g.fpd_r as f64 * eps;
                }

                //NOTE was 'long double'
                let dry_sample_l = input_sample_l;
                let dry_sample_r = input_sample_r;

                g.vib_m += g.oldfpd * drift;
                if g.vib_m > 3.141592653589793238 * 2.0 {
                    g.vib_m = 0.0;
                    g.oldfpd = 0.4294967295 + (g.fpd_l as f64 * 0.0000000000618);
                }

                let i = g.count_m;
                g.a_ml[i] = input_sample_l * attenuate;
                g.a_mr[i] = input_sample_r * attenuate;
                g.count_m += 1;

                if g.count_m > delay_m {
                    g.count_m = 0;
                }

                // == begin: pre-delay + vibrato
                // Compute interpol_m
                let interpol_m_comp = |offset: f64, work: usize, del: usize, arr: [f64; 3111]| {
                    let mut w = work;
                    let mut x = 0;

                    if w > del {
                        x = del + 1;
                    }
                    w -= x;

                    let a = arr[w];
                    let b = 1.0 - offset.fract();
                    x = 0;

                    if (w + 1) > del {
                        x = del + 1;
                    }
                    let c = arr[w + 1 - x];
                    let d = offset - offset.floor();

                    a * b + c * d
                };

                let offset_ml = (g.vib_m.sin() + 1.0) * 127.0;
                let offset_mr = ((g.vib_m + (3.141592653589793238 / 2.0)).sin() + 1.0) * 127.0;
                let working_ml = g.count_m + offset_ml as usize;
                let working_mr = g.count_m + offset_mr as usize;
                input_sample_l = interpol_m_comp(offset_ml, working_ml, delay_m, g.a_ml);
                input_sample_r = interpol_m_comp(offset_mr, working_mr, delay_m, g.a_mr);
                // == end: pre-delay + vibrato

                let filter = |iir: &mut f64, eps: f64, input_sample: f64| {
                    if iir.abs() < eps {
                        *iir = 0.0;
                    }
                    *iir = *iir * (1.0 - lowpass) + input_sample * lowpass;
                    *iir
                };
                input_sample_l = filter(&mut g.iir_al, 1.18e-37f64, input_sample_l);
                input_sample_r = filter(&mut g.iir_ar, 1.18e-37f64, input_sample_r);
                // initial filter

                g.cycle += 1;
                if g.cycle == cycle_end {  // hit the end point and do a reverb sample
                    let fdbk = |index: usize, input: f64, fdbk: f64, v: &mut [f64]| {
                        v[index] = input + fdbk * regen;
                    };

                    fdbk(g.count_i, input_sample_l, g.feedback_ar, &mut g.a_il);
                    fdbk(g.count_j, input_sample_l, g.feedback_br, &mut g.a_jl);
                    fdbk(g.count_k, input_sample_l, g.feedback_cr, &mut g.a_kl);
                    fdbk(g.count_l, input_sample_l, g.feedback_dr, &mut g.a_ll);
                    fdbk(g.count_i, input_sample_l, g.feedback_al, &mut g.a_ir);
                    fdbk(g.count_j, input_sample_r, g.feedback_bl, &mut g.a_jl);
                    fdbk(g.count_k, input_sample_r, g.feedback_cl, &mut g.a_kr);
                    fdbk(g.count_l, input_sample_r, g.feedback_dl, &mut g.a_lr);

                    let wrap_count = |count: &mut usize, delay: usize| {
                        *count += 1;
                        if *count > delay {
                            *count = 0;
                        }
                    };
                    wrap_count(&mut g.count_i, delay_i);
                    wrap_count(&mut g.count_j, delay_j);
                    wrap_count(&mut g.count_k, delay_k);
                    wrap_count(&mut g.count_l, delay_l);

                    // == begin: apply delays
                    let mut i = g.count_i;
                    if g.count_i > delay_i {
                        i -= (delay_i) + 1;
                    }
                    let out_il = g.a_il[i];

                    i = g.count_j;
                    if g.count_j > delay_j {
                        i -= (delay_j) + 1;
                    }
                    let out_jl = g.a_jl[i];

                    i = g.count_k;
                    if g.count_k > delay_k {
                        i -= (delay_k) + 1;
                    }
                    let out_kl = g.a_kl[i];
                    
                    i = g.count_l;
                    if g.count_l > delay_l {
                        i -= (delay_l) + 1;
                    }
                    let out_ll = g.a_jl[i];

                    i = g.count_i;
                    if g.count_i > delay_i {
                        i -= (delay_i) + 1;
                    }
                    let out_ir = g.a_ir[i];

                    i = g.count_j;
                    if g.count_j > delay_j {
                        i -= (delay_j) + 1;
                    }
                    let out_jr = g.a_jr[i];

                    i = g.count_k;
                    if g.count_k > delay_k {
                        i -= (delay_k) + 1;
                    }
                    let out_kr = g.a_kr[i];

                    i = g.count_l;
                    if g.count_l > delay_l {
                        i -= (delay_l) + 1;
                    }
                    let out_lr = g.a_lr[i];
                    // first block: now we have four outputs

                    let x = g.count_a;
                    g.a_al[x] = out_il - (out_jl + out_kl + out_ll);
                    g.a_ar[x] = out_ir - (out_jr + out_kr + out_lr);
                    let x = g.count_b;
                    g.a_bl[x] = out_jl - (out_il + out_kl + out_ll);
                    g.a_br[x] = out_jr - (out_ir + out_kr + out_lr);
                    let x = g.count_c;
                    g.a_cl[x] = out_kl - (out_il + out_jl + out_ll);
                    g.a_cr[x] = out_kr - (out_ir + out_jr + out_lr);
                    let x = g.count_d;
                    g.a_dl[x] = out_ll - (out_il + out_jl + out_kl);
                    g.a_dr[x] = out_lr - (out_ir + out_jr + out_kr);

                    wrap_count(&mut g.count_a, delay_a);
                    wrap_count(&mut g.count_b, delay_b);
                    wrap_count(&mut g.count_c, delay_c);
                    wrap_count(&mut g.count_d, delay_d);

                    let mut i = g.count_a;
                    if g.count_a > delay_a {
                        i -= (delay_a) + 1;
                    }
                    let out_al = g.a_al[i];
                    
                    i = g.count_b;
                    if g.count_b > delay_b {
                        i -= (delay_b) + 1;
                    }
                    let out_bl = g.a_bl[i];

                    i = g.count_c;
                    if g.count_c > delay_c {
                        i -= (delay_c) + 1;
                    }
                    let out_cl = g.a_cl[i];

                    i = g.count_d;
                    if g.count_d > delay_d {
                        i -= (delay_d) + 1;
                    }
                    let out_dl = g.a_dl[i];

                    i = g.count_a;
                    if g.count_a > delay_a {
                        i -= (delay_a) + 1;
                    }
                    let out_ar = g.a_ar[i];

                    i = g.count_b;
                    if g.count_b > delay_b {
                        i -= (delay_b) + 1;
                    }
                    let out_br = g.a_br[i];

                    i = g.count_c;
                    if g.count_c > delay_c {
                        i -= (delay_c) + 1;
                    }
                    let out_cr = g.a_cr[i];

                    i = g.count_d;
                    if g.count_d > delay_d {
                        i -= (delay_d) + 1;
                    }
                    let out_dr = g.a_dr[i];
                    // second block: now we have four more outputs

                    let x = g.count_e;
                    g.a_el[x] = out_al - (out_bl + out_cl + out_dl);
                    g.a_er[x] = out_ar - (out_br + out_cr + out_dr);
                    let x = g.count_f;
                    g.a_fl[x] = out_bl - (out_al + out_cl + out_dl);
                    g.a_fr[x] = out_br - (out_ar + out_cr + out_dr);
                    let x = g.count_g;
                    g.a_gl[x] = out_cl - (out_al + out_bl + out_dl);
                    g.a_gr[x] = out_cr - (out_ar + out_br + out_dr);
                    let x = g.count_h;
                    g.a_hl[x] = out_dl - (out_al + out_bl + out_cl);
                    g.a_hr[x] = out_dr - (out_ar + out_br + out_cr);

                    wrap_count(&mut g.count_e, delay_e);
                    wrap_count(&mut g.count_f, delay_f);
                    wrap_count(&mut g.count_g, delay_g);
                    wrap_count(&mut g.count_h, delay_h);

                    let mut i = g.count_e;
                    if g.count_e > delay_e {
                        i -= (delay_e) + 1;
                    }
                    let out_el = g.a_el[i];
                    i = g.count_f;
                    if g.count_f > delay_f {
                        i -= (delay_f) + 1;
                    }
                    let out_fl = g.a_fl[i];

                    i = g.count_g;
                    if g.count_g > delay_g {
                        i -= (delay_g) + 1;
                    }
                    let out_gl = g.a_gl[i];
                    i = g.count_h;
                    if g.count_h > delay_h {
                        i -= (delay_h) + 1;
                    }
                    let out_hl = g.a_hl[i];
                    i = g.count_e;
                    if g.count_e > delay_e {
                        i -= (delay_e) + 1;
                    }
                    let out_er = g.a_er[i];
                    i = g.count_f;
                    if g.count_f > delay_f {
                        i -= (delay_f) + 1;
                    }
                    let out_fr = g.a_fr[i];
                    i = g.count_g;
                    if g.count_g > delay_g {
                        i -= (delay_g) + 1;
                    }
                    let out_gr = g.a_gr[i];
                    i = g.count_h;
                    if g.count_h > delay_h {
                        i -= (delay_h) + 1;
                    }
                    let out_hr = g.a_hr[i];
                    // third block: final outputs

                    g.feedback_al = out_el - (out_fl + out_gl + out_hl);
                    g.feedback_bl = out_fl - (out_el + out_gl + out_hl);
                    g.feedback_cl = out_gl - (out_el + out_fl + out_hl);
                    g.feedback_dl = out_hl - (out_el + out_fl + out_gl);
                    g.feedback_ar = out_er - (out_fr + out_gr + out_hr);
                    g.feedback_br = out_fr - (out_er + out_gr + out_hr);
                    g.feedback_cr = out_gr - (out_er + out_fr + out_hr);
                    g.feedback_dr = out_hr - (out_er + out_fr + out_gr);
                    // feed back into the input again a bit

                    input_sample_l = (out_el + out_fl + out_gl + out_hl) / 8.0;
                    input_sample_r = (out_er + out_fr + out_gr + out_hr) / 8.0;
                    // take the final combined sum

                    match cycle_end {
                        4 => {
                            let comp = |v: &mut [f64], input_sample: f64| {
                                v[0] = v[4];  // start from previous last
                                v[2] = (v[0] + input_sample) / 2.0;  // half
                                v[1] = (v[0] + v[2]) / 2.0;  // one quarter
                                v[3] = (v[2] + input_sample) / 2.0;  // three quarters
                                v[4] = input_sample;  // full
                            };
                            comp(&mut g.last_ref_l, input_sample_l);
                            comp(&mut g.last_ref_r, input_sample_r);
                        }

                        3 => {
                            let comp = |v: &mut [f64], input_sample: f64| {
                                v[0] = v[3];  // start from previous last
                                v[2] = (v[0] + v[0] + input_sample) / 3.0;  // third
                                v[1] = (v[0] + input_sample + input_sample) / 3.0;  // two thirds
                                v[3] = input_sample;  // full
                            };
                            comp(&mut g.last_ref_l, input_sample_l);
                            comp(&mut g.last_ref_r, input_sample_r);
                        }

                        2 => {
                            let comp = |v: &mut [f64], input_sample: f64| {
                                v[0] = v[2];  // start from previous last
                                v[1] = (v[0] + input_sample) / 2.0;  // half
                                v[2] = input_sample;  // full
                            };
                            comp(&mut g.last_ref_l, input_sample_l);
                            comp(&mut g.last_ref_r, input_sample_r);
                        }

                        _ => ()
                    }
                    g.cycle = 0; //reset
                } else {
                    let i = g.cycle;
                    input_sample_l = g.last_ref_l[i];
                    input_sample_r = g.last_ref_r[i];
                }
                // end feedback

                input_sample_l = filter(&mut g.iir_bl, 1.18e-37f64, input_sample_l);
                input_sample_r = filter(&mut g.iir_br, 1.18e-37f64, input_sample_r);
                // end filter

                // dry/wet
                if wet < 1.0 {
                    input_sample_l = input_sample_l * wet + dry_sample_l * (1.0 - wet);
                    input_sample_r = input_sample_r * wet + dry_sample_r * (1.0 - wet);
                }

                // todo: 64 bit stereo floating point dither

                outs[0][counter] = input_sample_l;
                outs[1][counter] = input_sample_r;
                counter += 1;
            }
        }

        // Register attributes
        fn class_setup(c: &mut Class<MSPObjWrapper<Self>>) {
            c.add_attribute(
                AttrBuilder::new_accessors(
                    "replace",
                    AttrType::Float32,
                    Self::replace_tramp,
                    Self::set_replace_tramp,
                )
                .build()
                .unwrap(),
            )
                .expect("failed to add attribute");
            c.add_attribute(
                AttrBuilder::new_accessors(
                    "brightness",
                    AttrType::Float32,
                    Self::brightness_tramp,
                    Self::set_brightness_tramp,
                )
                .build()
                .unwrap(),
            )
                .expect("failed to add attribute");
            c.add_attribute(
                AttrBuilder::new_accessors(
                    "detune",
                    AttrType::Float32,
                    Self::detune_tramp,
                    Self::set_detune_tramp,
                )
                .build()
                .unwrap(),
            )
                .expect("failed to add attribute");
            c.add_attribute(
                AttrBuilder::new_accessors(
                    "bigness",
                    AttrType::Float32,
                    Self::bigness_tramp,
                    Self::set_bigness_tramp,
                )
                .build()
                .unwrap(),
            )
                .expect("failed to add attribute");

            c.add_attribute(
                AttrBuilder::new_accessors(
                    "drywet",
                    AttrType::Float32,
                    Self::drywet_tramp,
                    Self::set_drywet_tramp,
                )
                .build()
                .unwrap(),
            )
                .expect("failed to add attribute");
            }
    }

    impl Galactic {
        #[attr_get_tramp]
        pub fn replace(&self) -> f64 {
            self.a.get().clamp(0.0, 1.0)
        }

        #[attr_set_tramp]
        pub fn set_replace(&self, v: f64) {
            self.a.set(v.clamp(0.0, 1.0))
        }

        #[attr_get_tramp]
        pub fn brightness(&self) -> f64 {
            self.b.get().clamp(0.0, 1.0)
        }

        #[attr_set_tramp]
        pub fn set_brightness(&self, v: f64) {
            self.b.set(v.clamp(0.0, 1.0))
        }

        #[attr_get_tramp]
        pub fn detune(&self) -> f64 {
            self.c.get().clamp(0.0, 1.0)
        }

        #[attr_set_tramp]
        pub fn set_detune(&self, v: f64) {
            self.c.set(v.clamp(0.0, 1.0))
        }

        #[attr_get_tramp]
        pub fn bigness(&self) -> f64 {
            self.d.get().clamp(0.0, 1.0)
        }

        #[attr_set_tramp]
        pub fn set_bigness(&self, v: f64) {
            self.d.set(v.clamp(0.0, 1.0))
        }
        #[attr_get_tramp]
        pub fn drywet(&self) -> f64 {
            self.e.get().clamp(0.0, 1.0)
        }

        #[attr_set_tramp]
        pub fn set_drywet(&self, v: f64) {
            self.e.set(v.clamp(0.0, 1.0))
        }
    }
}
