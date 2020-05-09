use seed::{prelude::*, *};
use seed::fetch;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use esdh_data_structs::*; //{Case, Inspect, ViewTypes};
use strum::{IntoEnumIterator}; 
//use strum_macros::{EnumIter};

#[derive(Clone)]
enum Permission {
    Read,
    Write,
    None
}

#[derive(Clone)]
enum Permissions {
    Source(Permission),
    Raw(Permission),
    Transient(Permission),
    Curated(Permission)
}

struct DatasetPermission {
    dataset: String,
    permissions: Vec<Permissions>
}


struct Relation {
    user: String,
    relations: Vec<DatasetPermission>
}

struct Dataset {
    name: String
}

struct Model {
    datasets: Vec<Dataset>,
    relations: Vec<Relation>,
    counter: i32
}

// Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {
            datasets: vec![Dataset{name: "test".into()}, Dataset{name: "c4c".into()}],
            relations: vec![Relation{user: "lars".into(),
				     relations: vec![DatasetPermission{dataset: "test".into(),
								       permissions: vec![Permissions::Source(Permission::Read),
											 Permissions::Raw(Permission::Write),
											 Permissions::Transient(Permission::None),
											 Permissions::Curated(Permission::Write)]},
						     DatasetPermission{dataset: "c4c".into(),
								       permissions: vec![Permissions::Source(Permission::Read),
											 Permissions::Raw(Permission::Write),
											 Permissions::Transient(Permission::Write),
											 Permissions::Curated(Permission::Write)]}]},
			    Relation{user: "hans".into(),
				     relations: vec![DatasetPermission{dataset: "test".into(),
								       permissions: vec![Permissions::Source(Permission::Read),
											 Permissions::Raw(Permission::Write),
											 Permissions::Transient(Permission::None),
											 Permissions::Curated(Permission::Write)]}]},
			    Relation{user: "steen".into(),
				     relations: vec![DatasetPermission{dataset: "test".into(),
								       permissions: vec![Permissions::Source(Permission::Read),
											 Permissions::Raw(Permission::Write),
											 Permissions::Transient(Permission::None),
											 Permissions::Curated(Permission::Write)]}]}],
	    counter: 0
        }
    }
}

#[derive(Clone)]
enum Msg {
    Increment,
    Decrement,
    ChangePermSet(String),
//    DataFetched(fetch::ResponseDataResult<ResponseBody>)
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.counter += 1,
        Msg::Decrement => model.counter -= 1,
	Msg::ChangePermSet(perms) => {log!(format!("Permissions {}",perms))}
//	Msg::DataFetched(Ok(response_data)) => {
//            log!(format!("Response data: {:#?}", response_data));
//            orders.skip();
//        }
//	Msg::DataFetched(Err(fail_reason)) => {
//            error!(format!(
//                "Fetch error - Sending message failed - {:#?}",
//                fail_reason
//            ));
//            orders.skip();
//        }
    }
}

fn head(model: &Model) -> Node<Msg> {
    let mut cols = Vec::<Node<Msg>>::new();
    for col in model.datasets.iter() {
	cols.push(th![&col.name]);
    }
    tr![
	th!["User"],
	cols
    ]
}

fn perm_to_string(p: &Permission) -> String {
    match p {
	Permission::Read => return "R".to_string(),
	Permission::Write => return "W".to_string(),
	Permission::None => return "N".to_string()
    }
}

fn rows(model: &Model) -> Vec<Node<Msg>> {
    let mut rows = Vec::<Node<Msg>>::new();
    let mut cols = Vec::<Node<Msg>>::new();
    let mut temp = HashMap::new();
    for user in model.relations.iter() {
	log!("test0");
	temp.insert("user", user.user.clone());
	for (i, _) in user.relations.iter().enumerate() {
	    log!("test1");
	    for j in 0..4 {
		log!("test2");
		match &user.relations[i].permissions[j] {
		    Permissions::Source(s) => temp.insert("source", perm_to_string(s)),
		    Permissions::Raw(r) => temp.insert("raw", perm_to_string(r)),
		    Permissions::Transient(t) => temp.insert("transient", perm_to_string(t)),
		    Permissions::Curated(c) => temp.insert("curated", perm_to_string(c))
		};
	    }
	    cols.push(td![format!("{},{},{},{}", temp["source"], temp["raw"], temp["transient"], temp["curated"]), input_ev(Ev::Input, Msg::ChangePermSet)]);	    
	}
	rows.push(tr![td![temp["user"].clone()], cols.clone()]);
	cols.clear();
    }
    rows
}

