use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstellationStar {
    pub id: String,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub desc: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstellationData {
    pub id: String,
    pub label: String,
    pub stars: Vec<ConstellationStar>,
    pub links: Vec<(usize, usize)>,
    #[serde(default)]
    pub season: Option<String>,
    #[serde(default)]
    pub position: Option<String>,
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub lore: Option<String>,
}

pub fn get_all_constellations() -> Vec<ConstellationData> {
    let raw = include_str!("../../../constellation/data/constellations.json");
    let mut list: Vec<ConstellationData> = serde_json::from_str(raw).unwrap_or_default();
    
    for c in &mut list {
        let (season, pos, kind, lore) = match c.id.as_str() {
            "aries" => ("autumn", "equatorial", "zodiac", "The Ram — Golden Fleece, leader of the ancient zodiac flock"),
            "taurus" => ("winter", "equatorial", "zodiac", "The Bull — Eye Aldebaran, Seven Sisters Pleiades, celestial horn"),
            "gemini" => ("winter", "north", "zodiac", "The Twins — Castor and Pollux, protectors of seafarers and wanderers"),
            "cancer" => ("spring", "north", "zodiac", "The Crab — Houses Praesepe Beehive Cluster, portal of mortal descent"),
            "leo" => ("spring", "north", "zodiac", "The Lion — Heart Regulus, celestial mane, harbinger of summer solstice"),
            "virgo" => ("spring", "equatorial", "zodiac", "The Maiden — Diamond Spica, celestial ear of wheat, justice maiden"),
            "libra" => ("summer", "equatorial", "zodiac", "The Scales — Emerald Zubeneschamali, cosmic balance of day and night"),
            "scorpius" => ("summer", "south", "zodiac", "The Scorpion — Supergiant Antares, curved sting reaching into Milky Way"),
            "sagittarius" => ("summer", "south", "zodiac", "The Archer — Celestial Teapot pointing towards the supermassive Galactic Center"),
            "capricornus" => ("autumn", "south", "zodiac", "The Sea Goat — Ancient gateway of the gods and winter solstice in antiquity"),
            "aquarius" => ("autumn", "equatorial", "zodiac", "The Water Bearer — Sadalsuud, pouring the heavenly river into the abyss"),
            "pisces" => ("autumn", "north", "zodiac", "The Fishes — Joined by celestial ribbon, marking the vernal equinox"),
            
            "orion" => ("winter", "equatorial", "asterism", "The Hunter — Belt Alnitak-Alnilam-Mintaka, glowing nebula M42, Betelgeuse & Rigel"),
            "ursa_major" => ("spring", "north", "asterism", "Big Dipper / Great Bear — Merak and Dubhe pointer stars towards the North Star"),
            "ursa_minor" => ("winter", "north", "asterism", "Little Dipper — Contains Polaris, eternal anchor of northern celestial navigation"),
            "cassiopeia" => ("autumn", "north", "asterism", "The Queen — Famous W / M asterism circling the northern celestial pole"),
            "cygnus" => ("summer", "north", "asterism", "The Swan / Northern Cross — Brilliant Deneb soaring down the galactic plane"),
            "lyra" => ("summer", "north", "asterism", "The Harp / Lyre — Sapphire Vega, apex of the celebrated Summer Triangle"),
            "pegasus" => ("autumn", "north", "asterism", "The Winged Horse — Great Square of Pegasus guiding navigators across autumn sky"),
            "andromeda" => ("autumn", "north", "asterism", "The Chained Maiden — Host of Spiral Galaxy M31, 2.5 million light years away"),
            "draco" => ("summer", "north", "asterism", "The Dragon — Winding serpent guarding the North Celestial Pole and Thuban"),
            "crux" => ("spring", "south", "asterism", "The Southern Cross — Most distinct navigation asterism of the Southern Hemisphere"),
            "centaurus" => ("spring", "south", "asterism", "The Centaur — Contains Alpha & Beta Centauri, our closest stellar neighbors"),
            "hercules" => ("summer", "north", "asterism", "The Kneeler — Strongman holding Great Globular Cluster M13 with 300,000 stars"),
            "cepheus" => ("autumn", "north", "asterism", "The King — Crown of the North, host to Garnet Star and pulsating Delta Cephei"),
            "perseus" => ("winter", "north", "asterism", "The Hero — Algol Demon Star and radiant of annual August Perseid meteor showers"),
            "bootes" => ("spring", "north", "asterism", "The Herdsman — Guided by golden giant Arcturus, fourth brightest star in the sky"),
            "vela" => ("spring", "south", "asterism", "The Sails — Celestial canvas of ancient flagship constellation Argo Navis"),
            "puppis" => ("winter", "south", "asterism", "The Stern — Mighty stern deck of legendary Argo navigating the southern sky"),
            "pavo" => ("summer", "south", "asterism", "The Peacock — Splendid southern constellation featuring deep blue Peacock star"),
            "auriga" => ("winter", "north", "asterism", "The Charioteer — Crowned by blazing golden Capella, charioteer of the gods"),
            "lupus" => ("spring", "south", "asterism", "The Wolf — Ancient southern asterism perched beside the celestial Centaur"),
            _ => ("spring", "north", "asterism", "Classical celestial asterism recorded in astronomical catalogues"),
        };
        c.season = Some(season.to_string());
        c.position = Some(pos.to_string());
        c.kind = Some(kind.to_string());
        c.lore = Some(lore.to_string());
    }
    
    list
}
