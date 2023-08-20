use tokio::net::TcpStream;
use tokio_xmpp::xmpp_stream::XMPPStream;
use quick_xml::Writer;
use quick_xml::events::{Event, BytesStart, BytesEnd};
use quick_xml::events::attributes::Attribute;
use std::str::from_utf8;

async fn send_message(xmpp_stream: &mut XMPPStream<TcpStream>, jid: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = Writer::new(Vec::new());
    writer.write_event(Event::Start(BytesStart::borrowed_name(b"message")))?;
    writer.write_event(Event::Start(BytesStart::borrowed(b"body", "body".len())))?;
    writer.write_event(Event::Text(BytesText::from_plain_str(message)))?;
    writer.write_event(Event::End(BytesEnd::borrowed(b"body")))?;
    writer.write_event(Event::End(BytesEnd::borrowed(b"message")))?;

    let message_stanza = from_utf8(writer.inner())?;
    xmpp_stream.send_stanza(message_stanza).await?;

    Ok(())
}

async fn receive_message(xmpp_stream: &mut XMPPStream<TcpStream>) -> Result<String, Box<dyn std::error::Error>> {
    loop {
        let stanza = xmpp_stream.next_stanza().await?;
        let reader = Reader::from_str(&stanza);
        let mut buf = Vec::new();

        for e in reader {
            match e? {
                Event::Start(ref e) if e.name() == b"message" => {
                    // We found a message stanza, now we need to look for the body
                    for e in reader {
                        match e? {
                            Event::Start(ref e) if e.name() == b"body" => {
                                // We found the body, let's return its text
                                let text = reader.read_text(b"body", &mut buf)?;
                                return Ok(text);
                            },
                            _ => {},
                        }
                    }
                },
                _ => {},
            }
        }
    }
}
