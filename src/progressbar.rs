use std::{fmt::Display, ops::Mul, sync::{Arc, RwLock}, time::Duration};

use console::Term;

use crate::{term_write, theme::THEME, Theme, ThemeState};

#[derive(Debug, Clone)]
pub struct MultiLineProgressState {
    pub is_last: bool,
}

impl MultiLineProgressState {
    pub fn new(is_last: bool) -> Self {
        Self {
            is_last
        }
    }

    pub fn is_last(&self) -> bool {
        self.is_last
    }

    pub fn set_is_last(&mut self, is_last: bool) {
        self.is_last = is_last;
    }
}

/// A spinner + progressbar that renders progress indication using current/total
/// semantics. If you're looking for a download bar (or a bar that deals with
/// bytes and formatting of bytes/KB/MB/GB, etc.), see [`DownloadBar`](crate::DownloadBar).
///
/// Implemented via theming of [`indicatif::ProgressBar`](https://docs.rs/indicatif).
pub struct ProgressBar {
    pub(crate) progress_bar: RwLock<indicatif::ProgressBar>,
    kind: RwLock<ProgressBarKind>,
    multiline: RwLock<Option<MultiLineProgressState>>
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self {
            progress_bar: RwLock::new(indicatif::ProgressBar::new(100)),
            multiline: RwLock::new(None),
            kind: RwLock::new(ProgressBarKind::Progress)
        }
        .as_progressbar()
    }
}

impl ProgressBar {
    pub(crate) fn set_style(&self, style: indicatif::ProgressStyle) {
        self.get_bar().set_style(style);
    }

    pub(crate) fn is_last(&self) -> bool {
        self.multiline.read().unwrap().as_ref().unwrap().is_last
    }

    pub(crate) fn kind(&self) -> ProgressBarKind {
        self.kind.read().unwrap().clone()
    }

    fn get_bar(&self) -> std::sync::RwLockReadGuard<indicatif::ProgressBar> {
        self.progress_bar.read().unwrap()
    }

    fn set_kind(&self, kind: ProgressBarKind) {
        let mut k = self.kind.write().unwrap();
        *k = kind;
    }

    /// Starts the progressbar.
    pub fn start(&self, length: u64, message: impl Display) {
        let pb = self.get_bar();
        pb.set_length(length);
        pb.set_message(message.to_string());
    }

    pub fn increment(&self, delta: u64) {
        self.get_bar().inc(delta);
    }

    /// Stops the progressbar.
    pub fn stop(&self, message: impl Display)-> std::io::Result<()> {
        let theme = THEME.lock().unwrap();
        let pb = self.get_bar();

        if self.multiline.read().unwrap().is_none() {
            Term::stderr().move_cursor_up(1)?;
            pb.println(theme.format_progressbar_multi_stop(&message.to_string(), self.multiline.read().unwrap().as_ref().unwrap().is_last));
            pb.finish_and_clear();
        } else {
            pb.finish_and_clear();
            //TODO: CHANGE TO MULTI, this prints the symbol of the state
            //pb.println(theme.format_progressbar_with_state(&message.to_string(), &ThemeState::Submit)?);
            pb.println(theme.format_progressbar_multi_stop(&message.to_string(), self.multiline.read().unwrap().as_ref().unwrap().is_last));
        }
        
        Ok(())
    }

    /// Makes the progressbar stop with an error.
    pub fn error(&self, message: impl Display) -> std::io::Result<()> {
        let theme = THEME.lock().unwrap();
        let state = &ThemeState::Error("".into());
        let pb = self.get_bar();

        if self.multiline.read().unwrap().is_none() {
            Term::stderr().move_cursor_up(1)?;
            pb.println(theme.format_progressbar_with_state(&message.to_string(), state)?);
            pb.finish_and_clear();
        } else {
            pb.finish_and_clear();
            pb.println(theme.format_progressbar_with_state(&message.to_string(), state)?);
        }
        
        Ok(())
    }

