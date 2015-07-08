use distribution;
use random;

/// A Gaussian distribution.
#[derive(Clone, Copy)]
pub struct Gaussian {
    mu: f64,
    sigma: f64,
}

impl Gaussian {
    /// Create a Gaussian distribution with mean `mu` and standard deviation
    /// `sigma`.
    ///
    /// It should hold that `sigma > 0`.
    #[inline]
    pub fn new(mu: f64, sigma: f64) -> Gaussian {
        should!(sigma > 0.0);
        Gaussian { mu: mu, sigma: sigma }
    }

    /// Return the mean.
    #[inline(always)]
    pub fn mu(&self) -> f64 { self.mu }

    /// Return the standard deviation.
    #[inline(always)]
    pub fn sigma(&self) -> f64 { self.sigma }
}

impl distribution::Distribution for Gaussian {
    type Value = f64;

    #[inline]
    fn cdf(&self, x: f64) -> f64 {
        use special::erf;
        use std::f64::consts::SQRT_2;
        (1.0 + erf((x - self.mu) / (self.sigma * SQRT_2))) / 2.0
    }

    #[inline]
    fn mean(&self) -> f64 { self.mu }

    #[inline]
    fn var(&self) -> f64 {
        self.sigma * self.sigma
    }

    #[inline]
    fn sd(&self) -> f64 { self.sigma }
}

impl distribution::Continuous for Gaussian {
    #[inline]
    fn pdf(&self, x: f64) -> f64 {
        use distribution::Distribution;
        use std::f64::consts::PI;
        (-(x - self.mu).powi(2) / (2.0 * self.var())).exp() / ((2.0 * PI).sqrt() * self.sigma)
    }
}

impl distribution::Entropy for Gaussian {
    #[inline]
    fn entropy(&self) -> f64 {
        use distribution::Distribution;
        use std::f64::consts::{E, PI};
        0.5 * (2.0 * PI * E * self.var()).ln()
    }
}

impl distribution::Inverse for Gaussian {
    /// Compute the inverse of the cumulative distribution function.
    ///
    /// ## References
    ///
    /// 1. M. J. Wichura, “Algorithm as 241: The percentage points of the normal
    ///    distribution,” Journal of the Royal Statistical Society. Series C
    ///    (Applied Statistics), vol. 37, no. 3, pp. pp. 477–484, 1988.
    ///
    /// 2. http://people.sc.fsu.edu/~jburkardt/c_src/asa241/asa241.html
    #[inline(always)]
    fn inv_cdf(&self, p: f64) -> f64 {
        self.mu + self.sigma * inv_cdf(p)
    }
}

impl distribution::Kurtosis for Gaussian {
    #[inline]
    fn kurtosis(&self) -> f64 { 0.0 }
}

impl distribution::Median for Gaussian {
    #[inline]
    fn median(&self) -> f64 { self.mu }
}

impl distribution::Modes for Gaussian {
    #[inline]
    fn modes(&self) -> Vec<f64> {
        vec![self.mu]
    }
}

impl distribution::Sample for Gaussian {
    /// Draw a sample.
    ///
    /// ## References
    ///
    /// 1. G. Marsaglia and W. W. Tsang, “The ziggurat method for generating
    ///    random variables," Journal of Statistical Software, vol. 5, no. 8,
    ///    pp. 1–7, 10 2000.
    ///
    /// 2. D. Eddelbuettel, “Ziggurat Revisited,” 2014.
    #[inline]
    fn sample<S>(&self, source: &mut S) -> f64 where S: random::Source {
        self.sigma * sample(source) + self.mu
    }
}

impl distribution::Skewness for Gaussian {
    #[inline]
    fn skewness(&self) -> f64 { 0.0 }
}

