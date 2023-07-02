use strum_macros::{Display, EnumString, EnumVariantNames};

#[derive(Clone, Display, Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum SubtitleExtension {
    Srt,
    Sub,
    Ssa,
    Smi,
    Vtt,
}