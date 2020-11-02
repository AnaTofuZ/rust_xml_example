use quick_xml::events::{BytesEnd, BytesText, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::fs::File;
use std::io::{BufReader, BufWriter, Error};

const XML_NAME_ATTRIBUTE: &[u8; 4] = b"name";

fn main() -> Result<(), Error> {
    let file = "./template.xml";
    let mut reader = Reader::from_reader(BufReader::new(File::open(file)?));
    let mut writer = Writer::new(BufWriter::new(File::create("dump.xml").unwrap()));
    let mut buf = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) if e.name() == XML_NAME_ATTRIBUTE => {
                writer
                    .write_event(Event::Start(e.clone()))
                    .expect("faild write event");
                reader
                    .read_event(&mut Vec::new())
                    .expect("faild read event");
                let elem = BytesText::from_plain_str("anatofuz-vm");
                writer.write_event(Event::Text(elem)).unwrap();
            }

            Ok(Event::Text(ref e)) if e.escaped() == b"ie-virsh-template" => {
                let elem = BytesText::from_plain_str("anatofuz-vm");
                writer.write_event(Event::Text(elem)).unwrap();
            }
            Ok(Event::End(ref e)) if e.name() == b"this_tag" => {
                assert!(writer
                    .write_event(Event::End(BytesEnd::borrowed(b"my_elem")))
                    .is_ok());
            }
            Ok(Event::Eof) => break,
            // you can use either `e` or `&e` if you don't want to move the event
            Ok(e) => writer.write_event(&e).unwrap(),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
        }
        buf.clear();
    }

    Ok(())
}