/// Compute the inverse cumulative distribution function of the standard
/// Gaussian distribution.
pub fn inv_cdf(p: f64) -> f64 {
    use std::f64::{INFINITY, NEG_INFINITY};

    should!(0.0 <= p && p <= 1.0);

    const CONST1: f64 = 0.180625;
    const CONST2: f64 = 1.6;
    const SPLIT1: f64 = 0.425;
    const SPLIT2: f64 = 5.0;
    const A: [f64; 8] = [
        3.3871328727963666080e+00, 1.3314166789178437745e+02, 1.9715909503065514427e+03,
        1.3731693765509461125e+04, 4.5921953931549871457e+04, 6.7265770927008700853e+04,
        3.3430575583588128105e+04, 2.5090809287301226727e+03,
    ];
    const B: [f64; 8] = [
        1.0000000000000000000e+00, 4.2313330701600911252e+01, 6.8718700749205790830e+02,
        5.3941960214247511077e+03, 2.1213794301586595867e+04, 3.9307895800092710610e+04,
        2.8729085735721942674e+04, 5.2264952788528545610e+03,
    ];
    const C: [f64; 8] = [
        1.42343711074968357734e+00, 4.63033784615654529590e+00, 5.76949722146069140550e+00,
        3.64784832476320460504e+00, 1.27045825245236838258e+00, 2.41780725177450611770e-01,
        2.27238449892691845833e-02, 7.74545014278341407640e-04,
    ];
    const D: [f64; 8] = [
        1.00000000000000000000e+00, 2.05319162663775882187e+00, 1.67638483018380384940e+00,
        6.89767334985100004550e-01, 1.48103976427480074590e-01, 1.51986665636164571966e-02,
        5.47593808499534494600e-04, 1.05075007164441684324e-09,
    ];
    const E: [f64; 8] = [
        6.65790464350110377720e+00, 5.46378491116411436990e+00, 1.78482653991729133580e+00,
        2.96560571828504891230e-01, 2.65321895265761230930e-02, 1.24266094738807843860e-03,
        2.71155556874348757815e-05, 2.01033439929228813265e-07,
    ];
    const F: [f64; 8] = [
        1.00000000000000000000e+00, 5.99832206555887937690e-01, 1.36929880922735805310e-01,
        1.48753612908506148525e-02, 7.86869131145613259100e-04, 1.84631831751005468180e-05,
        1.42151175831644588870e-07, 2.04426310338993978564e-15,
    ];

    #[inline(always)]
    fn poly(c: &[f64], x: f64) -> f64 {
        c[0] + x * (c[1] + x * (c[2] + x * (c[3] + x * (
        c[4] + x * (c[5] + x * (c[6] + x * (c[7])))))))
    }

    if p <= 0.0 {
        return NEG_INFINITY;
    }
    if 1.0 <= p {
        return INFINITY;
    }

    let q = p - 0.5;

    if (if q < 0.0 { -q } else { q }) <= SPLIT1 {
        let x = CONST1 - q * q;
        return q * poly(&A, x) / poly(&B, x);
    }

    let mut x = if q < 0.0 { p } else { 1.0 - p };

    x = (-x.ln()).sqrt();

    if x <= SPLIT2 {
        x -= CONST2;
        x = poly(&C, x) / poly(&D, x);
    } else {
        x -= SPLIT2;
        x = poly(&E, x) / poly(&F, x);
    }

    if q < 0.0 { -x } else { x }
}

/// Draw a sample from the standard Gaussian distribution.
pub fn sample<S: random::Source>(source: &mut S) -> f64 {
    loop {
        let u = source.read::<u64>();

        let i = (u & 0x7F) as usize;
        let j = ((u >> 8) & 0xFFFFFF) as u32;
        let s = if u & 0x80 != 0 { 1.0 } else { -1.0 };

        if j < K[i] {
            let x = j as f64 * W[i];
            return s * x;
        }

        let (x, y) = if i < 127 {
            let x = j as f64 * W[i];
            let y = Y[i + 1] + (Y[i] - Y[i + 1]) * source.read::<f64>();
            (x, y)
        } else {
            let x = R - (1.0 - source.read::<f64>()).ln() / R;
            let y = (-R * (x - 0.5 * R)).exp() * source.read::<f64>();
            (x, y)
        };

        if y < (-0.5 * x * x).exp() {
            return s * x;
        }
    }
}

const R: f64 = 3.44428647676;

