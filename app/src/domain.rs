// app/src/domain.rs

// dependencies
use pavex::time::Timestamp;

// struct type to represent a blog post
#[derive(Debug, Default, Clone)]
pub struct BlogPost {
  pub id: i64,
  pub date: Timestamp,
  pub title: String,
  pub summary: String,
  pub content: String,
  pub slug: String,
  pub categories: Vec<String>,
  pub tags: Vec<String>,
  pub edited: bool,
  pub updated: Timestamp,
}