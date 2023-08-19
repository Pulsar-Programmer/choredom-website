//Goodbye WebSockets. Hello XMTPPPSPSPPSPSPSSPPPP or whatever
//XMPPS (XMPP + TLS)

use actix_session::Session;
use actix_web::{web, Responder, get, HttpResponse};

#[get("/chat/{room}")]
async fn chat(room: web::Path<String>, session: Session) -> impl Responder {
    HttpResponse::Ok().body("Sent XMPP message: {message} to {recipient}!")
}




// use std::error::Error;
// use std::str::FromStr;
// use tokio::stream::StreamExt;
// use tokio_xmpp::{Client, Jid, Message, MessageType, Stanza};




// async fn send_message(client: &Client, recipient: &str, message: &str) -> Result<(), Box<dyn Error>> {
//     let message = Message::new(
//         Some(Jid::from_str(recipient)?),
//         Some(Jid::from_str("your_jid")?),
//         MessageType::Chat,
//         message.to_string(),
//         None,
//     );
//     client.send(message).await?;
//     Ok(())
// }


// async fn receive_message(client: &Client) -> Result<(), Box<dyn Error>> {
//     let mut stream = client.stream().await?;
//     while let Some(stanza) = stream.next().await {
//         match stanza? {
//             Stanza::Message(message) => {
//                 println!("Received message: {:?}", message);
//             }
//             _ => (),
//         }
//     }
//     Ok(())
// }








// use tokio::net::TcpStream;
// use tokio_xmpp::xmpp_stream::XMPPStream;
// use quick_xml::Writer;
// use quick_xml::events::{Event, BytesStart, BytesEnd};
// use quick_xml::events::attributes::Attribute;
// use std::str::from_utf8;

// async fn send_message(jid: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
//     let stream = TcpStream::connect("your_xmpp_server:5222").await?;
//     let mut xmpp_stream = XMPPStream::new(stream);

//     let mut writer = Writer::new(Vec::new());
//     writer.write_event(Event::Start(BytesStart::borrowed_name(b"message")))?;
//     writer.write_event(Event::Start(BytesStart::borrowed(b"body", "body".len())))?;
//     writer.write_event(Event::Text(BytesText::from_plain_str(message)))?;
//     writer.write_event(Event::End(BytesEnd::borrowed(b"body")))?;
//     writer.write_event(Event::End(BytesEnd::borrowed(b"message")))?;

//     let message_stanza = from_utf8(writer.inner())?;
//     xmpp_stream.send_stanza(message_stanza).await?;

//     Ok(())
// }


