use serde::{Serialize, Deserialize};
use super::vertical_aligment::VerticalAligment;
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Объект хранящий номер элемента
/// `id - #number`
pub struct Number
{
    ///Основной номер элемента
    val : String,
    ///Значение после номера элемента
    postfix : String,
    ///Изображение закодированное в формате base64
    number_extension : Option<NumberExtension>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
///Cодержит расширение основного номера
struct NumberExtension
{
    ///Значение после основного номера
    val : String,
    ///Формат отображения расширения номера
    format : VerticalAligment
}

