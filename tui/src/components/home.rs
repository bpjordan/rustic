use super::{
    notes::Notes, time_entry::time_entry_container::TimeEntryContainer, top_bar::layout::TopBar,
    Component, Frame,
};
use crate::{
    action::{
        Action, TTAct,
        UIAct::{self, *},
    },
    api_client::models::{day::Day, FullState},
    config::Config,
};
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::*;
use tokio::sync::mpsc::UnboundedSender;

pub struct Home<'a> {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    // components
    top_bar: TopBar,
    time_entry_container: TimeEntryContainer,
    notes: Notes<'a>,
    // data
    full_state: FullState,
    current_day: Day,
}

impl Home<'_> {
    pub fn new(starting_state: FullState) -> Self {
        Self {
            command_tx: None,
            config: Config::default(),
            top_bar: TopBar::default(),
            time_entry_container: TimeEntryContainer::default(),
            notes: Notes::default(),
            full_state: starting_state,
            current_day: Day::get_current_day(),
        }
    }
}

impl Component for Home<'_> {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx.clone());
        self.top_bar.register_action_handler(tx.clone())?;
        self.time_entry_container
            .register_action_handler(tx.clone())?;
        self.notes.register_action_handler(tx)?;

        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::UI(ui_action) => match ui_action {
                UIAct::Tick => {}
                UIAct::Quit => {}
                _ => {}
            },
            Action::TT(tt_action) => match tt_action {
                TTAct::ChangeDay(day) => {
                    self.time_entry_container
                        .set_time_entries(self.full_state.get_time_entries_for_day(day));
                    self.current_day = day;
                }
                TTAct::UpdateNote(_new_note) => todo!(),
            },
        }
        Ok(None)
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        if self.notes.is_edit_mode {
            self.notes.handle_key_events(key)?;
        } else {
            if let KeyCode::Char('q') = key.code {
                return Ok(Some(Action::UI(Quit)));
            }
            self.top_bar.handle_key_events(key)?;
            self.time_entry_container.handle_key_events(key)?;
            self.notes.handle_key_events(key)?;
        }

        Ok(None)
    }
    fn draw(&mut self, f: &mut Frame<'_>, _area: Rect) -> Result<()> {
        let layout = Layout::new()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(5),
                Constraint::Percentage(70),
                Constraint::Percentage(20),
            ])
            .split(f.size());

        self.top_bar.draw(f, layout[0])?;
        self.time_entry_container.draw(f, layout[1])?;
        self.notes.draw(f, layout[2])?;

        Ok(())
    }
}
