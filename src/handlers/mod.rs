use std::marker::PhantomData;

use futures::Future;
use sea_orm::{DatabaseConnection, EntityTrait};
use serde_json::json;

pub trait Handler {
    fn handle(
        &self,
        http_method: &str,
        uri: &str,
    ) -> impl Future<Output = Option<Result<String, HandlerError>>>;

    fn get_all(&self) -> impl Future<Output = Result<String, HandlerError>>;
    fn get(&self, id: &str) -> impl Future<Output = Result<String, HandlerError>>;
    fn create(&self) -> impl Future<Output = Result<String, HandlerError>>;
    fn delete(&self, id: &str) -> impl Future<Output = Result<String, HandlerError>>;
    fn update(&self, id: &str) -> impl Future<Output = Result<String, HandlerError>>;
}

pub struct SimpleHandler<'a, T: EntityTrait> {
    db: &'a DatabaseConnection,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: EntityTrait> SimpleHandler<'a, T> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        SimpleHandler::<T> {
            db,
            phantom: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct HandlerError;

impl<T: EntityTrait> Handler for SimpleHandler<'_, T> {
    // Restful paths are like these:
    // let's imagine we have authors, and we have books written by them
    //
    // GET /authors
    // GET /authors/:id
    // PUT /authors
    // PATCH /authors/:id
    // DELETE /authors/:id
    //
    // GET /authors/:id/books
    // GET /authors/:id/books/:book_id
    // ...
    // ...
    //
    async fn handle(&self, http_method: &str, uri: &str) -> Option<Result<String, HandlerError>> {
        let mut pieces = uri.split("/");
        pieces.next();
        if let Some(some) = pieces.next() {
            if some == T::default().table_name() {
                let id = pieces.next();
                let tuple = (http_method, id);

                println!("Trying to match this: {:?}", tuple);

                match tuple {
                    ("GET", None) => {
                        return Some(self.get_all().await);
                    }

                    ("PUT", None) => {
                        return Some(self.create().await);
                    }

                    ("POST", Some(id)) => {
                        return Some(self.update(id).await);
                    }

                    ("DELETE", Some(id)) => {
                        return Some(self.delete(id).await);
                    }

                    ("GET", Some(id)) => {
                        return Some(self.get(id).await);
                    }

                    (_, _) => {
                        return None;
                    }
                }
            }
        } else {
            println!("I really do not know what to do!");
        }
        return None;
    }

    async fn get_all(&self) -> Result<String, HandlerError> {
        let data = T::find().into_json().all(self.db).await.unwrap();
        Ok(json!({
          "result": data
        })
        .to_string())
    }
    async fn get(&self, id: &str) -> Result<String, HandlerError> {
        // let data = T::find_by_id(1).into_json().one(self.db).await.unwrap();
        Ok(json!({
          "result": {
            "id": id,
            "name": "Pippo"
          }
        })
        .to_string())
    }
    async fn create(&self) -> Result<String, HandlerError> {
        Ok(json!({
          "result": {}
        })
        .to_string())
    }
    async fn delete(&self, _id: &str) -> Result<String, HandlerError> {
        Ok(json!({
          "result": {}
        })
        .to_string())
    }
    async fn update(&self, _id: &str) -> Result<String, HandlerError> {
        Ok(json!({
          "result": {}
        })
        .to_string())
    }
}
