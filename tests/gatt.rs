extern crate bluenrg;
extern crate bluetooth_hci as hci;
extern crate nb;

mod fixture;

use bluenrg::gatt::*;
use fixture::Fixture;

#[test]
fn init() {
    let mut fixture = Fixture::new();
    fixture.act(|controller| controller.init()).unwrap();
    assert!(fixture.wrote_header());
    assert!(fixture.wrote(&[1, 0x01, 0xFD, 0]));
}

#[test]
fn add_service_16() {
    let mut fixture = Fixture::new();
    fixture
        .act(|controller| {
            controller.add_service(&AddServiceParameters {
                uuid: Uuid::Uuid16(0x0201),
                service_type: ServiceType::Primary,
                max_attribute_records: 3,
            })
        }).unwrap();
    assert!(fixture.wrote_header());
    assert!(fixture.wrote(&[1, 0x02, 0xFD, 5, 0x01, 0x01, 0x02, 0x01, 3]));
}

#[test]
fn add_service_128() {
    let mut fixture = Fixture::new();
    fixture
        .act(|controller| {
            controller.add_service(&AddServiceParameters {
                uuid: Uuid::Uuid128([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]),
                service_type: ServiceType::Secondary,
                max_attribute_records: 255,
            })
        }).unwrap();
    assert!(fixture.wrote_header());
    assert!(fixture.wrote(&[
        1, 0x02, 0xFD, 19, 0x02, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 0x02, 255
    ]));
}

#[test]
fn include_service_16() {
    let mut fixture = Fixture::new();
    fixture
        .act(|controller| {
            controller.include_service(&IncludeServiceParameters {
                service_handle: ServiceHandle(0x0201),
                include_handle_range: Range::new(ServiceHandle(0x0403), ServiceHandle(0x0605))
                    .unwrap(),
                include_uuid: Uuid::Uuid16(0x0807),
            })
        }).unwrap();
    assert!(fixture.wrote_header());
    assert!(
        fixture.wrote(&[1, 0x03, 0xFD, 9, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x01, 0x07, 0x08])
    );
}

#[test]
fn include_service_128() {
    let mut fixture = Fixture::new();
    fixture
        .act(|controller| {
            controller.include_service(&IncludeServiceParameters {
                service_handle: ServiceHandle(0x0201),
                include_handle_range: Range::new(ServiceHandle(0x0403), ServiceHandle(0x0605))
                    .unwrap(),
                include_uuid: Uuid::Uuid128([
                    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C,
                    0x1D, 0x1E, 0x1F,
                ]),
            })
        }).unwrap();
    assert!(fixture.wrote_header());
    assert!(fixture.wrote(&[
        1, 0x03, 0xFD, 23, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x02, 0x10, 0x11, 0x12, 0x13, 0x14,
        0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
    ]));
}

#[test]
fn bad_range() {
    let err = Range::new(ServiceHandle(0x0201), ServiceHandle(0x0102))
        .err()
        .unwrap();
    assert_eq!(err, RangeError::Inverted);

    // Both ends of the range may be equal
    Range::new(ServiceHandle(0x0201), ServiceHandle(0x0201)).unwrap();
}

#[test]
fn add_characteristic_16() {
    let mut fixture = Fixture::new();
    fixture
        .act(|controller| {
            controller.add_characteristic(&AddCharacteristicParameters {
                service_handle: ServiceHandle(0x0201),
                characteristic_uuid: Uuid::Uuid16(0x0403),
                characteristic_value_len: 0x0605,
                characteristic_properties: CharacteristicProperty::BROADCAST
                    | CharacteristicProperty::READ
                    | CharacteristicProperty::NOTIFY,
                security_permissions: CharacteristicPermission::AUTHENTICATED_READ
                    | CharacteristicPermission::AUTHENTICATED_WRITE,
                gatt_event_mask: CharacteristicEvent::ATTRIBUTE_WRITE
                    | CharacteristicEvent::CONFIRM_WRITE
                    | CharacteristicEvent::CONFIRM_READ,
                encryption_key_size: EncryptionKeySize::with_value(8).unwrap(),
                is_variable: true,
            })
        }).unwrap();
    assert!(fixture.wrote_header());
    assert!(fixture.wrote(&[
        1, 0x04, 0xFD, 12, 0x01, 0x02, 0x01, 0x03, 0x04, 0x05, 0x06, 0x13, 0x09, 0x07, 8, 1
    ]));
}

