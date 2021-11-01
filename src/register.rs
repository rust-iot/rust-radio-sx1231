// This file is part of the rust-radio-sx1231 project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright 2018 Ryan Kurte
// Copyright 2020-2021 Erik Henriksson

use modular_bitfield::prelude::*;

pub trait Reg {
  const ADDRESS: u8;
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Fifo {}

impl Reg for Fifo{
  const ADDRESS: u8 = 0x00;
}

#[derive(BitfieldSpecifier, Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[bits = 3]
pub enum ModemMode {
  Sleep = 0,
  Standby = 1,
  FreqSynthesizer =  2,
  Transmitter = 3,
  Receiver = 4,
}

#[bitfield]
#[derive(BitfieldSpecifier, Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OpMode {
  reserved: B2,
  pub modem_mode: ModemMode,
  pub listen_abort: bool,
  pub listen_on: bool,
  pub sequencer_off: bool,
}

impl Reg for OpMode{
  const ADDRESS: u8 = 0x01;
}

#[derive(BitfieldSpecifier, Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[bits = 5]
pub enum ModulationType {
  Fsk     = 0b00000,
  Gfsk1   = 0b00001,
  Gfsk05  = 0b00010,
  Gfsk03  = 0b00011,
  Ook     = 0b01000,
  OokBr   = 0b01001,
  Ook2Br  = 0b01010,
  // 0b01011 reserved
  // 0b1xxxx reserved
}

#[derive(BitfieldSpecifier, Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[bits = 2]
pub enum DataMode {
  Packet = 0,
  ContinousWithBitSync = 1,
  // 0b01 reserved
  Continous = 3,
}

#[bitfield]
#[derive(BitfieldSpecifier, Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DataModulation {
  pub mod_type: ModulationType,
  pub mode: DataMode,
  unused: B1,
}

impl Reg for DataModulation{
  const ADDRESS: u8 = 0x02;
}

#[bitfield]
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Bitrate {
  pub bitrate: u16,
}

impl Reg for Bitrate{
  const ADDRESS: u8 = 0x03;
}

#[bitfield]
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Fdev {
  pub freq_dev: u16,
}

impl Reg for Fdev{
  const ADDRESS: u8 = 0x05;
}

#[bitfield]
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct FreqDev {
  pub freq_dev: u16,
}

impl Reg for FreqDev{
  const ADDRESS: u8 = 0x05;
}

#[bitfield]
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CarrierFreq {
  pub freq: B24,
}

impl Reg for CarrierFreq {
  const ADDRESS: u8 = 0x07;
}

#[bitfield]
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Oscillator {
  unused: B6,
  pub rc_cal_done: bool,
  pub rc_cal_start: bool,
}

impl Reg for Oscillator {
  const ADDRESS: u8 = 0x0A;
}

#[bitfield]
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AfcControl {
  unused: B5,
  pub low_beta_on: bool,
  unused2: B2,
}

impl Reg for AfcControl {
  const ADDRESS: u8 = 0x0B;
}

/// Action taken after acceptance of a packet in Listen mode
#[derive(BitfieldSpecifier, Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[bits = 2]
pub enum ListenEnd {
  Rx = 0,
  Mode = 1,
  Idle = 2,
}

/// Criteria for packet acceptance in Listen mode
#[derive(BitfieldSpecifier, Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[bits = 1]
pub enum ListenCriteria {
  Rssi = 0,
  RssiAndSync = 1,
}

/// Criteria for packet acceptance in Listen mode
#[derive(BitfieldSpecifier, Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[bits = 2]
pub enum ListenBaseDuration {
  Dur64us = 0,
  Dur4_1ms = 1,
  Dur262ms = 2,
}

#[bitfield]
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Listen {
  unused: B1,
  pub end: ListenEnd,
  pub criteria: ListenCriteria,
  pub rx_base: ListenBaseDuration,
  pub idle_base: ListenBaseDuration,
  // Idle duration = coeff_idle * idle_base_time
  pub coeff_idle: u8,
  // RX duration = coeff_idle * idle_base_time
  pub coeff_rx: u8,
}

impl Reg for Listen {
  const ADDRESS: u8 = 0x0D;
}

#[bitfield]
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AfcFei {
  unused: B5,
  pub afc_auto_on: bool,
  pub afc_clear: bool,
  pub afc_start: bool,
}

impl Reg for AfcFei {
  const ADDRESS: u8 = 0x0E;
}

#[bitfield]
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Version {
  version: u8,
}

impl Reg for Version {
  const ADDRESS: u8 = 0x10;
}

