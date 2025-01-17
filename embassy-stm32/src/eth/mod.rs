#![macro_use]

#[cfg_attr(eth_v1c, path = "v1c/mod.rs")]
#[cfg_attr(eth_v2, path = "v2/mod.rs")]
#[cfg_attr(eth_v1, path = "v1.rs")]
mod _version;
pub mod lan8742a;

pub use _version::*;

/// Station Management Interface (SMI) on an ethernet PHY
///
/// # Safety
///
/// The methods cannot move out of self
pub unsafe trait StationManagement {
    /// Read a register over SMI.
    fn smi_read(&mut self, reg: u8) -> u16;
    /// Write a register over SMI.
    fn smi_write(&mut self, reg: u8, val: u16);
}

/// Traits for an Ethernet PHY
///
/// # Safety
///
/// The methods cannot move S
pub unsafe trait PHY {
    /// Reset PHY and wait for it to come out of reset.
    fn phy_reset<S: StationManagement>(sm: &mut S);
    /// PHY initialisation.
    fn phy_init<S: StationManagement>(sm: &mut S);
    /// Poll link to see if it is up and FD with 100Mbps
    fn poll_link<S: StationManagement>(sm: &mut S) -> bool;
}

pub(crate) mod sealed {
    pub trait Instance {
        fn regs() -> crate::pac::eth::Eth;
    }
}

pub trait Instance: sealed::Instance + Send + 'static {}

impl sealed::Instance for crate::peripherals::ETH {
    fn regs() -> crate::pac::eth::Eth {
        crate::pac::ETH
    }
}
impl Instance for crate::peripherals::ETH {}

pin_trait!(RefClkPin, Instance);
pin_trait!(MDIOPin, Instance);
pin_trait!(MDCPin, Instance);
pin_trait!(CRSPin, Instance);
pin_trait!(RXD0Pin, Instance);
pin_trait!(RXD1Pin, Instance);
pin_trait!(TXD0Pin, Instance);
pin_trait!(TXD1Pin, Instance);
pin_trait!(TXEnPin, Instance);

crate::pac::peripheral_pins!(
    ($inst:ident, eth, ETH, $pin:ident, REF_CLK, $af:expr) => {
        pin_trait_impl!(RefClkPin, $inst, $pin, $af);
    };
    ($inst:ident, eth, ETH, $pin:ident, MDIO, $af:expr) => {
        pin_trait_impl!(MDIOPin, $inst, $pin, $af);
    };
    ($inst:ident, eth, ETH, $pin:ident, MDC, $af:expr) => {
        pin_trait_impl!(MDCPin, $inst, $pin, $af);
    };
    ($inst:ident, eth, ETH, $pin:ident, CRS_DV, $af:expr) => {
        pin_trait_impl!(CRSPin, $inst, $pin, $af);
    };
    ($inst:ident, eth, ETH, $pin:ident, RXD0, $af:expr) => {
        pin_trait_impl!(RXD0Pin, $inst, $pin, $af);
    };
    ($inst:ident, eth, ETH, $pin:ident, RXD1, $af:expr) => {
        pin_trait_impl!(RXD1Pin, $inst, $pin, $af);
    };
    ($inst:ident, eth, ETH, $pin:ident, TXD0, $af:expr) => {
        pin_trait_impl!(TXD0Pin, $inst, $pin, $af);
    };
    ($inst:ident, eth, ETH, $pin:ident, TXD1, $af:expr) => {
        pin_trait_impl!(TXD1Pin, $inst, $pin, $af);
    };
    ($inst:ident, eth, ETH, $pin:ident, TX_EN, $af:expr) => {
        pin_trait_impl!(TXEnPin, $inst, $pin, $af);
    };
);
