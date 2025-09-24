use calamine::{open_workbook, Reader, Xlsx};
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct SimpleDataFrame {
    headers: Vec<String>,
    columns: Vec<Vec<String>>,
}

type Error = Box<dyn std::error::Error + Send + Sync>;

impl SimpleDataFrame {
    pub fn from_xlsx(path: &str) -> Result<Self, Error> {
        let mut workbook: Xlsx<_> = open_workbook(path)?;
        let name_first_sheet = workbook.sheet_names().first().cloned().unwrap();
        let sheet = workbook.worksheet_range(name_first_sheet.as_str())?;

        let headers: Vec<String> = sheet
            .rows()
            .nth(5)
            .unwrap()
            .iter()
            .map(|cell| cell.to_string().to_ascii_lowercase().replace(" ", "_"))
            .collect();

        let row_to_process = sheet.rows().skip(6).take(sheet.height() - 6);
        let rows_vec: Vec<_> = row_to_process.collect();
        let is_process_last_row = if !rows_vec.is_empty() {
            let last_row = rows_vec.last().unwrap();
            last_row
                .iter()
                .any(|cell| cell.to_string().eq_ignore_ascii_case("Totale complessivo"))
        } else {
            false
        };

        let rows_count = if is_process_last_row || rows_vec.is_empty() {
            sheet.height() - (1 + 6)
        } else {
            rows_vec.len()
        };

        // estrapolo tutti i dati e li salvo per colonne
        let mut column_data: Vec<Vec<String>> = vec![Vec::new(); headers.len()];
        for row_index in 0..rows_count {
            let row = rows_vec.get(row_index).unwrap();
            for (i, cell) in row.iter().enumerate() {
                if i < column_data.len() {
                    column_data[i].push(cell.to_string());
                }
            }
        }

        Ok(SimpleDataFrame {
            headers,
            columns: column_data,
        })
    }

    pub fn column(&self, col_name: &str) -> Result<Vec<String>, Error> {
        let index = self
            .headers
            .iter()
            .position(|header| header == col_name)
            .unwrap();
        self.columns
            .get(index)
            .cloned()
            .ok_or(format!("Column {} not found", col_name).into())
    }

    pub fn set_column(&mut self, col_name: &str, value: Vec<String>) -> Result<(), Error> {
        let index = self
            .headers
            .iter()
            .position(|header| header == col_name)
            .ok_or_else(|| format!("Column {} not found", col_name))?;
        self.columns[index] = value;
        Ok(())
    }

    pub fn select(&mut self, col_names: &[&str]) -> Result<(), Error> {
        let mut indices = col_names
            .iter()
            .map(|name| {
                self.headers
                    .iter()
                    .position(|header| header.eq_ignore_ascii_case(name))
                    .ok_or_else(|| format!("Column {} not found", name))
            })
            .collect::<Result<Vec<_>, _>>()?;
        indices.sort();

        let new_headers = indices.iter().map(|&i| self.headers[i].clone()).collect();

        let new_rows = self
            .columns
            .iter()
            .enumerate()
            .filter(|(index, _)| indices.contains(index))
            .map(|(_, row)| row.iter().map(|cell| cell.to_string()).collect())
            .collect();
        
        self.headers = new_headers;
        self.columns = new_rows;
        Ok(())
    }

    pub fn unique(&mut self) {
        let mut seen = HashSet::new();
        let mut unique_rows: Vec<Vec<String>> = vec![vec![]; self.columns.len()];

        for i in 0..self.columns[0].len() {
            let mut row = Vec::new();
            for j in 0..self.columns.len() {
                row.push(self.columns[j][i].clone());
            }
            if !seen.contains(&row) {
                seen.insert(row.clone());
            }
        }

        seen.iter().for_each(|row| {
            for j in 0..row.len() {
                unique_rows[j].push(row[j].clone());
            }
        });

        self.columns = unique_rows;
    }

    pub fn traspose(&self) -> TransposedDataFrame {
        TransposedDataFrame::from_dataframe(self)
    }
}

