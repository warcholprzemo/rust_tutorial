pub fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

pub fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

/* Instead of 2 above implementations we can use one that use generic types and traits */
pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    /* PartialOrd is build-in Trait so function can work on slices of any type that we can compare.
     * Copy forces the function to use only Copy-able parameters (like i32 or char)
     */
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

/* ---------------- TRAITS FROM SCRATCH ---------------- */

pub trait Summary {
    fn summarize(&self) -> String;  // every type which will use this trait needs to implement `summarize`
    fn help(&self) -> String {  // trait can also contains a default implementation
        String::from("Go to generics to discover the implementation")
    }
    // we can also combine mandatory functions with the default representation
    fn get_popularity(&self) -> i32;
    fn rating(&self) -> String {
        format!("Rating {}", self.get_popularity())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub occ: i32
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("<Article>: {} by {}", self.headline, self.author)
    }
    fn get_popularity(&self) -> i32{
        self.occ
    }
}

pub struct Tweet {
    pub tweet: String,
    pub username: String,
    pub likes: i32
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("<Tweet>: {} created by {}", self.tweet, self.username)
    }
    fn get_popularity(&self) -> i32 {
        self.likes
    }
}
