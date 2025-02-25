use ::windows::core::Interface;
use test_winrt::Windows;

#[test]
fn implement() -> ::windows::core::Result<()> {
    let (sender, receiver) = std::sync::mpsc::channel();
    {
        let t = Thing { value: "hello".to_string(), sender };

        let s: Windows::Foundation::IStringable = t.into();
        assert!(s.ToString()? == "hello");

        let c: Windows::Foundation::IClosable = s.cast()?;
        c.Close()?;
        assert!(receiver.recv().unwrap() == "close: hello");
    }
    assert!(receiver.recv().unwrap() == "drop: hello");

    let (sender, receiver) = std::sync::mpsc::channel();
    {
        let t = Thing { value: "world".to_string(), sender };

        let c: Windows::Foundation::IClosable = t.into();
        c.Close()?;
        assert!(receiver.recv().unwrap() == "close: world");

        let s: Windows::Foundation::IStringable = c.cast()?;
        assert!(s.ToString()? == "world");
    }
    assert!(receiver.recv().unwrap() == "drop: world");

    let (sender, receiver) = std::sync::mpsc::channel();
    {
        let t = Thing { value: "object".to_string(), sender };

        let s: Windows::Foundation::IStringable = t.into();
        assert!(s.ToString()? == "object");

        // Confirms that the conversion to `IInspectable` properly handles
        // reference counting.
        let _: ::windows::core::IInspectable = s.into();
    }
    assert!(receiver.recv().unwrap() == "drop: object");

    Ok(())
}

#[::windows::core::implement(Windows::Foundation::{IStringable, IClosable})]
struct Thing {
    value: String,
    sender: std::sync::mpsc::Sender<String>,
}

impl Drop for Thing {
    fn drop(&mut self) {
        self.sender.send(format!("drop: {}", self.value)).unwrap();
    }
}

#[allow(non_snake_case)]
impl Thing {
    fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        Ok(::windows::core::HSTRING::from(&self.value))
    }

    fn Close(&self) -> ::windows::core::Result<()> {
        self.sender.send(format!("close: {}", self.value)).unwrap();
        Ok(())
    }
}
