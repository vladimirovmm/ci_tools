use std::fs::File;
use std::io::{Read, Write};
use rustc_serialize::json;

///
/// Настройки для работы CiTools
///
#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct CiToolsSettings{
    pub ci_path:Option<String>,                                                                     // Рабочая директория
    pub langs:Option<Vec<String>>,                                                                  // Список языков
    pub yandexapikey:Option<String>,                                                                // key для yandex-translate
}
impl CiToolsSettings{
    ///
    /// Получить Объект настроек
    ///
    pub fn new()->CiToolsSettings{
        match CiToolsSettings::load() {
            Some(t) => t,
            None=>{
                CiToolsSettings{
                    ci_path: None,
                    langs: None,
                    yandexapikey:None
                }
            }
        }
    }
    ///
    /// Инициализировать объект настроек из файла
    ///
    pub fn load()->Option<CiToolsSettings>{
        let mut buf = String::new();
        let file = File::open("citool.conf");
        if file.is_err() || file.unwrap().read_to_string(&mut buf).is_err() {
            println!("CiToolsSettings: Нет сохранёных настроек или не удалось их получить");
            return None;
        }
        let t = json::decode::<CiToolsSettings>(buf.as_str());
        if t.is_err() {
            println!("CiToolsSettings: ошибки в файле настроек");
            return None;
        }
        return Some(t.unwrap());
    }
    ///
    /// Сохранить настройки в файл
    ///
    pub fn save(&mut self)->&mut CiToolsSettings{
        let buf = json::encode(self);
        if let Ok(mut file) = File::create("citool.conf"){
            write!(file, "{}", buf.unwrap()).unwrap();
        }else{ println!("CiToolsSettings: не удаётся сохранить настройки"); }
        self
    }
}