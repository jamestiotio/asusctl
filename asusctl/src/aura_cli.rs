use std::str::FromStr;

use gumdrop::Options;
use rog_aura::error::Error;
use rog_aura::{AuraEffect, AuraModeNum, AuraZone, Colour, Direction, Speed};

#[derive(Options, Debug)]
pub struct LedPowerCommand1 {
    #[options(help = "print help message")]
    pub help: bool,
    #[options(meta = "", help = "Control if LEDs enabled while awake <true/false>")]
    pub awake: Option<bool>,
    #[options(meta = "", help = "Use with awake option <true/false>")]
    pub keyboard: Option<bool>,
    #[options(meta = "", help = "Use with awake option <true/false>")]
    pub lightbar: Option<bool>,
    #[options(meta = "", help = "Control boot animations <true/false>")]
    pub boot: Option<bool>,
    #[options(meta = "", help = "Control suspend animations <true/false>")]
    pub sleep: Option<bool>,
}

#[derive(Options, Debug)]
pub struct LedPowerCommand2 {
    #[options(help = "print help message")]
    pub help: bool,
    #[options(command)]
    pub command: Option<SetAuraZoneEnabled>,
}

#[derive(Options, Debug)]
pub enum SetAuraZoneEnabled {
    /// Applies to both old and new models
    #[options(help = "")]
    Keyboard(AuraPowerStates),
    #[options(help = "")]
    Logo(AuraPowerStates),
    #[options(help = "")]
    Lightbar(AuraPowerStates),
    #[options(help = "")]
    Lid(AuraPowerStates),
    #[options(help = "")]
    RearGlow(AuraPowerStates),
}

#[derive(Debug, Clone, Options)]
pub struct AuraPowerStates {
    #[options(help = "print help message")]
    pub help: bool,
    #[options(help = "defaults to false if option unused")]
    pub boot: bool,
    #[options(help = "defaults to false if option unused")]
    pub awake: bool,
    #[options(help = "defaults to false if option unused")]
    pub sleep: bool,
    #[options(help = "defaults to false if option unused")]
    pub shutdown: bool,
}

#[derive(Options)]
pub struct LedBrightness {
    level: Option<u8>,
}
impl LedBrightness {
    pub fn new(level: Option<u8>) -> Self {
        LedBrightness { level }
    }

    pub fn level(&self) -> Option<u8> {
        self.level
    }
}
impl FromStr for LedBrightness {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "off" => Ok(LedBrightness { level: Some(0x00) }),
            "low" => Ok(LedBrightness { level: Some(0x01) }),
            "med" => Ok(LedBrightness { level: Some(0x02) }),
            "high" => Ok(LedBrightness { level: Some(0x03) }),
            _ => {
                print!("Invalid argument, must be one of: off, low, med, high");
                Err(Error::ParseBrightness)
            }
        }
    }
}
impl ToString for LedBrightness {
    fn to_string(&self) -> String {
        let s = match self.level {
            Some(0x00) => "low",
            Some(0x01) => "med",
            Some(0x02) => "high",
            _ => "unknown",
        };
        s.to_owned()
    }
}

#[derive(Debug, Clone, Options, Default)]
pub struct SingleSpeed {
    #[options(help = "print help message")]
    help: bool,
    #[options(no_long, meta = "WORD", help = "set the speed: low, med, high")]
    pub speed: Speed,
    #[options(
        no_long,
        meta = "",
        help = "set the zone for this effect e.g, 0, 1, one, logo, lightbar-left"
    )]
    pub zone: AuraZone,
}

#[derive(Debug, Clone, Options, Default)]
pub struct SingleSpeedDirection {
    #[options(help = "print help message")]
    help: bool,
    #[options(no_long, meta = "", help = "set the direction: up, down, left, right")]
    pub direction: Direction,
    #[options(no_long, meta = "", help = "set the speed: low, med, high")]
    pub speed: Speed,
    #[options(
        no_long,
        meta = "",
        help = "set the zone for this effect e.g, 0, 1, one, logo, lightbar-left"
    )]
    pub zone: AuraZone,
}

