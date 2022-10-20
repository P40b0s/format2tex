use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VerticalAligment
{
    ///Нет особенного форматирования
    None,
    ///Нижний индекс - ₁
    Subscrypt,
	///Верхний индекс - ¹
    Superscrypt
}

impl std::fmt::Display for VerticalAligment
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result 
    {
        let obj = match self
        {
            VerticalAligment::None => "none",
            VerticalAligment::Superscrypt => "superscrypt",
            VerticalAligment::Subscrypt => "subscrypt",
        };
        write!(f, "{}", obj)
    }
}