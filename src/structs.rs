

//ALL FROM BELOW: ADD THEIR RESPECTIVE LIBRARIES

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Money(pub f32);// use from another lib like color

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct Color(u8, u8, u8); //We need better info here