#[bitfield]
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PaLevel {
  pub level: B5,
  pub pa2_on: bool,
  pub pa1_on: bool,
  pub pa0_on: bool,
}

impl Reg for PaLevel {
  const ADDRESS: u8 = 0x11;
}

// Rise/Fall time of ramp up/down in FSK
#[derive(BitfieldSpecifier, Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[bits = 4]
pub enum RampTime {
  Ramp3400us = 0,
  Ramp2000us = 1,
  Ramp1000us = 2,
  Ramp500us = 3,
  Ramp250us = 4,
  Ramp125us = 5,
  Ramp100us = 6,
  Ramp62us = 7,
  Ramp50us = 8,
  Ramp40us = 9,
  Ramp31us = 10,
  Ramp25us = 11,
  Ramp20us = 12,
  Ramp15us = 13,
  Ramp12us = 14,
  Ramp10us = 15,
}

#[bitfield]
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PaRamp {
  pub time: RampTime,
  unused: B4,
}

impl Reg for PaRamp {
  const ADDRESS: u8 = 0x12;
}

#[bitfield]
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Ocp {
  pub trim: B4,
  pub ocp_on: bool,
  unused: B3
}

impl Reg for Ocp {
  const ADDRESS: u8 = 0x13;
}

#[derive(BitfieldSpecifier, Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[bits = 3]
pub enum LnaGainSelect {
  Agc = 0,
  G1 = 1,
  G2 = 2,
  G3 = 3,
  G4 = 4,
  G5 = 5,
  G6 = 6,
}

#[derive(BitfieldSpecifier, Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[bits = 1]
pub enum LnaZin {
  Zin50Ohm = 0,
  Zin200Ohm = 1,
}

#[bitfield]
#[derive(BitfieldSpecifier, Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Lna {
  pub gain_select: LnaGainSelect,
  pub current_gain: B3,
  unused: B1,
  pub zin: LnaZin,
}

impl Reg for Lna {
  const ADDRESS: u8 = 0x18;
}

// FSK bandwidth register values
#[derive(BitfieldSpecifier, Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[bits = 5]
pub enum Bandwidth {
    Bw2600 = 0x17,
    Bw3100 = 0x0F,
    Bw3900 = 0x07,
    Bw5200 = 0x16,
    Bw6300 = 0x0E,
    Bw7800 = 0x06,
    Bw10400 = 0x15,
    Bw12500 = 0x0D,
    Bw15600 = 0x05,
    Bw20800 = 0x14,
    Bw25000 = 0x0C,
    Bw31300 = 0x04,
    Bw41700 = 0x13,
    Bw50000 = 0x0B,
    Bw62500 = 0x03,
    Bw83333 = 0x12,
    Bw100000 = 0x0A,
    Bw125000 = 0x02,
    Bw166700 = 0x11,
    Bw200000 = 0x09,
    Bw250000 = 0x01,
}

#[derive(BitfieldSpecifier, Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[bits = 3]
pub enum DccFreq {
  Fc16 = 0,
  Fc8 = 1,
  Fc4 = 2,
  Fc2 = 3,
  Fc1 = 4,
  Fc05 = 5,
  Fc025 = 6,
  Fc0125 = 7,
}

#[bitfield]
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RxBw {
  pub bw: Bandwidth,
  pub dcc: DccFreq,
}

impl Reg for RxBw {
  const ADDRESS: u8 = 0x19;
}

#[bitfield]
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AfcBw {
  pub bw: Bandwidth,
  pub dcc: DccFreq,
}

impl Reg for AfcBw {
  const ADDRESS: u8 = 0x20;
}

#[bitfield]
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct RssiConfig {
  pub start: bool,
  pub done: bool,
  unused: B6,
}

impl Reg for RssiConfig {
  const ADDRESS: u8 = 0x23;
}


#[bitfield]
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RssiValue {
  pub value: u8,
}

impl Reg for RssiValue {
  const ADDRESS: u8 = 0x24;
}

#[bitfield]
#[derive(Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct IrqFlags {
  pub sync_address_match: bool,
  pub auto_mode: bool,
  pub timeout: bool,
  pub rssi: bool,
  pub pll_lock: bool,
  pub tx_ready: bool,
  pub rx_ready: bool,
  pub mode_ready: bool,
  unused: bool,
  pub crc_ok: bool,
  pub payload_ready: bool,
  pub packet_sent: bool,
  pub fifo_overrun: bool,
  pub fifo_level: bool,
  pub fifo_not_empty: bool,
  pub fifo_full: bool,
}

impl Reg for IrqFlags {
  const ADDRESS: u8 = 0x27;
}

