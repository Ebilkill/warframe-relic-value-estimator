use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RelicDrop {
    pub tier: RelicTier, // This is Lith, Meso, Neo, Axi
    pub name: String, // This is for instance A1
    pub rarity: RelicRarity, // This is Common, Uncommon, Rare (bronze, silver, gold)
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RelicTier {
    Lith, Meso, Neo, Axi
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RelicRarity {
    Common, Uncommon, Rare
}

#[derive(Debug, Copy, Clone)]
pub enum RelicUpgrade {
    Intact, Exceptional, Flawless, Radiant
}

impl RelicDrop {
    pub fn try_from_string(s: &str) -> Result<RelicDrop, String> {
        let parts: Vec<&str> = s.trim().split(' ').collect();

        if parts.len() != 3 {
            return Err("Incorrect amount of Relic string components".to_owned());
        }

        let tier = RelicTier::from_string(parts[0])?;
        let name = parts[1].to_owned();
        let rarity = RelicRarity::from_string(parts[2])?;

        Ok(RelicDrop {
            tier,
            name,
            rarity,
        })
    }

    pub fn from_string(s: &str) -> RelicDrop {
        RelicDrop::try_from_string(s).unwrap()
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.tier.to_string(), self.name, self.rarity.to_string())
    }
}

impl RelicTier {
    pub fn from_string(s: &str) -> Result<RelicTier, String> {
        use self::RelicTier::*;
        match s {
            "Lith" => Ok(Lith),
            "Meso" => Ok(Meso),
            "Neo"  => Ok(Neo),
            "Axi"  => Ok(Axi),
            _      => Err(format!("Invalid relic tier: {}", s).to_owned())
        }
    }

    pub fn to_string(&self) -> String {
        use self::RelicTier::*;
        match self {
            &Lith => "Lith",
            &Meso => "Meso",
            &Neo  => "Neo",
            &Axi  => "Axi"
        }.to_owned()
    }
}

impl RelicRarity {
    pub fn from_string(s: &str) -> Result<RelicRarity, String> {
        use self::RelicRarity::*;
        match s {
            "Common"   => Ok(Common),
            "Uncommon" => Ok(Uncommon),
            "Rare"     => Ok(Rare),
            _          => Err(format!("Invalid relic rarity: {}", s).to_owned())
        }
    }

    pub fn to_string(&self) -> String {
        use self::RelicRarity::*;
        match self {
            &Common   => "Common",
            &Uncommon => "Uncommon",
            &Rare     => "Rare",
        }.to_owned()
    }

    /// Returns the chance that we get this relic drop based on the upgrade level of the relic,
    /// i.e. Intact, Exceptional, Flawless, Radiant.
    ///
    /// # Note
    ///
    /// This function returns the chance in PERCENTAGES.
    pub fn chance_for_upgrade(&self, upgrade: RelicUpgrade) -> f64 {
        use self::RelicRarity::*;
        use self::RelicUpgrade::*;
        match self {
            &Common   => match upgrade {
                Intact      => 25.33,
                Exceptional => 23.33,
                Flawless    => 20.0,
                Radiant     => 16.67
            },
            &Uncommon => match upgrade {
                Intact      => 11.0,
                Exceptional => 13.0,
                Flawless    => 17.0,
                Radiant     => 20.0
            },
            &Rare     => match upgrade {
                Intact      => 2.0,
                Exceptional => 4.0,
                Flawless    => 6.0,
                Radiant     => 10.0
            }
        }
    }
}

impl RelicUpgrade {
    pub fn to_string(&self) -> String {
        use self::RelicUpgrade::*;
        match self {
            &Intact => "intact",
            &Exceptional => "exceptional",
            &Flawless => "flawless",
            &Radiant => "radiant"
        }.to_owned()
    }

    pub fn all_upgrade_tiers() -> Vec<RelicUpgrade> {
        use self::RelicUpgrade::*;
        vec![Intact, Exceptional, Flawless, Radiant]
    }
}

