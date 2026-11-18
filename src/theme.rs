//! Astryx Neutral semantic roles, mapped to native GPUI colors and pixels.
//!
//! Reference: Meta Astryx `f4fa3661a25403ded7ee4c4c2ac5c2a355eb74ef`,
//! `packages/themes/neutral/src/{neutralTheme,neutralPaletteRefs.generated}.ts`.
//! The full upstream MIT notice is preserved in `THIRD_PARTY_NOTICES.md`.

use gpui::{Hsla, rgb, rgba};

pub const BODY_SIZE: f32 = 14.;
pub const BODY_LINE_HEIGHT: f32 = 20.;
pub const SUPPORT_SIZE: f32 = 12.;
pub const SUPPORT_LINE_HEIGHT: f32 = 20.;
pub const HEADING_SIZE: f32 = 17.;
pub const HEADING_LINE_HEIGHT: f32 = 24.;
pub const CARD_RADIUS: f32 = 12.;
/// Neutral overrides Card to spacing-3 rather than the generic content inset.
pub const CARD_PADDING: f32 = 12.;
pub const ELEMENT_RADIUS: f32 = 10.;
pub const INNER_RADIUS: f32 = 6.;
pub const CONTENT_INSET: f32 = 16.;

/// The two currently supported Neutral appearances.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Appearance {
    #[default]
    Light,
    Dark,
}

/// Semantic roles can be replaced by a host without changing component code.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Colors {
    pub surface: Hsla,
    pub canvas: Hsla,
    pub card: Hsla,
    pub text: Hsla,
    pub muted: Hsla,
    pub faint: Hsla,
    pub border: Hsla,
    pub border_emphasized: Hsla,
    pub accent: Hsla,
    pub on_accent: Hsla,
    pub neutral: Hsla,
    pub hover: Hsla,
    pub pressed: Hsla,
    pub success: Hsla,
    pub warning: Hsla,
    pub danger: Hsla,
    pub info: Hsla,
    pub focus: Hsla,
    pub neutral_background: Hsla,
    pub neutral_foreground: Hsla,
    pub success_background: Hsla,
    pub success_foreground: Hsla,
    pub warning_background: Hsla,
    pub warning_foreground: Hsla,
    pub danger_background: Hsla,
    pub danger_foreground: Hsla,
    pub info_background: Hsla,
    pub info_foreground: Hsla,
}

/// An explicit, copyable theme. No application-global theme state is installed.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Theme {
    pub appearance: Appearance,
    pub colors: Colors,
}

impl Default for Theme {
    fn default() -> Self {
        Self::light()
    }
}

impl Theme {
    pub fn light() -> Self {
        Self {
            appearance: Appearance::Light,
            colors: Colors {
                surface: rgb(0xffffff).into(),
                canvas: rgb(0xf1f1f1).into(),
                card: rgb(0xffffff).into(),
                text: rgb(0x000000).into(),
                muted: rgb(0x474747).into(),
                faint: rgb(0x6a6a6a).into(),
                border: rgba(0x00000014).into(),
                border_emphasized: rgb(0xd4d4d4).into(),
                accent: rgb(0x1b1b1b).into(),
                on_accent: rgb(0xffffff).into(),
                neutral: rgba(0x0000000f).into(),
                hover: rgba(0x0000000d).into(),
                pressed: rgba(0x0000001a).into(),
                success: rgb(0x2f7d33).into(),
                warning: rgb(0x836700).into(),
                danger: rgb(0xc0262c).into(),
                info: rgb(0x0067ce).into(),
                focus: rgb(0x1b1b1b).into(),
                neutral_background: rgb(0xe2e2e2).into(),
                neutral_foreground: rgb(0x1b1b1b).into(),
                success_background: rgb(0xbce0bb).into(),
                success_foreground: rgb(0x00490b).into(),
                warning_background: rgb(0xfae19e).into(),
                warning_foreground: rgb(0x4b3900).into(),
                danger_background: rgb(0xffc4be).into(),
                danger_foreground: rgb(0x76000c).into(),
                info_background: rgb(0xb9d7ff).into(),
                info_foreground: rgb(0x003978).into(),
            },
        }
    }

