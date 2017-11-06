
use rocket_contrib::{Json, Value};
use rocket::State;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ServerCondition {
    Running,
    Starting,
    Error,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerStatus {
    status: ServerCondition,
    version: String,
}

impl ServerStatus {
    pub fn new(v: String) -> Self {
        ServerStatus {
            status: ServerCondition::Starting,
            version: v,
        }
    }

    pub fn set_status(&mut self, s: ServerCondition) {
        self.status = s;
    }
}

#[get("/", format = "application/json")]
pub fn get(v: State<ServerStatus>) -> Json<ServerStatus> {
    Json { 0: (*v).clone() }
}
