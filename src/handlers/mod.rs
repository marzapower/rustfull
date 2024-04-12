use serde_json::json;

pub trait Handler {
  fn handle(&self, http_method: &str, uri: &str) -> Option<Result<String, HandlerError>>;

  fn get_all(&self) -> Result<String, HandlerError>;
  fn get(&self, id: &str) -> Result<String, HandlerError>;
  fn create(&self) -> Result<String, HandlerError>;
  fn delete(&self, id: &str) -> Result<String, HandlerError>;
  fn update(&self, id: &str) -> Result<String, HandlerError>;
}

pub struct SimpleHandler {
  resources_name: String
}

impl SimpleHandler {
  pub fn new(name: &str) -> Self {
    SimpleHandler {
      resources_name: String::from(name)
    }
  }
}

#[derive(Debug)]
pub struct HandlerError;

impl Handler for SimpleHandler {
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
  fn handle(&self, http_method: &str, uri: &str) -> Option<Result<String, HandlerError>> {
    println!(
      "I should handle {http_method} on {uri} for {}", self.resources_name
    );
    let mut pieces = uri.split("/");
    pieces.next();
    if let Some(some) = pieces.next() {
      if *some == self.resources_name {

        let id = pieces.next();
        let tuple = (http_method, id);

        println!("Trying to match this: {:?}", tuple);

        match tuple {
          ("GET", None) => {
            return Some(self.get_all());
          }

          ("PUT", None) => {
            return Some(self.create());
          }

          ("POST", Some(id)) => {
            return Some(self.update(id));
          }

          ("DELETE", Some(id)) => {
            return Some(self.delete(id));
          }

          ("GET", Some(id)) => {
            return Some(self.get(id));
          }

          (_,_) => {
            return None;
          }        
        }        
      }
    }
    return None;
  }

  fn get_all(&self) -> Result<String, HandlerError> {
    Ok(json!({
      "result": [{
        "id": "1",
        "name": "Pippo"
      }]
    }).to_string())
  }
  fn get(&self, id: &str) -> Result<String, HandlerError> {
    Ok(json!({
      "result": {
        "id": id,
        "name": "Pippo"
      }
    }).to_string())
  }
  fn create(&self) -> Result<String, HandlerError> {
    Ok(json!({
      "result": {}
    }).to_string())
  }
  fn delete(&self, id: &str) -> Result<String, HandlerError> {
    Ok(json!({
      "result": {}
    }).to_string())
  }
  fn update(&self, id: &str) -> Result<String, HandlerError> {
    Ok(json!({
      "result": {}
    }).to_string())
  }
}