#[test]
fn add_characteristic_128() {
    let mut fixture = Fixture::new();
    fixture
        .act(|controller| {
            controller.add_characteristic(&AddCharacteristicParameters {
                service_handle: ServiceHandle(0x0201),
                characteristic_uuid: Uuid::Uuid128([
                    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C,
                    0x1D, 0x1E, 0x1F,
                ]),
                characteristic_value_len: 0x0605,
                characteristic_properties: CharacteristicProperty::BROADCAST
                    | CharacteristicProperty::READ
                    | CharacteristicProperty::NOTIFY,
                security_permissions: CharacteristicPermission::AUTHENTICATED_READ
                    | CharacteristicPermission::AUTHENTICATED_WRITE,
                gatt_event_mask: CharacteristicEvent::ATTRIBUTE_WRITE
                    | CharacteristicEvent::CONFIRM_WRITE
                    | CharacteristicEvent::CONFIRM_READ,
                encryption_key_size: EncryptionKeySize::with_value(8).unwrap(),
                is_variable: true,
            })
        }).unwrap();
    assert!(fixture.wrote_header());
    assert!(fixture.wrote(&[
        1, 0x04, 0xFD, 26, 0x01, 0x02, 0x02, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18,
        0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 0x05, 0x06, 0x13, 0x09, 0x07, 8, 1
    ]));
}

#[test]
fn encryption_key_size_range() {
    assert_eq!(
        EncryptionKeySizeError::TooShort,
        EncryptionKeySize::with_value(6).err().unwrap()
    );
    for size in 7..=16 {
        assert_eq!(EncryptionKeySize::with_value(size).unwrap().value(), size);
    }
    assert_eq!(
        EncryptionKeySizeError::TooLong,
        EncryptionKeySize::with_value(17).err().unwrap()
    );
}

#[test]
fn add_characteristic_descriptor_16() {
    let mut fixture = Fixture::new();
    fixture
        .act(|controller| {
            controller.add_characteristic_descriptor(&AddDescriptorParameters {
                service_handle: ServiceHandle(0x0201),
                characteristic_handle: CharacteristicHandle(0x0403),
                descriptor_uuid: KnownDescriptor::CharacteristicExtendedProperties.into(),
                descriptor_value_max_len: 7,
                descriptor_value: &[1, 2, 3, 4],
                security_permissions: DescriptorPermission::AUTHENTICATED
                    | DescriptorPermission::AUTHORIZED,
                access_permissions: AccessPermission::READ | AccessPermission::WRITE,
                gatt_event_mask: CharacteristicEvent::ATTRIBUTE_WRITE
                    | CharacteristicEvent::CONFIRM_WRITE
                    | CharacteristicEvent::CONFIRM_READ,
                encryption_key_size: EncryptionKeySize::with_value(8).unwrap(),
                is_variable: true,
            })
        }).unwrap();
    assert!(fixture.wrote_header());
    assert!(fixture.wrote(&[
        1, 0x05, 0xFD, 18, 0x01, 0x02, 0x03, 0x04, 0x01, 0x00, 0x29, 7, 4, 1, 2, 3, 4, 0x03, 0x03,
        0x07, 8, 1,
    ]));
}

#[test]
fn add_characteristic_descriptor_128() {
    let mut fixture = Fixture::new();
    fixture
        .act(|controller| {
            controller.add_characteristic_descriptor(&AddDescriptorParameters {
                service_handle: ServiceHandle(0x0201),
                characteristic_handle: CharacteristicHandle(0x0403),
                descriptor_uuid: Uuid::Uuid128([
                    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C,
                    0x1D, 0x1E, 0x1F,
                ]),
                descriptor_value_max_len: 7,
                descriptor_value: &[1, 2, 3, 4, 5, 6],
                security_permissions: DescriptorPermission::AUTHENTICATED
                    | DescriptorPermission::AUTHORIZED,
                access_permissions: AccessPermission::READ | AccessPermission::WRITE,
                gatt_event_mask: CharacteristicEvent::ATTRIBUTE_WRITE
                    | CharacteristicEvent::CONFIRM_WRITE
                    | CharacteristicEvent::CONFIRM_READ,
                encryption_key_size: EncryptionKeySize::with_value(8).unwrap(),
                is_variable: true,
            })
        }).unwrap();
    assert!(fixture.wrote_header());
    assert!(fixture.wrote(&[
        1, 0x05, 0xFD, 34, 0x01, 0x02, 0x03, 0x04, 0x02, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16,
        0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 7, 6, 1, 2, 3, 4, 5, 6, 0x03, 0x03,
        0x07, 8, 1,
    ]));
}

