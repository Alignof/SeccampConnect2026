//! Defines default keymap and constants.

use rmk::encoder;
use rmk::types::action::{EncoderAction, KeyAction};
use rmk::{k, layer, mo};

/// Number of columns.
pub(crate) const COL: usize = 3;
/// Number of rows.
pub(crate) const ROW: usize = 2;
/// Number of keymap layers.
pub(crate) const NUM_LAYER: usize = 2;
/// Number of rotary encoders.
pub(crate) const NUM_ENCODER: usize = 1;

/// Define default keymap.
#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        layer!([
            [k!(A), k!(B), k!(Backspace)],
            [k!(C), k!(D), mo!(1)]
        ]),
        layer!([
            [k!(E), k!(F), k!(Enter)],
            [k!(Kc4), k!(Kc5), k!(Kc6)]
        ]),
    ]
}

/// Define default rotary encoder keymap.
#[rustfmt::skip]
pub const fn get_default_encoder_map() -> [[EncoderAction; NUM_ENCODER]; NUM_LAYER] {
    [
        [
            // Encoder 0: (Clockwise, Counter-Clockwise)
            encoder!(k!(MouseWheelUp), k!(MouseWheelDown)),
        ],
        [
            // Encoder 0: (Clockwise, Counter-Clockwise)
            encoder!(k!(MouseWheelUp), k!(MouseWheelDown)),
        ],
    ]
}
