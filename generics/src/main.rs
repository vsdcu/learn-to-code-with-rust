#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String
}

impl ChatMessage<DigitalContent> {
    fn  consume_entertainment(&self) {
        println!("Content is {:?}", self.content);
    }
}

impl<T> ChatMessage<T> {
    fn retrieve_time(&self) -> String {
        self.time.clone()
    }
}

fn main() {

let message_1 = ChatMessage {
    content: "myContent".to_string(),
    time: "2.00".to_string()

};

let message_2 = ChatMessage {
    content: "myContentSlice",
    time: "3.00".to_string()
};

let message_3 = ChatMessage {
    content: DigitalContent::VideoFile,
    time: "4.00".to_string()
};

message_3.consume_entertainment();

println!("{}",message_1.retrieve_time());
println!("{}",message_2.retrieve_time());
println!("{}",message_3.retrieve_time());


}
