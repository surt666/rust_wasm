use seed::{prelude::*, *};
use seed::fetch;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use futures::Future;

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
    DataFetched(fetch::ResponseDataResult<ResponseBody>)
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.counter += 1,
        Msg::Decrement => model.counter -= 1,
	Msg::ChangePermSet(perms) => {log!(format!("Permissions {}",perms))}
	Msg::DataFetched(Ok(response_data)) => {
            log!(format!("Response data: {:#?}", response_data));
            orders.skip();
        }
	Msg::DataFetched(Err(fail_reason)) => {
            error!(format!(
                "Fetch error - Sending message failed - {:#?}",
                fail_reason
            ));
            orders.skip();
        }
    }
}

fn head(model: &Model) -> Node<Msg> {
    let mut cols = Vec::<Node<Msg>>::new();
    for col in model.datasets.iter() {
	cols.push(th![col.name]);
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
	rows.push(tr![td![temp["user"]], cols.clone()]);
	cols.clear();
    }
    rows
}

fn view(model: &Model) -> Node<Msg> {
    div![attrs!{At::Class => "container"},
	 div![attrs!{At::Class => "row", At::Id => "header"},
	      h2!["Header"]
	 ],
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

async fn fetch_data() -> Result<Msg, Msg> {
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
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
	.after_mount(after_mount)
	.build_and_start();
}
