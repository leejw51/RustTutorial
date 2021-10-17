use serde::{Deserialize, Serialize};
use serde_rusqlite::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Example {
    id: i64,
    name: String,
}

fn test() -> std::result::Result<(), failure::Error> {
    let connection = rusqlite::Connection::open_in_memory().unwrap();
    connection.execute("CREATE TABLE example (id INT, name TEXT)", [])?;

    for i in 1..20 {
        let row1 = Example {
            id: i,
            name: format!("first name = {}", i * 100).into(),
        };
        connection.execute(
            "INSERT INTO example (id, name) VALUES (:id, :name)",
            to_params_named(&row1).unwrap().to_slice().as_slice(),
        )?;
    }

    let mut statement = connection.prepare("SELECT * FROM example")?;
    let mut res = from_rows::<Example>(statement.query([])?);
    for r in res {
        println!("{}", serde_json::to_string(&r?)?);
    }

    /*let mut statement = connection.prepare("SELECT * FROM example")?;
    let mut rows = statement.query_and_then([], from_row::<Example>)?;

    let mut statement = connection.prepare("SELECT * FROM example")?;
    let columns = columns_from_statement(&statement);
    let mut rows =
        statement.query_and_then([], |row| from_row_with_columns::<Example>(row, &columns))?;

    let mut statement = connection.prepare("SELECT * FROM example")?;
    let mut rows = statement.query([])?;
    {
        let mut res = from_rows_ref::<Example>(&mut rows);
    }*/
    Ok(())
}

fn main() {
    test().expect("");
}
