//! Interface with the DWT (data watchpoint and trace) unit.
//!
//! This unit can monitor specific memory locations for write / read
//! access, this could be handy to debug a system :).
//!
//! See ARMv7-M architecture reference manual C1.8 for some additional
//! info about this stuff.

use bitfield::bitfield;

use super::super::memory::romtable::Component;
use super::DebugRegister;
use crate::{Core, Error};

/// A struct representing a DWT unit on target.
pub struct Dwt<'probe: 'core, 'core> {
    component: &'core Component,
    core: &'core mut Core<'probe>,
}

impl<'probe: 'core, 'core> Dwt<'probe, 'core> {
    /// Creates a new DWT component representation.
    pub fn new(core: &'core mut Core<'probe>, component: &'core Component) -> Self {
        Dwt { component, core }
    }

    /// Logs some info about the DWT component.
    pub fn info(&mut self) -> Result<(), Error> {
        let ctrl = Ctrl::load(self.component, self.core)?;

        log::info!("DWT info:");
        log::info!("  number of comparators available: {}", ctrl.numcomp());
        log::info!("  trace sampling support: {}", !ctrl.notrcpkt());
        log::info!("  compare match support: {}", !ctrl.noexttrig());
        log::info!("  cyccnt support: {}", !ctrl.nocyccnt());
        log::info!("  performance counter support: {}", !ctrl.noprfcnt());

        Ok(())
    }

    /// Enables the DWT component.
    pub fn enable(&mut self) -> Result<(), Error> {
        let mut ctrl = Ctrl::load(self.component, self.core)?;
        ctrl.set_synctap(0x01);
        ctrl.set_cyccntena(true);
        ctrl.store(self.component, self.core)
    }

    /// Enables data tracing on a specific address in memory on a specific DWT unit.
    pub fn enable_data_trace(&mut self, unit: usize, address: u32) -> Result<(), Error> {
        let mut comp = Comp::load_unit(self.component, self.core, unit)?;
        comp.set_comp(address);
        comp.store_unit(self.component, self.core, unit)?;

        let mut mask = Mask::load_unit(self.component, self.core, unit)?;
        mask.set_mask(0x0);
        mask.store_unit(self.component, self.core, unit)?;

        let mut function = Function::load_unit(self.component, self.core, unit)?;
        function.set_datavsize(0x10);
        function.set_emitrange(false);
        function.set_datavmatch(false);
        function.set_cycmatch(false);
        function.set_function(0b11);

        function.store_unit(self.component, self.core, unit)
    }

    /// Disables data tracing on the given unit.
    pub fn disable_data_trace(&mut self, unit: usize) -> Result<(), Error> {
        let mut function = Function::load_unit(self.component, self.core, unit)?;
        function.set_function(0x0);
        function.store_unit(self.component, self.core, unit)
    }

    /// Enable exception tracing.
    pub fn enable_exception_trace(&mut self) -> Result<(), Error> {
        let mut ctrl = Ctrl::load(self.component, self.core)?;
        ctrl.set_exctrcena(true);
        ctrl.store(self.component, self.core)
    }

    /// Disable exception tracing.
    pub fn disable_exception_trace(&mut self) -> Result<(), Error> {
        let mut ctrl = Ctrl::load(self.component, self.core)?;
        ctrl.set_exctrcena(false);
        ctrl.store(self.component, self.core)
    }
}

bitfield! {
    #[derive(Clone)]
    pub struct Ctrl(u32);
    impl Debug;
    pub u8, numcomp, _: 31, 28;
    pub notrcpkt, _: 27;
    pub noexttrig, _: 26;
    pub nocyccnt, _: 25;
    pub noprfcnt, _: 24;
    pub cycevtena, set_cycevtena: 22;
    pub foldevtena, set_foldevtena: 21;
    pub lsuevtena, set_lsuevtena: 20;
    pub sleepevtena, set_sleepevtena: 19;
    pub excevtena, set_excevtena: 18;
    pub cpievtena, set_cpievtena: 17;
    pub exctrcena, set_exctrcena: 16;
    pub pcsamplena, set_pcsamplena: 12;
    /// 00 Disabled. No Synchronization packets.
    /// 01 Synchronization counter tap at CYCCNT[24].
    /// 10 Synchronization counter tap at CYCCNT[26].
    /// 11 Synchronization counter tap at CYCCNT[28].
    pub u8, synctap, set_synctap: 11, 10;
    pub cyctap, set_cyctap: 9;
    pub u8, postinit, set_postinit: 8, 5;
    pub postpreset, set_postpreset: 4, 1;
    pub cyccntena, set_cyccntena: 0;

}

