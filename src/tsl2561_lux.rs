//! Blame this gross copy and paste on the lack of floating point and this implementation:
//! https://cdn-shop.adafruit.com/datasheets/TSL2561.pdf
use tsl256x::{Gain, IntegrationTime};

const LUX_SCALE: u8 = 14; // scale by 2^14
const RATIO_SCALE: u8 = 9; // scale ratio by 2^9

const CH_SCALE: u8 = 10; // scale channel values by 2^10
const CHSCALE_TINT0: u16 = 0x7517; // 322/11 * 2^CH_SCALE
const CHSCALE_TINT1: u16 = 0x0fe7; // 322/81 * 2^CH_SCALE

const K1T: u32 = 0x0040; // 0.125 * 2^RATIO_SCALE
const B1T: u16 = 0x01f2; // 0.0304 * 2^LUX_SCALE
const M1T: u16 = 0x01be; // 0.0272 * 2^LUX_SCALE
const K2T: u32 = 0x0080; // 0.250 * 2^RATIO_SCALE
const B2T: u16 = 0x0214; // 0.0325 * 2^LUX_SCALE
const M2T: u16 = 0x02d1; // 0.0440 * 2^LUX_SCALE
const K3T: u32 = 0x00c0; // 0.375 * 2^RATIO_SCALE
const B3T: u16 = 0x023f; // 0.0351 * 2^LUX_SCALE
const M3T: u16 = 0x037b; // 0.0544 * 2^LUX_SCALE
const K4T: u32 = 0x0100; // 0.50 * 2^RATIO_SCALE
const B4T: u16 = 0x0270; // 0.0381 * 2^LUX_SCALE
const M4T: u16 = 0x03fe; // 0.0624 * 2^LUX_SCALE
const K5T: u32 = 0x0138; // 0.61 * 2^RATIO_SCALE
const B5T: u16 = 0x016f; // 0.0224 * 2^LUX_SCALE
const M5T: u16 = 0x01fc; // 0.0310 * 2^LUX_SCALE
const K6T: u32 = 0x019a; // 0.80 * 2^RATIO_SCALE
const B6T: u8 = 0x00d2; // 0.0128 * 2^LUX_SCALE
const M6T: u8 = 0x00fb; // 0.0153 * 2^LUX_SCALE
const K7T: u32 = 0x029a; // 1.3 * 2^RATIO_SCALE
const B7T: u8 = 0x0018; // 0.00146 * 2^LUX_SCALE
const M7T: u8 = 0x0012; // 0.00112 * 2^LUX_SCALE
const K8T: u32 = 0x029a; // 1.3 * 2^RATIO_SCALE
const B8T: u8 = 0x0000; // 0.000 * 2^LUX_SCALE
const M8T: u8 = 0x0000; // 0.000 * 2^LUX_SCALE

const K1C: u32 = 0x0043; // 0.130 * 2^RATIO_SCALE
const B1C: u16 = 0x0204; // 0.0315 * 2^LUX_SCALE
const M1C: u16 = 0x01ad; // 0.0262 * 2^LUX_SCALE
const K2C: u32 = 0x0085; // 0.260 * 2^RATIO_SCALE
const B2C: u16 = 0x0228; // 0.0337 * 2^LUX_SCALE
const M2C: u16 = 0x02c1; // 0.0430 * 2^LUX_SCALE
const K3C: u32 = 0x00c8; // 0.390 * 2^RATIO_SCALE
const B3C: u16 = 0x0253; // 0.0363 * 2^LUX_SCALE
const M3C: u16 = 0x0363; // 0.0529 * 2^LUX_SCALE
const K4C: u32 = 0x010a; // 0.520 * 2^RATIO_SCALE
const B4C: u16 = 0x0282; // 0.0392 * 2^LUX_SCALE
const M4C: u16 = 0x03df; // 0.0605 * 2^LUX_SCALE
const K5C: u32 = 0x014d; // 0.65 * 2^RATIO_SCALE
const B5C: u16 = 0x0177; // 0.0229 * 2^LUX_SCALE
const M5C: u16 = 0x01dd; // 0.0291 * 2^LUX_SCALE
const K6C: u32 = 0x019a; // 0.80 * 2^RATIO_SCALE
const B6C: u16 = 0x0101; // 0.0157 * 2^LUX_SCALE
const M6C: u16 = 0x0127; // 0.0180 * 2^LUX_SCALE
const K7C: u32 = 0x029a; // 1.3 * 2^RATIO_SCALE
const B7C: u8 = 0x0037; // 0.00338 * 2^LUX_SCALE
const M7C: u8 = 0x002b; // 0.00260 * 2^LUX_SCALE
const K8C: u32 = 0x029a; // 1.3 * 2^RATIO_SCALE
const B8C: u8 = 0x0000; // 0.000 * 2^LUX_SCALE
const M8C: u8 = 0x0000; // 0.000 * 2^LUX_SCALE

