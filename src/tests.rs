use crate::{Profile, analyze::detect};

#[test]
fn correct_detection() {
    let sample = Profile {
        name: String::from("french"),
        vvvp: 1.2998,
        vvcp: 10.7661,
        vcvp: 14.1201,
        vccp: 16.6853,
        cvvp: 12.8101,
        cvcp: 28.4311,
        ccvp: 14.3076,
        cccp: 1.5798,
        cvrp: 123.2194,
    };
    let french_average = Profile {
        name: String::from("french"),
        vvvp: 1.2009,
        vvcp: 11.2616,
        vcvp: 13.9650,
        vccp: 16.6226,
        cvvp: 13.2290,
        cvcp: 27.2125,
        ccvp: 14.4992,
        cccp: 1.8485,
        cvrp: 122.3352,
    };
    let english_average = Profile {
        name: String::from("english"),
        vvvp: 0.1506,
        vvcp: 6.6830,
        vcvp: 11.1354,
        vccp: 20.2217,
        cvvp: 7.1086,
        cvcp: 33.5556,
        ccvp: 17.2219,
        cccp: 3.9790,
        cvrp: 164.3126,
    };

    let german_average = Profile {
        name: String::from("german"),
        vvvp: 0.1988,
        vvcp: 6.3473,
        vcvp: 8.9321,
        vccp: 21.8884,
        cvvp: 6.4674,
        cvcp: 33.6667,
        ccvp: 15.3587,
        cccp: 7.1683,
        cvrp: 164.2006,
    };
    let profiles = [french_average, english_average, german_average];
    let (_, detection_result) = detect(sample, &profiles);

    assert_eq!(detection_result, String::from("french"))
}