#[test]
fn add_characteristic_descriptor_too_long() {
    let mut fixture = Fixture::new();
    let err = fixture
        .act(|controller| {
            controller.add_characteristic_descriptor(&AddDescriptorParameters {
                service_handle: ServiceHandle(0x0201),
                characteristic_handle: CharacteristicHandle(0x0403),
                descriptor_uuid: Uuid::Uuid128([
                    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C,
                    0x1D, 0x1E, 0x1F,
                ]),
                descriptor_value_max_len: 7,
                descriptor_value: &[1, 2, 3, 4, 5, 6, 7, 8],
                security_permissions: DescriptorPermission::AUTHENTICATED
                    | DescriptorPermission::AUTHORIZED,
                access_permissions: AccessPermission::READ | AccessPermission::WRITE,
                gatt_event_mask: CharacteristicEvent::ATTRIBUTE_WRITE
                    | CharacteristicEvent::CONFIRM_WRITE
                    | CharacteristicEvent::CONFIRM_READ,
                encryption_key_size: EncryptionKeySize::with_value(8).unwrap(),
                is_variable: true,
            })
        }).err()
        .unwrap();
    assert!(!fixture.wrote_header());
    assert_eq!(err, nb::Error::Other(Error::DescriptorTooLong));
}

#[test]
fn add_characteristic_descriptor_buffer_too_long() {
    let mut fixture = Fixture::new();
    let err = fixture
        .act(|controller| {
            controller.add_characteristic_descriptor(&AddDescriptorParameters {
                service_handle: ServiceHandle(0x0201),
                characteristic_handle: CharacteristicHandle(0x0403),
                descriptor_uuid: Uuid::Uuid128([
                    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C,
                    0x1D, 0x1E, 0x1F,
                ]),
                descriptor_value_max_len: 256 - 28,
                descriptor_value: &[0; 8],
                security_permissions: DescriptorPermission::AUTHENTICATED
                    | DescriptorPermission::AUTHORIZED,
                access_permissions: AccessPermission::READ | AccessPermission::WRITE,
                gatt_event_mask: CharacteristicEvent::ATTRIBUTE_WRITE
                    | CharacteristicEvent::CONFIRM_WRITE
                    | CharacteristicEvent::CONFIRM_READ,
                encryption_key_size: EncryptionKeySize::with_value(8).unwrap(),
                is_variable: true,
            })
        }).err()
        .unwrap();
    assert!(!fixture.wrote_header());
    assert_eq!(err, nb::Error::Other(Error::DescriptorBufferTooLong));
}

#[test]
fn update_characteristic_value() {
    let mut fixture = Fixture::new();
    fixture
        .act(|controller| {
            controller.update_characteristic_value(&UpdateCharacteristicValueParameters {
                service_handle: ServiceHandle(0x0201),
                characteristic_handle: CharacteristicHandle(0x0403),
                offset: 0,
                value: &[1, 2, 3, 4, 5],
            })
        }).unwrap();
    assert!(fixture.wrote_header());
    assert!(fixture.wrote(&[1, 0x06, 0xFD, 11, 0x01, 0x02, 0x03, 0x04, 0, 5, 1, 2, 3, 4, 5]));
}

#[test]
fn delete_characteristic() {
    let mut fixture = Fixture::new();
    fixture
        .act(|controller| {
            controller.delete_characteristic(ServiceHandle(0x0201), CharacteristicHandle(0x0403))
        }).unwrap();
    assert!(fixture.wrote_header());
    assert!(fixture.wrote(&[1, 0x07, 0xFD, 4, 0x01, 0x02, 0x03, 0x04]));
}

#[test]
fn delete_service() {
    let mut fixture = Fixture::new();
    fixture
        .act(|controller| controller.delete_service(ServiceHandle(0x0201)))
        .unwrap();
    assert!(fixture.wrote_header());
    assert!(fixture.wrote(&[1, 0x08, 0xFD, 2, 0x01, 0x02]));
}

#[test]
fn delete_included_service() {
    let mut fixture = Fixture::new();
    fixture
        .act(|controller| {
            controller.delete_included_service(&DeleteIncludedServiceParameters {
                service: ServiceHandle(0x0201),
                included_service: ServiceHandle(0x0403),
            })
        }).unwrap();
    assert!(fixture.wrote_header());
    assert!(fixture.wrote(&[1, 0x09, 0xFD, 4, 0x01, 0x02, 0x03, 0x04]));
}

#[test]
fn set_event_mask() {
    let mut fixture = Fixture::new();
    fixture
        .act(|controller| {
            controller.set_event_mask(
                Event::ATTRIBUTE_MODIFIED
                    | Event::FIND_INFORMATION_RESPONSE
                    | Event::INDICATION
                    | Event::NOTIFICATION,
            )
        }).unwrap();
    assert!(fixture.wrote_header());
    assert!(fixture.wrote(&[1, 0x0A, 0xFD, 4, 0x09, 0x60, 0x00, 0x00]));
}

