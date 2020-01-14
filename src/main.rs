extern crate tui;
extern crate termion;

use std::io;
use std::thread;
use std::time;
use std::sync::mpsc;

use termion::event;
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::RawBackend;
use tui::widgets::{Widget, Block, SelectableList, Gauge, Paragraph, Borders, Tabs};
use tui::layout::{Group, Direction, Size, Rect};
use tui::style::{Style, Color, Modifier};

pub struct MyTabs<'a> {
    pub titles: Vec<&'a str>,
    pub selection: usize,
}

impl<'a> MyTabs<'a> {
    pub fn next(&mut self) {
        self.selection = (self.selection + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.selection > 0 {
            self.selection -= 1;
        } else {
            self.selection = self.titles.len() - 1;
        }
    }
}

struct App<'a> {
    size: Rect,
    tabs: MyTabs<'a>
}

enum Event {
    Input(event::Key),
    Tick,
}

fn main() {
    let mut app = App {
        size: Rect::default(),
        tabs: MyTabs {
            titles: vec!["Home", "About", "Skills", "Experience", "Education", "Projects", "Objective"],
            selection: 0,
        }
    };
    let (tx, rx) = mpsc::channel();
    let input_tx = tx.clone();

    thread::spawn(move || {
        let stdin = io::stdin();
        for c in stdin.keys() {
            let evt = c.unwrap();
            input_tx.send(Event::Input(evt)).unwrap();
            if evt == event::Key::Char('q') {
                break;
            }
        }
    });

    thread::spawn(move || {
        let tx = tx.clone();
        loop {
            tx.send(Event::Tick).unwrap();
            thread::sleep(time::Duration::from_millis(200));
        }
    });

    let backend = RawBackend::new().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();

    loop {
        let size = terminal.size().unwrap();
        if size != app.size {
            terminal.resize(size).unwrap();
            app.size = size;
        }
        draw(&mut terminal, &app).unwrap();
        let evt = rx.recv().unwrap();
        match evt {
            Event::Input(input) => {
                match input {
                    event::Key::Char('q') => {
                        break;
                    }
                    event::Key::Left => {
                        app.tabs.previous();
                    }
                    event::Key::Right => {
                        app.tabs.next();
                    }
                    _ => {}
                }
            }
            Event::Tick => {}
        }
    }
    terminal.show_cursor().unwrap();
    terminal.clear().unwrap();
}

fn draw(t: &mut Terminal<RawBackend>, app: &App) -> Result<(), io::Error> {

    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Fixed(3), Size::Min(0)])
        .render(t, &app.size, |t, chunks| {
            Tabs::default()
                .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan)).title("Sections").title_style(Style::default().fg(Color::Cyan)))
                .titles(&app.tabs.titles)
                .style(Style::default().fg(Color::Magenta))
                .highlight_style(Style::default().fg(Color::Yellow))
                .select(app.tabs.selection)
                .render(t, &chunks[0]);
            match app.tabs.selection {
                0 => {
                    draw_home(t, &chunks[1]);
                }
                1 => {
                    draw_about(t, &chunks[1]);
                }
                2 => {
                    draw_skills(t, &chunks[1]);
                }
                3 => {
                    draw_experience(t, &chunks[1]);
                }
                4 => {
                    draw_education(t, &chunks[1]);
                }
                5 => {
                    draw_projects(t, &chunks[1]);
                }
                6 => {
                    draw_objective(t, &chunks[1]);
                }
                _ => {}
            };
        });
    t.draw()?;
    Ok(())
}