#[derive(Debug, Clone, Default, Options)]
pub struct SingleColour {
    #[options(help = "print help message")]
    help: bool,
    #[options(no_long, meta = "", help = "set the RGB value e.g, ff00ff")]
    pub colour: Colour,
    #[options(
        no_long,
        meta = "",
        help = "set the zone for this effect e.g, 0, 1, one, logo, lightbar-left"
    )]
    pub zone: AuraZone,
}

#[derive(Debug, Clone, Default, Options)]
pub struct SingleColourSpeed {
    #[options(help = "print help message")]
    help: bool,
    #[options(no_long, meta = "", help = "set the RGB value e.g, ff00ff")]
    pub colour: Colour,
    #[options(no_long, meta = "", help = "set the speed: low, med, high")]
    pub speed: Speed,
    #[options(
        no_long,
        meta = "",
        help = "set the zone for this effect e.g, 0, 1, one, logo, lightbar-left"
    )]
    pub zone: AuraZone,
}

#[derive(Debug, Clone, Options, Default)]
pub struct TwoColourSpeed {
    #[options(help = "print help message")]
    help: bool,
    #[options(no_long, meta = "", help = "set the first RGB value e.g, ff00ff")]
    pub colour: Colour,
    #[options(no_long, meta = "", help = "set the second RGB value e.g, ff00ff")]
    pub colour2: Colour,
    #[options(no_long, meta = "", help = "set the speed: low, med, high")]
    pub speed: Speed,
    #[options(
        no_long,
        meta = "",
        help = "set the zone for this effect e.g, 0, 1, one, logo, lightbar-left"
    )]
    pub zone: AuraZone,
}

#[derive(Debug, Clone, Default, Options)]
pub struct MultiZone {
    #[options(help = "print help message")]
    help: bool,
    #[options(short = "a", meta = "", help = "set the RGB value e.g, ff00ff")]
    pub colour1: Colour,
    #[options(short = "b", meta = "", help = "set the RGB value e.g, ff00ff")]
    pub colour2: Colour,
    #[options(short = "c", meta = "", help = "set the RGB value e.g, ff00ff")]
    pub colour3: Colour,
    #[options(short = "d", meta = "", help = "set the RGB value e.g, ff00ff")]
    pub colour4: Colour,
}

#[derive(Debug, Clone, Default, Options)]
pub struct MultiColourSpeed {
    #[options(help = "print help message")]
    help: bool,
    #[options(short = "a", meta = "", help = "set the RGB value e.g, ff00ff")]
    pub colour1: Colour,
    #[options(short = "b", meta = "", help = "set the RGB value e.g, ff00ff")]
    pub colour2: Colour,
    #[options(short = "c", meta = "", help = "set the RGB value e.g, ff00ff")]
    pub colour3: Colour,
    #[options(short = "d", meta = "", help = "set the RGB value e.g, ff00ff")]
    pub colour4: Colour,
    #[options(no_long, meta = "", help = "set the speed: low, med, high")]
    pub speed: Speed,
}

/// Byte value for setting the built-in mode.
///
/// Enum corresponds to the required integer value
// NOTE: The option names here must match those in rog-aura crate
#[derive(Options)]
pub enum SetAuraBuiltin {
    #[options(help = "set a single static colour")]
    Static(SingleColour),
    #[options(help = "pulse between one or two colours")]
    Breathe(TwoColourSpeed),
    #[options(help = "strobe through all colours")]
    Strobe(SingleSpeed),
    #[options(help = "rainbow cycling in one of four directions")]
    Rainbow(SingleSpeedDirection),
    #[options(help = "rain pattern mimicking raindrops")]
    Stars(TwoColourSpeed),
    #[options(help = "rain pattern of three preset colours")]
    Rain(SingleSpeed),
    #[options(help = "pressed keys are highlighted to fade")]
    Highlight(SingleColourSpeed),
    #[options(help = "pressed keys generate horizontal laser")]
    Laser(SingleColourSpeed),
    #[options(help = "pressed keys ripple outwards like a splash")]
    Ripple(SingleColourSpeed),
    #[options(help = "set a rapid pulse")]
    Pulse(SingleColour),
    #[options(help = "set a vertical line zooming from left")]
    Comet(SingleColour),
    #[options(help = "set a wide vertical line zooming from left")]
    Flash(SingleColour),
}

