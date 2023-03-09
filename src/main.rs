#![no_std]
#![no_main]

use atsam3x8e_wrap::{PIOA, PIOB, PIOC, RTT};
use panic_halt as _;

fn delay_ms(rtt: &RTT, ms: u32) {
    let now = rtt.vr.read().bits();
    let until = now + ms;

    while rtt.vr.read().bits() < until {}
}

#[cortex_m_rt::entry]
unsafe fn main() -> ! {
    let p = atsam3x8e_wrap::Peripherals::take().unwrap();

    let pmc = p.PMC;
    pmc.pmc_pcer0.write_with_zero(|w| w.pid13().set_bit());

    // Configure RTT resolution to approx. 1ms, write 32 bits
    let rtt = p.RTT;
    rtt.mr.write_with_zero(|w|  w.rtpres().bits(0x20) );

    let parallel_input_output_controller_a = p.PIOA;
    let parallel_input_output_controller_b = p.PIOB;
    let parallel_input_output_controller_c = p.PIOC;

    configure_registries(&parallel_input_output_controller_a,
                         &parallel_input_output_controller_b,
                         &parallel_input_output_controller_c);

    loop {
        write_to_output_data_register(&parallel_input_output_controller_a,
                                      &parallel_input_output_controller_b,
                                      &parallel_input_output_controller_c);
        delay_ms(&rtt, 100);
        write_to_clear_output_register(&parallel_input_output_controller_a,
                                       &parallel_input_output_controller_b,
                                       &parallel_input_output_controller_c);
        delay_ms(&rtt, 100);
    }
}

#[doc = "Configure registries: PIO, Output enabling, Pull-up disabling" ]
unsafe fn configure_registries(pioa: &PIOA, piob: &PIOB, pioc: &PIOC) {

    piob.per.write_with_zero(|w| w.p27().set_bit());
    piob.oer.write_with_zero(|w| w.p27().set_bit());
    piob.pudr.write_with_zero(|w| w.p27().set_bit());

    pioc.per.write_with_zero(|w| w.p30().set_bit());
    pioc.oer.write_with_zero(|w| w.p30().set_bit());
    pioc.pudr.write_with_zero(|w| w.p30().set_bit());

    pioa.per.write_with_zero(|w| w.p21().set_bit());
    pioa.oer.write_with_zero(|w| w.p21().set_bit());
    pioa.pudr.write_with_zero(|w| w.p21().set_bit());
}

unsafe fn write_to_output_data_register(pioa: &PIOA, piob: &PIOB, pioc: &PIOC) {
    pioa.sodr.write_with_zero(|w| w.p21().set_bit());
    piob.sodr.write_with_zero(|w| w.p27().set_bit());
    pioc.sodr.write_with_zero(|w| w.p30().set_bit());
}

unsafe fn write_to_clear_output_register(pioa: &PIOA, piob: &PIOB, pioc: &PIOC) {
    pioa.codr.write_with_zero(|w| w.p21().set_bit());
    piob.codr.write_with_zero(|w| w.p27().set_bit());
    pioc.codr.write_with_zero(|w| w.p30().set_bit());
}