fn draw_home(t: &mut Terminal<RawBackend>, area: &Rect) {
    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Percent(10), Size::Percent(80), Size::Percent(10)])
        .render(t, area, |t, chunks| {
                Group::default()
                .direction(Direction::Horizontal)
                .sizes(&[Size::Percent(10), Size::Percent(80), Size::Percent(10)])
                .render(t, &chunks[1], |t, chunks| {
                    Paragraph::default()
                    .block(Block::default()
                        .borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan))
                        .title("DAISY T'S RESUME")
                        .title_style(Style::default().fg(Color::Magenta).modifier(Modifier::Bold)))
                    .wrap(true)
                    .text("\nUse {mod=bold;fg=yellow ←}  and {mod=bold;fg=yellow →}  to navigate between the sections.\n\n\
                           Type {mod=bold;fg=yellow q} to exit the application.\n\n\
                                                    -----------------------------------------------
                                                                    .'`. ,'`.
                                                              .---./    u    \\,---.
                                                           ___|    \\    |    /    |___
                                                          \\    `.   \\   |   /   .'    /
                                                           \\_    `.  \\  |  /  .'    _/
                                                         .-' `-._  `.:::::::.'  _.-' `-.
                                                         \\       `-;:::::::::;-'       /
                                                          >~------~:::::::::::~------~<
                                                         /      _.-;:::::::::;-._      \\
                                                         `-._.-'   .`.::::::'.   `-._.-'
                                                            /    .'  /  |  \\  `.    \\
                                                           /___.'   /   |   \\   `.___\\
                                                               |   /    |    \\    |
                                                               `--'\\   .n.   /`---'
                                                                    `.'   `.'          

                           ")
                    .style(Style::default().fg(Color::LightMagenta))
                    .render(t, &chunks[1]);
                });
        });
}

fn draw_about(t: &mut Terminal<RawBackend>, area: &Rect) {
    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Percent(40), Size::Percent(60)])
        .render(t, area, |t, chunks| {
            Group::default()
            .direction(Direction::Horizontal)
            .sizes(&[Size::Percent(50), Size::Percent(50)])
            .render(t, &chunks[0], |t, chunks| {
                Paragraph::default()
                .block(Block::default()
                    .borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan))
                    .title("Information")
                    .title_style(Style::default().fg(Color::Magenta).modifier(Modifier::Bold)))
                .wrap(true)
                .text("\n{mod=bold;fg=yellow Name:} Daisy T\n\n\
                       {mod=bold;fg=yellow Nationality:} Canadian\n\n\
                       {mod=bold;fg=yellow Currently based in:} Berlin, Germany\n\n\
                      ")
                .style(Style::default().fg(Color::LightMagenta))
                .render(t, &chunks[0]);
                Paragraph::default()
                .block(Block::default()
                    .borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan))
                    .title("Languages")
                    .title_style(Style::default().fg(Color::Magenta).modifier(Modifier::Bold)))
                .wrap(true)
                .text("\n{mod=bold;fg=yellow English:} Native\n\n\
                       {mod=bold;fg=yellow French:} Good Knowledge\n\n\
                       {mod=bold;fg=yellow German:} Good Knowledge\n\n\
                       {mod=bold;fg=yellow Cantonese:} Conversational\n\n\
                      ")
                .style(Style::default().fg(Color::LightMagenta))
                .render(t, &chunks[1]);
            });
            Group::default()
            .direction(Direction::Horizontal)
            .sizes(&[Size::Percent(50), Size::Percent(50)])
            .render(t, &chunks[1], |t, chunks| {
                Paragraph::default()
                .block(Block::default()
                    .borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan))
                    .title("Contact")
                    .title_style(Style::default().fg(Color::Magenta).modifier(Modifier::Bold)))
                .wrap(true)
                .text("\n{mod=bold;fg=yellow Email:} daisyts@gmx.com\n\n\
                       {mod=bold;fg=yellow Phone:} +49 (0) 176 3163 5400\n\n\
                       {mod=bold;fg=yellow Website:} https://infoverload.ca/\n\n\
                       {mod=bold;fg=yellow Twitter:} https://twitter.com/1nfoverload\n\n\
                       {mod=bold;fg=yellow LinkedIn:} http://linkedin.com/in/daisyts\n\n\
                       {mod=bold;fg=yellow LinkedIn:} https://github.com/infoverload\n\n\
                      ")
                .style(Style::default().fg(Color::LightMagenta))
                .render(t, &chunks[0]);
                Paragraph::default()
                .block(Block::default()
                    .borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan))
                    .title("About me")
                    .title_style(Style::default().fg(Color::Magenta).modifier(Modifier::Bold)))
                .wrap(true)
                .text("\nI am a Software Developer, Technical Writer, Developer Advocate,\n\
                       and Open-Source Enthusiast with experience building\n\
                       web applications. \n\n\
                       I am keen on community work and sharing knowledge and have\n\
                       a particular interest in backend and infrastructure projects.\n\n\
                       My non-technical interests include knitting, baking,\n\
                       & learning new natural languages.\n\n\
                      ")
                .style(Style::default().fg(Color::LightMagenta))
                .render(t, &chunks[1]);
            });
        });
}

