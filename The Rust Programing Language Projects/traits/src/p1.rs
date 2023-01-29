pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {}
    /*
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
*/

pub trait Summary {
    fn summarize(&self) -> String {
        return String::from("(Read more...)") // default implementation
    }
}
pub trait Display {
    // --snip--
}
pub trait Debug {
    // --snip--
}
pub trait Clone {
    // --snip--
}

/* shorthand
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// longhand
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
*/
/* shorthand
pub fn notify(item: &(impl Summary + Display), item2: &impl Summary) { // multiple traits
    println!("Breaking news! {}", item.summarize());
}
// longhand
pub fn notify2<T: Summary + Display>(item: &T, item2: &T) { // multiple traits
    println!("Breaking news! {}", item.summarize());
}
*/
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
pub fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    return 0;
}

pub fn run() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("horse_ebooks"),
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new article: {}", article.summarize());

    notify(&article);
}