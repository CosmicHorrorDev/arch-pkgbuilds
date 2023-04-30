use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Versions {
    versions: Vec<Version>,
}

#[derive(Deserialize, Debug)]
struct Version {
    num: semver::Version,
}

#[derive(Deserialize, Debug)]
struct AurResps {
    results: (AurResp,),
}

#[derive(Deserialize, Debug)]
struct AurResp {
    #[serde(rename = "Version")]
    version: String,
}

#[derive(Deserialize, Debug)]
struct GhResps(Vec<GhResp>);

#[derive(Deserialize, Debug)]
struct GhResp {
    name: semver::Version,
}

fn main() {
    let body: Versions = ureq::get("https://crates.io/api/v1/crates/cargo-insta/versions")
        .call()
        .unwrap()
        .into_json()
        .unwrap();
    let crates_version = body
        .versions
        .into_iter()
        .map(|version| version.num)
        .max()
        .expect("Way more than one version published");

    println!("Highest version: {crates_version}");

    let body: AurResps = ureq::get("https://aur.archlinux.org/rpc/v5/info/cargo-insta")
        .call()
        .unwrap()
        .into_json()
        .unwrap();
    let aur_version =
        semver::Version::parse(body.results.0.version.split_once('-').unwrap().0).unwrap();
    println!("AUR version: {aur_version}");

    let body: GhResps = ureq::get("https://api.github.com/repos/mitsuhiko/insta/tags")
        .set("X-GitHub-Api-Version", "2022-11-28")
        .call()
        .unwrap()
        .into_json()
        .unwrap();
    let gh_version = body
        .0
        .into_iter()
        .map(|resp| resp.name)
        .max()
        .expect("More than one tag published");
    println!("GitHub version: {gh_version}");
}
