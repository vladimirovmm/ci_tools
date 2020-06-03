use std::collections::HashMap;

pub mod langs;
pub mod ci_commands;
pub mod ci_tools_settings;

use ci_commands::CiToolsCommand;
use crate::citool::ci_tools_settings::CiToolsSettings;
use crate::citool::langs::CILangsTools;

///
/// Структура для работы с командами.
/// Через неё происходит обработка аргументов отправленых
/// и регистрация возможных запускаемых команд
///
#[derive(Clone)]
pub struct CiTools{
    // Набор зарегестрированых команд
    pub events:Option<HashMap<String,Vec<fn(CiToolsCommand,&mut CiTools)->Result<String,String>>>>,
    // Список команд для запуска
    pub run_events: Option<Vec<CiToolsCommand>>,
    // Настроки
    pub config:CiToolsSettings,
}
impl CiTools{
    ///
    /// Создать объект структуры управления
    ///
    pub fn new(system_args:Vec<String>)->CiTools{
        let mut n = CiTools{
            events:None,                                                                     // Набор зарегестрированых команд
            run_events:None,                                                                      // Список команд для запуска
            config:ci_tools_settings::CiToolsSettings::new(),                                       // Настройки для интсрумента
        };
        n.add_default_exec_function()                                                               // Команды по умолчанию
            .add_system_args(system_args);                                                          // Инициализация команд из командной строки
        CILangsTools::inic_exec_citools( &mut n);
        n
    }
    // =============================================================================================
    // ARGS - COMMAND. Команды от клиента
    // =============================================================================================
    pub fn add_system_args(&mut self, system_args:Vec<String>) ->&mut CiTools{
        for x in system_args.iter().skip(1) { self.add_command(x.clone()); }
        self
    }
    pub fn add_command(&mut self, comand: String) ->&mut CiTools{
        if let Some(x)=CiToolsCommand::new_by_argv(comand){
            if self.run_events.is_none() { self.run_events = Some(Vec::new()); }
            let t = self.run_events.as_mut().unwrap();
            t.push(x);
        }
        self
    }
    // =============================================================================================
    // exec - function. Обработчики команд
    // =============================================================================================
    pub fn add_default_exec_function(&mut self)->&mut CiTools{
        self
            // тестовая функция
            .register_exec_function("ping".to_string(), |_,_|->Result<String,String>{ Ok("Ping: Pong".to_string()) })
            // Вывод доступных команд
            .register_exec_function("help".to_string(), |_,_|->Result<String,String>{
                Ok(
                    "\n- - -\n\
                    Основные команды:\n\
                    help - Вывести справку\n\
                    ci_path=[путь до корня CI проекта] - установить директорию проекта\
                    \n- - -\n".to_string())
            })
            // Установить CI PATH
            .register_exec_function("ci_path".to_string(), |command, tools|->Result<String,String>{
                tools.config.ci_path = command.value.clone();
                tools.config.save();
                Ok("CI_PATH set = ".to_string() )
            })
    }
    pub fn register_exec_function(&mut self, name:String, f:fn(CiToolsCommand, &mut CiTools)->Result<String,String>)->&mut CiTools{
        if self.events.is_none() { self.events = Some(HashMap::new()); }
        let exec = self.events.as_mut().unwrap();

        if exec.contains_key(name.as_str()) {
            let list = exec.get_mut(name.as_str()).unwrap();
            list.push(f);
        }else{
            exec.insert(name, vec![f]);
        }
        self
    }
    // =============================================================================================
    //  RUN
    // =============================================================================================
    ///
    /// Запустить исполнение команд
    ///
    pub fn run(&mut self)->&mut CiTools{
        if self.run_events.is_some() && self.events.is_some() {
            let exec = self.events.clone().unwrap();
            for command in self.run_events.clone().unwrap() {
                if let Some(f_list) = exec.get(command.name.as_str()) {
                    for f in f_list {
                        let result:Result<String,String> = f(command.clone(), self);
                        match result {
                            Ok(text) => { println!("{}", text); },
                            Err(error) =>  { println!("error {}", error); },
                        }
                    }
                }
            }
        }
        self
    }
    // =============================================================================================
    //  Debug
    // =============================================================================================
    fn debug_events(&self)->String{
        match &self.events {
            Some(list) =>
                "[".to_string()
                    + list.iter()
                    .map(|(x,_)| x.clone())
                    .collect::<Vec<String>>()
                    .join(",").as_str()
                    + "]",
            None => "None".to_string()
        }
    }
    fn debug_commands(&self)->String{
        match &self.run_events {
            Some(list) =>
                "[".to_string()
                    + list.iter()
                    .map(|x| {
                        if x.value.is_some() { x.name.clone()+"="+x.value.clone().unwrap().as_str() }
                        else{ x.name.clone() }
                    })
                    .collect::<Vec<String>>()
                    .join(",").as_str()
                    + "]",
            None => "None".to_string()
        }
    }
}

impl std::fmt::Debug for CiTools {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"\
                events: {}\n\
                runs: {}\
            ",
                    self.debug_events(),
                    self.debug_commands(),
        )
    }

}