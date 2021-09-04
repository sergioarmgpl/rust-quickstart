#[macro_use] extern crate rocket;
use sqlx::mysql::MySqlPoolOptions;

async fn insert_data(id: u8, nombre: &str) -> Result<(), sqlx::Error> {
    println!("entre");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:PASSWORD@localhost:3306/multiverso").await?;

    sqlx::query("INSERT INTO multiverso.persona (id,nombre) VALUES (33,\"juanxyz\");").execute(&pool).await?;

    Ok(())
}


#[get("/")]
fn index() -> &'static str {
   "OK"
}

#[get("/_health")]
fn health() -> &'static str {
   "OK"
}

#[get("/data/id/<id>/nombre/<nombre>")]
fn insert(id: u8, nombre: &str) -> String {
//   insert_data(id,nombre);
   format!("id: {0} nombre: {1}", id, nombre)
//   "OK"
}

#[get("/data")]
fn get() -> &'static str {
   "OK"
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, health, insert])
}
