use simple_table_view::table_view;
use simple_table_view::itable_view::ItableView;

#[test]
fn test_ui() {
    let mut a = table_view::TableView::new();
    a.make_headers( "nome".to_string());
    a.make_headers("idade".to_string());
    a.write_cell("vegeta".to_string());
    a.write_cell("52".to_string());
    a.new_line();
    a.write_cell("supremo senhor kayo".to_string());
    a.write_cell("5000".to_string());
    a.show();
}