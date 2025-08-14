use std::cmp::PartialOrd;

pub trait Summary {
    fn summarize (&self) -> String;
}
pub struct News{
    pub headline: String,
    pub author: String,
}
pub struct Tweet{
    pub username: String,
    pub content: String,
}
impl Summary for News {
    fn summarize (&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
}
impl Summary for Tweet {
    fn summarize (&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}
pub fn notify (item: &impl Summary){
    println! ("Breaking news! {}", item.summarize())
}

fn largest_i32 (lista: &[i32]) -> i32 {
    let mut larger = lista[0];
    for &item in lista.iter() {
        if item > larger {
            larger = item;
        }
    }
    larger
}
fn largest_i64 (lista: &[i64]) -> i64 {
    let mut larger = lista[0];
    for &item in lista.iter() {
        if item > larger {
            larger = item;
        }
    }
    larger
}
fn largest<T: PartialOrd + Copy> (lista: &[T]) -> T {
    let mut larger = lista[0];
    for &item in lista.iter() {
        if item > larger {
            larger = item;
        }
    }
    larger
}

fn main() {

    let tweet_pepe = Tweet{
        username: "Pepe_eaea_pepe".to_string(),
        content: "Andate Riquelme!!!".to_string()
    };
    let news_pepe = News{
        headline: "Riquelme se tiene que ir".to_string(),
        author: "Pepe_eaea_pepe".to_string()
    };

    notify(&tweet_pepe);
    notify(&news_pepe);

    let var = largest (&[34, 50, 75, 90, 140]);
    println!("{}", var);
}
