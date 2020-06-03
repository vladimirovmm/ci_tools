#[derive(Debug, Clone)]
pub struct CiToolsCommand{
    pub name:String,
    pub value:Option<String>
}
impl CiToolsCommand{
    pub fn new_by_argv(comand: String)->Option<CiToolsCommand>{
        let comand:Vec<&str> = comand.split("=").map(|x| x.trim() ).collect();
        if comand.first().unwrap().clone().len() == 0 { return None; }

        if comand.len() > 1 {
            Some(CiToolsCommand{
                name: comand[0].to_string(),
                value: Some(comand[1].to_string())
            })
        }else{
            Some(CiToolsCommand{
                name: comand[0].to_string(),
                value: None
            })
        }
    }
}