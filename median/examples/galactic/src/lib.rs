use median::{
    attr::{AttrBuilder, AttrType},
    builder::MSPWrappedBuilder,
    num::Float64,
    class::Class,
    object::MSPObj,
    wrapper::{
        attr_get_tramp, attr_set_tramp, tramp, MSPObjWrapped, MSPObjWrapper, WrapperWrapped,
    },
};


struct GalacticInner {
    aIL: [f64; 6480],
    aJL: [f64; 3660],
    aKL: [f64; 1720],
    aLL: [f64; 680],

    aAL: [f64; 9700],
    aBL: [f64; 6000],
    aCL: [f64; 2320],
    aDL: [f64; 940],

    aEL: [f64; 15220],
    aFL: [f64; 8460],
    aGL: [f64; 4540],
    aHL: [f64; 3200],

    aML: [f64; 3111],
    aMR: [f64; 3111],
    vibML: f64,
    vibMR: f64,
    depthM: f64,
    oldfpd: f64,

    feedbackAL: f64,
    feedbackBL: f64,
    feedbackCL: f64,
    feedbackDL: f64,

    lastRefL: [f64; 7],
    thunderL: f64,

    iirAR: f64,
    iirBR: f64,

    aIR: [f64; 6480],
    aJR: [f64; 3660],
    aKR: [f64; 1720],
    aLR: [f64; 680],

    aAR: [f64; 9700],
    aBR: [f64; 6000],
    aCR: [f64; 2320],
    aDR: [f64; 940],

    aER: [f64; 15220],
    aFR: [f64; 8460],
    aGR: [f64; 4540],
    aHR: [f64; 3200],
    feedbackAR: f64,
    feedbackBR: f64,
    feedbackCR: f64,
    feedbackDR: f64,

    lastRefR: [f64; 7],
    thunderR: f64,

    countA: i64,
    delayA: i64,
    countB: i64,
    delayB: i64,
    countC: i64,
    delayC: i64,
    countD: i64,
    delayD: i64,
    countE: i64,
    delayE: i64,
    countF: i64,
    delayF: i64,
    countG: i64,
    delayG: i64,
    countH: i64,
    delayH: i64,
    countI: i64,
    delayI: i64,
    countJ: i64,
    delayJ: i64,
    countK: i64,
    delayK: i64,
    countL: i64,
    delayL: i64,
    countM: i64,
    delayM: i64,
    cycle: i64, //all these ints are shared across channels, not duplicated

    vibM: f64,

    fpdL: u32,
    fpdR: u32,
}

impl Default for GalacticInner {
    fn default() -> GalacticInner {
        GalacticInner {
            aIL: [0f64; 6480],
            aJL: [0f64; 3660],
            aKL: [0f64; 1720],
            aLL: [0f64; 680],
            aAL: [0f64; 9700],
            aBL: [0f64; 6000],
            aCL: [0f64; 2320],
            aDL: [0f64; 940],
            aEL: [0f64; 15220],
            aFL: [0f64; 8460],
            aGL: [0f64; 4540],
            aHL: [0f64; 3200],
            aML: [0f64; 3111],
            aMR: [0f64; 3111],
            vibML: 0f64,
            vibMR: 0f64,
            depthM: 0f64,
            oldfpd: 429496.7295f64,
            feedbackAL: 0f64,
            feedbackBL: 0f64,
            feedbackCL: 0f64,
            feedbackDL: 0f64,
            lastRefL: [0f64; 7],
            thunderL: 0f64,
            iirAR: 0f64,
            iirBR: 0f64,
            aIR: [0f64; 6480],
            aJR: [0f64; 3660],
            aKR: [0f64; 1720],
            aLR: [0f64; 680],
            aAR: [0f64; 9700],
            aBR: [0f64; 6000],
            aCR: [0f64; 2320],
            aDR: [0f64; 940],
            aER: [0f64; 15220],
            aFR: [0f64; 8460],
            aGR: [0f64; 4540],
            aHR: [0f64; 3200],
            feedbackAR: 0f64,
            feedbackBR: 0f64,
            feedbackCR: 0f64,
            feedbackDR: 0f64,
            lastRefR: [0f64; 7],
            thunderR: 0f64,
            countA: 1i64,
            delayA: 0i64,
            countB: 1i64,
            delayB: 0i64,
            countC: 1i64,
            delayC: 0i64,
            countD: 1i64,
            delayD: 0i64,
            countE: 1i64,
            delayE: 0i64,
            countF: 1i64,
            delayF: 0i64,
            countG: 1i64,
            delayG: 0i64,
            countH: 1i64,
            delayH: 0i64,
            countI: 1i64,
            delayI: 0i64,
            countJ: 1i64,
            delayJ: 0i64,
            countK: 1i64,
            delayK: 0i64,
            countL: 1i64,
            delayL: 0i64,
            countM: 1i64,
            delayM: 0i64,
            cycle: 0i64, //all these ints are shared across channels, not duplicated
            vibM: 3f64,
            fpdL: 3856986592u32,
            fpdR: 81192u32,
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
        inner: parking_lot::Mutex<GalacticInner>
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
                inner: parking_lot::Mutex::new(GalacticInner::default())
            }
        }

        fn perform(&self, _ins: &[&[f64]], outs: &mut [&mut [f64]], _nframes: usize) {
            for o in outs[0].iter_mut() {
                *o = 0.;
            }
            for o in outs[1].iter_mut() {
                *o = 2.;
            }
    
            {
                let mut g = self.inner.lock();
                // g.aIL[10] = 0.0;
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