#[test]
fn exchange_configuration() {
    let mut fixture = Fixture::new();
    fixture
        .act(|controller| controller.exchange_configuration(hci::ConnectionHandle(0x0201)))
        .unwrap();
    assert!(fixture.wrote_header());
    assert!(fixture.wrote(&[1, 0x0B, 0xFD, 2, 0x01, 0x02]));
}

#[test]
fn find_information_request() {
    let mut fixture = Fixture::new();
    fixture
        .act(|controller| {
            controller.find_information_request(
                hci::ConnectionHandle(0x0201),
                Range::new(AttributeHandle(0x0403), AttributeHandle(0x0605)).unwrap(),
            )
        }).unwrap();
    assert!(fixture.wrote_header());
    assert!(fixture.wrote(&[1, 0x0C, 0xFD, 6, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06]));
}

#[test]
fn find_by_type_value_request() {
    let mut fixture = Fixture::new();
    fixture
        .act(|controller| {
            controller.find_by_type_value_request(&FindByTypeValueParameters {
                conn_handle: hci::ConnectionHandle(0x0201),
                attribute_handle_range: Range::new(
                    AttributeHandle(0x0403),
                    AttributeHandle(0x0605),
                ).unwrap(),
                uuid: Uuid16(0x0807),
                value: &[9, 10, 11, 12],
            })
        }).unwrap();
    assert!(fixture.wrote_header());
    assert!(fixture.wrote(&[
        1, 0x0D, 0xFD, 13, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 4, 9, 10, 11, 12,
    ]));
}

#[test]
fn find_by_type_value_request_value_too_long() {
    let mut fixture = Fixture::new();
    let err = fixture
        .act(|controller| {
            controller.find_by_type_value_request(&FindByTypeValueParameters {
                conn_handle: hci::ConnectionHandle(0x0201),
                attribute_handle_range: Range::new(
                    AttributeHandle(0x0403),
                    AttributeHandle(0x0605),
                ).unwrap(),
                uuid: Uuid16(0x0807),
                value: &[0; 247],
            })
        }).err()
        .unwrap();
    assert_eq!(err, nb::Error::Other(Error::ValueBufferTooLong));
    assert!(!fixture.wrote_header());
}

#[test]
fn read_by_type_request_16() {
    let mut fixture = Fixture::new();
    fixture
        .act(|controller| {
            controller.read_by_type_request(&ReadByTypeParameters {
                conn_handle: hci::ConnectionHandle(0x0201),
                attribute_handle_range: Range::new(
                    AttributeHandle(0x0403),
                    AttributeHandle(0x0605),
                ).unwrap(),
                uuid: Uuid::Uuid16(0x0807),
            })
        }).unwrap();
    assert!(fixture.wrote_header());
    assert!(
        fixture.wrote(&[1, 0x0E, 0xFD, 9, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x01, 0x07, 0x08,])
    );
}

#[test]
fn read_by_type_request_128() {
    let mut fixture = Fixture::new();
    fixture
        .act(|controller| {
            controller.read_by_type_request(&ReadByTypeParameters {
                conn_handle: hci::ConnectionHandle(0x0201),
                attribute_handle_range: Range::new(
                    AttributeHandle(0x0403),
                    AttributeHandle(0x0605),
                ).unwrap(),
                uuid: Uuid::Uuid128([
                    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C,
                    0x1D, 0x1E, 0x1F,
                ]),
            })
        }).unwrap();
    assert!(fixture.wrote_header());
    assert!(fixture.wrote(&[
        1, 0x0E, 0xFD, 23, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x02, 0x10, 0x11, 0x12, 0x13, 0x14,
        0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
    ]));
}

#[test]
fn read_by_group_type_request_16() {
    let mut fixture = Fixture::new();
    fixture
        .act(|controller| {
            controller.read_by_group_type_request(&ReadByTypeParameters {
                conn_handle: hci::ConnectionHandle(0x0201),
                attribute_handle_range: Range::new(
                    AttributeHandle(0x0403),
                    AttributeHandle(0x0605),
                ).unwrap(),
                uuid: Uuid::Uuid16(0x0807),
            })
        }).unwrap();
    assert!(fixture.wrote_header());
    assert!(
        fixture.wrote(&[1, 0x0F, 0xFD, 9, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x01, 0x07, 0x08,])
    );
}