fn draw_skills(t: &mut Terminal<RawBackend>, area: &Rect) {
    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Percent(60), Size::Percent(40)])
        .render(t, area, |t, chunks| {
            Block::default()
                .borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan))
                .title("Programming Languages")
                .title_style(Style::default().fg(Color::Magenta).modifier(Modifier::Bold))
                .render(t, &chunks[0]);
            Group::default()
                .direction(Direction::Vertical)
                .margin(1)
                .sizes(&[Size::Fixed(2),Size::Fixed(2),Size::Fixed(2),Size::Fixed(2),Size::Fixed(2),Size::Fixed(2),Size::Fixed(2)])
                .render(t, &chunks[0], |t, chunks| {
                Gauge::default()
                    .block(Block::default().title("Go").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                    .style(Style::default()
                            .fg(Color::Magenta)
                            .bg(Color::Black)
                            .modifier(Modifier::Italic))
                    .label(&format!("70 / 100"))
                    .percent(70)
                    .render(t, &chunks[0]);
                Gauge::default()
                    .block(Block::default().title("JavaScript").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                    .style(Style::default()
                            .fg(Color::Magenta)
                            .bg(Color::Black)
                            .modifier(Modifier::Italic))
                    .label(&format!("70 / 100"))
                    .percent(70)
                    .render(t, &chunks[1]);
                Gauge::default()
                    .block(Block::default().title("PHP").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                    .style(Style::default()
                            .fg(Color::Magenta)
                            .bg(Color::Black)
                            .modifier(Modifier::Italic))
                    .label(&format!("70 / 100"))
                    .percent(70)
                    .render(t, &chunks[2]);
                Gauge::default()
                    .block(Block::default().title("Ruby").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                    .style(Style::default()
                            .fg(Color::Magenta)
                            .bg(Color::Black)
                            .modifier(Modifier::Italic))
                    .label(&format!("60 / 100"))
                    .percent(60)
                    .render(t, &chunks[3]);
                Gauge::default()
                    .block(Block::default().title("Rust (learning)").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                    .style(Style::default()
                            .fg(Color::Magenta)
                            .bg(Color::Black)
                            .modifier(Modifier::Italic))
                    .label(&format!("40 / 100"))
                    .percent(40)
                    .render(t, &chunks[4]);
                Gauge::default()
                    .block(Block::default().title("Python (learning)").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                    .style(Style::default()
                            .fg(Color::Magenta)
                            .bg(Color::Black)
                            .modifier(Modifier::Italic))
                    .label(&format!("40 / 100"))
                    .percent(40)
                    .render(t, &chunks[5]);
            });
            Block::default()
                .borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan))
                .title("Others")
                .title_style(Style::default().fg(Color::Magenta).modifier(Modifier::Bold))
                .render(t, &chunks[1]);
            Group::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .sizes(&[Size::Fixed(15),Size::Fixed(2),Size::Fixed(24),Size::Fixed(2),Size::Fixed(25),Size::Fixed(2),Size::Fixed(15),Size::Fixed(2),Size::Fixed(16),Size::Fixed(2)])
                .render(t, &chunks[1], |t, chunks| {
                    SelectableList::default()
                        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan)).title("Frameworks").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .items(&vec!["Symfony", "Laravel", "Rails"])
                        .style(Style::default().fg(Color::LightMagenta))
                        .render(t, &chunks[0]);
                    SelectableList::default()
                        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan)).title("Technologies").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .items(&vec!["GNU / Linux", "OSX", "MySQL", "AJAX", "jQuery", "OOP", "MVC", "Wordpress", "Prometheus", "Docker"])
                        .style(Style::default().fg(Color::LightMagenta))
                        .render(t, &chunks[2]);
                    SelectableList::default()
                        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan)).title("Areas").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .items(&vec!["Web Development", "Databases", "Software Engineering", "Monitoring", "Project Management"])
                        .style(Style::default().fg(Color::LightMagenta))
                        .render(t, &chunks[4]);
                    SelectableList::default()
                        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan)).title("Version Control").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .items(&vec!["Git", "Mercurial", "SVN"])
                        .style(Style::default().fg(Color::LightMagenta))
                        .render(t, &chunks[6]);
                    SelectableList::default()
                        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan)).title("Task Tracking").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .items(&vec!["JIRA", "AutoTask", "Trello", "Asana"])
                        .style(Style::default().fg(Color::LightMagenta))
                        .render(t, &chunks[8]);
            });
    });
}

