#[cfg(test)]
mod tests{
    use crate::table_view;
    use crate::itable_view::ItableView;
    use crate::table_view::TableView;
    use std::any::Any;

    fn is_table_view(s: &dyn Any) -> bool{
        if s.is::<TableView>() {
            true
        } else {
            false
        }
    }

    #[test]
    fn test_valid_type(){

        let a = table_view::TableView::new();
        assert!( is_table_view(&a))
    }

    #[test]
    fn test_make_header(){
        let mut a = table_view::TableView::new();
        a.make_headers("maria".to_string());

        assert_eq!(a.headers.len(), 1);
    }

    #[test]
    #[should_panic("Headers nao iniciada")]
    fn test_write_cell_not_initiate_headers(){
        let mut a = table_view::TableView::new();
        a.write_cell("lisane".to_string());
    }

    #[test]
    fn test_jump_cell(){
        let mut a = table_view::TableView::new();
        assert_eq!(a.current_columns, 0);
        a.jump_cell();
        a.jump_cell();
        assert_eq!(a.current_columns, 2);
        assert_eq!(a.lines.last().unwrap().len(), 2);
        assert!(a.lines.last().unwrap().last().unwrap().is_none());
    }

    #[test]
    fn test_new_line(){
        let mut a = table_view::TableView::new();
        a.jump_cell();
        a.jump_cell();
        a.new_line();
        assert_eq!(a.lines.first().unwrap().len(), 2);
        assert!(a.lines.first().unwrap().last().unwrap().is_none());
        assert_eq!(a.lines.last().unwrap().len(), 0);
        assert_eq!(a.lines.len(), 2);
    }


    #[test]
    fn test_complete_not_contemplate_show(){
        let mut a = table_view::TableView::new();
        a.make_headers("nome".to_string());
        a.make_headers("idade".to_string());
        a.write_cell("vegeta".to_string());
        a.write_cell("52".to_string());
        a.new_line();
        a.write_cell("supremo senhor kayo".to_string());
        a.write_cell("5000".to_string());
        assert_eq!(a.lines.len(), 2);
        assert_eq!(a.headers[0].1, 19);
        assert_eq!(a.headers[0].0, "nome");
        assert_eq!(a.lines[0][0].as_ref().unwrap(), "vegeta");
        assert_eq!(a.lines[1][1].as_ref().unwrap(), "5000");
    }




}

use crate::itable_view::ItableView;
use std::borrow::Borrow;
use crate::auxiliaries;

type MaxSizeForValue = usize;
type NameHeader = String;
type Cell = Option<String>;
type Column = Vec<Cell>;

pub struct TableView{
    headers: Vec<(NameHeader, MaxSizeForValue)>,
    lines: Vec<Column>,
    current_columns: usize
}

 impl ItableView for TableView {


    fn new() -> Self {
        let first_line_columns = vec![Vec::new()];
        Self {
            headers: Vec::new(),
            lines: first_line_columns,
            current_columns:0
        }
    }

    fn make_headers(&mut self, name: String) {
        let size_string = name.len();
        self.headers.push((name, size_string));
    }

    fn write_cell(&mut self, value_cell: String) {
        let length = (&value_cell).len();
        let last_bigger_string_for_current_columns = self.headers
            .get(self.current_columns)
            .expect("Headers nao iniciada").1;

        self.headers[self.current_columns].1  = *auxiliaries::get_bigger(
            &length,
            last_bigger_string_for_current_columns.borrow()
        );
        let position_last_line= self.lines.len() -1;
        let mut vec_line= self.lines[position_last_line].to_owned();
        vec_line.push(Some(value_cell));
        self.lines[position_last_line] = vec_line;
        self.current_columns = self.current_columns +1;
    }

     fn jump_cell(&mut self) {
         let position_last_line= self.lines.len() - 1;
         let mut vec_line  = self.lines[position_last_line].to_owned();
         vec_line.push(None);
         self.lines[position_last_line] = vec_line;
         self.current_columns = self.current_columns +1;
     }

     fn new_line(&mut self) {
         self.current_columns = 0;
         self.lines.push(Vec::new());
    }

    fn show(&self) {
        let sizes_string = self.headers
            .iter()
            .map(|(header, size_string)|{
                self::auxiliaries::print_cell_with_size(header.to_owned(), size_string.clone());
                *size_string
            })
            .collect::<Vec<usize>>();

        self::auxiliaries::print_new_line();

        self.lines
            .iter()
            .for_each(|column|{


                let range =  0..column.len();
                for position_column in range {

                    let size_string = sizes_string[position_column];
                    let opt_value = column[position_column].to_owned();
                    let value = opt_value.unwrap_or(String::new());
                    self::auxiliaries::print_cell_with_size(
                        value,
                        size_string.clone()
                    )
                }
                self::auxiliaries::print_new_line();

            })

    }
}
