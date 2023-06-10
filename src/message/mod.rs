pub struct Message
{
    content: String,
}

impl Message {
    pub fn make(content: String) -> Message
    {
        return Message {content};
    }

    pub fn content(&self) -> &str
    {
        return &self.content
    }
}