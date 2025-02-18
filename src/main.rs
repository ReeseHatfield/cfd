use core::str;
use std::error::Error;
use std::process::Command;
use std::{env, io};

use crossterm::event::{KeyCode, KeyEvent};
use crossterm::event;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;


use ratatui::{prelude::*, widgets::*};


fn main() -> Result<(), Box<dyn Error>>{

    let options = get_options()?;
    // println!("options: {:?}", options);



    enable_raw_mode()?;
    let stdout = io::stdout();




    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    let mut selected_index = 0;

    loop {
        terminal.draw(|frame| {
            let size = frame.size();
            let menu: Vec<ListItem> = options
                .iter()
                .enumerate()
                .map(|(i, &ref item)| {
                    let style = if i == selected_index {
                        Style::default().fg(Color::Yellow).bg(Color::Blue)
                    } else {
                        Style::default()
                    };
                    ListItem::new(item.as_str()).style(style)
                })
                .collect();

            let list = List::new(menu).block(Block::default().borders(Borders::ALL).title("Menu"));
            frame.render_widget(list, size);
        })?;


        // you'd think a tui app would just have an api for this
        if let event::Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Up => {
                    if selected_index > 0 {
                        selected_index -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected_index < options.len() - 1 {
                        selected_index += 1;
                    }
                }
                KeyCode::Enter => break,
                KeyCode::Esc => {
                    //graceful exit?
                    std::process::exit(1);
                },
                _ => {}
            }
        }
    }

    disable_raw_mode()?;

    let selected_dir = options[selected_index].clone();

    println!("Selected item was {}", selected_dir);


    Ok(())
}


fn parse_args() -> Result<String, Box<dyn Error>> {

    let full_args = env::args();

    if full_args.len() == 1 {
        return Err("Usage: cfg [fuzzy string]".into());
    }

    let query = full_args
        .skip(1) // skip bin name
        .map(|arg| arg.to_owned())
        .collect::<Vec<String>>()
        .join("");


    Ok(query)
}


fn get_options() -> Result<Vec<String>, Box<dyn Error>> {
    let z_query = parse_args()?;

    // println!("query: {:?}", z_query);

    let child = Command::new("zoxide")
        .arg("query")
        .arg("-l")
        .arg(z_query)
        .output()?;
    
    let query_res = str::from_utf8(&child.stdout)?;


    let options: Vec<String> = query_res
        .split("\n")
        .map(|s| s.to_owned())
        .filter(|s| !s.is_empty()) // remove newline at EOF
        .collect();


    Ok(options)

}