pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub trait Summary {
    fn name(&self) -> String;
}

impl Summary for Tweet {
    fn name(&self) -> String {
        format!("{}", self.username)
    }
}
impl Summary for NewsArticle {
    fn name(&self) -> String {
        format!("{}", self.author)
    }
}

// Code won't compile because the return type is different
/*
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins once again are the best \
            hockey team in the NHL."),
        }
    } else {
        {
            Tweet {
                username: String::from("horse_ebooks"),
                content: String::from("of course, as you probably already know, people"),
                reply: false,
                retweet: false,
            }
        }
    }
}
*/
pub fn run() {
    //println!("{}", returns_summarizable(true).summarize());
}