const K: [u32; 128] = [
    00000000, 12590644, 14272653, 14988939,
    15384584, 15635009, 15807561, 15933577,
    16029594, 16105155, 16166147, 16216399,
    16258508, 16294295, 16325078, 16351831,
    16375291, 16396026, 16414479, 16431002,
    16445880, 16459343, 16471578, 16482744,
    16492970, 16502368, 16511031, 16519039,
    16526459, 16533352, 16539769, 16545755,
    16551348, 16556584, 16561493, 16566101,
    16570433, 16574511, 16578353, 16581977,
    16585398, 16588629, 16591685, 16594575,
    16597311, 16599901, 16602354, 16604679,
    16606881, 16608968, 16610945, 16612818,
    16614592, 16616272, 16617861, 16619363,
    16620782, 16622121, 16623383, 16624570,
    16625685, 16626730, 16627708, 16628619,
    16629465, 16630248, 16630969, 16631628,
    16632228, 16632768, 16633248, 16633671,
    16634034, 16634340, 16634586, 16634774,
    16634903, 16634972, 16634980, 16634926,
    16634810, 16634628, 16634381, 16634066,
    16633680, 16633222, 16632688, 16632075,
    16631380, 16630598, 16629726, 16628757,
    16627686, 16626507, 16625212, 16623794,
    16622243, 16620548, 16618698, 16616679,
    16614476, 16612071, 16609444, 16606571,
    16603425, 16599973, 16596178, 16591995,
    16587369, 16582237, 16576520, 16570120,
    16562917, 16554758, 16545450, 16534739,
    16522287, 16507638, 16490152, 16468907,
    16442518, 16408804, 16364095, 16301683,
    16207738, 16047994, 15704248, 15472926
];

const Y: [f64; 128] = [
    1.0000000000000, 0.96359862301100, 0.93628081335300, 0.91304110425300,
    0.8922785066960, 0.87323935691900, 0.85549640763400, 0.83877892834900,
    0.8229020836990, 0.80773273823400, 0.79317104551900, 0.77913972650500,
    0.7655774360820, 0.75243445624800, 0.73966978767700, 0.72724912028500,
    0.7151433774130, 0.70332764645500, 0.69178037703500, 0.68048276891000,
    0.6694182972330, 0.65857233912000, 0.64793187618900, 0.63748525489600,
    0.6272219914500, 0.61713261153200, 0.60720851746700, 0.59744187729600,
    0.5878255314650, 0.57835291380300, 0.56901798419800, 0.55981517091100,
    0.5507393208770, 0.54178565668200, 0.53294973914500, 0.52422743462800,
    0.5156148863730, 0.50710848925300, 0.49870486747800, 0.49040085481200,
    0.4821934769860, 0.47407993601000, 0.46605759612500, 0.45812397121400,
    0.4502767134670, 0.44251360317100, 0.43483253947300, 0.42723153202200,
    0.4197086933790, 0.41226223212000, 0.40489044654800, 0.39759171895500,
    0.3903645103820, 0.38320735581600, 0.37611885978800, 0.36909769233400,
    0.3621425852820, 0.35525232883400, 0.34842576841500, 0.34166180177600,
    0.3349593763110, 0.32831748658800, 0.32173517206300, 0.31521151497000,
    0.3087456383670, 0.30233670433800, 0.29598391232000, 0.28968649757100,
    0.2834437297390, 0.27725491156000, 0.27111937764900, 0.26503649338700,
    0.2590056539120, 0.25302628318300, 0.24709783313900, 0.24121978293200,
    0.2353916382390, 0.22961293064900, 0.22388321712200, 0.21820207951800,
    0.2125691242010, 0.20698398170900, 0.20144630649600, 0.19595577674500,
    0.1905120942560, 0.18511498440600, 0.17976419618500, 0.17445950232400,
    0.1692006994920, 0.16398760860000, 0.15882007519500, 0.15369796996400,
    0.1486211893480, 0.14358965629500, 0.13860332114300, 0.13366216266900,
    0.1287661893090, 0.12391544058200, 0.11910998874500, 0.11434994070300,
    0.1096354402300, 0.10496667053300, 0.10034385723200, 0.09576727182660,
    0.0912372357329, 0.08675412501270, 0.08231837593200, 0.07793049152950,
    0.0735910494266, 0.06930071117420, 0.06506023352900, 0.06087048217450,
    0.0567324485840, 0.05264727098000, 0.04861626071630, 0.04464093597690,
    0.0407230655415, 0.03686472673860, 0.03306838393780, 0.02933699774110,
    0.0256741818288, 0.02208443726340, 0.01857352005770, 0.01514905528540,
    0.0118216532614, 0.00860719483079, 0.00553245272614, 0.00265435214565,
];

