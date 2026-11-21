// SPDX-FileCopyrightText: 2026 Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! I2C sensors the bench programs know, on pic32c-hal's async `I2c`:
//! Sensirion's `SHT4x` (temperature and humidity, at 0x44, 0x45 or 0x46) and
//! Bosch's BMP280 and BME280 (temperature and pressure, and humidity on the
//! BME280; at 0x76 or 0x77). Readings are in hundredths: degrees C, %RH, and
//! pressure in pascals.

use embassy_time::{Duration, Timer};
use pic32c_hal::i2c::I2c;
use pic32c_hal::mode::Async;

/// The `SHT4x`'s addresses (SHT40-AD1B and the like at 0x44, -BD1B at 0x45,
/// -CD1B at 0x46).
pub const SHT4X: [u8; 3] = [0x44, 0x45, 0x46];
/// The BMP280's and BME280's addresses (SDO low, SDO high).
pub const BMX280: [u8; 2] = [0x76, 0x77];

/// A reading: temperature and humidity in hundredths, pressure in pascals.
#[derive(Clone, Copy, Debug)]
pub struct Reading {
    pub centidegrees: i64,
    pub centi_rh: Option<i64>,
    pub pascals: Option<i64>,
}

/// Sensirion's CRC-8: polynomial 0x31, initial 0xFF.
fn crc8(bytes: &[u8]) -> u8 {
    let mut crc = 0xFFu8;
    for &b in bytes {
        crc ^= b;
        for _ in 0..8 {
            crc = if crc & 0x80 != 0 { (crc << 1) ^ 0x31 } else { crc << 1 };
        }
    }
    crc
}

/// Two CRC-checked words from an `SHT4x` answer.
fn words(answer: &[u8; 6]) -> Option<(u16, u16)> {
    let ok = crc8(&answer[0..2]) == answer[2] && crc8(&answer[3..5]) == answer[5];
    ok.then(|| (u16::from_be_bytes([answer[0], answer[1]]), u16::from_be_bytes([answer[3], answer[4]])))
}

