pub mod rest {
    use std::sync::Mutex;

    use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Result};
    use serde::{Deserialize, Serialize};

    // Rest (https://learn.microsoft.com/en-us/azure/architecture/best-practices/api-design)
    #[derive(Debug, Default, Serialize, Deserialize, Clone)]
    pub struct Fruit {
        pub id: u32,
        pub name: String,
    }

    #[derive(Serialize)]
    pub struct FruitList {
        pub fruits: Mutex<Vec<Fruit>>,
    }

    // Rest - get resource
    #[get("/fruits/{id}")]
    async fn get_fruit(fruit_id: web::Path<u32>, fruit_list: web::Data<FruitList>) -> HttpResponse {
        let id = fruit_id.into_inner();
        let maybe_fruit = fruit_list
            .fruits
            .lock()
            .unwrap()
            .iter()
            .find(|&fruit| fruit.id == id)
            .map(|fruit| fruit.clone());
        maybe_fruit
            .map(|fruit| HttpResponse::Ok().json(fruit))
            .unwrap_or(HttpResponse::NotFound().finish())
    }

    // Rest - update resource
    #[put("/fruits/{id}")]
    async fn update_fruit(
        fruit: web::Json<Fruit>,
        fruit_list: web::Data<FruitList>,
    ) -> Result<impl Responder> {
        let mut fruits = fruit_list.fruits.lock().unwrap();
        let maybe_fruit = fruits.iter_mut().find(|frt| frt.id == fruit.id);
        match maybe_fruit {
            Some(found_fruit) => (*found_fruit).name = fruit.into_inner().name,
            None => fruits.push(fruit.into_inner()),
        };
        Ok("".to_string())
    }

    // Rest - delete resource
    #[delete("/fruits/{id}")]
    async fn delete_fruit(
        fruit_id: web::Path<u32>,
        fruit_list: web::Data<FruitList>,
    ) -> Result<impl Responder> {
        let id = fruit_id.into_inner();
        let mut fruits = fruit_list.fruits.lock().unwrap();
        if let Some(pos) = fruits.iter_mut().position(|fruit| fruit.id == id) {
            fruits.remove(pos);
        }
        Ok("".to_string())
    }

    // Rest - list resources
    #[get("/fruits")]
    async fn get_fruits(fruit_list: web::Data<FruitList>) -> HttpResponse {
        let fruits = fruit_list.fruits.lock().unwrap();
        HttpResponse::Ok().json(fruits.clone())
    }

    // Rest - list resources
    #[post("/fruits")]
    async fn create_fruit() -> Result<impl Responder> {
        Ok("")
    }
}
