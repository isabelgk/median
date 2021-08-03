use median::{
    attr::{AttrBuilder, AttrType},
    builder::MSPWrappedBuilder,
    class::Class,
    num::Float64,
    wrapper::{
        attr_get_tramp, attr_set_tramp, MSPObjWrapped, MSPObjWrapper, WrapperWrapped,
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
    iirAL: f64,
    iirBL: f64,

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
            iirAL: 0f64,
            iirBL: 0f64,
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
            let mut cycle_end = overallscale.floor() as i64;
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
            let wet = 0.9999;

            g.delayI = (3407.0 * size) as i64;
            g.delayJ = (1823.0 * size) as i64;
            g.delayK = (859.0 * size) as i64;
            g.delayL = (331.0 * size) as i64;
            g.delayA = (4801.0 * size) as i64;
            g.delayB = (2909.0 * size) as i64;
            g.delayC = (1153.0 * size) as i64;
            g.delayD = (461.0 * size) as i64;
            g.delayE = (7607.0 * size) as i64;
            g.delayF = (4217.0 * size) as i64;
            g.delayG = (2269.0 * size) as i64;
            g.delayH = (1597.0 * size) as i64;
            g.delayM = 256;
            let mut counter = 0;
            while counter < _nframes {
                let mut inputSampleL = _ins[0][counter];
                let mut inputSampleR = _ins[1][counter];

                let eps = 1.18f64.powi(-43);
                if inputSampleL.abs() < eps {
                    inputSampleL = g.fpdL as f64 * eps;
                }
                if inputSampleR.abs() < eps {
                    inputSampleR = g.fpdR as f64 * eps;
                }

                let drySampleL = inputSampleL;
                let drySampleR = inputSampleR;

                g.vibM += g.oldfpd * drift;
                if g.vibM > 3.141592653589793238 * 2.0 {
                    g.vibM = 0.0;
                    g.oldfpd = 0.4294967295 + (g.fpdL as f64 * 0.0000000000618);
                }

                let i = g.countM as usize;
                g.aML[i] = inputSampleL * attenuate;
                g.aMR[i] = inputSampleR * attenuate;
                g.countM += 1;

                if g.countM < 0 || g.countM > g.delayM {
                    g.countM = 0;
                }

                // left
                let offsetML = (g.vibM.sin() + 1.0) * 127.0;
                let workingML = g.countM + offsetML as i64;
                let mut workingML = workingML as usize;
                // Calculate interpolML in parts
                // aML[workingML-( (workingML > delayM) ? delayM + 1 : 0 )]
                //         * ( 1 - (offsetML - floor(offsetML)) )
                //     + aML[workingML + 1 - ( (workingML + 1 > delayM) ? delayM + 1 : 0 )]
                //         * ( offsetML - floor(offsetML) );
                // call it: a * b + c * d;
                let mut interpolML = 0f64;
                {
                    // a = aML[workingML-( (workingML > delayM) ? delayM + 1 : 0 )]
                    let mut x = 0i64;
                    if workingML as i64 > g.delayM {
                        x = g.delayM + 1;
                    }
                    workingML -= x as usize;
                    let a = g.aML[workingML];
                    // b = 1 - offsetML - floor(offsetML)
                    let b = 1.0 - offsetML - offsetML.floor();
                    // c = aML[workingML + 1 - ( (workingML + 1 > delayM) ? delayM + 1 : 0 )]
                    x = 0;
                    if (workingML + 1) as i64 > g.delayM {
                        x = g.delayM + 1;
                    }
                    let c = g.aML[workingML + 1 - x as usize];
                    // d = offsetML - floor(offsetML)
                    let d = offsetML - offsetML.floor();

                    interpolML = a * b + c * d;
                }
                inputSampleL = interpolML;

                // Do the same on the right with a shifted offsetMR
                let offsetMR = ((g.vibM + (3.141592653589793238 / 2.0)).sin() + 1.0) * 127.0;
                let workingMR = g.countM + offsetMR as i64;
                let mut workingMR = workingMR as usize;
                let mut interpolMR = 0f64;
                {
                    // a = aML[workingML-( (workingML > delayM) ? delayM + 1 : 0 )]
                    let mut x = 0i64;
                    if workingMR as i64 > g.delayM {
                        x = g.delayM + 1;
                    }
                    workingMR -= x as usize;
                    let a = g.aMR[workingMR];
                    // b = 1 - offsetMR - floor(offsetMR)
                    let b = 1.0 - offsetMR - offsetMR.floor();
                    // c = aMR[workingMR + 1 - ( (workingMR + 1 > delayM) ? delayM + 1 : 0 )]
                    x = 0;
                    if (workingMR + 1) as i64 > g.delayM {
                        x = g.delayM + 1;
                    }
                    let c = g.aMR[workingMR + 1 - x as usize];
                    // d = offsetMR - floor(offsetMR)
                    let d = offsetMR - offsetMR.floor();

                    interpolMR = a * b + c * d;
                }
                inputSampleR = interpolMR;
                // Pre-delay + vibrato

                if g.iirAL.abs() < eps {
                    g.iirAL = 0.0;
                }
                g.iirAL = g.iirAL * (1.0 - lowpass) + inputSampleL * lowpass;
                inputSampleL = g.iirAL;

                if g.iirAR.abs() < eps {
                    g.iirAR = 0.0;
                }
                g.iirAR = g.iirAR * (1.0 - lowpass) + inputSampleR * lowpass;
                inputSampleR = g.iirAR;
                // Initial filter

                g.cycle += 1;
                if g.cycle == cycle_end {  // hit the end point and do a reverb sample
                    let mut i = g.countI as usize;
                    g.aIL[i] = inputSampleL + (g.feedbackAR * regen);
                    i = g.countJ as usize;
                    g.aJL[i] = inputSampleL + (g.feedbackBR * regen);
                    i = g.countK as usize;
                    g.aKL[i] = inputSampleL + (g.feedbackCR * regen);
                    i = g.countL as usize;
                    g.aLL[i] = inputSampleL + (g.feedbackDR * regen);
                    i = g.countI as usize;
                    g.aIR[i] = inputSampleR + (g.feedbackAL * regen);
                    i = g.countJ as usize;
                    g.aJR[i] = inputSampleR + (g.feedbackBL * regen);
                    i = g.countK as usize;
                    g.aKR[i] = inputSampleR + (g.feedbackCL * regen);
                    i = g.countL as usize;
                    g.aLR[i] = inputSampleR + (g.feedbackDL * regen);

                    g.countI += 1;
                    if g.countI < 0 || g.countI > g.delayI {
                        g.countI = 0;
                    }
                    g.countJ += 1;
                    if g.countJ < 0 || g.countJ > g.delayJ {
                        g.countJ = 0;
                    }
                    g.countK += 1;
                    if g.countK < 0 || g.countK > g.delayK {
                        g.countK = 0;
                    }
                    g.countL += 1;
                    if g.countL < 0 || g.countL > g.delayL {
                        g.countL = 0;
                    }

                    let mut i = g.countI as usize;
                    if g.countI > g.delayI {
                        i -= (g.delayI as usize) + 1;
                    }
                    let outIL = g.aIL[i];

                    i = g.countJ as usize;
                    if g.countJ > g.delayJ {
                        i -= (g.delayJ as usize) + 1;
                    }
                    let outJL = g.aJL[i];

                    i = g.countK as usize;
                    if g.countK > g.delayK {
                        i -= (g.delayK as usize) + 1;
                    }
                    let outKL = g.aKL[i];
                    i = g.countL as usize;
                    if g.countL > g.delayL {
                        i -= (g.delayL as usize) + 1;
                    }
                    let outLL = g.aJL[i];
                    i = g.countI as usize;
                    if g.countI > g.delayI {
                        i -= (g.delayI as usize) + 1;
                    }
                    let outIR = g.aIR[i];
                    i = g.countJ as usize;
                    if g.countJ > g.delayJ {
                        i -= (g.delayJ as usize) + 1;
                    }
                    let outJR = g.aJR[i];
                    i = g.countK as usize;
                    if g.countK > g.delayK {
                        i -= (g.delayK as usize) + 1;
                    }
                    let outKR = g.aKR[i];
                    i = g.countL as usize;
                    if g.countL > g.delayL {
                        i -= (g.delayL as usize) + 1;
                    }
                    let outLR = g.aLR[i];
                    // first block: now we have four outputs

                    let x = g.countA as usize;
                    g.aAL[x] = outIL - (outJL + outKL + outLL);
                    g.aAR[x] = outIR - (outJR + outKR + outLR);
                    let x = g.countB as usize;
                    g.aBL[x] = outJL - (outIL + outKL + outLL);
                    g.aBR[x] = outJR - (outIR + outKR + outLR);
                    let x = g.countC as usize;
                    g.aCL[x] = outKL - (outIL + outJL + outLL);
                    g.aCR[x] = outKR - (outIR + outJR + outLR);
                    let x = g.countD as usize;
                    g.aDL[x] = outLL - (outIL + outJL + outKL);
                    g.aDR[x] = outLR - (outIR + outJR + outKR);
                    g.countA += 1;
                    if g.countA < 0 || g.countA > g.delayA {
                        g.countA = 0;
                    }
                    g.countB += 1;
                    if g.countB < 0 || g.countB > g.delayB {
                        g.countB = 0;
                    }
                    g.countC += 1;
                    if g.countC < 0 || g.countC > g.delayC {
                        g.countC = 0;
                    }
                    g.countD += 1;
                    if g.countD < 0 || g.countD > g.delayD {
                        g.countD = 0;
                    }

                    let mut i = g.countA as usize;
                    if g.countA > g.delayA {
                        i -= (g.delayA as usize) + 1;
                    }
                    let outAL = g.aAL[i];
                    i = g.countB as usize;
                    if g.countB > g.delayB {
                        i -= (g.delayB as usize) + 1;
                    }
                    let outBL = g.aBL[i];

                    i = g.countC as usize;
                    if g.countC > g.delayC {
                        i -= (g.delayC as usize) + 1;
                    }
                    let outCL = g.aCL[i];
                    i = g.countD as usize;
                    if g.countD > g.delayD {
                        i -= (g.delayD as usize) + 1;
                    }
                    let outDL = g.aDL[i];
                    i = g.countA as usize;
                    if g.countA > g.delayA {
                        i -= (g.delayA as usize) + 1;
                    }
                    let outAR = g.aAR[i];
                    i = g.countB as usize;
                    if g.countB > g.delayB {
                        i -= (g.delayB as usize) + 1;
                    }
                    let outBR = g.aBR[i];
                    i = g.countC as usize;
                    if g.countC > g.delayC {
                        i -= (g.delayC as usize) + 1;
                    }
                    let outCR = g.aCR[i];
                    i = g.countD as usize;
                    if g.countD > g.delayD {
                        i -= (g.delayD as usize) + 1;
                    }
                    let outDR = g.aDR[i];
                    // second block: now we have four more outputs

                    let x = g.countE as usize;
                    g.aEL[x] = outAL - (outBL + outCL + outDL);
                    g.aER[x] = outAR - (outBR + outCR + outDR);
                    let x = g.countF as usize;
                    g.aFL[x] = outBL - (outAL + outCL + outDL);
                    g.aFR[x] = outBR - (outAR + outCR + outDR);
                    let x = g.countG as usize;
                    g.aGL[x] = outCL - (outAL + outBL + outDL);
                    g.aGR[x] = outCR - (outAR + outBR + outDR);
                    let x = g.countH as usize;
                    g.aHL[x] = outDL - (outAL + outBL + outCL);
                    g.aHR[x] = outDR - (outAR + outBR + outCR);
                    g.countE += 1;
                    if g.countE < 0 || g.countE > g.delayE {
                        g.countE = 0;
                    }
                    g.countF += 1;
                    if g.countF < 0 || g.countF > g.delayF {
                        g.countF = 0;
                    }
                    g.countG += 1;
                    if g.countG < 0 || g.countG > g.delayG {
                        g.countG = 0;
                    }
                    g.countH += 1;
                    if g.countH < 0 || g.countH > g.delayH {
                        g.countH = 0;
                    }

                    let mut i = g.countE as usize;
                    if g.countE > g.delayE {
                        i -= (g.delayE as usize) + 1;
                    }
                    let outEL = g.aEL[i];
                    i = g.countF as usize;
                    if g.countF > g.delayF {
                        i -= (g.delayF as usize) + 1;
                    }
                    let outFL = g.aFL[i];

                    i = g.countG as usize;
                    if g.countG > g.delayG {
                        i -= (g.delayG as usize) + 1;
                    }
                    let outGL = g.aGL[i];
                    i = g.countH as usize;
                    if g.countH > g.delayH {
                        i -= (g.delayH as usize) + 1;
                    }
                    let outHL = g.aHL[i];
                    i = g.countE as usize;
                    if g.countE > g.delayE {
                        i -= (g.delayE as usize) + 1;
                    }
                    let outER = g.aER[i];
                    i = g.countF as usize;
                    if g.countF > g.delayF {
                        i -= (g.delayF as usize) + 1;
                    }
                    let outFR = g.aFR[i];
                    i = g.countG as usize;
                    if g.countG > g.delayG {
                        i -= (g.delayG as usize) + 1;
                    }
                    let outGR = g.aGR[i];
                    i = g.countH as usize;
                    if g.countH > g.delayH {
                        i -= (g.delayH as usize) + 1;
                    }
                    let outHR = g.aHR[i];
                    // third block: final outputs

                    g.feedbackAL = outEL - (outFL + outGL + outHL);
                    g.feedbackBL = outFL - (outEL + outGL + outHL);
                    g.feedbackCL = outGL - (outEL + outFL + outHL);
                    g.feedbackDL = outHL - (outEL + outFL + outGL);
                    g.feedbackAR = outER - (outFR + outGR + outHR);
                    g.feedbackBR = outFR - (outER + outGR + outHR);
                    g.feedbackCR = outGR - (outER + outFR + outHR);
                    g.feedbackDR = outHR - (outER + outFR + outGR);
                    // feed back into the input again a bit

                    inputSampleL = (outEL + outFL + outGL + outHL) / 8.0;
                    inputSampleR = (outER + outFR + outGR + outHR) / 8.0;
                    // take the final combined sum

                    if cycle_end == 4 {
                        g.lastRefL[0] = g.lastRefL[4]; //start from previous last
                        g.lastRefL[2] = (g.lastRefL[0] + inputSampleL) / 2.0; //half
                        g.lastRefL[1] = (g.lastRefL[0] + g.lastRefL[2]) / 2.0; //one quarter
                        g.lastRefL[3] = (g.lastRefL[2] + inputSampleL) / 2.0; //three quarters
                        g.lastRefL[4] = inputSampleL; //full
                        g.lastRefR[0] = g.lastRefR[4]; //start from previous last
                        g.lastRefR[2] = (g.lastRefR[0] + inputSampleR) / 2.0; //half
                        g.lastRefR[1] = (g.lastRefR[0] + g.lastRefR[2]) / 2.0; //one quarter
                        g.lastRefR[3] = (g.lastRefR[2] + inputSampleR) / 2.0; //three quarters
                        g.lastRefR[4] = inputSampleR; //full
                    }
                    if cycle_end == 3 {
                        g.lastRefL[0] = g.lastRefL[3]; //start from previous last
                        g.lastRefL[2] = (g.lastRefL[0]+g.lastRefL[0]+inputSampleL) / 3.0; //third
                        g.lastRefL[1] = (g.lastRefL[0]+inputSampleL+inputSampleL) / 3.0; //two thirds
                        g.lastRefL[3] = inputSampleL; //full
                        g.lastRefR[0] = g.lastRefR[3]; //start from previous last
                        g.lastRefR[2] = (g.lastRefR[0]+g.lastRefR[0]+inputSampleR) / 3.0; //third
                        g.lastRefR[1] = (g.lastRefR[0]+inputSampleR+inputSampleR) / 3.0; //two thirds
                        g.lastRefR[3] = inputSampleR; //full
                    }
                    if cycle_end == 2 {
                        g.lastRefL[0] = g.lastRefL[2]; //start from previous last
                        g.lastRefL[1] = (g.lastRefL[0] + inputSampleL) / 2.0; //half
                        g.lastRefL[2] = inputSampleL; //full
                        g.lastRefR[0] = g.lastRefR[2]; //start from previous last
                        g.lastRefR[1] = (g.lastRefR[0] + inputSampleR) / 2.0; //half
                        g.lastRefR[2] = inputSampleR; //full
                    }
                    g.cycle = 0; //reset
                } else {
                    let i = g.cycle as usize;
                    inputSampleL = g.lastRefL[i];
                    inputSampleR = g.lastRefR[i];
                }
                // end feedback

                if g.iirBL < eps {
                    g.iirBL = 0.0;
                }
                g.iirBL = g.iirBL * (1.0 - lowpass) + inputSampleL * lowpass;
                inputSampleL = g.iirBL;

                if g.iirBR < eps {
                    g.iirBR = 0.0;
                }
                g.iirBR = g.iirBR * (1.0 - lowpass) + inputSampleR * lowpass;
                inputSampleR = g.iirBR;
                // end filter

                // dry/wet
                if wet < 1.0 {
                    inputSampleL = inputSampleL * wet + drySampleL * (1.0 - wet);
                    inputSampleR = inputSampleR * wet + drySampleR * (1.0 - wet);
                }

                // todo: 64 bit stereo floating point dither

                outs[0][counter] = inputSampleL;
                outs[1][counter] = inputSampleR;
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
