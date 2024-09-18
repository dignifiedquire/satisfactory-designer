use egui::Color32;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum Building {
    Miner(Miner),
    Smelter(Smelter),
    Splitter(Splitter),
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Splitter {}

impl Splitter {
    pub fn header_image(&self) -> &'static str {
        "file://assets/img/20px-Conveyor_Splitter.png"
    }

    pub fn name(&self) -> String {
        "Splitter".to_string()
    }

    pub fn description(&self) -> String {
        "Splits things".to_string()
    }

    pub fn num_inputs(&self) -> usize {
        1
    }

    pub fn num_outputs(&self) -> usize {
        3
    }

    pub fn input_material(&self) -> Option<Material> {
        None
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Miner {
    pub resource: Option<ResourceType>,
    pub resource_purity: ResourcePurity,
    pub level: MinerLevel,
    pub speed: f32,
}

impl Default for Miner {
    fn default() -> Self {
        Self {
            resource: None,
            resource_purity: ResourcePurity::Normal,
            level: MinerLevel::Mk1,
            speed: 100.,
        }
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum MinerLevel {
    Mk1,
    Mk2,
    Mk3,
}

impl MinerLevel {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Mk1 => "Mk.1",
            Self::Mk2 => "Mk.2",
            Self::Mk3 => "Mk.3",
        }
    }

    pub fn mining_speed(&self) -> usize {
        match self {
            Self::Mk1 => 60,
            Self::Mk2 => 120,
            Self::Mk3 => 240,
        }
    }
}

impl Miner {
    pub fn header_image(&self) -> &'static str {
        "file://assets/img/20px-Miner_Mk.1.png"
    }

    pub fn name(&self) -> String {
        match &self.resource {
            Some(r) => format!(
                "Miner {:?} ({} {})",
                self.level,
                r.name(),
                self.resource_purity.name()
            ),
            None => "Resource Node".to_string(),
        }
    }

    pub fn description(&self) -> String {
        "Mines things".to_string()
    }

    pub fn available_purities(&self) -> Vec<ResourcePurity> {
        vec![
            ResourcePurity::Impure,
            ResourcePurity::Normal,
            ResourcePurity::Pure,
        ]
    }

    pub fn available_levels(&self) -> Vec<MinerLevel> {
        vec![MinerLevel::Mk1, MinerLevel::Mk2, MinerLevel::Mk3]
    }

    pub fn available_resources(&self) -> Vec<ResourceType> {
        use ResourceType::*;
        vec![
            Bauxite,
            CateriumOre,
            CopperOre,
            IronOre,
            Limestone,
            RawQuartz,
            SamOre,
            Sulfur,
            Uranium,
        ]
    }

    pub fn num_inputs(&self) -> usize {
        0
    }

    pub fn num_outputs(&self) -> usize {
        1
    }

    pub fn input_speed(&self) -> usize {
        0
    }

    pub fn output_speed(&self) -> usize {
        match self.resource {
            Some(_) => {
                // (Mining Speed) in items/min = (Purity Modifier) * (Overclock percentage) / 100 * (Default Mining Speed) items/min
                let val = self.resource_purity.modifier()
                    * (self.speed / 100.)
                    * self.level.mining_speed() as f32;
                val.round() as usize
            }
            None => 0,
        }
    }

    pub fn input_material(&self) -> Option<Material> {
        None
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum ResourcePurity {
    Impure,
    Normal,
    Pure,
}

impl ResourcePurity {
    pub fn modifier(&self) -> f32 {
        match self {
            Self::Impure => 0.5,
            Self::Normal => 1.,
            Self::Pure => 2.,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Impure => "Impure",
            Self::Normal => "Normal",
            Self::Pure => "Pure",
        }
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum ResourceType {
    Bauxite,
    CateriumOre,
    CopperOre,
    IronOre,
    Limestone,
    RawQuartz,
    SamOre,
    Sulfur,
    Uranium,
}

impl ResourceType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Bauxite => "Bauxite",
            Self::CateriumOre => "Caterium Ore",
            Self::CopperOre => "Copper Ore",
            Self::IronOre => "Iron Ore",
            Self::Limestone => "Limestone",
            Self::RawQuartz => "Raw Quartz",
            Self::SamOre => "SAM Ore",
            Self::Sulfur => "Sulfur",
            Self::Uranium => "Uranium",
        }
    }

    pub fn output_material(&self) -> Material {
        match self {
            Self::Bauxite => Material::Bauxite,
            Self::CateriumOre => Material::CateriumOre,
            Self::CopperOre => Material::CopperOre,
            Self::IronOre => Material::IronOre,
            Self::Limestone => Material::Limestone,
            Self::RawQuartz => Material::RawQuartz,
            Self::SamOre => Material::SamOre,
            Self::Sulfur => Material::Sulfur,
            Self::Uranium => Material::Uranium,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum SmelterRecipie {
    CateriumIngot,
    CopperIngot,
    IronIngot,
    PureAluminiumIngot,
}

impl SmelterRecipie {
    pub fn name(&self) -> String {
        match self {
            SmelterRecipie::CateriumIngot => "Caterium Ingot".to_string(),
            SmelterRecipie::CopperIngot => "Copper Ingot".to_string(),
            SmelterRecipie::IronIngot => "Iron Ingot".to_string(),
            SmelterRecipie::PureAluminiumIngot => "Pure Aluminium Ingot".to_string(),
        }
    }

    pub fn output_color(&self) -> Color32 {
        self.output_material().color()
    }

    pub fn input_material(&self) -> Material {
        match self {
            Self::CateriumIngot => Material::CateriumOre,
            Self::CopperIngot => Material::CopperOre,
            Self::IronIngot => Material::IronOre,
            Self::PureAluminiumIngot => Material::AluminiumScrap,
        }
    }

    pub fn output_material(&self) -> Material {
        match self {
            Self::CateriumIngot => Material::CateriumIngot,
            Self::CopperIngot => Material::CopperIngot,
            Self::IronIngot => Material::IronIngot,
            Self::PureAluminiumIngot => Material::AluminiumIngot,
        }
    }

    pub fn input_speed(&self) -> usize {
        match self {
            Self::CateriumIngot => 45,
            Self::CopperIngot => 30,
            Self::IronIngot => 30,
            Self::PureAluminiumIngot => 60,
        }
    }

    pub fn output_speed(&self, input_size: usize) -> usize {
        if input_size == 0 {
            return 0;
        }

        let (duration, output_size, input_base) = match self {
            Self::CateriumIngot => (4., 1., 3.),
            Self::CopperIngot => (2., 1., 1.),
            Self::IronIngot => (2., 1., 1.),
            Self::PureAluminiumIngot => (2., 1., 2.),
        };

        let input_size = (input_size as f32 / 60.) * duration;

        // 45/60 * 4secs = 3
        let a = if input_size < input_base {
            input_size / input_base
        } else {
            1.
        };

        // 60/4 * 1 = 15
        let b = (60. / duration) * a;

        b.round() as usize
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum Material {
    CopperOre,
    IronOre,
    CateriumOre,
    CopperIngot,
    IronIngot,
    CateriumIngot,
    AluminiumScrap,
    AluminiumIngot,
    Bauxite,
    Limestone,
    RawQuartz,
    SamOre,
    Sulfur,
    Uranium,
}

impl Material {
    pub fn color(&self) -> Color32 {
        // Colors based on https://www.reddit.com/r/SatisfactoryGame/comments/154vft6/vencams_colour_list_25/
        match self {
            Self::CopperOre => Color32::from_hex("#BD4C39").unwrap(),
            Self::IronOre => Color32::from_hex("#8E5C5C").unwrap(),
            Self::CateriumOre => Color32::from_hex("#E2B148").unwrap(),
            Self::CopperIngot => Color32::from_hex("#A56355").unwrap(),
            Self::IronIngot => Color32::from_hex("#989A9D").unwrap(),
            Self::CateriumIngot => Color32::from_hex("#CCA566").unwrap(),
            Self::AluminiumScrap => Color32::from_hex("#BCC0C9").unwrap(),
            Self::AluminiumIngot => Color32::from_hex("#D2D3D4").unwrap(),
            Self::Bauxite => Color32::from_hex("#CD7660").unwrap(),
            Self::Limestone => Color32::from_hex("#C8BFA7").unwrap(),
            Self::RawQuartz => Color32::from_hex("#F177B5").unwrap(),
            Self::SamOre => Color32::from_hex("#AE1CD7").unwrap(), // TODO: better color
            Self::Sulfur => Color32::from_hex("#FCDC48").unwrap(),
            Self::Uranium => Color32::from_hex("#88D288").unwrap(),
        }
    }
}

impl Building {
    pub fn input_material(&self) -> Option<Material> {
        match self {
            Self::Miner(m) => m.input_material(),
            Self::Smelter(s) => s.input_material(),
            Self::Splitter(s) => s.input_material(),
        }
    }

    pub fn outputs(&self) -> usize {
        match self {
            Self::Miner(m) => m.num_outputs(),
            Self::Smelter(s) => s.num_outputs(),
            Self::Splitter(s) => s.num_outputs(),
        }
    }

    pub fn inputs(&self) -> usize {
        match self {
            Self::Miner(m) => m.num_inputs(),
            Self::Smelter(s) => s.num_inputs(),
            Self::Splitter(s) => s.num_inputs(),
        }
    }

    pub fn name(&self) -> String {
        match self {
            Self::Miner(m) => m.name(),
            Self::Smelter(s) => s.name(),
            Self::Splitter(s) => s.name(),
        }
    }

    pub fn description(&self) -> String {
        match self {
            Self::Miner(m) => m.description(),
            Self::Smelter(s) => s.description(),
            Self::Splitter(s) => s.description(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Smelter {
    pub recipie: Option<SmelterRecipie>,
    pub speed: f32,
    pub amplified: bool,
}

impl Default for Smelter {
    fn default() -> Self {
        Self {
            recipie: None,
            speed: 100.,
            amplified: false,
        }
    }
}

impl Smelter {
    pub fn header_image(&self) -> &'static str {
        "file://assets/img/20px-Smelter.png"
    }

    pub fn available_recipies(&self) -> Vec<SmelterRecipie> {
        vec![
            SmelterRecipie::CateriumIngot,
            SmelterRecipie::CopperIngot,
            SmelterRecipie::IronIngot,
            SmelterRecipie::PureAluminiumIngot,
        ]
    }

    pub fn name(&self) -> String {
        match &self.recipie {
            Some(r) => format!("Smelter ({})", r.name()),
            None => "Smelter".to_string(),
        }
    }

    pub fn description(&self) -> String {
        "Smelts things".to_string()
    }

    pub fn num_inputs(&self) -> usize {
        1
    }

    pub fn num_outputs(&self) -> usize {
        1
    }

    pub fn input_speed(&self) -> usize {
        let base = self
            .recipie
            .as_ref()
            .map(|r| r.input_speed())
            .unwrap_or_default();
        (base as f32 * (self.speed / 100.)).round() as usize
    }

    pub fn output_speed(&self, input_size: usize) -> usize {
        let base = self
            .recipie
            .as_ref()
            .map(|r| r.output_speed(input_size))
            .unwrap_or_default();
        let amplification = if self.amplified { 2. } else { 1. };

        // TODO: take speed into account for input_size

        (base as f32 * (self.speed / 100.) * amplification).round() as usize
    }

    pub fn input_material(&self) -> Option<Material> {
        match self.recipie {
            Some(ref r) => Some(r.input_material()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_speed() {
        assert_eq!(SmelterRecipie::CateriumIngot.output_speed(0), 0);
        assert_eq!(SmelterRecipie::CateriumIngot.output_speed(10), 3);
        assert_eq!(SmelterRecipie::CateriumIngot.output_speed(45), 15);
        assert_eq!(SmelterRecipie::CateriumIngot.output_speed(60), 15);

        assert_eq!(SmelterRecipie::IronIngot.output_speed(0), 0);
        assert_eq!(SmelterRecipie::IronIngot.output_speed(10), 10);
        assert_eq!(SmelterRecipie::IronIngot.output_speed(30), 30);
        assert_eq!(SmelterRecipie::IronIngot.output_speed(60), 30);

        assert_eq!(SmelterRecipie::PureAluminiumIngot.output_speed(0), 0);
        assert_eq!(SmelterRecipie::PureAluminiumIngot.output_speed(10), 5);
        assert_eq!(SmelterRecipie::PureAluminiumIngot.output_speed(30), 15);
        assert_eq!(SmelterRecipie::PureAluminiumIngot.output_speed(60), 30);
        assert_eq!(SmelterRecipie::PureAluminiumIngot.output_speed(120), 30);
    }
}
