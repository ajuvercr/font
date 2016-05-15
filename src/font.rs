/// A font.
pub struct Font {
    /// The number of units per em.
    pub units_per_em: usize,
    /// The ascender line relative to the base line.
    pub ascender: isize,
    /// The descender line relative to the base line.
    pub descender: isize,
}