    /// Cancel the progressbar (stop with cancelling style).
    pub fn cancel(&self, message: impl Display) -> std::io::Result<()> {
        let theme = THEME.lock().unwrap();
        let state = &ThemeState::Cancel;
        let pb = self.get_bar();

        if self.multiline.read().unwrap().is_none() {
            Term::stderr().move_cursor_up(1)?;
            // Workaround: the next line doesn't "jump" around while resizing the terminal.
            pb.println(theme.format_progressbar_with_state(&message.to_string(), state)?);
            pb.finish_and_clear();
        }

        
        Ok(())
    }

    /// Retrieves the current position of the progressbar.
    /// Note that this is _not_ the same as the current progress, which is
    /// `position / length`.
    pub fn get_position(&self) -> u64 {
        self.get_bar().position()
    }

    pub fn is_finished(&self) -> bool {
        self.get_bar().is_finished()
    }

    /// Sets the position of the progressbar.
    pub fn set_position(&self, position: u64) {
        self.get_bar().set_position(position);
    }

    /// Retrieves the length of the progressbar. This is the total number of
    /// steps, bytes, etc. and is used to calculate the progress, which is
    /// `position / length`.
    pub fn get_length(&self) -> u64 {
        self.get_bar().length().unwrap()
    }

    /// Sets the length of the progressbar. This is the total number of steps,
    /// bytes, etc. and is used to calculate the progress, which is
    /// `position / length`.
    pub fn set_length(&self, length: u64) {
        self.get_bar().set_length(length);
    }

    /// Formats the progressbar as a progressbar, using steps as the unit (i.e.
    /// 1/25, 2/25, etc.).
    pub fn as_progressbar(self) -> Self {
        self.format_as_progressbar();
        self
    }

    fn format_as_progressbar(&self) {
        let theme = THEME.lock().unwrap();
        let pb = self.get_bar();
        self.set_kind(ProgressBarKind::Progress);

        pb.enable_steady_tick(Duration::from_millis(100));
        pb.set_style(theme.format_progressbar_start());
    }

    fn format_as_progressbar_multi(&self, theme: &Box<dyn Theme + Send + Sync + 'static>) {
        //let theme = THEME.lock().unwrap();
        let pb = self.get_bar();
        self.set_kind(ProgressBarKind::Progress);

        pb.enable_steady_tick(Duration::from_millis(100));
        pb.set_style(theme.multiprogress_template(self));
    }

    /// Formats the progressbar as a download bar, using bytes as the unit (i.e.
    /// 1.2MB/5.0MB, etc.).
    pub fn as_downloadbar(self) -> Self {
        self.format_as_downloadbar();
        self
    }

    /// TODO: Doc me
    fn format_as_downloadbar(&self) {
        let theme = THEME.lock().unwrap();
        let pb = self.get_bar();
        self.set_kind(ProgressBarKind::Download);

        pb.enable_steady_tick(Duration::from_millis(100));
        pb.set_style(theme.format_downloadbar_start());
    }
}

pub struct ProgressBarWrapper<'a> {
    progress_bar: Arc<ProgressBar>,
    multi_progress_bar: &'a MultiProgressBar,
    is_last: bool
}

impl<'a> ProgressBarWrapper<'a> {
    pub fn new(progress_bar: Arc<ProgressBar>, multi_progress_bar: &'a MultiProgressBar) -> Self {
        Self {
            progress_bar,
            multi_progress_bar,
            is_last: true
        }
    }
}