#[test]
fn read_by_group_type_request_128() {
    let mut fixture = Fixture::new();
    fixture
        .act(|controller| {
            controller.read_by_group_type_request(&ReadByTypeParameters {
                conn_handle: hci::ConnectionHandle(0x0201),
                attribute_handle_range: Range::new(
                    AttributeHandle(0x0403),
                    AttributeHandle(0x0605),
                ).unwrap(),
                uuid: Uuid::Uuid128([
                    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C,
                    0x1D, 0x1E, 0x1F,
                ]),
            })
        }).unwrap();
    assert!(fixture.wrote_header());
    assert!(fixture.wrote(&[
        1, 0x0F, 0xFD, 23, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x02, 0x10, 0x11, 0x12, 0x13, 0x14,
        0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
    ]));
}

#[test]
fn prepare_write_request() {
    let mut fixture = Fixture::new();
    fixture
        .act(|controller| {
            controller.prepare_write_request(&WriteRequest {
                conn_handle: hci::ConnectionHandle(0x0201),
                attribute_handle: AttributeHandle(0x0403),
                offset: 0x0605,
                value: &[8, 9, 10, 11, 12, 13, 14],
            })
        }).unwrap();
    assert!(fixture.wrote_header());
    assert!(fixture.wrote(&[
        1, 0x10, 0xFD, 14, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xA, 0xB, 0xC, 0xD, 0xE,
    ]));
}

#[test]
fn prepare_write_request_too_long() {
    let mut fixture = Fixture::new();
    let err = fixture
        .act(|controller| {
            controller.prepare_write_request(&WriteRequest {
                conn_handle: hci::ConnectionHandle(0x0201),
                attribute_handle: AttributeHandle(0x0403),
                offset: 0x0605,
                value: &[0; 248],
            })
        }).err()
        .unwrap();
    assert_eq!(err, nb::Error::Other(Error::ValueBufferTooLong));
    assert!(!fixture.wrote_header());
}

#[test]
fn execute_write_request() {
    let mut fixture = Fixture::new();
    fixture
        .act(|controller| controller.execute_write_request(hci::ConnectionHandle(0x0201)))
        .unwrap();
    assert!(fixture.wrote_header());
    assert!(fixture.wrote(&[1, 0x11, 0xFD, 3, 0x1, 0x2, 1]));
}

#[test]
fn cancel_write_request() {
    let mut fixture = Fixture::new();
    fixture
        .act(|controller| controller.cancel_write_request(hci::ConnectionHandle(0x0201)))
        .unwrap();
    assert!(fixture.wrote_header());
    assert!(fixture.wrote(&[1, 0x11, 0xFD, 3, 0x1, 0x2, 0]));
}

#[test]
fn discover_all_primary_services() {
    let mut fixture = Fixture::new();
    fixture
        .act(|controller| controller.discover_all_primary_services(hci::ConnectionHandle(0x0201)))
        .unwrap();
    assert!(fixture.wrote_header());
    assert!(fixture.wrote(&[1, 0x12, 0xFD, 2, 0x1, 0x2]));
}

#[test]
fn discovery_primary_services_by_uuid_16() {
    let mut fixture = Fixture::new();
    fixture
        .act(|controller| {
            controller.discover_primary_services_by_uuid(
                hci::ConnectionHandle(0x0201),
                Uuid::Uuid16(0x0403),
            )
        }).unwrap();
    assert!(fixture.wrote_header());
    assert!(fixture.wrote(&[1, 0x13, 0xFD, 5, 0x01, 0x02, 0x01, 0x03, 0x04]));
}

#[test]
fn discovery_primary_services_by_uuid_128() {
    let mut fixture = Fixture::new();
    fixture
        .act(|controller| {
            controller.discover_primary_services_by_uuid(
                hci::ConnectionHandle(0x0201),
                Uuid::Uuid128([
                    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C,
                    0x1D, 0x1E, 0x1F,
                ]),
            )
        }).unwrap();
    assert!(fixture.wrote_header());
    assert!(fixture.wrote(&[
        1, 0x13, 0xFD, 19, 0x01, 0x02, 0x02, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18,
        0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
    ]));
}

#[test]
fn find_included_services() {
    let mut fixture = Fixture::new();
    fixture
        .act(|controller| {
            controller.find_included_services(
                hci::ConnectionHandle(0x0201),
                Range::new(ServiceHandle(0x0403), ServiceHandle(0x0605)).unwrap(),
            )
        }).unwrap();
    assert!(fixture.wrote_header());
    assert!(fixture.wrote(&[1, 0x14, 0xFD, 6, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06]));
}
