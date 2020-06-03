use std::collections::HashMap;

#[derive(Clone,Debug)]
pub struct CILangsToolsRow{
    pub file:String,                                                                                // имя файла относительное без языкового префикса
    pub index:String,                                                                               // Индекс перевода
    pub translate:HashMap<String,String>                                                            // Фразы на разных языках
}
impl CILangsToolsRow{
    pub fn new(file:String, index:String, translate:HashMap<String,String>)->CILangsToolsRow{
        CILangsToolsRow{
            file:file,
            index:index,
            translate:translate
        }
    }
}