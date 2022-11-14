use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{param::Query, payload::PlainText, OpenApi, OpenApiService};

use crate::ipxe::ipxe_script;

pub struct Api {
    pub config: String,
}

#[OpenApi]
impl Api {
    #[oai(path = "/ipxe/script", method = "get")]
    async fn ipxe(
        &self,
        uuid: Query<Option<String>>,
        ip: Query<Option<String>>,
        mac: Query<Option<String>>,
        serial: Query<Option<String>>,
        platform: Query<Option<String>>,
    ) -> PlainText<String> {
        let r = ipxe_script(&self.config, &uuid, &ip, &mac, &serial, &platform).await;

        PlainText(r)
    }
}

pub async fn start(enable_ui: bool, api: Api) -> Result<(), std::io::Error> {
    let api_service =
        OpenApiService::new(api, "Hello World", "1.0").server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();

    let server = Server::new(TcpListener::bind("127.0.0.1:3000"));

    let route = if enable_ui {
        Route::new().nest("/", ui)
    } else {
        Route::new()
    };

    let route = route.nest("/api", api_service);

    server.run(route).await
}