    pub fn dark() -> Self {
        Self {
            appearance: Appearance::Dark,
            colors: Colors {
                surface: rgb(0x262626).into(),
                canvas: rgb(0x1b1b1b).into(),
                card: rgb(0x1b1b1b).into(),
                text: rgb(0xffffff).into(),
                muted: rgb(0x9e9e9e).into(),
                faint: rgb(0x9e9e9e).into(),
                border: rgba(0xffffff1a).into(),
                border_emphasized: rgb(0x525252).into(),
                accent: rgb(0xf1f1f1).into(),
                on_accent: rgb(0x111111).into(),
                neutral: rgba(0xffffff1a).into(),
                hover: rgba(0xffffff0d).into(),
                pressed: rgba(0xffffff1a).into(),
                success: rgb(0x90ca90).into(),
                warning: rgb(0xeec448).into(),
                danger: rgb(0xffaea7).into(),
                info: rgb(0xa1caff).into(),
                focus: rgb(0xf1f1f1).into(),
                neutral_background: rgb(0x303030).into(),
                neutral_foreground: rgb(0xd4d4d4).into(),
                success_background: rgb(0x2b422b).into(),
                success_foreground: rgb(0x90ca90).into(),
                warning_background: rgb(0x453a1c).into(),
                warning_foreground: rgb(0xeec448).into(),
                danger_background: rgb(0x5b2b28).into(),
                danger_foreground: rgb(0xffaea7).into(),
                info_background: rgb(0x253c5a).into(),
                info_foreground: rgb(0xa1caff).into(),
            },
        }
    }

    pub fn for_appearance(appearance: Appearance) -> Self {
        match appearance {
            Appearance::Light => Self::light(),
            Appearance::Dark => Self::dark(),
        }
    }
}

/// Astryx's nominal desktop control sizes.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ControlSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl ControlSize {
    pub fn height(self) -> f32 {
        match self {
            Self::Small => 28.,
            Self::Medium => 32.,
            Self::Large => 36.,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::Rgba;

    fn luminance(color: Hsla) -> f32 {
        let color = Rgba::from(color);
        let linear = |value: f32| {
            if value <= 0.04045 {
                value / 12.92
            } else {
                ((value + 0.055) / 1.055).powf(2.4)
            }
        };
        0.2126 * linear(color.r) + 0.7152 * linear(color.g) + 0.0722 * linear(color.b)
    }

    fn contrast(first: Hsla, second: Hsla) -> f32 {
        let first = luminance(first);
        let second = luminance(second);
        (first.max(second) + 0.05) / (first.min(second) + 0.05)
    }

    #[test]
    fn both_appearances_keep_body_labels_and_primary_actions_readable() {
        for theme in [Theme::light(), Theme::dark()] {
            let c = theme.colors;
            assert!(contrast(c.text, c.card) >= 7.);
            assert!(contrast(c.muted, c.card) >= 4.5);
            assert!(contrast(c.muted, c.surface) >= 4.5);
            assert!(contrast(c.on_accent, c.accent) >= 7.);
        }
    }

    #[test]
    fn status_badges_keep_normal_text_contrast_in_both_appearances() {
        for theme in [Theme::light(), Theme::dark()] {
            let c = theme.colors;
            for (foreground, background) in [
                (c.neutral_foreground, c.neutral_background),
                (c.success_foreground, c.success_background),
                (c.warning_foreground, c.warning_background),
                (c.danger_foreground, c.danger_background),
                (c.info_foreground, c.info_background),
            ] {
                assert!(contrast(foreground, background) >= 4.5);
            }
        }
    }

    #[test]
    fn all_control_sizes_fit_the_body_line_without_clipping() {
        let heights =
            [ControlSize::Small, ControlSize::Medium, ControlSize::Large].map(ControlSize::height);
        assert!(heights[0] >= BODY_LINE_HEIGHT + 8.);
        assert!(heights.windows(2).all(|pair| pair[0] < pair[1]));
    }
}
