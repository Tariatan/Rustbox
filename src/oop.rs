pub struct IncapsulatedCollection {
    list: Vec<i32>,
    average: f64,
}
impl IncapsulatedCollection {
    pub fn new() -> Self {
        IncapsulatedCollection {
            list: vec![],
            average: 0.0,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
    
    pub fn remove(&mut self)  -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }
    
    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

// Bounded Parametric Polymorphism

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // Vector of trait objects - any type inside a Box that implements the Draw trait
    pub components: Vec<Box<dyn Draw>>,
}

#[allow(unused)]
impl Screen {
    pub fn new() -> Self {
        Screen {
            components: vec![],
        }
    }
    
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
    
    pub fn demo() -> Self {
        Screen {
            components: vec![
                Box::new(SelectBox {
                    width: 75,
                    height: 10,
                    options: vec![
                        String::from("Yes"),
                        String::from("No"),
                        String::from("Maybe"),
                    ],
                }),
                Box::new(Button {
                    width: 50,
                    height: 10,
                    label: String::from("OK"),
                }),
            ],
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("{} {} {}", self.label, self.width, self.height);
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("{:?} {} {}", self.options, self.width, self.height);
    }
}

// State Pattern
pub fn state_pattern() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
    post.request_review();
    // No post content, since it is still in a draft
    assert_eq!("", post.content());
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Self {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
    pub fn request_review(&mut self) {
        // Call the take() method to take the Some value out of the state field
        // and leave a None in its place because Rust doesn't allow unpopulated fields in structs.
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_review());
        }
    }
    pub fn approve(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.approve())
        }
    }
}

trait State {
    // self: Box<Self> means this method is only valid when called on a Box holding the type.
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview{})
    }
    fn approve(self: Box<Self>) -> Box<(dyn State)> {
        self
    }
}

struct PendingReview {}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<(dyn State)> {
        Box::new(Published{})
    }
}

struct Published {}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}


// Encode the state into the type system
pub fn encode_state_into_type_system() {
    let mut post = TypedPost::new();
    post.add_text("I ate a salad for lunch today");
    let post = post.request_review();
    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());    
}
pub struct TypedPost {
    pub content: String,
}

pub struct DraftPost {
    pub content: String,
}

impl TypedPost {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }
    
    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}
impl PendingReviewPost {
    pub fn approve(self) -> TypedPost {
        TypedPost {
            content: self.content,
        }
    }
}