/// An `SHT4x`'s command and its answer: two CRC-checked words.
async fn sht4x_command(bus: &mut I2c<'_, Async>, address: u8, command: u8, wait: Duration) -> Result<(u16, u16), &'static str> {
    bus.write(address, &[command]).await.map_err(|_| "command not acknowledged")?;
    Timer::after(wait).await;
    let mut answer = [0u8; 6];
    bus.read(address, &mut answer).await.map_err(|_| "answer not read")?;
    words(&answer).ok_or("CRC mismatch")
}

/// An `SHT4x`'s serial number.
pub async fn sht4x_serial(bus: &mut I2c<'_, Async>, address: u8) -> Result<u32, &'static str> {
    let (high, low) = sht4x_command(bus, address, 0x89, Duration::from_millis(2)).await?;
    Ok((u32::from(high) << 16) | u32::from(low))
}

/// An `SHT4x`'s high-precision measurement (8.3 ms at most).
pub async fn sht4x_measure(bus: &mut I2c<'_, Async>, address: u8) -> Result<Reading, &'static str> {
    let (t, rh) = sht4x_command(bus, address, 0xFD, Duration::from_millis(10)).await?;
    Ok(Reading {
        centidegrees: -4500 + 17_500 * i64::from(t) / 65_535,
        centi_rh: Some((-600 + 12_500 * i64::from(rh) / 65_535).clamp(0, 10_000)),
        pascals: None,
    })
}

/// Which Bosch part answers at `address`: its chip ID (0x58 BMP280, 0x60
/// BME280), if it is one of them.
pub async fn bmx280_id(bus: &mut I2c<'_, Async>, address: u8) -> Option<u8> {
    let mut id = [0u8];
    bus.write_read(address, &[0xD0], &mut id).await.ok()?;
    matches!(id[0], 0x58 | 0x60).then_some(id[0])
}

/// A BMP280 or BME280 with its calibration: temperature and pressure from
/// 0x88 to 0x9F; on the BME280, humidity's H1 at 0xA1 and H2 to H6 from 0xE1
/// to 0xE7.
pub struct Bmx280 {
    address: u8,
    t: [i64; 3],
    p: [i64; 9],
    /// H1 to H6, on the BME280.
    h: Option<[i64; 6]>,
}

impl Bmx280 {
    /// Reads the calibration of the part at `address` with chip ID `id`.
    pub async fn new(bus: &mut I2c<'_, Async>, address: u8, id: u8) -> Result<Self, &'static str> {
        let mut tp = [0u8; 24];
        bus.write_read(address, &[0x88], &mut tp).await.map_err(|_| "calibration not read")?;
        let u = |i: usize| i64::from(u16::from_le_bytes([tp[i], tp[i + 1]]));
        let s = |i: usize| i64::from(i16::from_le_bytes([tp[i], tp[i + 1]]));
        let h = if id == 0x60 {
            let (mut h1, mut h) = ([0u8], [0u8; 7]);
            bus.write_read(address, &[0xA1], &mut h1).await.map_err(|_| "calibration not read")?;
            bus.write_read(address, &[0xE1], &mut h).await.map_err(|_| "calibration not read")?;
            #[expect(clippy::cast_possible_wrap, reason = "the datasheet's signed bytes")]
            let (e4, e6, h6) = (i64::from(h[3] as i8), i64::from(h[5] as i8), i64::from(h[6] as i8));
            // H4 is 0xE4 (signed) above 0xE5's low nibble, H5 0xE6 (signed)
            // above its high nibble.
            Some([i64::from(h1[0]), i64::from(i16::from_le_bytes([h[0], h[1]])), i64::from(h[2]), (e4 << 4) | i64::from(h[4] & 0x0F), (e6 << 4) | i64::from(h[4] >> 4), h6])
        } else {
            None
        };
        Ok(Self { address, t: [u(0), s(2), s(4)], p: [u(6), s(8), s(10), s(12), s(14), s(16), s(18), s(20), s(22)], h })
    }

    /// A forced measurement: humidity x1 on the BME280 (`ctrl_hum` takes
    /// effect with the next `ctrl_meas` write), temperature x2, pressure x16.
    pub async fn measure(&self, bus: &mut I2c<'_, Async>) -> Result<Reading, &'static str> {
        if self.h.is_some() {
            bus.write(self.address, &[0xF2, 0b001]).await.map_err(|_| "ctrl_hum not written")?;
        }
        bus.write(self.address, &[0xF4, (0b010 << 5) | (0b101 << 2) | 0b01]).await.map_err(|_| "ctrl_meas not written")?;
        Timer::after(Duration::from_millis(50)).await;
        let mut m = [0u8; 8];
        let n = if self.h.is_some() { 8 } else { 6 };
        bus.write_read(self.address, &[0xF7], &mut m[..n]).await.map_err(|_| "measurement not read")?;
        let adc_p = (i64::from(m[0]) << 12) | (i64::from(m[1]) << 4) | (i64::from(m[2]) >> 4);
        let adc_t = (i64::from(m[3]) << 12) | (i64::from(m[4]) << 4) | (i64::from(m[5]) >> 4);
        let t_fine = self.t_fine(adc_t);
        Ok(Reading {
            centidegrees: (t_fine * 5 + 128) >> 8,
            pascals: Some(self.pressure(t_fine, adc_p)),
            centi_rh: self.h.map(|h| Self::humidity(&h, t_fine, (i64::from(m[6]) << 8) | i64::from(m[7])) * 100 / 1024),
        })
    }

    /// Bosch's integer compensation, temperature.
    fn t_fine(&self, adc_t: i64) -> i64 {
        let [t1, t2, t3] = self.t;
        let var1 = (((adc_t >> 3) - (t1 << 1)) * t2) >> 11;
        let d = (adc_t >> 4) - t1;
        let var2 = (((d * d) >> 12) * t3) >> 14;
        var1 + var2
    }

    /// Pressure in pascals (the 64-bit form).
    fn pressure(&self, t_fine: i64, adc_p: i64) -> i64 {
        let [p1, p2, p3, p4, p5, p6, p7, p8, p9] = self.p;
        let mut var1 = t_fine - 128_000;
        let mut var2 = var1 * var1 * p6;
        var2 += (var1 * p5) << 17;
        var2 += p4 << 35;
        var1 = ((var1 * var1 * p3) >> 8) + ((var1 * p2) << 12);
        var1 = (((1i64 << 47) + var1) * p1) >> 33;
        if var1 == 0 {
            return 0;
        }
        let mut p = 1_048_576 - adc_p;
        p = (((p << 31) - var2) * 3125) / var1;
        let var1 = (p9 * (p >> 13) * (p >> 13)) >> 25;
        let var2 = (p8 * p) >> 19;
        p = ((p + var1 + var2) >> 8) + (p7 << 4);
        p / 256
    }

    /// Humidity in 1/1024 %RH.
    fn humidity(h: &[i64; 6], t_fine: i64, adc_h: i64) -> i64 {
        let [h1, h2, h3, h4, h5, h6] = *h;
        let x = t_fine - 76_800;
        let mut v = (((adc_h << 14) - (h4 << 20) - (h5 * x)) + 16_384) >> 15;
        v *= ((((((x * h6) >> 10) * (((x * h3) >> 11) + 32_768)) >> 10) + 2_097_152) * h2 + 8192) >> 14;
        v -= ((((v >> 15) * (v >> 15)) >> 7) * h1) >> 4;
        v.clamp(0, 419_430_400) >> 12
    }
}

/// Hundredths as `whole.hundredths`, for the log.
pub struct Centi(pub i64);

impl core::fmt::Display for Centi {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let sign = if self.0 < 0 { "-" } else { "" };
        write!(f, "{sign}{}.{:02}", self.0.abs() / 100, self.0.abs() % 100)
    }
}
