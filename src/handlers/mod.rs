pub trait Handler {
  fn handle(&self, http_method: &str, uri: &str) -> Result<String, HandlerError>;
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
  fn handle(&self, http_method: &str, uri: &str) -> Result<String, HandlerError> {
    println!(
      "I should handle {http_method} on {uri} for {}", self.resources_name
    );
    Ok(String::from("ciao"))
  }
}