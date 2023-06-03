pub struct Message
{
    content: String,
}

impl Message {
    fn make(content: String) -> Message
    {
        return Message { content: content };
    } 
}