impl Default for SetAuraBuiltin {
    fn default() -> Self {
        SetAuraBuiltin::Static(SingleColour::default())
    }
}

impl From<&SingleColour> for AuraEffect {
    fn from(aura: &SingleColour) -> Self {
        Self {
            colour1: aura.colour,
            zone: aura.zone,
            ..Default::default()
        }
    }
}

impl From<&SingleSpeed> for AuraEffect {
    fn from(aura: &SingleSpeed) -> Self {
        Self {
            speed: aura.speed,
            zone: aura.zone,
            ..Default::default()
        }
    }
}

impl From<&SingleColourSpeed> for AuraEffect {
    fn from(aura: &SingleColourSpeed) -> Self {
        Self {
            colour1: aura.colour,
            speed: aura.speed,
            zone: aura.zone,
            ..Default::default()
        }
    }
}

impl From<&TwoColourSpeed> for AuraEffect {
    fn from(aura: &TwoColourSpeed) -> Self {
        Self {
            colour1: aura.colour,
            colour2: aura.colour2,
            zone: aura.zone,
            ..Default::default()
        }
    }
}

impl From<&SingleSpeedDirection> for AuraEffect {
    fn from(aura: &SingleSpeedDirection) -> Self {
        Self {
            speed: aura.speed,
            direction: aura.direction,
            zone: aura.zone,
            ..Default::default()
        }
    }
}

impl From<&SetAuraBuiltin> for AuraEffect {
    fn from(aura: &SetAuraBuiltin) -> Self {
        match aura {
            SetAuraBuiltin::Static(x) => {
                let mut data: AuraEffect = x.into();
                data.mode = AuraModeNum::Static;
                data
            }
            SetAuraBuiltin::Breathe(x) => {
                let mut data: AuraEffect = x.into();
                data.mode = AuraModeNum::Breathe;
                data
            }
            SetAuraBuiltin::Strobe(x) => {
                let mut data: AuraEffect = x.into();
                data.mode = AuraModeNum::Strobe;
                data
            }
            SetAuraBuiltin::Rainbow(x) => {
                let mut data: AuraEffect = x.into();
                data.mode = AuraModeNum::Rainbow;
                data
            }
            SetAuraBuiltin::Stars(x) => {
                let mut data: AuraEffect = x.into();
                data.mode = AuraModeNum::Star;
                data
            }
            SetAuraBuiltin::Rain(x) => {
                let mut data: AuraEffect = x.into();
                data.mode = AuraModeNum::Rain;
                data
            }
            SetAuraBuiltin::Highlight(x) => {
                let mut data: AuraEffect = x.into();
                data.mode = AuraModeNum::Highlight;
                data
            }
            SetAuraBuiltin::Laser(x) => {
                let mut data: AuraEffect = x.into();
                data.mode = AuraModeNum::Laser;
                data
            }
            SetAuraBuiltin::Ripple(x) => {
                let mut data: AuraEffect = x.into();
                data.mode = AuraModeNum::Ripple;
                data
            }
            SetAuraBuiltin::Pulse(x) => {
                let mut data: AuraEffect = x.into();
                data.mode = AuraModeNum::Pulse;
                data
            }
            SetAuraBuiltin::Comet(x) => {
                let mut data: AuraEffect = x.into();
                data.mode = AuraModeNum::Comet;
                data
            }
            SetAuraBuiltin::Flash(x) => {
                let mut data: AuraEffect = x.into();
                data.mode = AuraModeNum::Flash;
                data
            }
        }
    }
}