impl Display for SimpleDataFrame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for header in &self.headers {
            write!(f, "{}\t", header)?;
        }
        writeln!(f)?;
        for i in 0..self.columns[0].len() {
            for j in 0..self.columns.len() {
                write!(f, "{}\t", self.columns[j][i])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct TransposedDataFrame {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl TransposedDataFrame {
    pub fn from_dataframe(df: &SimpleDataFrame) -> Self {
        let mut rows: Vec<Vec<String>> = vec![vec![]; df.columns[0].len()];
        rows.iter_mut().enumerate().for_each(|(i, row)| {
            for j in 0..df.headers.len() {
                row.push(df.columns[j][i].clone());
            }
        });
        TransposedDataFrame {
            headers: df.headers.clone(),
            rows,
        }
    }

    pub fn traspose(&self) -> SimpleDataFrame {
        let mut columns: Vec<Vec<String>> = vec![vec![]; self.headers.len()];
        columns.iter_mut().enumerate().for_each(|(i, row)| {
            for j in 0..self.rows.len() {
                row.push(self.rows[j][i].clone());
            }
        });

        SimpleDataFrame {
            headers: self.headers.clone(),
            columns,
        }
    }

    pub fn iter_rows(&'_ self) -> DataFrameIterator<'_> {
        DataFrameIterator {
            headers: &self.headers,
            rows_iter: self.rows.iter(),
        }
    }
}

impl Display for TransposedDataFrame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for header in &self.headers {
            write!(f, "{}\t", header)?;
        }
        writeln!(f)?;
        for row in &self.rows {
            writeln!(f, "{}", row.join("\t"))?;
        }
        Ok(())
    }
}

impl<'a> IntoIterator for &'a TransposedDataFrame {
    type Item = HashMap<&'a str, &'a str>;
    type IntoIter = DataFrameIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_rows()
    }
}

pub struct DataFrameIterator<'a> {
    headers: &'a [String],
    rows_iter: std::slice::Iter<'a, Vec<String>>,
}

impl<'a> Iterator for DataFrameIterator<'a> {
    type Item = HashMap<&'a str, &'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        self.rows_iter.next().map(|row| {
            self.headers
                .iter()
                .zip(row.iter())
                .map(|(header, value)| (header.as_str(), value.as_str()))
                .collect()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use app_utils::test::ResultTest;
    use std::iter;

    #[test]
    fn test_import() {
        let df = SimpleDataFrame::from_xlsx(
            "/home/maty/Downloads/Telegram Desktop/scuole massalongo e coccinelle.xlsx",
        )
            .unwrap();
        assert_eq!(df.headers.len(), 11);
        assert_eq!(df.columns[0].len(), 68);
        println!("{df}")
    }

    #[test]
    fn test_set_column() -> ResultTest {
        let mut df = SimpleDataFrame::from_xlsx(
            "/home/maty/Downloads/Telegram Desktop/scuole massalongo e coccinelle.xlsx",
        )?;
        df.select(&["fascicolo", "nome_via", "chiave", "piano"])?;


        let old_column = df.column("chiave")?;
        let new_column: Vec<String> =
            iter::repeat_n("1452-45".to_string(), old_column.len()).collect();

        df.set_column("chiave", new_column.clone())?;

        assert_eq!(df.column("chiave")?, new_column);

        println!("{df}");

        Ok(())
    }

    #[test]
    fn test_select() {
        let mut df = SimpleDataFrame::from_xlsx(
            "/home/maty/Downloads/Telegram Desktop/scuole massalongo e coccinelle.xlsx",
        )
            .unwrap();
        df.select(&["fascicolo", "nome_via", "chiave"]).unwrap();
        assert_eq!(df.headers.len(), 3);
        assert_eq!(df.columns.len(), 3);
        println!("{df}")
    }

    #[test]
    fn test_unique() {
        let mut df = SimpleDataFrame::from_xlsx(
            "/home/maty/Downloads/Telegram Desktop/scuole massalongo e coccinelle.xlsx",
        )
            .unwrap();
        df
            .select(&["fascicolo", "nome_via", "chiave", "piano"])
            .unwrap();
        df.unique();
        assert_eq!(df.columns[0].len(), 4);
        println!("{df}");
    }

    #[test]
    fn test_transpose() {
        let mut df = SimpleDataFrame::from_xlsx(
            "/home/maty/Downloads/Telegram Desktop/scuole massalongo e coccinelle.xlsx",
        )
            .unwrap();
         df
            .select(&["fascicolo", "nome_via", "chiave", "piano"])
            .unwrap();
        let df = TransposedDataFrame::from_dataframe(&df);
        println!("{df}")
    }

    #[test]
    fn test_iter() {
        let df = SimpleDataFrame::from_xlsx(
            "/home/maty/Downloads/Telegram Desktop/scuole massalongo e coccinelle.xlsx",
        )
            .unwrap();
        let df = df.traspose();
        for row in df.iter_rows() {
            println!("{row:?}");
        }
    }

    #[test]
    fn test_traspose_traspose() {
        let mut df = SimpleDataFrame::from_xlsx(
            "/home/maty/Downloads/Telegram Desktop/scuole massalongo e coccinelle.xlsx",
        )
            .unwrap();
        df
            .select(&["fascicolo", "nome_via", "chiave", "piano"])
            .unwrap();
        let df_t = df.clone().traspose();
        let df_t_t = df_t.traspose();
        assert_eq!(df_t_t, df);
    }
}
