use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct YandeAnswer{
    code:i32,
    lang:String,
    text:Vec<String>
}

pub fn yandext_translate(word:Vec<&str>, lang_to_lang:&str, key:&str)->Option<Vec<String>>{
    use isahc::prelude::*;
    use serde::{Serialize, Deserialize};

    let mut data = url::form_urlencoded::Serializer::new(String::new());
    for x in word { data.append_pair("text", x); }
    let data = data.finish();

    let mut response =
        isahc::prelude::Request::post("https://translate.yandex.net/api/v1.5/tr.json/translate?key=".to_string()+key+"&lang="+lang_to_lang)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(data).unwrap()
            .send().unwrap();
    let text = response.text().unwrap();
    let ans =
        serde_json::from_str::<YandeAnswer>(text.clone().as_str()).unwrap();

    if ans.code == 200 {
        Some(ans.text)
    }else{
        None
    }
}