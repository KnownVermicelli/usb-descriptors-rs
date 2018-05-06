#[macro_export]
macro_rules! device_descriptor {
    // version with default everything
    () => {
        device_descriptor!(0x1d6b);
    };
    // version with default productID
    ($vendorID:expr) => {
        device_descriptor!($vendorID, 0);
    };

    // version with default release number (1.0)
    ($vendorID:expr, $productID:expr) => {
        device_descriptor!($vendorID, $productID, 0, 1)
    };
    // version with default number of configurations(1)
    ($vendorID:expr, $productID:expr, $releaseNumberMinor:expr, $releaseNumberMajor:expr) => {
        device_descriptor!(
            $vendorID,
            $productID,
            $releaseNumberMinor,
            $releaseNumberMajor,
            1
        )
    };
    // version with default usb version (2.0)
    (
        $vendorID:expr,
        $productID:expr,
        $releaseNumberMinor:expr,
        $releaseNumberMajor:expr,
        $numberOfConfigurations:expr
    ) => {
        device_descriptor!(
            $vendorID,
            $productID,
            $releaseNumberMinor,
            $releaseNumberMajor,
            $numberOfConfigurations,
            2,
            0
        )
    };
    // version with default device protocols (forward to interface)
    (
        $vendorID:expr,
        $productID:expr,
        $releaseNumberMinor:expr,
        $releaseNumberMajor:expr,
        $numberOfConfigurations:expr,
        $usbMajor:expr,
        $usbMinor:expr
    ) => {
        device_descriptor!(
            $vendorID,
            $productID,
            $releaseNumberMinor,
            $releaseNumberMajor,
            $numberOfConfigurations,
            $usbMajor,
            $usbMinor,
            0,
            0,
            0
        )
    };
    // version with default package size
    (
        $vendorID:expr,
        $productID:expr,
        $releaseNumberMinor:expr,
        $releaseNumberMajor:expr,
        $numberOfConfigurations:expr,
        $usbMajor:expr,
        $usbMinor:expr,
        $deviceClass:expr,
        $deviceSubclass:expr,
        $deviceProtocol:expr
    ) => {
        device_descriptor!(
            $vendorID,
            $productID,
            $releaseNumberMinor,
            $releaseNumberMajor,
            $numberOfConfigurations,
            $usbMajor,
            $usbMinor,
            $deviceClass,
            $deviceSubclass,
            $deviceProtocol,
            64
        )
    };

    // specify everything version
    (
        $vendorID:expr,
        $productID:expr,
        $releaseNumberMinor:expr,
        $releaseNumberMajor:expr,
        $numberOfConfigurations:expr,
        $usbMajor:expr,
        $usbMinor:expr,
        $deviceClass:expr,
        $deviceSubclass:expr,
        $deviceProtocol:expr,
        $maximumPacketSize:expr
    ) => {
        [
            18u8, // Length of this descriptor
            0x01, // Type of this descriptor
            $usbMinor,
            $usbMajor,
            $deviceClass,    // Device class - 0 means read from interface descriptor
            $deviceSubclass, // bDeviceSubClass
            $deviceProtocol, // bDeviceProtocol
            $maximumPacketSize, // 64  - maximum packet size. Potentially device-specific
            lower!($vendorID),
            upper!($vendorID), // VendorID - 1d6b is Linux Foundation VendorID,
            lower!($productID),
            upper!($productID), // ProductID - 0012; they are not using this productId
            $releaseNumberMinor,
            $releaseNumberMajor,     // Device release number - 0.01
            0x01,                    // Manufacturer string index
            0x02,                    // Product string index
            0x03,                    // SerialNumber string index
            $numberOfConfigurations, // Number of configurations
        ]
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn vendor_id() {
        let descriptor = device_descriptor!();
        assert_eq!(descriptor[8], 0x6b); // default value
        assert_eq!(descriptor[9], 0x1d); // default value
        let descriptor2 = device_descriptor!(0xffee);
        assert_eq!(descriptor2[8], 0xee);
        assert_eq!(descriptor2[9], 0xff);
    }

    #[test]
    fn product_id() {
        let descriptor = device_descriptor!();
        assert_eq!(descriptor[10], 0); // default value
        assert_eq!(descriptor[11], 0); // default value
        let descriptor2 = device_descriptor!(0, 0xabcd);
        assert_eq!(descriptor2[10], 0xcd);
        assert_eq!(descriptor2[11], 0xab);
    }

    #[test]
    fn release_number() {
        let descriptor = device_descriptor!();
        assert_eq!(descriptor[12], 0); // default value
        assert_eq!(descriptor[13], 1); // default value
        let descriptor2 = device_descriptor!(0, 0, 13, 2);
        assert_eq!(descriptor2[12], 13);
        assert_eq!(descriptor2[13], 2);
    }

    #[test]
    fn number_of_configs() {
        let descriptor = device_descriptor!();
        assert_eq!(descriptor[17], 1); // default value

        let descriptor2 = device_descriptor!(0, 0, 13, 2, 12);
        assert_eq!(descriptor2[17], 12);
    }

    #[test]
    fn usb_version() {
        let descriptor = device_descriptor!();
        assert_eq!(descriptor[2], 0); // default value
        assert_eq!(descriptor[3], 2); // default value

        let descriptor2 = device_descriptor!(0, 0, 13, 2, 12, 3, 1);
        assert_eq!(descriptor2[2], 1);
        assert_eq!(descriptor2[3], 3);
    }

    #[test]
    fn device_protocols() {
        let descriptor = device_descriptor!();
        assert_eq!(descriptor[4], 0); // default value
        assert_eq!(descriptor[5], 0); // default value
        assert_eq!(descriptor[6], 0); // default value

        let descriptor2 = device_descriptor!(0, 0, 13, 2, 12, 3, 1, 0xa, 0xb, 0xc);
        assert_eq!(descriptor2[4], 0xa);
        assert_eq!(descriptor2[5], 0xb);
        assert_eq!(descriptor2[6], 0xc);
    }

    #[test]
    fn package_size() {
        let descriptor = device_descriptor!();
        assert_eq!(descriptor[7], 64); // default value

        let descriptor2 = device_descriptor!(0, 0, 13, 2, 12, 3, 1, 0xa, 0xb, 0xc, 32);
        assert_eq!(descriptor2[7], 32);
    }
}
