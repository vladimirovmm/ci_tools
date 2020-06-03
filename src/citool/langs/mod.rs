use crate::citool::CiTools;
use crate::citool::langs::file::CILangsToolsFile;
use crate::citool::langs::row::CILangsToolsRow;
use std::collections::HashMap;
use std::path::Path;
use regex::Regex;

pub mod row;
pub mod file;
pub mod yandex;

///
/// Для работы с языками в CI
///
#[derive(Clone,Debug)]
pub struct CILangsTools{
    pub ci_path:String
}
impl CILangsTools{
    pub fn new(ci_path:String) ->CILangsTools{
        CILangsTools{ ci_path: ci_path }
    }
    ///
    /// Регистрация команд для ARGV
    ///
    pub fn inic_exec_citools(tools:&mut CiTools){
        tools
            // Help
            .register_exec_function("help".to_string(), |_,_|->Result<String,String>{
                Ok(
                    "langs-show - Список языковых из настроек\n\
                    langs-foledrs - Список языков по папокам\n\
                    langs-foledrs-to-config - Определить языки по папкам и сохранить их в конфиги\n\
                    langs-set=[lang,lang,lang] - Установить языки\n\
                    langs-show-can-translate-files - вывести не переведёные файлы\n\
                    langs-show-can-translate-word - вывести не переведёные слова\n\
                    langs-yandex-translate-key-show - вывести ключ от yandex-translate\n\
                    langs-yandex-translate-key-set - установить ключ от yandex-translate\n\
                    langs-yandex-translate - перевести не переведённые фразы через yandex-translate\n\
                    \n- - -\n".to_string())
            })
            // Список языков из настроек
            .register_exec_function("langs-show".to_string(), |_, tools|->Result<String,String>{
                let text = match tools.config.langs.clone(){
                    Some(list)=>{ list.join(",") },
                    None=>{ " Нет данных".to_string() }
                };
                Ok("Список языков из настроек = ".to_string()+text.as_str()  )
            })
            // Отобразить список языков по папкам
            .register_exec_function("langs-foledrs".to_string(), |_, tools|->Result<String,String>{
                if tools.config.ci_path.is_none() {
                    return Err("ci_path не установлен".to_string());
                }
                let lang = CILangsTools::new(tools.config.ci_path.clone().unwrap());

                match lang.get_langs_by_path() {
                    Ok(ans) => {
                        Ok( "Отобразить список языков по папкам = ".to_string() + ans.join(",").as_str() )
                    },
                    Err( ans ) => {
                        Err( "Отобразить список языков по папкам = ".to_string() + ans.as_str() )
                    }
                }
            })
            // Определить языки по папкам и сохранить их в конфиги
            .register_exec_function("langs-foledrs-to-config".to_string(), |_, tools|->Result<String,String>{
                if tools.config.ci_path.is_none() {
                    return Err("ci_path не установлен".to_string());
                }
                let lang = CILangsTools::new(tools.config.ci_path.clone().unwrap());

                match lang.get_langs_by_path() {
                    Ok(ans) => {
                        tools.config.langs = Some(ans.clone());
                        tools.config.save();
                        Ok( "Определить языки по папкам и сохранить их в конфиги = ".to_string() + ans.join(",").as_str() )
                    },
                    Err( ans ) => {
                        Err( "Определить языки по папкам и сохранить их в конфиги = ".to_string() + ans.as_str() )
                    }
                }
            })
            // Установить языки
            .register_exec_function("langs-set".to_string(), |command, tools|->Result<String,String>{
                if command.value.is_none() { return Err("Установить языки = параметры не переданны".to_string()); }
                let value = command.value.unwrap();
                let list:Vec<String> = value.split(",")
                    .map(|x|x.trim().to_string())
                    .filter(|x|x.len()>0)
                    .collect();
                if list.len() == 0 { return Err("Установить языки = Нет параметров".to_string()); }
                tools.config.langs = Some(list.clone());
                tools.config.save();

                Ok("Установить языки = ".to_string()+ list.join(",").as_str()  )
            })
            // Поиск не переведёных файлов
            .register_exec_function("langs-show-can-translate-files".to_string(), |_, tools|->Result<String,String>{
                if tools.config.ci_path.is_none() { return Err("Поиск не переведёных = Каталог проект не установлен".to_string()); }
                if tools.config.langs.is_none() { return Err("Поиск не переведёных = Языки не установлены".to_string()); }

                let text = CILangsTools::get_print_wasnt_translate_files(tools.config.ci_path.clone().unwrap(), tools.config.langs.clone().unwrap() );

                Ok("Поиск не переведёных файлов: \n".to_string()+text.as_str() )
            })
            // Поиск не переведёных слов
            .register_exec_function("langs-show-can-translate-word".to_string(), |_, tools|->Result<String,String>{
                if tools.config.ci_path.is_none() { return Err("Поиск не переведёных = Каталог проект не установлен".to_string()); }
                if tools.config.langs.is_none() { return Err("Поиск не переведёных = Языки не установлены".to_string()); }

                Ok("Поиск не переведёных слов: \n".to_string()
                    +CILangsTools::get_print_wasnt_translate_words(tools.config.ci_path.clone().unwrap(), tools.config.langs.clone().unwrap()).as_str() )
            })
            // вывести ключ от yandex-translate
            .register_exec_function("langs-yandex-translate-key-show".to_string(), |_, tools|->Result<String,String>{
                Ok("Ключ от yandex-translate: ".to_string() +
                    match tools.config.yandexapikey.clone() {
                            Some(key)=> { key.clone() },
                            None => { " Не установлен".to_string() }
                        }.as_str()
                )
            })
            // установить ключ от yandex-translate
            .register_exec_function("langs-yandex-translate-key-set".to_string(), |command, tools|->Result<String,String>{
                if command.value.is_none(){ return Err("установить ключ от yandex-translate: Значение не переданно".to_string()); }
                tools.config.yandexapikey=command.value.clone();
                tools.config.save();
                Ok("Установлен ключ от yandex-translate: ".to_string()+tools.config.clone().yandexapikey.unwrap().as_str()+"\n" )
            })
            // перевести не переведённые фразы через yandex-translate
            .register_exec_function("langs-yandex-translate".to_string(), |_, tools|->Result<String,String>{
                if tools.config.ci_path.is_none() { return Err("Перевод через yandex-translate = Каталог проект не установлен".to_string()); }
                if tools.config.langs.is_none() { return Err("Перевод через yandex-translate = Языки не установлены".to_string()); }
                if tools.config.yandexapikey.is_none() { return Err("Перевод фраз через yandex-translate = yandex-key не установлен".to_string()); }

                let ans = CILangsTools::yandex_translate_files(tools.config.ci_path.clone().unwrap(), tools.config.langs.clone().unwrap(), tools.config.yandexapikey.clone().unwrap());

                Ok("Перевод через yandex-translate: \n".to_string()+ {
                        match ans { Ok(text) => text, Err(text) => text }
                    }.as_str())
            })
        ;
    }
    ///
    /// Получить языковые префикси для CI
    ///
    pub fn get_langs_by_path(&self)->Result<Vec<String>, String>{
        let path = self.ci_path.clone()+"/application/language/";
        let in_dir_paths = std::fs::read_dir(path.as_str() );
        if in_dir_paths.is_err() { return Err("Не корректный путь. Папки не существует".to_string()); }
        let mut paths:Vec<String> = Vec::new();
        for p in in_dir_paths.unwrap(){
            let path = p.unwrap();
            let path_string = path.path().display().to_string();
            if path.metadata().unwrap().is_dir(){
                paths.push(path_string.split("/").last().unwrap().to_string());
            }
        }
        Ok(paths)
    }
    // =============================================================================================
    // Не переведённые фразы
    // =============================================================================================
    pub fn yandex_translate_files(ci_path:String, langs:Vec<String>, key:String) ->Result<String, String>{
        let mut ans = String::new();
        let mut wastrans = false;
        'for_file:for (file_name, file, rows) in CILangsTools::get_all_translates(ci_path.clone(), langs.clone()) {
            // =====================================================================================
            // Собираем все слова по файлу которые нужно перевести
            // =====================================================================================
            let mut rows = rows.clone();

            'for_lang_from:for lang_from in langs.iter() {
                'for_lang_to:for lang_to in langs.iter().filter(|lang|{ *lang != lang_from }){
                    let for_trans:Vec<CILangsToolsRow> = rows.iter().filter_map(|(_, row)|{
                        if row.translate.get(lang_from.clone().as_str()).is_some() && row.translate.get(lang_to.clone().as_str()).is_none() {
                            Some(row.clone())
                        } else {
                            None
                        }
                    }).collect();
                    if for_trans.len() == 0 { continue; }
                    let lang_to_lang = lang_from.clone()+"-"+lang_to.as_str();

                    for chuncs_rows in for_trans.chunks(20){
                        let list:Vec<&str> = chuncs_rows.iter().map(|row|{ row.translate.get(lang_from.as_str()).unwrap().as_str() }).collect();
                        let yandex_ans = yandex::yandext_translate(list.clone(),lang_to_lang.as_str(), key.as_str());
                        if yandex_ans.is_none() { continue; }
                        let yandex_ans = yandex_ans.unwrap();

                        for (k, v) in chuncs_rows.iter().enumerate(){
                            let row = rows.get_mut( v.index.as_str() ).unwrap();
                            row.translate.insert(lang_to.clone(),yandex_ans[k].clone());
                        }
                        wastrans = true;
                        // break;
                    }
                    file.save_key_value(lang_to.clone(), rows.clone()).unwrap();
                    ans+=format!("[{}] {} \n", lang_to.clone(), file_name.clone()).as_str();

                    // break 'for_file;
                }
            }
        }
        if wastrans {
            Ok(ans)
        }else{
            Ok("Всё переведенно".to_string())
        }
    }

    ///
    /// Получить не переведёные слова
    ///
    pub fn get_wasnt_translate_words(ci_path:String, langs:Vec<String>)->HashMap<String,(String, CILangsToolsFile, HashMap<String,CILangsToolsRow>)>{
        CILangsTools::get_all_translates(ci_path.clone(), langs.clone()).iter()
            .filter_map(|(file_name, file, rows)|{
                    let not_translated_rows:HashMap<String,CILangsToolsRow> = rows.iter().filter_map(|(index, row)|{
                        let mut h = true;
                        for lang in langs.iter() {
                            let t = row.translate.get(lang.as_str());
                            if t.is_none() || t.unwrap().as_str().trim().len() == 0{
                                h = false;
                                break;
                            }
                        }
                        if !h { Some( (index.clone(), row.clone()) ) }
                        else{ None }
                    }).collect();

                    if not_translated_rows.len() > 0 { Some( (file_name.clone(), ( file_name.clone(), file.clone(), not_translated_rows.clone())) ) }
                    else{ None }
                })
            .collect()
    }
    pub fn get_print_wasnt_translate_words(ci_path:String, langs:Vec<String>)->String{
        let list = CILangsTools::get_wasnt_translate_words(ci_path.clone(), langs.clone());

        if list.len() == 0 { return "Всё переведенно".to_string(); }
        let mut ans = String::new();

        let len_index = 30;
        let len_word = 15;
        for (file_name, (_,_,rows)) in list {
            ans = ans +"Файл - "+file_name.as_str()+" :\n";
            ans += "=======================================================================================================\n";
            ans += format!("{1:>0$} |", len_index+5 , "INDEX" ).as_str();
            for lang in langs.clone() { ans += format!("{1:^0$} |", len_word+5, lang.to_uppercase() ).as_str(); }
            ans +="\n";
            ans += "=======================================================================================================\n";
            for (index,row) in rows {
                ans += format!("{1:>0$} |", len_index+5 , CILangsTools::strip_string_maxlen(&index, len_index) ).as_str();

                for lang in langs.iter(){
                    let text = match row.translate.get(lang) {
                        Some(text) => text.clone(),
                        None=>"null".to_string()
                    };
                    ans += format!(" {1:<0$}|", len_word+5 , CILangsTools::strip_string_maxlen(&text, len_word) ).as_str();
                }

                ans += "\n";
            }
            ans += "=======================================================================================================\n\n";
        }
        ans
    }
    fn strip_string_maxlen(text:&String, len:usize)->String{
        let count = text.chars().count();
        if count > len { {text.chars().take(len).clone().collect::<String>()+".."}.clone() }
        else{ text.clone() }
    }

    fn get_all_translates(ci_path:String, langs:Vec<String>)->Vec<(String,CILangsToolsFile,HashMap<String,CILangsToolsRow>)>{
        let mut ans = Vec::new();
        for (_, file) in CILangsTools::get_all_langs_files(ci_path.clone(), langs.clone()) {
            let result = file.get_key_value();
            ans.push( (file.index.clone(), file.clone(), result ) )
        };
        ans
    }
    // =============================================================================================
    //  Не переведённые файлы/не созданные файлы
    // =============================================================================================
    ///
    /// Получить не переведённые файлы
    ///
    pub fn get_wasnt_translate_files(ci_path:String, langs:Vec<String>)->HashMap<String, CILangsToolsFile>{
        let list_files:HashMap<String,CILangsToolsFile> = CILangsTools::get_all_langs_files(ci_path.clone(), langs.clone()).iter().filter_map(|(index, file)|{
            if langs.iter().filter(|lang|{ file.files.get(*lang ).is_none() }).count() > 0 { Some( (index.clone(), file.clone()) ) }
            else{ None }
        }).collect();
        list_files
    }
    ///
    /// Вывод визуально для консоли список не переведёных файлов(отсутсвующие)
    ///
    pub fn get_print_wasnt_translate_files(ci_path:String, langs:Vec<String>)->String{
        let list = CILangsTools::get_wasnt_translate_files(ci_path.clone(), langs.clone() );

        let mut ans = "=============================================\n".to_string();
        ans += format!("{:>40}", "INDEX" ).as_str();
        for lang in langs.clone() { ans += format!("{:^10}", lang.to_uppercase() ).as_str(); }
        ans +="\n";

        let mut keys = list.clone().keys().cloned().collect::<Vec<String>>();
        keys.sort();
        for index in keys {
            let file = list.get(index.as_str()).unwrap();
            let len = 30;
            let count = index.chars().count();
            let text:String = if count > len { "...".to_string()+index.chars().skip(count-len).clone().collect::<String>().as_str() }
            else{ index.clone() };
            ans += format!("{:>40}", text).as_str();
            for lang in langs.clone() {
                let char = if file.files.get(lang.as_str()).is_some(){ "+" }else{ "-"};
                ans += format!("{:^10}", char ).as_str();
            }
            ans += "\n";
        }
        ans+="=============================================\n";
        ans
    }
    // =============================================================================================
    // Список файлов перевода
    // =============================================================================================
    ///
    /// Получить список файлов первода CI
    ///
    pub fn get_all_langs_files(ci_path:String, langs:Vec<String>) ->HashMap<String, CILangsToolsFile>{
        let root_path_lang = ci_path +"/application/language/";

        let mut result:HashMap<String,CILangsToolsFile> = HashMap::new();
        for lang in langs {
            let list_files = CILangsTools::get_list_langs_files(root_path_lang.clone()+lang.as_str());
            CILangsTools::conver_list_files_to_citoolsfiles(&root_path_lang, &lang, &list_files, &mut result);
        }
        result
    }
    fn conver_list_files_to_citoolsfiles(root_path:&String, lang:&String, files_list:&Vec<String>, result:&mut HashMap<String,CILangsToolsFile> ){
        let r = root_path.to_string()+lang;
        let count_skip_chars = Path::new(r.as_str()).canonicalize().unwrap().to_str().unwrap().to_string().len();

        for file in files_list {
            let file = file.as_str();
            let index = &file[count_skip_chars..];

            match result.get_mut(index) {
                Some(item) => {
                    item.files.insert(lang.clone(), file.to_string());
                },
                None => {
                    let mut h = HashMap::new();
                    h.insert(lang.clone(), file.to_string());
                    let r = CILangsToolsFile{
                        root_path:root_path.clone(),
                        index:index.to_string(),
                        files:h.clone()
                    };
                    result.insert(index.to_string(), r);
                }
            };
        }
    }
    fn get_list_langs_files(path:String)->Vec<String>{
        let mut list:Vec<String> = Vec::new();
        let re = Regex::new(r"_lang\.php$").unwrap();
        for path in std::fs::read_dir(path).unwrap(){
            let path = path.unwrap();
            let path_string = path.path().display().to_string();
            if path.metadata().unwrap().is_dir(){
                list.extend(CILangsTools::get_list_langs_files(path_string));
            }else if path.metadata().unwrap().is_file(){
                if re.is_match(path_string.as_str()) {
                    list.push(path_string);
                }
            }
        }
        return list;
    }
}