use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// На выбор есть либо изображение либо svg два сразу быть не может
/// Изображение закодированное в формате base64
/// `id - #image`
pub struct Image
{
    ///Идентификатор изображения
    id : String,
    ///Размер изображения в байтах
    length : usize,
    ///Изображение закодированное в формате base64
    data : Option<String>,
    ///Векторное изображение в формате base64
    svg : Option<String>,
    ///Размер изображения в пикселях по оси X
    size_x : usize,
    ///Размер изображения в пикселях по оси Y
    size_y : usize
}