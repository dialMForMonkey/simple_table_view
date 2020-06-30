pub trait ItableView {
    fn new() -> Self;
    fn make_headers(&mut self, name: String);
    fn write_cell(&mut self, value: String);
    fn jump_cell(&mut self);
    fn new_line(&mut self);
    fn show(&self);
}