// #[cfg(feature = "std")]
impl core::fmt::Debug for IrqFlags {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    if self.sync_address_match() {
      f.write_str("SyncAddressMatch ")?;
    }
    if self.auto_mode() {
      f.write_str("AutoMode ")?;
    }
    if self.timeout() {
      f.write_str("Timeout ")?;
    }
    if self.rssi() {
      f.write_str("Rssi ")?;
    }
    if self.pll_lock() {
      f.write_str("PllLock ")?;
    }
    if self.tx_ready() {
      f.write_str("TxReady ")?;
    }
    if self.rx_ready() {
      f.write_str("RxReady ")?;
    }
    if self.mode_ready() {
      f.write_str("ModeReady ")?;
    }
    if self.crc_ok() {
      f.write_str("CrcOk ")?;
    }
    if self.payload_ready() {
      f.write_str("PayloadReady ")?;
    }
    if self.packet_sent() {
      f.write_str("PacketSent ")?;
    }
    if self.fifo_overrun() {
      f.write_str("FifoOverrun ")?;
    }
    if self.fifo_level() {
      f.write_str("FifoLevel ")?;
    }
    if self.fifo_not_empty() {
      f.write_str("FifoNotEmpty ")?;
    }
    if self.fifo_full() {
      f.write_str("FifoFull ")?;
    }
    Ok(())
  }
}

#[bitfield]
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RssiThreshold {
  pub value: u8,
}

impl Reg for RssiThreshold {
  const ADDRESS: u8 = 0x29;
}

#[bitfield]
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Preamble {
  pub size: u16,
}

impl Reg for Preamble {
  const ADDRESS: u8 = 0x2c;
}

#[derive(BitfieldSpecifier, Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[bits = 1]
pub enum FifoFillCondition {
  SyncAddressInterrupt = 0,
  FifoFillConditionBit = 1,
}

#[bitfield]
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SyncConfig {
  pub tolerated_bit_errors: B3,
  pub size_minus_one: B3,
  pub fifo_fill_condition: FifoFillCondition,
  pub sync_on: bool,
}

impl Reg for SyncConfig {
  const ADDRESS: u8 = 0x2e;
}

#[bitfield]
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SyncValue {
  unused: B8
}

impl Reg for SyncValue {
  const ADDRESS: u8 = 0x2f;
}

#[derive(BitfieldSpecifier, Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[bits = 2]
pub enum AddressFilter {
  Off = 0,
  MatchNode = 1,
  MatchNodeOrBroadcast = 2,
}

#[derive(BitfieldSpecifier, Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[bits = 1]
pub enum CrcAutoClear {
  On = 0,
  Off = 1,
}

#[derive(BitfieldSpecifier, Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[bits = 1]
pub enum Crc {
  Off = 0,
  On = 1,
}

#[derive(BitfieldSpecifier, Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[bits = 2]
pub enum DcFree {
  Off = 0,
  Manchester = 1,
  Whitening = 2,
}

#[derive(BitfieldSpecifier, Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[bits = 1]
pub enum PacketFormat {
  Fixed = 0,
  Variable = 1,
}

#[bitfield]
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PacketConfig1 {
  unused: B1,
  pub address_filter: AddressFilter,
  pub crc_auto_clear: CrcAutoClear,
  pub crc: Crc,
  pub dc_free: DcFree,
  pub packet_format: PacketFormat,
}

impl Reg for PacketConfig1 {
  const ADDRESS: u8 = 0x37;
}

#[bitfield]
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PayloadLength {
  pub value: u8
}

impl Reg for PayloadLength {
  const ADDRESS: u8 = 0x38;
}

#[derive(BitfieldSpecifier, Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[bits = 1]
pub enum TxStartCondition {
  /// The number of bytes in the FIFO exceeds FifoThreshold.
  FifoLevel = 0,
  /// At least one byte in the FIFO
  FifoNotEmpty = 1,
}

#[bitfield]
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct FifoThreshold {
  pub threshold: B7,
  pub start_condition: TxStartCondition,
}

impl Reg for FifoThreshold {
  const ADDRESS: u8 = 0x3C;
}

#[derive(BitfieldSpecifier, Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[bits = 1]
pub enum Aes {
  Off = 0,
  On = 1,
}

#[derive(BitfieldSpecifier, Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[bits = 1]
pub enum AutoRxRestart {
  Off = 0,
  On = 1,
}

#[bitfield]
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PacketConfig2 {
  pub aes: Aes,
  pub auto_rx_restart: AutoRxRestart,
  pub rx_restart: bool,
  pub inter_packet_rx_delay: B5
}

impl Reg for PacketConfig2 {
  const ADDRESS: u8 = 0x3D;
}