const W: [f64; 128] = [
    1.62318314817e-08, 2.16291505214e-08, 2.54246305087e-08, 2.84579525938e-08,
    3.10340022482e-08, 3.33011726243e-08, 3.53439060345e-08, 3.72152672658e-08,
    3.89509895720e-08, 4.05763964764e-08, 4.21101548915e-08, 4.35664624904e-08,
    4.49563968336e-08, 4.62887864029e-08, 4.75707945735e-08, 4.88083237257e-08,
    5.00063025384e-08, 5.11688950428e-08, 5.22996558616e-08, 5.34016475624e-08,
    5.44775307871e-08, 5.55296344581e-08, 5.65600111659e-08, 5.75704813695e-08,
    5.85626690412e-08, 5.95380306862e-08, 6.04978791776e-08, 6.14434034901e-08,
    6.23756851626e-08, 6.32957121259e-08, 6.42043903937e-08, 6.51025540077e-08,
    6.59909735447e-08, 6.68703634341e-08, 6.77413882848e-08, 6.86046683810e-08,
    6.94607844804e-08, 7.03102820203e-08, 7.11536748229e-08, 7.19914483720e-08,
    7.28240627230e-08, 7.36519550992e-08, 7.44755422158e-08, 7.52952223703e-08,
    7.61113773308e-08, 7.69243740467e-08, 7.77345662086e-08, 7.85422956743e-08,
    7.93478937793e-08, 8.01516825471e-08, 8.09539758128e-08, 8.17550802699e-08,
    8.25552964535e-08, 8.33549196661e-08, 8.41542408569e-08, 8.49535474601e-08,
    8.57531242006e-08, 8.65532538723e-08, 8.73542180955e-08, 8.81562980590e-08,
    8.89597752521e-08, 8.97649321908e-08, 9.05720531451e-08, 9.13814248700e-08,
    9.21933373471e-08, 9.30080845407e-08, 9.38259651738e-08, 9.46472835298e-08,
    9.54723502847e-08, 9.63014833769e-08, 9.71350089201e-08, 9.79732621669e-08,
    9.88165885297e-08, 9.96653446693e-08, 1.00519899658e-07, 1.01380636230e-07,
    1.02247952126e-07, 1.03122261554e-07, 1.04003996769e-07, 1.04893609795e-07,
    1.05791574313e-07, 1.06698387725e-07, 1.07614573423e-07, 1.08540683296e-07,
    1.09477300508e-07, 1.10425042570e-07, 1.11384564771e-07, 1.12356564007e-07,
    1.13341783071e-07, 1.14341015475e-07, 1.15355110887e-07, 1.16384981291e-07,
    1.17431607977e-07, 1.18496049514e-07, 1.19579450872e-07, 1.20683053909e-07,
    1.21808209468e-07, 1.22956391410e-07, 1.24129212952e-07, 1.25328445797e-07,
    1.26556042658e-07, 1.27814163916e-07, 1.29105209375e-07, 1.30431856341e-07,
    1.31797105598e-07, 1.33204337360e-07, 1.34657379914e-07, 1.36160594606e-07,
    1.37718982103e-07, 1.39338316679e-07, 1.41025317971e-07, 1.42787873535e-07,
    1.44635331499e-07, 1.46578891730e-07, 1.48632138436e-07, 1.50811780719e-07,
    1.53138707402e-07, 1.55639532047e-07, 1.58348931426e-07, 1.61313325908e-07,
    1.64596952856e-07, 1.68292495203e-07, 1.72541128694e-07, 1.77574279496e-07,
    1.83813550477e-07, 1.92166040885e-07, 2.05295471952e-07, 2.22600839893e-07,
];

#[cfg(test)]
mod tests {
    use assert;
    use prelude::*;
    use std::f64::{INFINITY, NEG_INFINITY};

    macro_rules! new(
        ($mu:expr, $sigma:expr) => (Gaussian::new($mu, $sigma));
    );

