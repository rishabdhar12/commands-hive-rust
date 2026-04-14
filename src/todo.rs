pub struct Todo {
    id: i32,
    title: String,
    description: String,
    status: i32, // 0 -> complete, 1 > incomplete
}


impl Todo {
    pub fn add_todo(todo: Todo) -> Result<(), String> {
        todo!()
    }

    pub fn list_todo() -> Result<(), String> {
        todo!()
    }

    pub fn update_todo(id: i32, todo: Todo) -> Result<(), String> {
        todo!()
    }

    pub fn delete_todo(id: i32) -> Result<(), String> {
        todo!()
    }
}
