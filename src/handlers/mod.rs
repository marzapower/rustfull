use std::marker::PhantomData;

use futures::Future;
use sea_orm::{DatabaseConnection, EntityTrait, PrimaryKeyTrait};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub trait Handler<T>
where
    T: EntityTrait,
    for<'a> <T::PrimaryKey as PrimaryKeyTrait>::ValueType: Deserialize<'a>,
{
    fn handle(
        &self,
        http_method: &str,
        uri: &str,
    ) -> impl Future<Output = Option<Result<String, HandlerError>>>;

    fn get_all(&self) -> impl Future<Output = Result<String, HandlerError>>;
    fn get<K>(&self, id: K) -> impl Future<Output = Result<String, HandlerError>>
    where
        K: Into<<T::PrimaryKey as PrimaryKeyTrait>::ValueType>,
        K: Serialize;
    fn create(&self) -> impl Future<Output = Result<String, HandlerError>>;
    fn delete<K>(&self, id: K) -> impl Future<Output = Result<String, HandlerError>>
    where
        K: Into<<T::PrimaryKey as PrimaryKeyTrait>::ValueType>,
        K: Serialize;
    fn update<K>(&self, id: K) -> impl Future<Output = Result<String, HandlerError>>
    where
        K: Into<<T::PrimaryKey as PrimaryKeyTrait>::ValueType>,
        K: Serialize;
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

impl<T: EntityTrait> Handler<T> for SimpleHandler<'_, T>
where
    T: EntityTrait,
    <T::PrimaryKey as PrimaryKeyTrait>::ValueType: Clone + Serialize,
    for<'a> <T::PrimaryKey as PrimaryKeyTrait>::ValueType: Deserialize<'a>,
{
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
        let mut pieces = uri.split('/');
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
                        return Some(
                            self
                                .update(
                                    serde_json::from_str::<
                                        <T::PrimaryKey as PrimaryKeyTrait>::ValueType,
                                    >(id)
                                    .unwrap(),
                                )
                                .await,
                        );
                    }

                    ("DELETE", Some(id)) => {
                        return Some(
                            self
                                .delete(
                                    serde_json::from_str::<
                                        <T::PrimaryKey as PrimaryKeyTrait>::ValueType,
                                    >(id)
                                    .unwrap(),
                                )
                                .await,
                        );
                    }

                    ("GET", Some(id)) => {
                        return Some(
                            self
                                .get(
                                    serde_json::from_str::<
                                        <T::PrimaryKey as PrimaryKeyTrait>::ValueType,
                                    >(id)
                                    .unwrap(),
                                )
                                .await,
                        );
                    }

                    (_, _) => {
                        return None;
                    }
                }
            }
        } else {
            println!("I really do not know what to do!");
        }
        None
    }

    async fn get_all(&self) -> Result<String, HandlerError> {
        let data = T::find().into_json().all(self.db).await.unwrap();
        Ok(json!({
          "result": data
        })
        .to_string())
    }
    async fn get<K>(&self, id: K) -> Result<String, HandlerError>
    where
        K: Into<<T::PrimaryKey as PrimaryKeyTrait>::ValueType>,
        K: Serialize,
    {
        let data = T::find_by_id(id).into_json().one(self.db).await.unwrap();

        Ok(json!({
          "result": data
        })
        .to_string())
    }
    async fn create(&self) -> Result<String, HandlerError> {
        Ok(json!({
          "result": {}
        })
        .to_string())
    }
    async fn delete<K>(&self, _id: K) -> Result<String, HandlerError>
    where
        K: Into<<T::PrimaryKey as PrimaryKeyTrait>::ValueType>,
        K: Serialize,
    {
        Ok(json!({
          "result": {}
        })
        .to_string())
    }
    async fn update<K>(&self, _id: K) -> Result<String, HandlerError>
    where
        K: Into<<T::PrimaryKey as PrimaryKeyTrait>::ValueType>,
        K: Serialize,
    {
        Ok(json!({
          "result": {}
        })
        .to_string())
    }
}