    #[test]
    fn cdf() {
        let d = new!(1.0, 2.0);
        let x = vec![
            -4.0, -3.5, -3.0, -2.5, -2.0, -1.5, -1.0, -0.5,
            0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0,
        ];
        let p = vec![
            6.209665325776139e-03, 1.222447265504470e-02, 2.275013194817922e-02,
            4.005915686381709e-02, 6.680720126885809e-02, 1.056497736668553e-01,
            1.586552539314571e-01, 2.266273523768682e-01, 3.085375387259869e-01,
            4.012936743170763e-01, 5.000000000000000e-01, 5.987063256829237e-01,
            6.914624612740131e-01, 7.733726476231317e-01, 8.413447460685429e-01,
            8.943502263331446e-01, 9.331927987311419e-01,
        ];

        assert::close(&x.iter().map(|&x| d.cdf(x)).collect::<Vec<_>>(), &p, 1e-14);
    }

    #[test]
    fn mean() {
        assert_eq!(new!(0.0, 1.0).mean(), 0.0);
    }

    #[test]
    fn var() {
        assert_eq!(new!(0.0, 2.0).var(), 4.0);
    }

    #[test]
    fn sd() {
        assert_eq!(new!(0.0, 2.0).sd(), 2.0);
    }

    #[test]
    fn pdf() {
        let d = new!(1.0, 2.0);
        let x = vec![
            -4.0, -3.5, -3.0, -2.5, -2.0, -1.5, -1.0, -0.5, 0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5,
             4.0
        ];
        let p = vec![
            8.764150246784270e-03, 1.586982591783371e-02, 2.699548325659403e-02,
            4.313865941325577e-02, 6.475879783294587e-02, 9.132454269451096e-02,
            1.209853622595717e-01, 1.505687160774022e-01, 1.760326633821498e-01,
            1.933340584014246e-01, 1.994711402007164e-01, 1.933340584014246e-01,
            1.760326633821498e-01, 1.505687160774022e-01, 1.209853622595717e-01,
            9.132454269451096e-02, 6.475879783294587e-02
        ];

        assert::close(&x.iter().map(|&x| d.pdf(x)).collect::<Vec<_>>(), &p, 1e-14);
    }

    #[test]
    fn entropy() {
        use std::f64::consts::PI;
        assert_eq!(new!(0.0, 1.0).entropy(), ((2.0 * PI).ln() + 1.0) / 2.0);
    }

    #[test]
    fn inv_cdf() {
        let d = new!(-1.0, 0.25);
        let p = vec![
            0.00, 0.05, 0.10, 0.15, 0.20, 0.25, 0.30, 0.35, 0.40, 0.45, 0.50,
            0.55, 0.60, 0.65, 0.70, 0.75, 0.80, 0.85, 0.90, 0.95, 1.00,
        ];
        let x = vec![
                      NEG_INFINITY, -1.411213406737868e+00, -1.320387891386150e+00,
            -1.259108347373447e+00, -1.210405308393228e+00, -1.168622437549020e+00,
            -1.131100128177010e+00, -1.096330116601892e+00, -1.063336775783950e+00,
            -1.031415336713768e+00, -1.000000000000000e+00, -9.685846632862315e-01,
            -9.366632242160501e-01, -9.036698833981082e-01, -8.688998718229899e-01,
            -8.313775624509796e-01, -7.895946916067714e-01, -7.408916526265525e-01,
            -6.796121086138498e-01, -5.887865932621319e-01, INFINITY,
        ];

        assert::close(&p.iter().map(|&p| d.inv_cdf(p)).collect::<Vec<_>>(), &x, 1e-14);
    }

    #[test]
    fn kurtosis() {
        assert_eq!(new!(0.0, 2.0).kurtosis(), 0.0);
    }

    #[test]
    fn median() {
        assert_eq!(new!(0.0, 2.0).median(), 0.0);
    }

    #[test]
    fn modes() {
        assert_eq!(new!(2.0, 5.0).modes(), vec![2.0]);
    }

    #[test]
    fn skewness() {
        assert_eq!(new!(0.0, 2.0).skewness(), 0.0);
    }
}
