extern crate bluenrg;
extern crate bluetooth_hci as hci;
extern crate embedded_hal as hal;
extern crate nb;

use bluenrg::*;
use hci::host::uart::Error as UartError;
use std::time::Duration;

struct RecordingSink {
    written_header: Vec<u8>,
    written_data: Vec<u8>,
    canned_reply: Vec<u8>,
}

impl RecordingSink {
    fn new() -> RecordingSink {
        RecordingSink {
            written_header: Vec::new(),
            written_data: Vec::new(),

            // The reply is returned in reverse order
            canned_reply: vec![0x00, 0x00, 0xFF, 0xFF, 0x02],
        }
    }
}

impl hal::spi::FullDuplex<u8> for RecordingSink {
    type Error = ();

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        Ok(self.canned_reply.pop().unwrap_or(0))
    }

    fn send(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        if !self.canned_reply.is_empty() {
            self.written_header.push(byte);
        } else {
            self.written_data.push(byte);
        }
        Ok(())
    }
}

impl hal::blocking::spi::transfer::Default<u8> for RecordingSink {}

impl hal::blocking::spi::write::Default<u8> for RecordingSink {}

struct DummyPin;

impl hal::digital::OutputPin for DummyPin {
    fn is_high(&self) -> bool {
        true // Needs to indicate data ready
    }

    fn is_low(&self) -> bool {
        false
    }

    fn set_low(&mut self) {}

    fn set_high(&mut self) {}
}

impl hal::digital::InputPin for DummyPin {
    fn is_high(&self) -> bool {
        true // Needs to indicate data ready
    }

    fn is_low(&self) -> bool {
        false
    }
}

macro_rules! bnrg_test {
    (fn $fn:ident() { | $controller:ident | $act:expr => | $sink:ident | $assert:expr }) => {
        #[test]
        fn $fn() {
            let mut $sink = RecordingSink::new();
            let mut rx_buffer = [0; 8];
            let cs = DummyPin {};
            let dr = DummyPin {};
            let rst = DummyPin {};
            let mut bnrg = BlueNRG::new(&mut rx_buffer, cs, dr, rst);
            bnrg.with_spi(&mut $sink, |$controller| $act);
            assert_eq!($sink.written_header, [0x0A, 0x00, 0x00, 0x00, 0x00]);
            $assert
        }
    };
}

bnrg_test! {
    fn aci_l2cap_connection_parameter_update_request() {
        |controller| {
            controller.aci_l2cap_connection_parameter_update_request(
                &L2CapConnectionParameterUpdateRequest {
                    conn_handle: hci::ConnectionHandle(0x0201),
                    interval: (Duration::from_millis(30), Duration::from_millis(300)),
                    conn_latency: 10,
                    timeout: Duration::from_millis(1000),
                }
            ).unwrap();
        } => |sink| {
            assert_eq!(
                sink.written_data,
                [1, 0x81, 0xFD, 10, 0x01, 0x02, 0x18, 0x00, 0xF0, 0x00, 0x0A, 0x00, 0x64, 0x00]
            );
        }
    }
}

bnrg_test! {
    fn aci_l2cap_connection_parameter_update_response() {
        |controller| {
            controller.aci_l2cap_connection_parameter_update_response(
                &L2CapConnectionParameterUpdateResponse {
                    conn_handle: hci::ConnectionHandle(0x0201),
                    interval: (Duration::from_millis(30), Duration::from_millis(300)),
                    conn_latency: 10,
                    timeout: Duration::from_millis(1000),
                    expected_connection_length_range: (Duration::from_millis(500),
                                                       Duration::from_millis(1250)),
                    identifier: 0x0F,
                    accepted: true,
                }
            ).unwrap();
        } => |sink| {
            assert_eq!(
                sink.written_data,
                [1, 0x82, 0xFD, 16, 0x01, 0x02, 0x18, 0x00, 0xF0, 0x00, 0x0A, 0x00, 0x64, 0x00,
                 0x20, 0x03, 0xD0, 0x07, 0x0F, 0x01]
            );
        }
    }
}

#[test]
fn aci_l2cap_connection_parameter_update_response_bad_connection_interval() {
    let mut sink = RecordingSink::new();
    let mut rx_buffer = [0; 8];
    let cs = DummyPin {};
    let dr = DummyPin {};
    let rst = DummyPin {};
    let mut bnrg = BlueNRG::new(&mut rx_buffer, cs, dr, rst);
    let err =
        bnrg.with_spi(&mut sink, |controller| {
            controller.aci_l2cap_connection_parameter_update_response(
                &L2CapConnectionParameterUpdateResponse {
                    conn_handle: hci::ConnectionHandle(0x0201),
                    interval: (Duration::from_millis(500), Duration::from_millis(499)),
                    conn_latency: 10,
                    timeout: Duration::from_millis(50),
                    expected_connection_length_range: (
                        Duration::from_millis(7),
                        Duration::from_millis(8),
                    ),
                    identifier: 0x10,
                    accepted: true,
                },
            )
        }).err()
            .unwrap();
    assert_eq!(
        err,
        nb::Error::Other(UartError::BLE(hci::event::Error::Vendor(
            BlueNRGError::BadConnectionInterval(
                Duration::from_millis(500),
                Duration::from_millis(499)
            )
        )))
    );
    assert_eq!(sink.written_data, []);
}

#[test]
fn aci_l2cap_connection_parameter_update_response_bad_expected_connection_length_range() {
    let mut sink = RecordingSink::new();
    let mut rx_buffer = [0; 8];
    let cs = DummyPin {};
    let dr = DummyPin {};
    let rst = DummyPin {};
    let mut bnrg = BlueNRG::new(&mut rx_buffer, cs, dr, rst);
    let err =
        bnrg.with_spi(&mut sink, |controller| {
            controller.aci_l2cap_connection_parameter_update_response(
                &L2CapConnectionParameterUpdateResponse {
                    conn_handle: hci::ConnectionHandle(0x0201),
                    interval: (Duration::from_millis(500), Duration::from_millis(501)),
                    conn_latency: 10,
                    timeout: Duration::from_millis(50),
                    expected_connection_length_range: (
                        Duration::from_millis(9),
                        Duration::from_millis(8),
                    ),
                    identifier: 0x10,
                    accepted: true,
                },
            )
        }).err()
            .unwrap();
    assert_eq!(
        err,
        nb::Error::Other(UartError::BLE(hci::event::Error::Vendor(
            BlueNRGError::BadConnectionLengthRange(
                Duration::from_millis(9),
                Duration::from_millis(8)
            )
        )))
    );
    assert_eq!(sink.written_data, []);
}