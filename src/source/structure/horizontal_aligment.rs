use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HorizontalAligment
{
    ///Нет особенного форматирования
    None,
    ///Нижний индекс - ₁
    Subscrypt,
	///Верхний индекс - ¹
    Superscrypt
}

impl std::fmt::Display for HorizontalAligment
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result 
    {
        let obj = match self
        {
            HorizontalAligment::None => "none",
            HorizontalAligment::Superscrypt => "superscrypt",
            HorizontalAligment::Subscrypt => "subscrypt",
        };
        write!(f, "{}", obj)
    }
}