fn draw_experience(t: &mut Terminal<RawBackend>, area: &Rect) {
    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Percent(15), Size::Percent(15), Size::Percent(25),Size::Percent(25), Size::Percent(19),Size::Fixed(1)])

        .render(t, area, |t, chunks| {
                Paragraph::default()
                .block(Block::default()
                    .borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan))
                    .title("2018 - current: Freelancer, Self-Employed (remote)")
                    .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                .wrap(true)
                .text("\nPerform ongoing consultative and development work ranging from website maintenance/upgrades to business strategy and development.\n\n\
                      ")
                .style(Style::default().fg(Color::LightMagenta))
                .render(t, &chunks[0]);
                Paragraph::default()
                .block(Block::default()
                    .borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan))
                    .title("2017 - current: Contributor, Fixate IO (remote)")
                    .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                .wrap(true)
                .text("\nGenerate in-depth technical content on an ongoing basis; Conduct market research and perform industry analysis;\n\n\
                       Cover wide range of topics relating to software development, memory handling, etc.\n\n\
                       Took over two projects to refactor, maintain and add new features.\n\n\
                      ")
                .style(Style::default().fg(Color::LightMagenta))
                .render(t, &chunks[1]);
                Paragraph::default()
                .block(Block::default()
                    .borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan))
                    .title("2017 - 2017: Software Developer, Project A Services GmbH (Germany)")
                    .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                .wrap(true)
                .text("\nSupported the backend development team by assisting them on various venture projects using Symfony and PHP; Collaborated \n\n\
                        with Product Managers to foster and implement Agile practices; Analyzed an existing prototype application, refactored it\n\n\
                        and added features, following the company's best practices and software development principles\n\n\
                      ")
                .style(Style::default().fg(Color::LightMagenta))
                .render(t, &chunks[2]);
                Paragraph::default()
                .block(Block::default()
                    .borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan))
                    .title("2014 - 2015: Web Developer, eKomi Ltd (Germany)")
                    .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                .wrap(true)
                .text("\nSupported IT team with both frontend and backend development and debugging tasks while working with large codebase;\n\n\
                       Performed both client-facing work (designing customized, responsive review pages for clients) and internal tooling\n\n\
                       for the rest of the team\n\n\
                      ")
                .style(Style::default().fg(Color::LightMagenta))
                .render(t, &chunks[3]);
                Paragraph::default()
                .block(Block::default()
                    .borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan))
                    .title("2011 - 2012: Software Developer, GrantStream Inc (Canada)")
                    .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                .wrap(true)
                .text("\nResponsible for a variety of development projects in grant management software including maintenance of PHP applications\n\n\
                        and MS SQL Server and MySQL databases; Configured UI of customized Web applications with PHP, MSSQL, JavaScript, jQuery, CSS\n\n\
                      ")
                .style(Style::default().fg(Color::LightMagenta))
                .render(t, &chunks[4]);
    });
}