impl Default for Ctrl {
    fn default() -> Self {
        Ctrl(0)
    }
}

impl From<u32> for Ctrl {
    fn from(raw: u32) -> Self {
        Ctrl(raw)
    }
}

impl From<Ctrl> for u32 {
    fn from(raw: Ctrl) -> Self {
        raw.0
    }
}

impl DebugRegister for Ctrl {
    const ADDRESS: u32 = 0x00;
    const NAME: &'static str = "DWT/CTRL";
}

bitfield! {
    #[derive(Clone)]
    pub struct Cyccnt(u32);
    impl Debug;
}

impl Default for Cyccnt {
    fn default() -> Self {
        Cyccnt(0)
    }
}

impl From<u32> for Cyccnt {
    fn from(raw: u32) -> Self {
        Cyccnt(raw)
    }
}

impl From<Cyccnt> for u32 {
    fn from(raw: Cyccnt) -> Self {
        raw.0
    }
}

impl DebugRegister for Cyccnt {
    const ADDRESS: u32 = 0x04;
    const NAME: &'static str = "DWT/CYCCNT";
}

bitfield! {
    #[derive(Clone)]
    pub struct Cpicnt(u32);
    impl Debug;
}

impl Default for Cpicnt {
    fn default() -> Self {
        Cpicnt(0)
    }
}

impl From<u32> for Cpicnt {
    fn from(raw: u32) -> Self {
        Cpicnt(raw)
    }
}

impl From<Cpicnt> for u32 {
    fn from(raw: Cpicnt) -> Self {
        raw.0
    }
}

impl DebugRegister for Cpicnt {
    const ADDRESS: u32 = 0x08;
    const NAME: &'static str = "DWT/CPICNT";
}

bitfield! {
    #[derive(Clone)]
    pub struct Exccnt(u32);
    impl Debug;
}

impl Default for Exccnt {
    fn default() -> Self {
        Exccnt(0)
    }
}

impl From<u32> for Exccnt {
    fn from(raw: u32) -> Self {
        Exccnt(raw)
    }
}

impl From<Exccnt> for u32 {
    fn from(raw: Exccnt) -> Self {
        raw.0
    }
}

impl DebugRegister for Exccnt {
    const ADDRESS: u32 = 0x0C;
    const NAME: &'static str = "DWT/EXCCNT";
}

bitfield! {
    #[derive(Clone)]
    pub struct Comp(u32);
    impl Debug;
    pub u32, comp, set_comp: 31, 0;
}

impl Default for Comp {
    fn default() -> Self {
        Comp(0)
    }
}

impl From<u32> for Comp {
    fn from(raw: u32) -> Self {
        Comp(raw)
    }
}

impl From<Comp> for u32 {
    fn from(raw: Comp) -> Self {
        raw.0
    }
}

impl DebugRegister for Comp {
    const ADDRESS: u32 = 0x20;
    const NAME: &'static str = "DWT/COMP";
}

bitfield! {
    #[derive(Clone)]
    pub struct Mask(u32);
    impl Debug;
    pub u32, mask, set_mask: 4, 0;
}

impl Default for Mask {
    fn default() -> Self {
        Mask(0)
    }
}

impl From<u32> for Mask {
    fn from(raw: u32) -> Self {
        Mask(raw)
    }
}

impl From<Mask> for u32 {
    fn from(raw: Mask) -> Self {
        raw.0
    }
}

impl DebugRegister for Mask {
    const ADDRESS: u32 = 0x24;
    const NAME: &'static str = "DWT/MASK";
}

bitfield! {
    #[derive(Clone)]
    pub struct Function(u32);
    impl Debug;
    pub matched, _: 24;
    pub u8, datavaddr1, set_datavaddr1: 19, 16;
    pub u8, datavaddr0, set_datavaddr0: 15, 12;
    /// 00 Byte.
    /// 01 Halfword.
    /// 10 Word.
    pub u8, datavsize, set_datavsize: 11, 10;
    pub lnk1ena, _: 9;
    pub datavmatch, set_datavmatch: 8;
    pub cycmatch, set_cycmatch: 7;
    pub emitrange, set_emitrange: 5;
    pub function, set_function: 3, 0;
}

impl Default for Function {
    fn default() -> Self {
        Function(0)
    }
}

impl From<u32> for Function {
    fn from(raw: u32) -> Self {
        Function(raw)
    }
}

impl From<Function> for u32 {
    fn from(raw: Function) -> Self {
        raw.0
    }
}

impl DebugRegister for Function {
    const ADDRESS: u32 = 0x28;
    const NAME: &'static str = "DWT/FUNCTION";
}
