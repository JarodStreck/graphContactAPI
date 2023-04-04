use actix_web::{get, web, App, HttpServer, Responder};
use neo4rs::*;
use uuid::Uuid;

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/user/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    let id = Uuid::new_v4().to_string();
    let config = config()
       .uri("127.0.0.1:7687")
       .user("neo4j")
       .password("neo4jpassword")
       .db("graphcontact").build().unwrap();
    let graph = Graph::connect(config).await.unwrap();
    let _result = graph.execute(
        query("CREATE (p:Person {id: $id,name:$name})").param("id", id.clone()).param("name", name.clone())
    ).await.unwrap();
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(index)
        .service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}