fn draw_education(t: &mut Terminal<RawBackend>, area: &Rect) {
    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Percent(45), Size::Percent(55)])
        .render(t, area, |t, chunks| {
                Block::default()
                    .borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan))
                    .title("Education")
                    .title_style(Style::default().fg(Color::Magenta).modifier(Modifier::Bold))
                    .render(t, &chunks[0]);
                Group::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .sizes(&[Size::Percent(33), Size::Percent(33), Size::Percent(33)])
                    .render(t, &chunks[0], |t, chunks| {
                        Paragraph::default()
                        .block(Block::default()
                            .borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan))
                            .title("University of Toronto: Certificate in Project Management (2010 - 2011)")
                            .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .wrap(true)
                        .text("\nStudied foundations of project management and how to apply the most effective tools & techniques to achieve project objectives\n\n\
                            ")
                        .style(Style::default().fg(Color::LightMagenta))
                        .render(t, &chunks[0]);
                        Paragraph::default()
                        .block(Block::default()
                            .borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan))
                            .title("University of Western Ontario: Bachelor of Arts (2004 - 2009)")
                            .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .wrap(true)
                        .text("\nObtained Double Major in Computer Science and Media Studies\n\n\
                            ")
                        .style(Style::default().fg(Color::LightMagenta))
                        .render(t, &chunks[1]);
                        Paragraph::default()
                        .block(Block::default()
                            .borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan))
                            .title("Stendhal University: Exchange Program (2008)")
                            .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .wrap(true)
                        .text("\nParticipated in Academic Exchange Program in Grenoble through the University of Western Ontario\n\n\
                            ")
                        .style(Style::default().fg(Color::LightMagenta))
                        .render(t, &chunks[2]);
                });
                Block::default()
                    .borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan))
                    .title("Continuing Education")
                    .title_style(Style::default().fg(Color::Magenta).modifier(Modifier::Bold))
                    .render(t, &chunks[1]);
                Group::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .sizes(&[Size::Percent(23), Size::Percent(23),Size::Percent(23), Size::Percent(31)])
                    .render(t, &chunks[1], |t, chunks| {
                    Paragraph::default()
                    .block(Block::default()
                        .borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan))
                        .title("Getting Started with Continuous Delivery (2018)")
                        .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                    .wrap(true)
                    .text("\nO'Reilly Live Online Training Course")
                    .style(Style::default().fg(Color::LightMagenta))
                    .render(t, &chunks[0]);
                    Paragraph::default()
                    .block(Block::default()
                        .borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan))
                        .title("Practical Kubernetes (2018)")
                        .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                    .wrap(true)
                    .text("\nO'Reilly Live Online Training Course")
                    .style(Style::default().fg(Color::LightMagenta))
                    .render(t, &chunks[1]);
                    Paragraph::default()
                    .block(Block::default()
                        .borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan))
                        .title("Bill Kennedy's Ultimate Go Workshop (2018)")
                        .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                    .wrap(true)
                    .text("\nA weekend course designed to provide an intensive idiomatic view of Go")
                    .style(Style::default().fg(Color::LightMagenta))
                    .render(t, &chunks[2]);
                    Paragraph::default()
                    .block(Block::default()
                        .borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan))
                        .title("Women Techmakers - JavaScript Crash Course (2017-2018)")
                        .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                    .wrap(true)
                    .text("\nA 12-week lecture course designed to expose participants to multiple levels of the software development stack\n\
                    with introductions to Node.js, Vue.js, MongoDB, Unit Testing, CI/CD, Design Patterns, Bridging APIs, and more")
                    .style(Style::default().fg(Color::LightMagenta))
                    .render(t, &chunks[3]);
                });
    });
}

