use crate::utils;

#[derive(Debug)]
pub enum Error {
  Format,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
  pub(crate) red: u8,
  pub(crate) green: u8,
  pub(crate) blue: u8,
}

impl Color {
  pub fn new(red: u8, green: u8, blue: u8) -> Self {
    Self { red, green, blue }
  }

  pub fn from_hex_string(hex: String) -> Result<Self, Error> {
    let mut hex_vec: Vec<&str> = hex.split("").collect();
    hex_vec.retain(|&x| x != "" && x != "#");
    let mut expanded_vec: Vec<&str> = vec![];
    match hex_vec.len() {
      3 => {
        for item in &hex_vec {
          expanded_vec.extend_from_slice(&[*item, *item]);
        }
      }
      6 => {
        expanded_vec.extend(&hex_vec);
      }
      _ => return Err(Error::Format),
    };
    let red = utils::map_hex(hex_vec[0]) * 16 + utils::map_hex(hex_vec[1]);
    let green = utils::map_hex(hex_vec[2]) * 16 + utils::map_hex(hex_vec[3]);
    let blue = utils::map_hex(hex_vec[4]) * 16 + utils::map_hex(hex_vec[5]);
    Ok(Self {
      red,
      green,
      blue,
    })
  }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_hex_string() {
        assert_eq!(Color::new(255, 0, 0), Color::from_hex_string("#ff0000".to_string()).unwrap());
        assert_eq!(Color::new(0, 255, 0), Color::from_hex_string("#00FF00".to_string()).unwrap());
    }
}