impl ProgressBarWrapper<'_> {
    pub fn start(&self, length: u64, message: impl Display) {
        self.multi_progress_bar.progress_bars
            .write()
            .unwrap()
            .push(self.progress_bar.clone());
        self.progress_bar.start(length, message);
    }

    pub fn increment(&self, delta: u64) {
        self.progress_bar.increment(delta);
    }

    pub fn stop(self, message: impl Display) -> std::io::Result<Self> {
        self.progress_bar.stop(message)?;
        self.multi_progress_bar.remove(self.progress_bar.clone());
        Ok(self)
    }

    pub fn error(&self, message: impl Display) -> std::io::Result<()> {
        self.progress_bar.error(message)
    }

    pub fn cancel(&self, message: impl Display) -> std::io::Result<()> {
        self.progress_bar.cancel(message)
    }

    pub fn get_position(&self) -> u64 {
        self.progress_bar.get_position()
    }

    pub fn is_finished(&self) -> bool {
        self.progress_bar.is_finished()
    }

    pub fn set_position(&self, position: u64) {
        self.progress_bar.set_position(position);
    }

    pub fn get_length(&self) -> u64 {
        self.progress_bar.get_length()
    }

    pub fn set_length(&self, length: u64) {
        self.progress_bar.set_length(length);
    }
}

pub struct MultiProgressBar {
    multi_progress_bar: indicatif::MultiProgress,
    progress_bars: RwLock<Vec<Arc<ProgressBar>>>,
    //heading: String
}

impl MultiProgressBar {
    pub fn new(heading: &str) -> Self {
        let this = Self {
            multi_progress_bar: indicatif::MultiProgress::new(),
            progress_bars: RwLock::new(Vec::new()),
            //heading: heading.to_string()
        };

        let theme = THEME.lock().unwrap();

        term_write(theme.format_multiprogress_start(heading))
            .expect("Failed to write multi-progress heading.");
        this
    }

    pub fn add_progressbar(&self) -> ProgressBarWrapper {
        let theme = THEME.lock().unwrap();

        let indicatif_pb = self.multi_progress_bar.add(
            indicatif::ProgressBar::new(100)
        );

        let pb = Arc::new(ProgressBar {
            progress_bar: RwLock::new(indicatif_pb),
            multiline: RwLock::new(Some(MultiLineProgressState::new(true))),
            kind: ProgressBarKind::Progress.into()
        });

        for bar in self.progress_bars.write().unwrap().iter_mut() {
            bar.multiline.write().unwrap().as_mut().unwrap().set_is_last(false);
            bar.format_as_progressbar_multi(&*theme);
        }
        pb.set_style(theme.multiprogress_template(&pb));
        self.progress_bars.write().unwrap().push(pb.clone());

        ProgressBarWrapper::new(pb.clone(), self)
    }

    pub fn add_downloadbar(&self) -> ProgressBarWrapper {
        let theme = THEME.lock().unwrap();

        let indicatif_pb = self.multi_progress_bar.add(
            indicatif::ProgressBar::new(100)
        );

        let pb = Arc::new(ProgressBar {
            progress_bar: RwLock::new(indicatif_pb),
            multiline: RwLock::new(Some(MultiLineProgressState::new(true))),
            kind: ProgressBarKind::Download.into()
        });

        for bar in self.progress_bars.read().unwrap().iter() {
            bar.multiline.write().unwrap().as_mut().unwrap().set_is_last(false);
            bar.format_as_progressbar_multi(&*theme);
        }
        pb.set_style(theme.multiprogress_template(&pb));
        self.progress_bars.write().unwrap().push(pb.clone());

        ProgressBarWrapper::new(pb.clone(), self)
    }

    pub fn add_spinner(&self) -> ProgressBarWrapper {
        let indicatif_pb = self.multi_progress_bar.add(
            indicatif::ProgressBar::new_spinner()
        );

        let pb = Arc::new(ProgressBar {
            progress_bar: RwLock::new(indicatif_pb),
            multiline: RwLock::new(None),
            kind: ProgressBarKind::Progress.into()
        });

        self.progress_bars.write().unwrap().push(pb.clone());

        ProgressBarWrapper::new(pb.clone(), self)
    }

    fn remove(&self, pb: Arc<ProgressBar>) {
        let mut bars = self.progress_bars.write().unwrap();
        bars.retain(|bar| !Arc::ptr_eq(&pb, bar));
    }
}

#[derive(Debug, Clone)]
pub enum ProgressBarKind {
    Progress,
    Download,
    Spinner
}