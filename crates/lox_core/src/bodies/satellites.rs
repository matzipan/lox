// Auto-generated by `lox_gen`. Do not edit!
use super::{Ellipsoid, NaifId, PointMass, TriAxial};
pub struct Moon;
impl NaifId for Moon {
    fn id() -> i32 {
        301i32
    }
}
impl Ellipsoid for Moon {
    fn polar_radius() -> f64 {
        1737.4f64
    }
    fn mean_radius() -> f64 {
        1737.4000000000003f64
    }
}
impl TriAxial for Moon {
    fn subplanetary_radius() -> f64 {
        1737.4f64
    }
    fn along_orbit_radius() -> f64 {
        1737.4f64
    }
}
impl PointMass for Moon {
    fn gravitational_parameter() -> f64 {
        4902.80011845755f64
    }
}
pub struct Phobos;
impl NaifId for Phobos {
    fn id() -> i32 {
        401i32
    }
}
impl Ellipsoid for Phobos {
    fn polar_radius() -> f64 {
        9.1f64
    }
    fn mean_radius() -> f64 {
        11.166666666666666f64
    }
}
impl TriAxial for Phobos {
    fn subplanetary_radius() -> f64 {
        13f64
    }
    fn along_orbit_radius() -> f64 {
        11.4f64
    }
}
impl PointMass for Phobos {
    fn gravitational_parameter() -> f64 {
        0.0007087546066894452f64
    }
}
pub struct Deimos;
impl NaifId for Deimos {
    fn id() -> i32 {
        402i32
    }
}
impl Ellipsoid for Deimos {
    fn polar_radius() -> f64 {
        5.1f64
    }
    fn mean_radius() -> f64 {
        6.3f64
    }
}
impl TriAxial for Deimos {
    fn subplanetary_radius() -> f64 {
        7.8f64
    }
    fn along_orbit_radius() -> f64 {
        6f64
    }
}
impl PointMass for Deimos {
    fn gravitational_parameter() -> f64 {
        0.00009615569648120313f64
    }
}
pub struct Io;
impl NaifId for Io {
    fn id() -> i32 {
        501i32
    }
}
impl Ellipsoid for Io {
    fn polar_radius() -> f64 {
        1815.7f64
    }
    fn mean_radius() -> f64 {
        1821.5f64
    }
}
impl TriAxial for Io {
    fn subplanetary_radius() -> f64 {
        1829.4f64
    }
    fn along_orbit_radius() -> f64 {
        1819.4f64
    }
}
impl PointMass for Io {
    fn gravitational_parameter() -> f64 {
        5959.915466180539f64
    }
}
pub struct Europa;
impl NaifId for Europa {
    fn id() -> i32 {
        502i32
    }
}
impl Ellipsoid for Europa {
    fn polar_radius() -> f64 {
        1559.5f64
    }
    fn mean_radius() -> f64 {
        1560.8f64
    }
}
impl TriAxial for Europa {
    fn subplanetary_radius() -> f64 {
        1562.6f64
    }
    fn along_orbit_radius() -> f64 {
        1560.3f64
    }
}
impl PointMass for Europa {
    fn gravitational_parameter() -> f64 {
        3202.712099607295f64
    }
}
pub struct Ganymede;
impl NaifId for Ganymede {
    fn id() -> i32 {
        503i32
    }
}
impl Ellipsoid for Ganymede {
    fn polar_radius() -> f64 {
        2631.2f64
    }
    fn mean_radius() -> f64 {
        2631.2f64
    }
}
impl TriAxial for Ganymede {
    fn subplanetary_radius() -> f64 {
        2631.2f64
    }
    fn along_orbit_radius() -> f64 {
        2631.2f64
    }
}
impl PointMass for Ganymede {
    fn gravitational_parameter() -> f64 {
        9887.832752719638f64
    }
}
pub struct Callisto;
impl NaifId for Callisto {
    fn id() -> i32 {
        504i32
    }
}
impl Ellipsoid for Callisto {
    fn polar_radius() -> f64 {
        2410.3f64
    }
    fn mean_radius() -> f64 {
        2410.3f64
    }
}
impl TriAxial for Callisto {
    fn subplanetary_radius() -> f64 {
        2410.3f64
    }
    fn along_orbit_radius() -> f64 {
        2410.3f64
    }
}
impl PointMass for Callisto {
    fn gravitational_parameter() -> f64 {
        7179.283402579837f64
    }
}
pub struct Amalthea;
impl NaifId for Amalthea {
    fn id() -> i32 {
        505i32
    }
}
impl Ellipsoid for Amalthea {
    fn polar_radius() -> f64 {
        64f64
    }
    fn mean_radius() -> f64 {
        87.33333333333333f64
    }
}
impl TriAxial for Amalthea {
    fn subplanetary_radius() -> f64 {
        125f64
    }
    fn along_orbit_radius() -> f64 {
        73f64
    }
}
impl PointMass for Amalthea {
    fn gravitational_parameter() -> f64 {
        0.1645634534798259f64
    }
}
pub struct Himalia;
impl NaifId for Himalia {
    fn id() -> i32 {
        506i32
    }
}
impl Ellipsoid for Himalia {
    fn polar_radius() -> f64 {
        85f64
    }
    fn mean_radius() -> f64 {
        85f64
    }
}
impl TriAxial for Himalia {
    fn subplanetary_radius() -> f64 {
        85f64
    }
    fn along_orbit_radius() -> f64 {
        85f64
    }
}
impl PointMass for Himalia {
    fn gravitational_parameter() -> f64 {
        0.1515524299611265f64
    }
}
pub struct Elara;
impl NaifId for Elara {
    fn id() -> i32 {
        507i32
    }
}
impl Ellipsoid for Elara {
    fn polar_radius() -> f64 {
        40f64
    }
    fn mean_radius() -> f64 {
        40f64
    }
}
impl TriAxial for Elara {
    fn subplanetary_radius() -> f64 {
        40f64
    }
    fn along_orbit_radius() -> f64 {
        40f64
    }
}
pub struct Pasiphae;
impl NaifId for Pasiphae {
    fn id() -> i32 {
        508i32
    }
}
impl Ellipsoid for Pasiphae {
    fn polar_radius() -> f64 {
        18f64
    }
    fn mean_radius() -> f64 {
        18f64
    }
}
impl TriAxial for Pasiphae {
    fn subplanetary_radius() -> f64 {
        18f64
    }
    fn along_orbit_radius() -> f64 {
        18f64
    }
}
pub struct Sinope;
impl NaifId for Sinope {
    fn id() -> i32 {
        509i32
    }
}
impl Ellipsoid for Sinope {
    fn polar_radius() -> f64 {
        14f64
    }
    fn mean_radius() -> f64 {
        14f64
    }
}
impl TriAxial for Sinope {
    fn subplanetary_radius() -> f64 {
        14f64
    }
    fn along_orbit_radius() -> f64 {
        14f64
    }
}
pub struct Lysithea;
impl NaifId for Lysithea {
    fn id() -> i32 {
        510i32
    }
}
impl Ellipsoid for Lysithea {
    fn polar_radius() -> f64 {
        12f64
    }
    fn mean_radius() -> f64 {
        12f64
    }
}
impl TriAxial for Lysithea {
    fn subplanetary_radius() -> f64 {
        12f64
    }
    fn along_orbit_radius() -> f64 {
        12f64
    }
}
pub struct Carme;
impl NaifId for Carme {
    fn id() -> i32 {
        511i32
    }
}
impl Ellipsoid for Carme {
    fn polar_radius() -> f64 {
        15f64
    }
    fn mean_radius() -> f64 {
        15f64
    }
}
impl TriAxial for Carme {
    fn subplanetary_radius() -> f64 {
        15f64
    }
    fn along_orbit_radius() -> f64 {
        15f64
    }
}
pub struct Ananke;
impl NaifId for Ananke {
    fn id() -> i32 {
        512i32
    }
}
impl Ellipsoid for Ananke {
    fn polar_radius() -> f64 {
        10f64
    }
    fn mean_radius() -> f64 {
        10f64
    }
}
impl TriAxial for Ananke {
    fn subplanetary_radius() -> f64 {
        10f64
    }
    fn along_orbit_radius() -> f64 {
        10f64
    }
}
pub struct Leda;
impl NaifId for Leda {
    fn id() -> i32 {
        513i32
    }
}
impl Ellipsoid for Leda {
    fn polar_radius() -> f64 {
        5f64
    }
    fn mean_radius() -> f64 {
        5f64
    }
}
impl TriAxial for Leda {
    fn subplanetary_radius() -> f64 {
        5f64
    }
    fn along_orbit_radius() -> f64 {
        5f64
    }
}
pub struct Thebe;
impl NaifId for Thebe {
    fn id() -> i32 {
        514i32
    }
}
impl Ellipsoid for Thebe {
    fn polar_radius() -> f64 {
        42f64
    }
    fn mean_radius() -> f64 {
        49.666666666666664f64
    }
}
impl TriAxial for Thebe {
    fn subplanetary_radius() -> f64 {
        58f64
    }
    fn along_orbit_radius() -> f64 {
        49f64
    }
}
impl PointMass for Thebe {
    fn gravitational_parameter() -> f64 {
        0.030148f64
    }
}
pub struct Adrastea;
impl NaifId for Adrastea {
    fn id() -> i32 {
        515i32
    }
}
impl Ellipsoid for Adrastea {
    fn polar_radius() -> f64 {
        7f64
    }
    fn mean_radius() -> f64 {
        8.333333333333334f64
    }
}
impl TriAxial for Adrastea {
    fn subplanetary_radius() -> f64 {
        10f64
    }
    fn along_orbit_radius() -> f64 {
        8f64
    }
}
impl PointMass for Adrastea {
    fn gravitational_parameter() -> f64 {
        0.000139f64
    }
}
pub struct Metis;
impl NaifId for Metis {
    fn id() -> i32 {
        516i32
    }
}
impl Ellipsoid for Metis {
    fn polar_radius() -> f64 {
        17f64
    }
    fn mean_radius() -> f64 {
        22.333333333333332f64
    }
}
impl TriAxial for Metis {
    fn subplanetary_radius() -> f64 {
        30f64
    }
    fn along_orbit_radius() -> f64 {
        20f64
    }
}
impl PointMass for Metis {
    fn gravitational_parameter() -> f64 {
        0.002501f64
    }
}
pub struct Callirrhoe;
impl NaifId for Callirrhoe {
    fn id() -> i32 {
        517i32
    }
}
pub struct Themisto;
impl NaifId for Themisto {
    fn id() -> i32 {
        518i32
    }
}
pub struct Magaclite;
impl NaifId for Magaclite {
    fn id() -> i32 {
        519i32
    }
}
pub struct Taygete;
impl NaifId for Taygete {
    fn id() -> i32 {
        520i32
    }
}
pub struct Chaldene;
impl NaifId for Chaldene {
    fn id() -> i32 {
        521i32
    }
}
pub struct Harpalyke;
impl NaifId for Harpalyke {
    fn id() -> i32 {
        522i32
    }
}
pub struct Kalyke;
impl NaifId for Kalyke {
    fn id() -> i32 {
        523i32
    }
}
pub struct Iocaste;
impl NaifId for Iocaste {
    fn id() -> i32 {
        524i32
    }
}
pub struct Erinome;
impl NaifId for Erinome {
    fn id() -> i32 {
        525i32
    }
}
pub struct Isonoe;
impl NaifId for Isonoe {
    fn id() -> i32 {
        526i32
    }
}
pub struct Praxidike;
impl NaifId for Praxidike {
    fn id() -> i32 {
        527i32
    }
}
pub struct Autonoe;
impl NaifId for Autonoe {
    fn id() -> i32 {
        528i32
    }
}
pub struct Thyone;
impl NaifId for Thyone {
    fn id() -> i32 {
        529i32
    }
}
pub struct Hermippe;
impl NaifId for Hermippe {
    fn id() -> i32 {
        530i32
    }
}
pub struct Aitne;
impl NaifId for Aitne {
    fn id() -> i32 {
        531i32
    }
}
pub struct Eurydome;
impl NaifId for Eurydome {
    fn id() -> i32 {
        532i32
    }
}
pub struct Euanthe;
impl NaifId for Euanthe {
    fn id() -> i32 {
        533i32
    }
}
pub struct Euporie;
impl NaifId for Euporie {
    fn id() -> i32 {
        534i32
    }
}
pub struct Orthosie;
impl NaifId for Orthosie {
    fn id() -> i32 {
        535i32
    }
}
pub struct Sponde;
impl NaifId for Sponde {
    fn id() -> i32 {
        536i32
    }
}
pub struct Kale;
impl NaifId for Kale {
    fn id() -> i32 {
        537i32
    }
}
pub struct Pasithee;
impl NaifId for Pasithee {
    fn id() -> i32 {
        538i32
    }
}
pub struct Hegemone;
impl NaifId for Hegemone {
    fn id() -> i32 {
        539i32
    }
}
pub struct Mneme;
impl NaifId for Mneme {
    fn id() -> i32 {
        540i32
    }
}
pub struct Aoede;
impl NaifId for Aoede {
    fn id() -> i32 {
        541i32
    }
}
pub struct Thelxinoe;
impl NaifId for Thelxinoe {
    fn id() -> i32 {
        542i32
    }
}
pub struct Arche;
impl NaifId for Arche {
    fn id() -> i32 {
        543i32
    }
}
pub struct Kallichore;
impl NaifId for Kallichore {
    fn id() -> i32 {
        544i32
    }
}
pub struct Helike;
impl NaifId for Helike {
    fn id() -> i32 {
        545i32
    }
}
pub struct Carpo;
impl NaifId for Carpo {
    fn id() -> i32 {
        546i32
    }
}
pub struct Eukelade;
impl NaifId for Eukelade {
    fn id() -> i32 {
        547i32
    }
}
pub struct Cyllene;
impl NaifId for Cyllene {
    fn id() -> i32 {
        548i32
    }
}
pub struct Kore;
impl NaifId for Kore {
    fn id() -> i32 {
        549i32
    }
}
pub struct Herse;
impl NaifId for Herse {
    fn id() -> i32 {
        550i32
    }
}
pub struct Dia;
impl NaifId for Dia {
    fn id() -> i32 {
        553i32
    }
}
pub struct Mimas;
impl NaifId for Mimas {
    fn id() -> i32 {
        601i32
    }
}
impl Ellipsoid for Mimas {
    fn polar_radius() -> f64 {
        190.6f64
    }
    fn mean_radius() -> f64 {
        198.36666666666667f64
    }
}
impl TriAxial for Mimas {
    fn subplanetary_radius() -> f64 {
        207.8f64
    }
    fn along_orbit_radius() -> f64 {
        196.7f64
    }
}
impl PointMass for Mimas {
    fn gravitational_parameter() -> f64 {
        2.503488768152587f64
    }
}
pub struct Enceladus;
impl NaifId for Enceladus {
    fn id() -> i32 {
        602i32
    }
}
impl Ellipsoid for Enceladus {
    fn polar_radius() -> f64 {
        248.3f64
    }
    fn mean_radius() -> f64 {
        252.1f64
    }
}
impl TriAxial for Enceladus {
    fn subplanetary_radius() -> f64 {
        256.6f64
    }
    fn along_orbit_radius() -> f64 {
        251.4f64
    }
}
impl PointMass for Enceladus {
    fn gravitational_parameter() -> f64 {
        7.210366688598896f64
    }
}
pub struct Tethys;
impl NaifId for Tethys {
    fn id() -> i32 {
        603i32
    }
}
impl Ellipsoid for Tethys {
    fn polar_radius() -> f64 {
        526.3f64
    }
    fn mean_radius() -> f64 {
        530.9999999999999f64
    }
}
impl TriAxial for Tethys {
    fn subplanetary_radius() -> f64 {
        538.4f64
    }
    fn along_orbit_radius() -> f64 {
        528.3f64
    }
}
impl PointMass for Tethys {
    fn gravitational_parameter() -> f64 {
        41.21352885489587f64
    }
}
pub struct Dione;
impl NaifId for Dione {
    fn id() -> i32 {
        604i32
    }
}
impl Ellipsoid for Dione {
    fn polar_radius() -> f64 {
        559.6f64
    }
    fn mean_radius() -> f64 {
        561.4333333333333f64
    }
}
impl TriAxial for Dione {
    fn subplanetary_radius() -> f64 {
        563.4f64
    }
    fn along_orbit_radius() -> f64 {
        561.3f64
    }
}
impl PointMass for Dione {
    fn gravitational_parameter() -> f64 {
        73.11607172482067f64
    }
}
pub struct Rhea;
impl NaifId for Rhea {
    fn id() -> i32 {
        605i32
    }
}
impl Ellipsoid for Rhea {
    fn polar_radius() -> f64 {
        762.4f64
    }
    fn mean_radius() -> f64 {
        763.5f64
    }
}
impl TriAxial for Rhea {
    fn subplanetary_radius() -> f64 {
        765f64
    }
    fn along_orbit_radius() -> f64 {
        763.1f64
    }
}
impl PointMass for Rhea {
    fn gravitational_parameter() -> f64 {
        153.9417519146563f64
    }
}
pub struct Titan;
impl NaifId for Titan {
    fn id() -> i32 {
        606i32
    }
}
impl Ellipsoid for Titan {
    fn polar_radius() -> f64 {
        2574.47f64
    }
    fn mean_radius() -> f64 {
        2574.7999999999997f64
    }
}
impl TriAxial for Titan {
    fn subplanetary_radius() -> f64 {
        2575.15f64
    }
    fn along_orbit_radius() -> f64 {
        2574.78f64
    }
}
impl PointMass for Titan {
    fn gravitational_parameter() -> f64 {
        8978.137095521046f64
    }
}
pub struct Hyperion;
impl NaifId for Hyperion {
    fn id() -> i32 {
        607i32
    }
}
impl Ellipsoid for Hyperion {
    fn polar_radius() -> f64 {
        102.7f64
    }
    fn mean_radius() -> f64 {
        138.6f64
    }
}
impl TriAxial for Hyperion {
    fn subplanetary_radius() -> f64 {
        180.1f64
    }
    fn along_orbit_radius() -> f64 {
        133f64
    }
}
impl PointMass for Hyperion {
    fn gravitational_parameter() -> f64 {
        0.3704913747932265f64
    }
}
pub struct Iapetus;
impl NaifId for Iapetus {
    fn id() -> i32 {
        608i32
    }
}
impl Ellipsoid for Iapetus {
    fn polar_radius() -> f64 {
        712.1f64
    }
    fn mean_radius() -> f64 {
        734.5f64
    }
}
impl TriAxial for Iapetus {
    fn subplanetary_radius() -> f64 {
        745.7f64
    }
    fn along_orbit_radius() -> f64 {
        745.7f64
    }
}
impl PointMass for Iapetus {
    fn gravitational_parameter() -> f64 {
        120.5151060137642f64
    }
}
pub struct Phoebe;
impl NaifId for Phoebe {
    fn id() -> i32 {
        609i32
    }
}
impl Ellipsoid for Phoebe {
    fn polar_radius() -> f64 {
        101.8f64
    }
    fn mean_radius() -> f64 {
        106.56666666666666f64
    }
}
impl TriAxial for Phoebe {
    fn subplanetary_radius() -> f64 {
        109.4f64
    }
    fn along_orbit_radius() -> f64 {
        108.5f64
    }
}
impl PointMass for Phoebe {
    fn gravitational_parameter() -> f64 {
        0.5547860052791678f64
    }
}
pub struct Janus;
impl NaifId for Janus {
    fn id() -> i32 {
        610i32
    }
}
impl Ellipsoid for Janus {
    fn polar_radius() -> f64 {
        76.3f64
    }
    fn mean_radius() -> f64 {
        90.33333333333333f64
    }
}
impl TriAxial for Janus {
    fn subplanetary_radius() -> f64 {
        101.7f64
    }
    fn along_orbit_radius() -> f64 {
        93f64
    }
}
impl PointMass for Janus {
    fn gravitational_parameter() -> f64 {
        0.1265765099012197f64
    }
}
pub struct Epimetheus;
impl NaifId for Epimetheus {
    fn id() -> i32 {
        611i32
    }
}
impl Ellipsoid for Epimetheus {
    fn polar_radius() -> f64 {
        53f64
    }
    fn mean_radius() -> f64 {
        58.4f64
    }
}
impl TriAxial for Epimetheus {
    fn subplanetary_radius() -> f64 {
        64.9f64
    }
    fn along_orbit_radius() -> f64 {
        57.3f64
    }
}
impl PointMass for Epimetheus {
    fn gravitational_parameter() -> f64 {
        0.03512333288208074f64
    }
}
pub struct Helene;
impl NaifId for Helene {
    fn id() -> i32 {
        612i32
    }
}
impl Ellipsoid for Helene {
    fn polar_radius() -> f64 {
        13.3f64
    }
    fn mean_radius() -> f64 {
        18.46666666666667f64
    }
}
impl TriAxial for Helene {
    fn subplanetary_radius() -> f64 {
        22.5f64
    }
    fn along_orbit_radius() -> f64 {
        19.6f64
    }
}
impl PointMass for Helene {
    fn gravitational_parameter() -> f64 {
        0.0004757419551776972f64
    }
}
pub struct Telesto;
impl NaifId for Telesto {
    fn id() -> i32 {
        613i32
    }
}
impl Ellipsoid for Telesto {
    fn polar_radius() -> f64 {
        9.8f64
    }
    fn mean_radius() -> f64 {
        12.633333333333335f64
    }
}
impl TriAxial for Telesto {
    fn subplanetary_radius() -> f64 {
        16.3f64
    }
    fn along_orbit_radius() -> f64 {
        11.8f64
    }
}
pub struct Calypso;
impl NaifId for Calypso {
    fn id() -> i32 {
        614i32
    }
}
impl Ellipsoid for Calypso {
    fn polar_radius() -> f64 {
        6.3f64
    }
    fn mean_radius() -> f64 {
        10.3f64
    }
}
impl TriAxial for Calypso {
    fn subplanetary_radius() -> f64 {
        15.3f64
    }
    fn along_orbit_radius() -> f64 {
        9.3f64
    }
}
pub struct Atlas;
impl NaifId for Atlas {
    fn id() -> i32 {
        615i32
    }
}
impl Ellipsoid for Atlas {
    fn polar_radius() -> f64 {
        9.4f64
    }
    fn mean_radius() -> f64 {
        15.899999999999999f64
    }
}
impl TriAxial for Atlas {
    fn subplanetary_radius() -> f64 {
        20.5f64
    }
    fn along_orbit_radius() -> f64 {
        17.8f64
    }
}
impl PointMass for Atlas {
    fn gravitational_parameter() -> f64 {
        0.0003718871247516475f64
    }
}
pub struct Prometheus;
impl NaifId for Prometheus {
    fn id() -> i32 {
        616i32
    }
}
impl Ellipsoid for Prometheus {
    fn polar_radius() -> f64 {
        28.2f64
    }
    fn mean_radius() -> f64 {
        46f64
    }
}
impl TriAxial for Prometheus {
    fn subplanetary_radius() -> f64 {
        68.2f64
    }
    fn along_orbit_radius() -> f64 {
        41.6f64
    }
}
impl PointMass for Prometheus {
    fn gravitational_parameter() -> f64 {
        0.0107520800100761f64
    }
}
pub struct Pandora;
impl NaifId for Pandora {
    fn id() -> i32 {
        617i32
    }
}
impl Ellipsoid for Pandora {
    fn polar_radius() -> f64 {
        31.5f64
    }
    fn mean_radius() -> f64 {
        41.5f64
    }
}
impl TriAxial for Pandora {
    fn subplanetary_radius() -> f64 {
        52.2f64
    }
    fn along_orbit_radius() -> f64 {
        40.8f64
    }
}
impl PointMass for Pandora {
    fn gravitational_parameter() -> f64 {
        0.009290325122028795f64
    }
}
pub struct Pan;
impl NaifId for Pan {
    fn id() -> i32 {
        618i32
    }
}
impl Ellipsoid for Pan {
    fn polar_radius() -> f64 {
        10.4f64
    }
    fn mean_radius() -> f64 {
        14.333333333333334f64
    }
}
impl TriAxial for Pan {
    fn subplanetary_radius() -> f64 {
        17.2f64
    }
    fn along_orbit_radius() -> f64 {
        15.4f64
    }
}
pub struct Ymir;
impl NaifId for Ymir {
    fn id() -> i32 {
        619i32
    }
}
pub struct Paaliaq;
impl NaifId for Paaliaq {
    fn id() -> i32 {
        620i32
    }
}
pub struct Tarvos;
impl NaifId for Tarvos {
    fn id() -> i32 {
        621i32
    }
}
pub struct Ijiraq;
impl NaifId for Ijiraq {
    fn id() -> i32 {
        622i32
    }
}
pub struct Suttungr;
impl NaifId for Suttungr {
    fn id() -> i32 {
        623i32
    }
}
pub struct Kiviuq;
impl NaifId for Kiviuq {
    fn id() -> i32 {
        624i32
    }
}
pub struct Mundilfari;
impl NaifId for Mundilfari {
    fn id() -> i32 {
        625i32
    }
}
pub struct Albiorix;
impl NaifId for Albiorix {
    fn id() -> i32 {
        626i32
    }
}
pub struct Skathi;
impl NaifId for Skathi {
    fn id() -> i32 {
        627i32
    }
}
pub struct Erriapus;
impl NaifId for Erriapus {
    fn id() -> i32 {
        628i32
    }
}
pub struct Siarnaq;
impl NaifId for Siarnaq {
    fn id() -> i32 {
        629i32
    }
}
pub struct Thrymr;
impl NaifId for Thrymr {
    fn id() -> i32 {
        630i32
    }
}
pub struct Narvi;
impl NaifId for Narvi {
    fn id() -> i32 {
        631i32
    }
}
pub struct Methone;
impl NaifId for Methone {
    fn id() -> i32 {
        632i32
    }
}
impl Ellipsoid for Methone {
    fn polar_radius() -> f64 {
        1.21f64
    }
    fn mean_radius() -> f64 {
        1.4799999999999998f64
    }
}
impl TriAxial for Methone {
    fn subplanetary_radius() -> f64 {
        1.94f64
    }
    fn along_orbit_radius() -> f64 {
        1.29f64
    }
}
pub struct Pallene;
impl NaifId for Pallene {
    fn id() -> i32 {
        633i32
    }
}
impl Ellipsoid for Pallene {
    fn polar_radius() -> f64 {
        1.8f64
    }
    fn mean_radius() -> f64 {
        2.2533333333333334f64
    }
}
impl TriAxial for Pallene {
    fn subplanetary_radius() -> f64 {
        2.88f64
    }
    fn along_orbit_radius() -> f64 {
        2.08f64
    }
}
pub struct Polydeuces;
impl NaifId for Polydeuces {
    fn id() -> i32 {
        634i32
    }
}
impl Ellipsoid for Polydeuces {
    fn polar_radius() -> f64 {
        1f64
    }
    fn mean_radius() -> f64 {
        1.2333333333333334f64
    }
}
impl TriAxial for Polydeuces {
    fn subplanetary_radius() -> f64 {
        1.5f64
    }
    fn along_orbit_radius() -> f64 {
        1.2f64
    }
}
pub struct Daphnis;
impl NaifId for Daphnis {
    fn id() -> i32 {
        635i32
    }
}
impl Ellipsoid for Daphnis {
    fn polar_radius() -> f64 {
        2.8f64
    }
    fn mean_radius() -> f64 {
        3.9666666666666663f64
    }
}
impl TriAxial for Daphnis {
    fn subplanetary_radius() -> f64 {
        4.6f64
    }
    fn along_orbit_radius() -> f64 {
        4.5f64
    }
}
pub struct Aegir;
impl NaifId for Aegir {
    fn id() -> i32 {
        636i32
    }
}
pub struct Bebhionn;
impl NaifId for Bebhionn {
    fn id() -> i32 {
        637i32
    }
}
pub struct Bergelmir;
impl NaifId for Bergelmir {
    fn id() -> i32 {
        638i32
    }
}
pub struct Bestla;
impl NaifId for Bestla {
    fn id() -> i32 {
        639i32
    }
}
pub struct Farbauti;
impl NaifId for Farbauti {
    fn id() -> i32 {
        640i32
    }
}
pub struct Fenrir;
impl NaifId for Fenrir {
    fn id() -> i32 {
        641i32
    }
}
pub struct Fornjot;
impl NaifId for Fornjot {
    fn id() -> i32 {
        642i32
    }
}
pub struct Hati;
impl NaifId for Hati {
    fn id() -> i32 {
        643i32
    }
}
pub struct Hyrrokkin;
impl NaifId for Hyrrokkin {
    fn id() -> i32 {
        644i32
    }
}
pub struct Kari;
impl NaifId for Kari {
    fn id() -> i32 {
        645i32
    }
}
pub struct Loge;
impl NaifId for Loge {
    fn id() -> i32 {
        646i32
    }
}
pub struct Skoll;
impl NaifId for Skoll {
    fn id() -> i32 {
        647i32
    }
}
pub struct Surtur;
impl NaifId for Surtur {
    fn id() -> i32 {
        648i32
    }
}
pub struct Anthe;
impl NaifId for Anthe {
    fn id() -> i32 {
        649i32
    }
}
impl Ellipsoid for Anthe {
    fn polar_radius() -> f64 {
        0.5f64
    }
    fn mean_radius() -> f64 {
        0.5f64
    }
}
impl TriAxial for Anthe {
    fn subplanetary_radius() -> f64 {
        0.5f64
    }
    fn along_orbit_radius() -> f64 {
        0.5f64
    }
}
pub struct Jarnsaxa;
impl NaifId for Jarnsaxa {
    fn id() -> i32 {
        650i32
    }
}
pub struct Greip;
impl NaifId for Greip {
    fn id() -> i32 {
        651i32
    }
}
pub struct Tarqeq;
impl NaifId for Tarqeq {
    fn id() -> i32 {
        652i32
    }
}
pub struct Aegaeon;
impl NaifId for Aegaeon {
    fn id() -> i32 {
        653i32
    }
}
impl Ellipsoid for Aegaeon {
    fn polar_radius() -> f64 {
        0.2f64
    }
    fn mean_radius() -> f64 {
        0.3833333333333333f64
    }
}
impl TriAxial for Aegaeon {
    fn subplanetary_radius() -> f64 {
        0.7f64
    }
    fn along_orbit_radius() -> f64 {
        0.25f64
    }
}
pub struct Ariel;
impl NaifId for Ariel {
    fn id() -> i32 {
        701i32
    }
}
impl Ellipsoid for Ariel {
    fn polar_radius() -> f64 {
        577.7f64
    }
    fn mean_radius() -> f64 {
        578.9f64
    }
}
impl TriAxial for Ariel {
    fn subplanetary_radius() -> f64 {
        581.1f64
    }
    fn along_orbit_radius() -> f64 {
        577.9f64
    }
}
impl PointMass for Ariel {
    fn gravitational_parameter() -> f64 {
        83.46344431770477f64
    }
}
pub struct Umbriel;
impl NaifId for Umbriel {
    fn id() -> i32 {
        702i32
    }
}
impl Ellipsoid for Umbriel {
    fn polar_radius() -> f64 {
        584.7f64
    }
    fn mean_radius() -> f64 {
        584.7f64
    }
}
impl TriAxial for Umbriel {
    fn subplanetary_radius() -> f64 {
        584.7f64
    }
    fn along_orbit_radius() -> f64 {
        584.7f64
    }
}
impl PointMass for Umbriel {
    fn gravitational_parameter() -> f64 {
        85.09338094489388f64
    }
}
pub struct Titania;
impl NaifId for Titania {
    fn id() -> i32 {
        703i32
    }
}
impl Ellipsoid for Titania {
    fn polar_radius() -> f64 {
        788.9f64
    }
    fn mean_radius() -> f64 {
        788.9f64
    }
}
impl TriAxial for Titania {
    fn subplanetary_radius() -> f64 {
        788.9f64
    }
    fn along_orbit_radius() -> f64 {
        788.9f64
    }
}
impl PointMass for Titania {
    fn gravitational_parameter() -> f64 {
        226.9437003741248f64
    }
}
pub struct Oberon;
impl NaifId for Oberon {
    fn id() -> i32 {
        704i32
    }
}
impl Ellipsoid for Oberon {
    fn polar_radius() -> f64 {
        761.4f64
    }
    fn mean_radius() -> f64 {
        761.4f64
    }
}
impl TriAxial for Oberon {
    fn subplanetary_radius() -> f64 {
        761.4f64
    }
    fn along_orbit_radius() -> f64 {
        761.4f64
    }
}
impl PointMass for Oberon {
    fn gravitational_parameter() -> f64 {
        205.3234302535623f64
    }
}
pub struct Miranda;
impl NaifId for Miranda {
    fn id() -> i32 {
        705i32
    }
}
impl Ellipsoid for Miranda {
    fn polar_radius() -> f64 {
        232.9f64
    }
    fn mean_radius() -> f64 {
        235.83333333333334f64
    }
}
impl TriAxial for Miranda {
    fn subplanetary_radius() -> f64 {
        240.4f64
    }
    fn along_orbit_radius() -> f64 {
        234.2f64
    }
}
impl PointMass for Miranda {
    fn gravitational_parameter() -> f64 {
        4.3195168992321f64
    }
}
pub struct Cordelia;
impl NaifId for Cordelia {
    fn id() -> i32 {
        706i32
    }
}
impl Ellipsoid for Cordelia {
    fn polar_radius() -> f64 {
        13f64
    }
    fn mean_radius() -> f64 {
        13f64
    }
}
impl TriAxial for Cordelia {
    fn subplanetary_radius() -> f64 {
        13f64
    }
    fn along_orbit_radius() -> f64 {
        13f64
    }
}
pub struct Ophelia;
impl NaifId for Ophelia {
    fn id() -> i32 {
        707i32
    }
}
impl Ellipsoid for Ophelia {
    fn polar_radius() -> f64 {
        15f64
    }
    fn mean_radius() -> f64 {
        15f64
    }
}
impl TriAxial for Ophelia {
    fn subplanetary_radius() -> f64 {
        15f64
    }
    fn along_orbit_radius() -> f64 {
        15f64
    }
}
pub struct Bianca;
impl NaifId for Bianca {
    fn id() -> i32 {
        708i32
    }
}
impl Ellipsoid for Bianca {
    fn polar_radius() -> f64 {
        21f64
    }
    fn mean_radius() -> f64 {
        21f64
    }
}
impl TriAxial for Bianca {
    fn subplanetary_radius() -> f64 {
        21f64
    }
    fn along_orbit_radius() -> f64 {
        21f64
    }
}
pub struct Cressida;
impl NaifId for Cressida {
    fn id() -> i32 {
        709i32
    }
}
impl Ellipsoid for Cressida {
    fn polar_radius() -> f64 {
        31f64
    }
    fn mean_radius() -> f64 {
        31f64
    }
}
impl TriAxial for Cressida {
    fn subplanetary_radius() -> f64 {
        31f64
    }
    fn along_orbit_radius() -> f64 {
        31f64
    }
}
pub struct Desdemona;
impl NaifId for Desdemona {
    fn id() -> i32 {
        710i32
    }
}
impl Ellipsoid for Desdemona {
    fn polar_radius() -> f64 {
        27f64
    }
    fn mean_radius() -> f64 {
        27f64
    }
}
impl TriAxial for Desdemona {
    fn subplanetary_radius() -> f64 {
        27f64
    }
    fn along_orbit_radius() -> f64 {
        27f64
    }
}
pub struct Juliet;
impl NaifId for Juliet {
    fn id() -> i32 {
        711i32
    }
}
impl Ellipsoid for Juliet {
    fn polar_radius() -> f64 {
        42f64
    }
    fn mean_radius() -> f64 {
        42f64
    }
}
impl TriAxial for Juliet {
    fn subplanetary_radius() -> f64 {
        42f64
    }
    fn along_orbit_radius() -> f64 {
        42f64
    }
}
pub struct Portia;
impl NaifId for Portia {
    fn id() -> i32 {
        712i32
    }
}
impl Ellipsoid for Portia {
    fn polar_radius() -> f64 {
        54f64
    }
    fn mean_radius() -> f64 {
        54f64
    }
}
impl TriAxial for Portia {
    fn subplanetary_radius() -> f64 {
        54f64
    }
    fn along_orbit_radius() -> f64 {
        54f64
    }
}
pub struct Rosalind;
impl NaifId for Rosalind {
    fn id() -> i32 {
        713i32
    }
}
impl Ellipsoid for Rosalind {
    fn polar_radius() -> f64 {
        27f64
    }
    fn mean_radius() -> f64 {
        27f64
    }
}
impl TriAxial for Rosalind {
    fn subplanetary_radius() -> f64 {
        27f64
    }
    fn along_orbit_radius() -> f64 {
        27f64
    }
}
pub struct Belinda;
impl NaifId for Belinda {
    fn id() -> i32 {
        714i32
    }
}
impl Ellipsoid for Belinda {
    fn polar_radius() -> f64 {
        33f64
    }
    fn mean_radius() -> f64 {
        33f64
    }
}
impl TriAxial for Belinda {
    fn subplanetary_radius() -> f64 {
        33f64
    }
    fn along_orbit_radius() -> f64 {
        33f64
    }
}
pub struct Puck;
impl NaifId for Puck {
    fn id() -> i32 {
        715i32
    }
}
impl Ellipsoid for Puck {
    fn polar_radius() -> f64 {
        77f64
    }
    fn mean_radius() -> f64 {
        77f64
    }
}
impl TriAxial for Puck {
    fn subplanetary_radius() -> f64 {
        77f64
    }
    fn along_orbit_radius() -> f64 {
        77f64
    }
}
pub struct Caliban;
impl NaifId for Caliban {
    fn id() -> i32 {
        716i32
    }
}
pub struct Sycorax;
impl NaifId for Sycorax {
    fn id() -> i32 {
        717i32
    }
}
pub struct Prospero;
impl NaifId for Prospero {
    fn id() -> i32 {
        718i32
    }
}
pub struct Setebos;
impl NaifId for Setebos {
    fn id() -> i32 {
        719i32
    }
}
pub struct Stephano;
impl NaifId for Stephano {
    fn id() -> i32 {
        720i32
    }
}
pub struct Trinculo;
impl NaifId for Trinculo {
    fn id() -> i32 {
        721i32
    }
}
pub struct Francisco;
impl NaifId for Francisco {
    fn id() -> i32 {
        722i32
    }
}
pub struct Margaret;
impl NaifId for Margaret {
    fn id() -> i32 {
        723i32
    }
}
pub struct Ferdinand;
impl NaifId for Ferdinand {
    fn id() -> i32 {
        724i32
    }
}
pub struct Perdita;
impl NaifId for Perdita {
    fn id() -> i32 {
        725i32
    }
}
pub struct Mab;
impl NaifId for Mab {
    fn id() -> i32 {
        726i32
    }
}
pub struct Cupid;
impl NaifId for Cupid {
    fn id() -> i32 {
        727i32
    }
}
pub struct Triton;
impl NaifId for Triton {
    fn id() -> i32 {
        801i32
    }
}
impl Ellipsoid for Triton {
    fn polar_radius() -> f64 {
        1352.6f64
    }
    fn mean_radius() -> f64 {
        1352.6f64
    }
}
impl TriAxial for Triton {
    fn subplanetary_radius() -> f64 {
        1352.6f64
    }
    fn along_orbit_radius() -> f64 {
        1352.6f64
    }
}
impl PointMass for Triton {
    fn gravitational_parameter() -> f64 {
        1428.495462910464f64
    }
}
pub struct Nereid;
impl NaifId for Nereid {
    fn id() -> i32 {
        802i32
    }
}
impl Ellipsoid for Nereid {
    fn polar_radius() -> f64 {
        170f64
    }
    fn mean_radius() -> f64 {
        170f64
    }
}
impl TriAxial for Nereid {
    fn subplanetary_radius() -> f64 {
        170f64
    }
    fn along_orbit_radius() -> f64 {
        170f64
    }
}
pub struct Naiad;
impl NaifId for Naiad {
    fn id() -> i32 {
        803i32
    }
}
impl Ellipsoid for Naiad {
    fn polar_radius() -> f64 {
        29f64
    }
    fn mean_radius() -> f64 {
        29f64
    }
}
impl TriAxial for Naiad {
    fn subplanetary_radius() -> f64 {
        29f64
    }
    fn along_orbit_radius() -> f64 {
        29f64
    }
}
impl PointMass for Naiad {
    fn gravitational_parameter() -> f64 {
        0.008530281246540886f64
    }
}
pub struct Thalassa;
impl NaifId for Thalassa {
    fn id() -> i32 {
        804i32
    }
}
impl Ellipsoid for Thalassa {
    fn polar_radius() -> f64 {
        40f64
    }
    fn mean_radius() -> f64 {
        40f64
    }
}
impl TriAxial for Thalassa {
    fn subplanetary_radius() -> f64 {
        40f64
    }
    fn along_orbit_radius() -> f64 {
        40f64
    }
}
impl PointMass for Thalassa {
    fn gravitational_parameter() -> f64 {
        0.0235887319799217f64
    }
}
pub struct Despina;
impl NaifId for Despina {
    fn id() -> i32 {
        805i32
    }
}
impl Ellipsoid for Despina {
    fn polar_radius() -> f64 {
        74f64
    }
    fn mean_radius() -> f64 {
        74f64
    }
}
impl TriAxial for Despina {
    fn subplanetary_radius() -> f64 {
        74f64
    }
    fn along_orbit_radius() -> f64 {
        74f64
    }
}
impl PointMass for Despina {
    fn gravitational_parameter() -> f64 {
        0.1167318403814998f64
    }
}
pub struct Galatea;
impl NaifId for Galatea {
    fn id() -> i32 {
        806i32
    }
}
impl Ellipsoid for Galatea {
    fn polar_radius() -> f64 {
        79f64
    }
    fn mean_radius() -> f64 {
        79f64
    }
}
impl TriAxial for Galatea {
    fn subplanetary_radius() -> f64 {
        79f64
    }
    fn along_orbit_radius() -> f64 {
        79f64
    }
}
impl PointMass for Galatea {
    fn gravitational_parameter() -> f64 {
        0.189898503906069f64
    }
}
pub struct Larissa;
impl NaifId for Larissa {
    fn id() -> i32 {
        807i32
    }
}
impl Ellipsoid for Larissa {
    fn polar_radius() -> f64 {
        96f64
    }
    fn mean_radius() -> f64 {
        96f64
    }
}
impl TriAxial for Larissa {
    fn subplanetary_radius() -> f64 {
        96f64
    }
    fn along_orbit_radius() -> f64 {
        96f64
    }
}
impl PointMass for Larissa {
    fn gravitational_parameter() -> f64 {
        0.2548437405693583f64
    }
}
pub struct Proteus;
impl NaifId for Proteus {
    fn id() -> i32 {
        808i32
    }
}
impl Ellipsoid for Proteus {
    fn polar_radius() -> f64 {
        201f64
    }
    fn mean_radius() -> f64 {
        209f64
    }
}
impl TriAxial for Proteus {
    fn subplanetary_radius() -> f64 {
        218f64
    }
    fn along_orbit_radius() -> f64 {
        208f64
    }
}
impl PointMass for Proteus {
    fn gravitational_parameter() -> f64 {
        2.583422379120727f64
    }
}
pub struct Halimede;
impl NaifId for Halimede {
    fn id() -> i32 {
        809i32
    }
}
pub struct Psamathe;
impl NaifId for Psamathe {
    fn id() -> i32 {
        810i32
    }
}
pub struct Sao;
impl NaifId for Sao {
    fn id() -> i32 {
        811i32
    }
}
pub struct Laomedeia;
impl NaifId for Laomedeia {
    fn id() -> i32 {
        812i32
    }
}
pub struct Neso;
impl NaifId for Neso {
    fn id() -> i32 {
        813i32
    }
}
pub struct Charon;
impl NaifId for Charon {
    fn id() -> i32 {
        901i32
    }
}
impl Ellipsoid for Charon {
    fn polar_radius() -> f64 {
        606f64
    }
    fn mean_radius() -> f64 {
        606f64
    }
}
impl TriAxial for Charon {
    fn subplanetary_radius() -> f64 {
        606f64
    }
    fn along_orbit_radius() -> f64 {
        606f64
    }
}
impl PointMass for Charon {
    fn gravitational_parameter() -> f64 {
        105.8799888601881f64
    }
}
pub struct Nix;
impl NaifId for Nix {
    fn id() -> i32 {
        902i32
    }
}
impl PointMass for Nix {
    fn gravitational_parameter() -> f64 {
        0.00304817564816976f64
    }
}
pub struct Hydra;
impl NaifId for Hydra {
    fn id() -> i32 {
        903i32
    }
}
impl PointMass for Hydra {
    fn gravitational_parameter() -> f64 {
        0.003211039206155255f64
    }
}
pub struct Kerberos;
impl NaifId for Kerberos {
    fn id() -> i32 {
        904i32
    }
}
impl PointMass for Kerberos {
    fn gravitational_parameter() -> f64 {
        0.001110040850536676f64
    }
}
pub struct Styx;
impl NaifId for Styx {
    fn id() -> i32 {
        905i32
    }
}
impl PointMass for Styx {
    fn gravitational_parameter() -> f64 {
        0f64
    }
}