trait Layout {
    fn layout(&self) -> Node<Msg>;
}

impl Layout for Case {
    fn layout(&self) -> Node<Msg> {
	let jp: Vec<Node<Msg>> = self.jp.as_ref().unwrap_or(&vec![]).into_iter().map(|x| li![x.layout()]).collect();
	let sec_act: Vec<Node<Msg>> = self.secondary_actors.as_ref().unwrap_or(&vec![]).into_iter().map(|x| option![attrs!{At::Selected => true}, x]).collect();
	let categories: Vec<Node<Msg>> = Categories::iter().map(|x| option![attrs!{At::Selected => false}, x.as_ref().to_string()]).collect();
	let discard_codes: Vec<Node<Msg>> = DiscardCodes::iter().map(|x| option![attrs!{At::Selected => false}, x.as_ref().to_string()]).collect();
	div![
	    div![label!["Type"], label![&self.r#type.as_ref()]],
	    div![label!["Pk"], label![&self.pk]],
	    div![label!["Title"], input![attrs!{At::Value => &self.title}]],
	    div![label!["Owner"], input![attrs!{At::Value => &self.owner}]],
	    div![label!["Responsible"], input![attrs!{At::Value => &self.responsible}]],
	    div![label!["Primary actor"], input![attrs!{At::Value => &self.primary_actor}]],
	    div![label!["Secondary actors"], select![sec_act]],
	    div![label!["Borrower"], input![attrs!{At::Value => &self.borrower.as_ref().unwrap_or(&"".into())}]],
	    div![label!["Created"], label![&self.created]],
	    div![label!["Updated"], label![&self.updated]],
	    div![label!["Archive"], input![attrs!{At::Value => &self.archive.as_ref().unwrap_or(&"".into())}]],
	    div![label!["Category"], select![categories]],
	    div![label!["Description"], textarea![attrs!{At::Value => &self.description}]],
	    div![label!["Legal basis"], input![attrs!{At::Value => &self.legal_basis.as_ref().unwrap_or(&"".into())}]],
	    div![label!["Publicly excepted"], input![attrs!{At::Type => "checkbox", At::Checked => &self.publicly_excepted}]],
	    div![label!["Principled"], input![attrs!{At::Type => "checkbox", At::Checked => &self.principled}]],
	    div![label!["Category"], select![discard_codes]],
	    div![label!["Delivered to archive"], input![attrs!{At::Type => "checkbox", At::Checked => &self.delivered_to_archive}]],
	    div![label!["Status"], label![&self.status.as_ref()]],
	    div![label!["Entity"], input![attrs!{At::Value => &self.entity.as_ref().unwrap_or(&"".into())}]],	    
	    ol![jp]
	]
    }
}

impl Layout for Jn {
    fn layout(&self) -> Node<Msg> {
	div![
	    div![label!["Type"], label![&self.r#type.as_ref()]],
	    div![label!["Pk"], label![&self.pk]],
	    div![label!["Title"], input![attrs!{At::Value => &self.title}]],
	    div![label!["Responsible"], input![attrs!{At::Value => &self.responsible}]],
	    div![label!["Primary actor"], input![attrs!{At::Value => &self.primary_actor}]],
	    div![label!["Created"], label![&self.created]],
	    div![label!["Updated"], label![&self.updated]],
	    div![label!["Text"], textarea![attrs!{At::Value => &self.text}]],
	]
    }
}

impl Layout for Doc {
    fn layout(&self) -> Node<Msg> {
	div![
	    div![label!["Type"], label![&self.r#type.as_ref()]],
	    div![label!["Pk"], label![&self.pk]],
	    div![label!["Title"], input![attrs!{At::Value => &self.title}]],
	    div![label!["Created"], label![&self.created]],
	    div![label!["Updated"], label![&self.updated]],
	    div![label!["Synopsis"], textarea![attrs!{At::Value => &self.synopsis}]],
	    div![label!["Letter date"], input![attrs!{At::Value => &self.letter_date}]],
	    div![label!["Link"], input![attrs!{At::Value => &self.link}]],
	]
    }
}

impl Layout for Jp {
    fn layout(&self) -> Node<Msg> {
	let jn: Vec<Node<Msg>> = self.jn.as_ref().unwrap_or(&vec![]).into_iter().map(|x| li![x.layout()]).collect();
	let docs: Vec<Node<Msg>> = self.docs.as_ref().unwrap_or(&vec![]).into_iter().map(|x| li![x.layout()]).collect();
	div![
	    div![label!["Type"], label![&self.r#type.as_ref()]],
	    div![label!["Pk"], label![&self.pk]],
	    div![label!["Title"], input![attrs!{At::Value => &self.title}]],
	    div![label!["Responsible"], input![attrs!{At::Value => &self.responsible}]],
	    div![label!["Primary actor"], input![attrs!{At::Value => &self.primary_actor}]],
	    div![label!["Created"], label![&self.created]],
	    div![label!["Updated"], label![&self.updated]],	    
	    ol![jn],
	    ol![docs]
	]
    }
}

fn get_layout<T: Inspect + Layout>(elem: T) -> Node<Msg> {
    elem.layout()
}

fn view(model: &Model) -> Node<Msg> {
    div![attrs!{At::Class => "container"},
	 div![attrs!{At::Class => "row", At::Id => "header"},
	      h2!["Header"]
	 ],
	 div![attrs!{At::Class => "row", At::Id => "content"},
	      get_layout(Case{
		  r#type: CaseElements::Case,
		  title: "ny title".to_string(),
		  jp: Some(vec![
		      Jp {
			  r#type: CaseElements::JP,
			  title: "jp title1".to_string(),
			  jn: Some(vec![
			      Jn {
				  r#type: CaseElements::JN,
				  title: "jn title1".to_string(),
				  ..Default::default()
			      }
			  ]),
			  docs: Some(vec![
			      Doc {
				  r#type: CaseElements::Doc,
				  title: "doc title1".to_string(),
				  ..Default::default()
			      }
			  ]), 
			  ..Default::default()
		      },
		      Jp {
			  r#type: CaseElements::JP,
			  title: "jp title2".to_string(),
			  ..Default::default()
		      }]),
		  ..Default::default()})],
	 div![attrs!{At::Class => "row", At::Id => "main"},
	     table![attrs!{"border" => "1"},
		 head(model),
		 rows(model)
	     ]
	 ],
	 div![attrs!{At::Class => "row", At::Id => "footer"},
	      h2!["Footer"],
              button![ ev(Ev::Click, |_| Msg::Decrement), "-" ],
	      div![model.counter.to_string()],
              button![ ev(Ev::Click, |_| Msg::Increment), "+" ],
	 ]
    ]
}

#[derive(Serialize)]
struct RequestBody {
    pub action: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ResponseBody {
    pub success: bool,
}

/*async fn fetch_data() -> Result<Msg, Msg> {
    let message = RequestBody {
        action: "GetDatasets".into()
    };
    let url = "https://spcoseon48.execute-api.eu-west-1.amazonaws.com/dev/hello";
    Request::new(url.to_string())
        .method(Method::Post)
        .send_json(&message)
        .fetch_json_data(Msg::DataFetched).await
}


fn after_mount(_: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    orders.perform_cmd(fetch_data());
    AfterMount::default()
}*/

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
//	.after_mount(after_mount)
	.build_and_start();
}
