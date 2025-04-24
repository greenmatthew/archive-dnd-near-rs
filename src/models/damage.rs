#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DamageType {
    // From https://www.dndbeyond.com/sources/dnd/phb-2024/rules-glossary#DamageTypes
    Acid,        // Corrosive liquids, digestive enzymes
    Bludgeoning, // Blunt objects, constriction, falling
    Cold,        // Freezing water, icy blasts
    Fire,        // Flames, unbearable heat
    Force,       // Pure magical energy
    Lightning,   // Electricity
    Necrotic,    // Life-draining energy
    Piercing,    // Fangs, puncturing objects
    Poison,      // Toxic gas, venom
    Psychic,     // Mind-rending energy
    Radiant,     // Holy energy, searing radiation
    Slashing,    // Claws, cutting objects
    Thunder,     // Concussive sound
}

impl std::fmt::Display for DamageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Convert the Debug representation to a string and remove the enum prefix
        write!(f, "{self:?}")
    }
}
