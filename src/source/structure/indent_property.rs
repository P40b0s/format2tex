use serde::{Serialize, Deserialize};
use super::vertical_aligment::VerticalAligment;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Свойства абзаца
/// `id - #indent_property`
pub struct IndentProperty
{
    ///Идентификатор элемента
    id : String,
    ///Растягивание текста по ширине. (по умолчанию - 1) <br>
    /// Может иметь значение от 1 до 600, что соответсвует процентному соотношению растягивания
    #[serde(default = "default_spacing")]
    spacing : u16,
    ///Жирный шрифт
    #[serde(default = "default_bool_false")]
    is_bold : bool,
    ///Курсив
    #[serde(default = "default_bool_false")]
    is_italic : bool,
    ///Подчеркивание
    #[serde(default = "default_bool_false")]
    underline : bool,
    ///Все буквы заглавные
    #[serde(default = "default_bool_false")]
    caps : bool,
    ///Капитель <br>
    ///Малые заглавные, или капите́ль — начертание в гарнитуре, в которой строчные знаки выглядят как уменьшенные заглавные.
    #[serde(default = "default_bool_false")]
    small_caps : bool,
    ///Цвет шрифта
    #[serde(default = "default_font_color")]
    color : String,
    ///Размер шрифта в типографских пунктах <br>
    /// Типографский пункт — единица измерения кегля шрифта и основная единица измерения в типографике. <br> В разных странах в типографиях с ручным набором пункт был различным — от 0,18 до 0,4 мм, в зависимости от определения фута.<br> К концу XIX века пункт определялся как 0,35–0,38 мм в зависимости от страны. <br> С конца XVIII века в Российских типографиях использовалась французская система Дидо, в которой фут был равен 32,48 мм, дюйм (1/12 фута равен 27,06 мм и пункт (1/72 дюйма) был равен 0,3759 мм. <br> (для перевода в пиксели необходимо умножить на 1.3281)
    #[serde(default = "default_font_size")]
    font_size : f32,
    ///Текст в виде верхнего или нижнего индекса
    #[serde(default = "default_va")]
    vertical_aligment : VerticalAligment
}
fn default_spacing() -> u16 
{
    1
}
fn default_bool_false() -> bool
{
    false
}
fn default_font_color() -> String
{
    String::from("#000000")
}
fn default_font_size() -> f32
{
    12f32
}
fn default_va() -> VerticalAligment
{
    VerticalAligment::None
}