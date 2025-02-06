pub fn draw(headers: Vec<&str>, rows: Vec<Vec<&str>>) {
	let mut column_widths = vec![0; headers.len()];

	for row in &rows {
		for (index, item) in row.iter().enumerate() {
			if column_widths[index] < item.len() {
				column_widths[index] = item.len();
			}
		}
	}

	print!("\n┌─");

	for (index, width) in column_widths.iter().enumerate() {
		if index == 0 {
			print!("{:─<width$}", "", width = width);
		} else {
			print!("─┬─{:─<width$}", "", width = width);
		}
	}

	println!("─┐");

	for (index, header) in headers.iter().enumerate() {
		print!("│ {:<width$} ", header, width = column_widths[index]);
	}

	println!("│");
	print!("├─");

	for (index, width) in column_widths.iter().enumerate() {
		if index == 0 {
			print!("{:─<width$}", "", width = width);
		} else {
			print!("─┼─{:─<width$}", "", width = width);
		}
	}

	println!("─┤");

	for row in rows {
		for (index, item) in row.iter().enumerate() {
			print!("│ {:<width$} ", item, width = column_widths[index]);
		}

		println!("│");
	}

	print!("└─");

	for (index, width) in column_widths.iter().enumerate() {
		if index == 0 {
			print!("{:─<width$}", "", width = width);
		} else {
			print!("─┴─{:─<width$}", "", width = width);
		}
	}

	println!("─┘\n");
}