#[allow(dead_code)]
pub enum PackageCoefficient {
    CS,
    T,
    FN,
    CL,
}

pub fn into_lux(
    channel_0: u16,
    channel_1: u16,
    integration_time: IntegrationTime,
    gain: Gain,
    package_coefficent: PackageCoefficient,
) -> u32 {
    let channel_scale = into_channel_scale(integration_time, gain);

    let channel_0 = (channel_0 as u32 * channel_scale) >> CH_SCALE;
    let channel_1 = (channel_1 as u32 * channel_scale) >> CH_SCALE;

    let ratio = into_ratio(channel_0, channel_1);
    let (b, m) = into_b_m(ratio, package_coefficent);

    let channel_0 = channel_0 * b;
    let channel_1 = channel_0 * m;

    let temp = channel_0.saturating_sub(channel_1) + (1 << LUX_SCALE - 1);
    let lux = temp >> LUX_SCALE;

    lux
}

fn into_channel_scale(integration_time: IntegrationTime, gain: Gain) -> u32 {
    let channel_scale = match integration_time {
        IntegrationTime::ms_13 => CHSCALE_TINT0,
        IntegrationTime::ms_101 => CHSCALE_TINT1.into(),
        IntegrationTime::ms_402 => 1 << CH_SCALE,
    };

    match gain {
        Gain::High => (channel_scale << 4),
        Gain::Low => channel_scale,
    }
    .into()
}

fn into_ratio(channel_0: u32, channel_1: u32) -> u32 {
    let ratio = if channel_0 != 0 {
        // fix
        channel_1 << (RATIO_SCALE as u32 + 1) / channel_0
    } else {
        0
    };

    (ratio + 1) >> 1
}

fn into_b_m(ratio: u32, package_coefficent: PackageCoefficient) -> (u32, u32) {
    // not CS
    #[allow(overlapping_range_endpoints)]
    let (b, m) = match package_coefficent {
        PackageCoefficient::CS => match ratio {
            0..=K1T => (B1T, M1T),
            K1T..=K2T => (B2T, M2T),
            K2T..=K3T => (B3T, M3T),
            K3T..=K4T => (B4T, M4T),
            K4T..=K5T => (B5T, M5T),
            K6T..=K6T => (B6T.into(), M6T.into()),
            K7T..=K8T => (B7T.into(), M7T.into()),
            _ => (B8T.into(), M8T.into()),
        },

        _ => match ratio {
            0..=K1C => (B1C, M1C),
            K1C..=K2C => (B2C, M2C),
            K2C..=K3C => (B3C, M3C),
            K3C..=K4C => (B4C, M4C),
            K4C..=K5C => (B5C, M5C),
            K6C..=K6C => (B6C.into(), M6C.into()),
            K7C..=K8C => (B7C.into(), M7C.into()),
            _ => (B8C.into(), M8C.into()),
        },
    };

    (b.into(), m.into())
}
