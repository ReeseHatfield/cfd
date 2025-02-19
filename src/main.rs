use core::str;
use std::error::Error;
use std::process::Command;
use std::{env, io};

use crossterm::event::{KeyCode, KeyEvent};
use crossterm::{event, ExecutableCommand};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;
use ratatui::crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;

use ratatui::{prelude::*, widgets::*};


fn main() -> Result<(), Box<dyn Error>>{

    let options = get_options()?;

    if options.len() == 1 {
        run_c(options[0].clone())?;

        return Ok(())
    }

    enable_raw_mode()?;


    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    let mut selected_index = 0;



    loop {

        terminal.draw(|frame| {
            // Get the full terminal size
            let size = frame.area();
            let width = size.width;
            let height = size.height;
        
            let start_y = 1; 
        
            let menu_area = Rect::new(0, start_y, width, height - start_y);
        
            let menu: Vec<ListItem> = options
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    let style = if i == selected_index {
                        Style::default().fg(Color::Yellow).bg(Color::Blue)
                    } else {
                        Style::default()
                    };
                    ListItem::new(item.as_str()).style(style)
                })
                .collect();
        
            let list = List::new(menu)
                .block(Block::default().borders(Borders::ALL).title("cfd"));
        
            // Render the list widget in the defined area below the terminal prompt or previous content
            frame.render_widget(list, menu_area);
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
                    // also need this for ctrl+c
                },
                _ => {}
            }
        }
    }

    disable_raw_mode()?;

    let selected_dir = options[selected_index].clone();

    terminal.backend_mut().execute(LeaveAlternateScreen)?;
    
    run_c(selected_dir)?;

    // kill parent process 
    // turn shell into code instance
    let ppid = unsafe { libc::getppid() };
    if ppid > 1 {
        let _ = kill(Pid::from_raw(ppid), Signal::SIGKILL);
    }


    Ok(())
}


fn run_c(selected_dir: String) -> Result<(), Box<dyn Error>>{

    let _child = Command::new("code")
        .arg("-n")
        .arg(selected_dir)
        .output();

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