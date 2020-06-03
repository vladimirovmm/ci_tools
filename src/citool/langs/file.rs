use std::collections::HashMap;
use crate::citool::langs::row::CILangsToolsRow;
use std::process::Command;
use std::path::Path;
use std::io::Write;

#[derive(Clone,Debug)]
pub struct CILangsToolsFile {
    pub root_path:String,                                                                               // Файл с относительным путя без языкового префикса
    pub index:String,                                                                               // Файл с относительным путя без языкового префикса
    pub files:HashMap<String,String>                                                                // Файл ru|uz|en
}
impl CILangsToolsFile {
    ///
    /// Получить значения файла для перевода
    ///
    pub fn get_key_value(&self)->HashMap<String, CILangsToolsRow>{
        let mut loaded :HashMap<String, HashMap<String,String>> = HashMap::new();
        for (lang, _) in self.files.iter() {
            let r = self.read_file(lang);
            if r.is_none() { continue }
            for (index, value) in r.clone().unwrap() {
                let row = loaded.get_mut(index.as_str());
                if row.is_none() {
                    let mut row = HashMap::new();
                    row.insert(lang.clone(), value.clone());
                    loaded.insert(index.clone(), row.clone() );
                }else{
                    row.unwrap().insert(lang.clone(),value.clone());
                }
            }
        }
        return loaded.iter().map(|(index, value)|{
                (index.clone(), CILangsToolsRow::new(self.index.clone(),index.clone(), value.clone()))
            }).collect();
    }
    pub fn save_key_value(&self, lang:String, list:HashMap<String,CILangsToolsRow>)->Result<(),()>{
        let file_path = match self.files.get(lang.as_str()){
            Some(path)=>Some(path.clone()),
            None=>{
                let path = self.root_path.clone()+"/"+lang.as_str()+"/"+self.index.as_str();
                let path = Path::new(path.as_str() );
                let dir = path.parent().unwrap();
                if !dir.is_dir() && std::fs::create_dir_all(dir).is_err(){
                    None
                }else{
                    Some(path.display().to_string())
                }
            }
        };
        if file_path.is_none() { return Err(()); }


        let mut cont = "<?php defined('BASEPATH') OR exit('No direct script access allowed');/* autogenerate */\n".to_string();
        cont+=list.iter().filter_map(|(_,row)|{
                let text = row.translate.get(lang.as_str());
                if text.is_none() { return None; }
                let text = text.unwrap().replace("'", "&#39;");
                Some(format!("$lang['{index}'] = \'{text}\';", index=row.index.clone(), text=text ).to_string())
            }).collect::<Vec<String>>().join("\r\n").as_str();

        if std::fs::File::create(file_path.unwrap()).unwrap().write_all(cont.as_bytes()).is_ok(){
            Ok(())
        }else{
            Err(())
        }

    }
    ///
    /// Получчить данные файла
    ///
    fn read_file(&self,lang:&String)->Option<HashMap<String,String>>{
        let file_path = self.files.get(lang.as_str());
        if file_path.is_none() { return None; }
        let file_path = file_path.unwrap().clone();

        let output = Command::new("php")
            .arg("-r")
            .arg( "define('BASEPATH',true); \
                    include '".to_string()+file_path.as_str()+"'; \
                    foreach($lang as $k=>$v){ \
                        echo $k.'<=====>'.$v.'###----###'.PHP_EOL; \
                    } ")
            .output()
            .expect("failed to execute process");
        let out = String::from_utf8(output.stdout);
        if out.is_err() { return None; }
        let out = out.unwrap();
        let out:HashMap<String,String> = out.split("###----###")
            .map(|x|x.trim())
            .filter(|x|x.len()>0)
            .filter_map(|x:&str|{
                let t:Vec<&str> = x.split("<=====>").collect();
                if t.len() < 2 { return None; }
                Some( (t[0].to_string(),t[1].to_string()) )
            })
            .collect();
        return
            if out.len() == 0 { None }
            else { Some(out.clone()) };
    }
}