fn draw_projects(t: &mut Terminal<RawBackend>, area: &Rect) {
    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Percent(50), Size::Percent(25), Size::Percent(25)])
        .render(t, area, |t, chunks| {
                Block::default()
                    .borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan))
                    .title("Personal Projects")
                    .title_style(Style::default().fg(Color::Magenta).modifier(Modifier::Bold))
                    .render(t, &chunks[0]);
                Group::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .sizes(&[Size::Percent(40), Size::Percent(40), Size::Percent(20)])
                    .render(t, &chunks[0], |t, chunks| {
                        Paragraph::default()
                        .block(Block::default()
                            .borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan))
                            .title("Observability in the Kitchen")
                            .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .wrap(true)
                        .text("\nThis project leverages sensors, open-source software, and Go to improve breadmaking and explores the relationship between\n\
                        sourdough cultures, humidity, and temperature and how one can use systems monitoring tools to gain insight into an age-old tradition.\n\n\
                            ")
                        .style(Style::default().fg(Color::LightMagenta))
                        .render(t, &chunks[0]);
                        Paragraph::default()
                        .block(Block::default()
                            .borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan))
                            .title("Wortschatz Logger")
                            .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .wrap(true)
                        .text("\nWeb application aimed at helping people familiarize themselves with German articles through personal user accounts that allows\n\
                        word tracking/categorizing and interactive quizzes. Built with PHP Laravel Framework, PostgreSQL, JavaScript, jQuery, SASS.\n\n\
                            ")
                        .style(Style::default().fg(Color::LightMagenta))
                        .render(t, &chunks[1]);
                        Paragraph::default()
                        .block(Block::default()
                            .borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan))
                            .title("Der Die Das Game")
                            .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .wrap(true)
                        .text("\nHTML5 browser game built with the Phaser.io framework and fully programmed in JavaScript. \n\n\
                            ")
                        .style(Style::default().fg(Color::LightMagenta))
                        .render(t, &chunks[2]);
                });
                Block::default()
                    .borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan))
                    .title("Volunteer Work")
                    .title_style(Style::default().fg(Color::Magenta).modifier(Modifier::Bold))
                    .render(t, &chunks[1]);
                Group::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .sizes(&[Size::Percent(99)])
                    .render(t, &chunks[1], |t, chunks| {
                    Paragraph::default()
                    .block(Block::default()
                        .borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan))
                        .title("Rails Girls Berlin (2017 - current)")
                        .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                    .wrap(true)
                    .text("\nVolunteer as a coach for the Rails Girls Berlin Community, an organization aimed at mentoring and encouraging women\n\
                    with no programming experience to learn the full programming stack and gain practical experience by building their\n\
                    own Ruby on Rails app in a safe and welcoming space")
                    .style(Style::default().fg(Color::LightMagenta))
                    .render(t, &chunks[0]);
                });
                Block::default()
                    .borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan))
                    .title("Open-Source Contributions")
                    .title_style(Style::default().fg(Color::Magenta).modifier(Modifier::Bold))
                    .render(t, &chunks[2]);
                Group::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .sizes(&[Size::Percent(99)])
                    .render(t, &chunks[2], |t, chunks| {
                    Paragraph::default()
                    .block(Block::default()
                        .borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan))
                        .title("Prometheus (2017 - current)")
                        .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                    .wrap(true)
                    .text("\nOngoing contributions to a systems monitoring toolkit written in Go: add new default metric go_info to Go client library;\n\
                    document config options; add new features to command line utility; create custom node exporter for BME280 module; ...
                     ")
                    .style(Style::default().fg(Color::LightMagenta))
                    .render(t, &chunks[0]);
                });
    });
}

fn draw_objective(t: &mut Terminal<RawBackend>, area: &Rect) {
    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Percent(10), Size::Percent(80), Size::Percent(10)])
        .render(t, area, |t, chunks| {
                Group::default()
                .direction(Direction::Horizontal)
                .sizes(&[Size::Percent(10), Size::Percent(80), Size::Percent(10)])
                .render(t, &chunks[1], |t, chunks| {
                    Paragraph::default()
                    .block(Block::default()
                        .borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan))
                        .title("What I am looking for?")
                        .title_style(Style::default().fg(Color::Magenta).modifier(Modifier::Bold)))
                    .wrap(true)
                    .text("\n{mod=bold;fg=yellow I am always open to be part of a team that does interesting work. :)}\n\n\n\
                           My ideal role involves a combination of the following:\n\n\
                           \t* Solve interesting backend and infrastructure problems\n\
                           \t* Create and improve the tools used during the development\n\
                           \t* Maintain a highly performant and reliable system\n\
                           \t* Create and integrate APIs to expose and extend functionality\n\
                           \t* Documentation\n\
                           \t* Contribute to open source software\n\
                           \t* Opportunities to attend and speak at conferences\n\
                           \t* Maintain healthy work-life balance\n\
                           \t* Receive and give mentorshop\
                           ")
                    .style(Style::default().fg(Color::LightMagenta))
                    .render(t, &chunks[1]);
                });
        });
}
