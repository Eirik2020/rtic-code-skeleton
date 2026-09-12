//! Physical board manifests. A board declares only what it enables and how
//! it wires it — no software capability ever belongs here (see
//! `docs/architecture/hardware-software-routing.md`). One file per board.

pub(crate) mod nucleo_f401re;
